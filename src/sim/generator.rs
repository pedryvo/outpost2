//! Deterministic Procedural Skirmish Map Generator & Terrain Topology Subsystem
//!
//! Generates balanced skirmish maps with natural choke points, canyon corridors,
//! plateaus, mineral deposits, geothermal vents, and multiplayer symmetry.
#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};
use serde::{Deserialize, Serialize};

use crate::sim::map::{GameMap, MapTile};
use crate::sim::mining::{OreDeposit, OreType};

/// Terrain biomes for procedural map generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainBiome {
    /// Molten volcanic basin with basalt flats, geothermal hot spots, and magma fissures
    VolcanicBasin,
    /// Rugged canyon corridors, iron-oxide plateaus, and steep sandstone ridges
    CanyonsAndCrags,
    /// Barren wasteland with fractured fissures and microbial blight infestation pockets
    BlightedBadlands,
    /// Dark obsidian spires, razor ridges, and high-yield contested mineral valleys
    ObsidianRidge,
}

/// Resource abundance multiplier for procedural map generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OreAbundance {
    /// Restricted resources: smaller veins and lower reserves
    Scarce,
    /// Standard Outpost 2 skirmish reserves
    Normal,
    /// Rich mineral caches with high extraction capacities
    Abundant,
}

/// Symmetry mode for competitive multiplayer and skirmish balance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapSymmetry {
    /// Asymmetric organic terrain
    None,
    /// 180-degree rotational symmetry around map center (standard 2-player / 2-team)
    Rotational2Way,
    /// 90-degree rotational symmetry around map center (standard 4-player quadrant)
    Rotational4Way,
    /// Bilateral mirror reflection across the central vertical axis
    MirroredX,
}

/// Configuration parameters for the procedural map generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapGeneratorConfig {
    /// Map width in tiles (e.g. 64, 96, 128)
    pub width: usize,
    /// Map height in tiles (e.g. 64, 96, 128)
    pub height: usize,
    /// Environmental terrain biome
    pub biome: TerrainBiome,
    /// Mineral beacon abundance
    pub ore_abundance: OreAbundance,
    /// Number of hydrothermal fumaroles for Geothermal Power Plants
    pub geothermal_vents: usize,
    /// Geometric layout symmetry
    pub symmetry: MapSymmetry,
    /// Random seed for deterministic multi-client lockstep synchronization
    pub seed: u64,
    /// Number of player spawn points (2 to 6)
    pub player_count: usize,
}

impl Default for MapGeneratorConfig {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            biome: TerrainBiome::CanyonsAndCrags,
            ore_abundance: OreAbundance::Normal,
            geothermal_vents: 4,
            symmetry: MapSymmetry::Rotational2Way,
            seed: 42,
            player_count: 2,
        }
    }
}

/// Type alias for GameMap representing the tile grid
pub type TileMap = GameMap;

/// Result of procedural map generation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedMap {
    /// Generated tile grid
    pub tile_map: TileMap,
    /// Player starting base coordinates (top-left of 6x6 base footprint)
    pub spawn_points: Vec<(i32, i32)>,
    /// Symmetrically balanced common and rare ore deposits
    pub ore_deposits: Vec<OreDeposit>,
    /// Hydrothermal fumaroles tagged for Geothermal Power Plants
    pub geothermal_vents: Vec<(i32, i32)>,
    /// Biome environment
    pub biome: TerrainBiome,
}

/// Deterministic 64-bit pseudo-random number generator (SplitMix64)
#[derive(Debug, Clone)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    /// Initialize PRNG from seed
    pub fn new(seed: u64) -> Self {
        let mut rng = Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        };
        // Warm up state
        rng.next_u64();
        rng
    }

    /// Generate next 64-bit unsigned integer
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    /// Generate float in [0.0, 1.0)
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Generate integer in [min, max]
    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        let span = (max - min + 1) as u64;
        min + (self.next_u64() % span) as i32
    }

    /// Generate usize in [min, max]
    pub fn range_usize(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        let span = (max - min + 1) as u64;
        min + (self.next_u64() % span) as usize
    }
}

/// Quintic S-curve smoothing function: 6t^5 - 15t^4 + 10t^3
#[inline]
fn smoothstep5(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// 2D hash function for deterministic gradient/value noise using SplitMix64 constants
#[inline]
fn hash2d(x: i32, y: i32, seed: u64) -> f64 {
    let mut h = seed
        .wrapping_add((x as u64).wrapping_mul(0x9e3779b97f4a7c15))
        .wrapping_add((y as u64).wrapping_mul(0xbf58476d1ce4e5b9));
    h = (h ^ (h >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94d049bb133111eb);
    h ^= h >> 31;
    (h >> 11) as f64 / ((1u64 << 53) as f64)
}

/// Smooth 2D interpolated value noise in [0.0, 1.0]
fn sample_value_noise2d(x: f64, y: f64, seed: u64) -> f64 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    let fx = smoothstep5(x - x.floor());
    let fy = smoothstep5(y - y.floor());

    let v00 = hash2d(x0, y0, seed);
    let v10 = hash2d(x1, y0, seed);
    let v01 = hash2d(x0, y1, seed);
    let v11 = hash2d(x1, y1, seed);

    let top = v00 + (v10 - v00) * fx;
    let bottom = v01 + (v11 - v01) * fx;

    top + (bottom - top) * fy
}

/// Fractal Brownian Motion (fBm) multi-octave noise generator
fn sample_fbm(
    x: f64,
    y: f64,
    seed: u64,
    octaves: usize,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
) -> f64 {
    let mut total = 0.0;
    let mut amp = 1.0;
    let mut freq = frequency;
    let mut max_amp = 0.0;

    for i in 0..octaves {
        let octave_seed = seed.wrapping_add((i as u64).wrapping_mul(1013904223));
        total += sample_value_noise2d(x * freq, y * freq, octave_seed) * amp;
        max_amp += amp;
        amp *= persistence;
        freq *= lacunarity;
    }

    if max_amp > 0.0 {
        total / max_amp
    } else {
        total
    }
}

/// Procedural Skirmish Map Generator engine
pub struct MapGenerator;

impl MapGenerator {
    /// Generate a complete skirmish map from the given configuration
    pub fn generate(config: &MapGeneratorConfig) -> GeneratedMap {
        let w = config.width.max(32);
        let h = config.height.max(32);
        let player_count = config.player_count.clamp(2, 6);

        let mut rng = SeededRng::new(config.seed);

        // 1. Calculate symmetrical player spawn points
        let spawn_points = Self::compute_spawn_points(w, h, player_count, config.symmetry, &mut rng);

        // 2. Generate elevation heightmap with symmetry enforcement
        let mut elevation = Self::generate_heightmap(w, h, config.symmetry, config.seed);

        // 3. Carve central contested valley and canyon corridors between bases
        Self::carve_valleys_and_corridors(w, h, &spawn_points, &mut elevation, config.symmetry);

        // 4. Flatten guaranteed 6x6 base footprints for all spawn sites
        for &(sx, sy) in &spawn_points {
            Self::clear_base_footprint(w, h, sx, sy, &mut elevation);
        }

        // 5. Construct initial GameMap from heightmap and biome rules
        let mut tile_map = Self::build_tile_map(w, h, &elevation, config.biome, config.seed, &spawn_points);

        // 6. Distribute common and rare mineral deposits
        let ore_deposits = Self::place_mineral_deposits(
            w,
            h,
            &spawn_points,
            config.ore_abundance,
            config.symmetry,
            &mut tile_map,
            &mut rng,
        );

        // 7. Place hydrothermal fumaroles for Geothermal energy extraction
        let geothermal_vents = Self::place_geothermal_vents(
            w,
            h,
            &spawn_points,
            config.geothermal_vents,
            config.symmetry,
            &mut tile_map,
            &mut rng,
        );

        // 8. Verify and guarantee vehicle navigation connectivity across all bases and center
        Self::ensure_connectivity(w, h, &spawn_points, &mut tile_map);

        GeneratedMap {
            tile_map,
            spawn_points,
            ore_deposits,
            geothermal_vents,
            biome: config.biome,
        }
    }

    /// Compute symmetrical spawn point coordinates ensuring sufficient margin
    fn compute_spawn_points(
        width: usize,
        height: usize,
        player_count: usize,
        symmetry: MapSymmetry,
        _rng: &mut SeededRng,
    ) -> Vec<(i32, i32)> {
        let w = width as i32;
        let h = height as i32;
        let cx = w / 2;
        let cy = h / 2;

        let margin = 8;
        let radius = ((w.min(h) as f64) * 0.32).clamp(10.0, (w.min(h) / 2 - margin) as f64);

        let mut spawns = Vec::new();

        match symmetry {
            MapSymmetry::Rotational2Way => {
                if player_count == 2 {
                    // Two players opposite each other across center
                    let dx = (radius * 0.7071).round() as i32;
                    let dy = (radius * 0.7071).round() as i32;
                    let p0 = (cx - dx - 3, cy - dy - 3);
                    let p1 = (w - 1 - (p0.0 + 5), h - 1 - (p0.1 + 5));
                    spawns.push(p0);
                    spawns.push(p1);
                } else {
                    // Even distribution with pairs 180 degrees apart
                    let half = player_count / 2;
                    let step = std::f64::consts::PI / (half as f64);
                    for i in 0..half {
                        let angle = std::f64::consts::PI * 0.25 + (i as f64) * step;
                        let dx = (radius * angle.cos()).round() as i32;
                        let dy = (radius * angle.sin()).round() as i32;
                        let p_a = (cx + dx - 3, cy + dy - 3);
                        let p_b = (w - 1 - (p_a.0 + 5), h - 1 - (p_a.1 + 5));
                        spawns.push(p_a);
                        spawns.push(p_b);
                    }
                    if spawns.len() < player_count {
                        let p_extra = (cx - 3, margin);
                        spawns.push(p_extra);
                    }
                }
            }
            MapSymmetry::Rotational4Way => {
                let dx = (radius * 0.7071).round() as i32;
                let dy = (radius * 0.7071).round() as i32;

                let p0 = (cx - dx - 3, cy - dy - 3);
                let p1 = (w - 1 - (p0.0 + 5), p0.1);
                let p2 = (w - 1 - (p0.0 + 5), h - 1 - (p0.1 + 5));
                let p3 = (p0.0, h - 1 - (p0.1 + 5));

                spawns.push(p0);
                spawns.push(p1);
                if player_count >= 3 {
                    spawns.push(p2);
                }
                if player_count >= 4 {
                    spawns.push(p3);
                }
                if player_count > 4 {
                    // Fill remaining spawns on cardinal axes
                    let extra1 = (cx - 3, margin);
                    let extra2 = (cx - 3, h - margin - 6);
                    spawns.push(extra1);
                    if spawns.len() < player_count {
                        spawns.push(extra2);
                    }
                }
            }
            MapSymmetry::MirroredX => {
                let dx = (radius * 0.85).round() as i32;
                if player_count == 2 {
                    let p0 = (cx - dx - 3, cy - 3);
                    let p1 = (w - 1 - (p0.0 + 5), cy - 3);
                    spawns.push(p0);
                    spawns.push(p1);
                } else {
                    let step_y = ((h - margin * 2 - 12) / ((player_count / 2).max(1) as i32)).max(8);
                    let half = player_count / 2;
                    for i in 0..half {
                        let py = margin + (i as i32) * step_y;
                        let p0 = (cx - dx - 3, py);
                        let p1 = (w - 1 - (p0.0 + 5), py);
                        spawns.push(p0);
                        spawns.push(p1);
                    }
                    if spawns.len() < player_count {
                        spawns.push((cx - 3, h - margin - 6));
                    }
                }
            }
            MapSymmetry::None => {
                let step = std::f64::consts::TAU / (player_count as f64);
                let base_angle = std::f64::consts::PI * 0.25;
                for i in 0..player_count {
                    let angle = base_angle + (i as f64) * step;
                    let dx = (radius * angle.cos()).round() as i32;
                    let dy = (radius * angle.sin()).round() as i32;
                    spawns.push((cx + dx - 3, cy + dy - 3));
                }
            }
        }

        // Clamp all spawn points strictly inside valid playable territory with 6x6 footprint
        for p in &mut spawns {
            p.0 = p.0.clamp(margin, w - margin - 6);
            p.1 = p.1.clamp(margin, h - margin - 6);
        }

        spawns
    }

    /// Generate an elevation heightmap in [0.0, 1.0] with multi-octave fBm and symmetry
    fn generate_heightmap(
        width: usize,
        height: usize,
        symmetry: MapSymmetry,
        seed: u64,
    ) -> Vec<f64> {
        let mut raw = vec![0.0; width * height];
        let freq = 0.045;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                raw[idx] = sample_fbm(x as f64, y as f64, seed, 4, freq, 0.5, 2.0);
            }
        }

        // Enforce geometric symmetry on heightmap
        let mut elevation = vec![0.0; width * height];
        for y in 0..height {
            for x in 0..width {
                let val = match symmetry {
                    MapSymmetry::Rotational2Way => {
                        let opp_x = width - 1 - x;
                        let opp_y = height - 1 - y;
                        0.5 * (raw[y * width + x] + raw[opp_y * width + opp_x])
                    }
                    MapSymmetry::Rotational4Way => {
                        let opp_x = width - 1 - x;
                        let opp_y = height - 1 - y;
                        let rot90_x = y.min(width - 1);
                        let rot90_y = (width - 1 - x).min(height - 1);
                        let rot270_x = (height - 1 - y).min(width - 1);
                        let rot270_y = x.min(height - 1);

                        0.25 * (raw[y * width + x]
                            + raw[opp_y * width + opp_x]
                            + raw[rot90_y * width + rot90_x]
                            + raw[rot270_y * width + rot270_x])
                    }
                    MapSymmetry::MirroredX => {
                        let opp_x = width - 1 - x;
                        0.5 * (raw[y * width + x] + raw[y * width + opp_x])
                    }
                    MapSymmetry::None => raw[y * width + x],
                };
                elevation[y * width + x] = val;
            }
        }

        elevation
    }

    /// Carve central valley and canyon corridors connecting bases to center and each other
    fn carve_valleys_and_corridors(
        width: usize,
        height: usize,
        spawn_points: &[(i32, i32)],
        elevation: &mut [f64],
        symmetry: MapSymmetry,
    ) {
        let cx = (width / 2) as i32;
        let cy = (height / 2) as i32;

        // 1. Carve central contested valley (radius 6-8 tiles)
        let valley_radius = (width.min(height) as f64 * 0.12).clamp(5.0, 9.0);
        for y in 0..height {
            for x in 0..width {
                let dx = (x as i32 - cx) as f64;
                let dy = (y as i32 - cy) as f64;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= valley_radius {
                    let factor = (dist / valley_radius).clamp(0.0, 1.0);
                    let target_elev = 0.25 + 0.10 * factor;
                    let idx = y * width + x;
                    if elevation[idx] > target_elev {
                        elevation[idx] = target_elev;
                    }
                }
            }
        }

        // 2. Carve corridors from each spawn base center to map center
        for &(sx, sy) in spawn_points {
            let base_cx = sx + 3;
            let base_cy = sy + 3;
            Self::carve_line_corridor(width, height, base_cx, base_cy, cx, cy, 2, elevation);
        }

        // 3. Carve flank routes between adjacent bases
        let n = spawn_points.len();
        if n >= 2 {
            for i in 0..n {
                let p1 = spawn_points[i];
                let p2 = spawn_points[(i + 1) % n];
                let mid_x = (p1.0 + 3 + p2.0 + 3) / 2;
                let mid_y = (p1.1 + 3 + p2.1 + 3) / 2;
                // Carve through perimeter midpoint
                Self::carve_line_corridor(width, height, p1.0 + 3, p1.1 + 3, mid_x, mid_y, 1, elevation);
                Self::carve_line_corridor(width, height, mid_x, mid_y, p2.0 + 3, p2.1 + 3, 1, elevation);
            }
        }

        // Re-enforce heightmap symmetry after corridor carving
        if symmetry != MapSymmetry::None {
            for y in 0..height {
                for x in 0..width {
                    let idx = y * width + x;
                    match symmetry {
                        MapSymmetry::Rotational2Way => {
                            let opp_idx = (height - 1 - y) * width + (width - 1 - x);
                            let min_val = elevation[idx].min(elevation[opp_idx]);
                            elevation[idx] = min_val;
                            elevation[opp_idx] = min_val;
                        }
                        MapSymmetry::Rotational4Way => {
                            let opp_idx = (height - 1 - y) * width + (width - 1 - x);
                            let r90_idx = ((width - 1 - x).min(height - 1)) * width + y.min(width - 1);
                            let r270_idx = (x.min(height - 1)) * width + (height - 1 - y).min(width - 1);
                            let min_val = elevation[idx]
                                .min(elevation[opp_idx])
                                .min(elevation[r90_idx])
                                .min(elevation[r270_idx]);
                            elevation[idx] = min_val;
                            elevation[opp_idx] = min_val;
                            elevation[r90_idx] = min_val;
                            elevation[r270_idx] = min_val;
                        }
                        MapSymmetry::MirroredX => {
                            let opp_idx = y * width + (width - 1 - x);
                            let min_val = elevation[idx].min(elevation[opp_idx]);
                            elevation[idx] = min_val;
                            elevation[opp_idx] = min_val;
                        }
                        MapSymmetry::None => {}
                    }
                }
            }
        }
    }

    /// Carve a passable corridor along a line between (x0, y0) and (x1, y1) with radius r
    fn carve_line_corridor(
        width: usize,
        height: usize,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        radius: i32,
        elevation: &mut [f64],
    ) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut cur_x = x0;
        let mut cur_y = y0;

        loop {
            // Lower elevation in radius around current tile
            for oy in -radius..=radius {
                for ox in -radius..=radius {
                    if ox * ox + oy * oy <= radius * radius + 1 {
                        let tx = cur_x + ox;
                        let ty = cur_y + oy;
                        if tx >= 0 && tx < width as i32 && ty >= 0 && ty < height as i32 {
                            let idx = (ty as usize) * width + (tx as usize);
                            if elevation[idx] > 0.35 {
                                elevation[idx] = 0.35;
                            }
                        }
                    }
                }
            }

            if cur_x == x1 && cur_y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                cur_x += sx;
            }
            if e2 <= dx {
                err += dx;
                cur_y += sy;
            }
        }
    }

    /// Guarantee a clean, flat 6x6 footprint (with 1-tile safety margin) at base spawn point
    fn clear_base_footprint(
        width: usize,
        height: usize,
        sx: i32,
        sy: i32,
        elevation: &mut [f64],
    ) {
        for dy in -1..=7 {
            for dx in -1..=7 {
                let tx = sx + dx;
                let ty = sy + dy;
                if tx >= 0 && tx < width as i32 && ty >= 0 && ty < height as i32 {
                    let idx = (ty as usize) * width + (tx as usize);
                    elevation[idx] = 0.28; // Completely flat, passable base level
                }
            }
        }
    }

    /// Build TileMap cells from elevation and biome rules
    fn build_tile_map(
        width: usize,
        height: usize,
        elevation: &[f64],
        biome: TerrainBiome,
        seed: u64,
        spawn_points: &[(i32, i32)],
    ) -> GameMap {
        let total = width * height;
        let mut tiles = Vec::with_capacity(total);

        // Precompute base footprints to protect them from obstacles/hazards
        let mut base_protected = vec![false; total];
        for &(sx, sy) in spawn_points {
            for dy in -1..=7 {
                for dx in -1..=7 {
                    let tx = sx + dx;
                    let ty = sy + dy;
                    if tx >= 0 && tx < width as i32 && ty >= 0 && ty < height as i32 {
                        base_protected[(ty as usize) * width + (tx as usize)] = true;
                    }
                }
            }
        }

        // Noise for micro-surface variation and localized hazards
        let mut micro_rng = SeededRng::new(seed.wrapping_add(987654321));

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let elev = elevation[idx];
                let is_protected = base_protected[idx];

                let mut tile = MapTile::default();

                if is_protected {
                    // Guaranteed clean flat ground for base construction
                    tile.cell_type = match biome {
                        TerrainBiome::VolcanicBasin => 0, // Basalt rock
                        TerrainBiome::CanyonsAndCrags => 1, // Iron-oxide dust
                        TerrainBiome::BlightedBadlands => 0, // Clean colony pad
                        TerrainBiome::ObsidianRidge => 0, // Smooth basalt
                    };
                    tile.is_wall_or_building = false;
                    tile.is_lava = false;
                    tile.is_lava_possible = false;
                    tile.is_blight = false;
                    tile.tile_mapping = ((x + y * 3) % 16) as u16;
                } else {
                    // Check local gradient to identify steep cliff walls
                    let mut max_grad = 0.0f64;
                    for &(ox, oy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let nx = x as i32 + ox;
                        let ny = y as i32 + oy;
                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let n_idx = (ny as usize) * width + (nx as usize);
                            let diff = (elevation[n_idx] - elev).abs();
                            if diff > max_grad {
                                max_grad = diff;
                            }
                        }
                    }

                    // Impassable cliff threshold: high elevation or steep gradient
                    let is_cliff = elev >= 0.68 || (elev >= 0.48 && max_grad >= 0.16);

                    if is_cliff {
                        tile.cell_type = 3; // Impassable ridge / cliff face
                        tile.is_wall_or_building = true;
                        tile.tile_mapping = (16 + ((x * 5 + y * 7) % 12)) as u16;
                    } else if elev < 0.32 {
                        // Low canyon floor / valley
                        tile.cell_type = 0; // Basalt / canyon road
                        tile.is_wall_or_building = false;
                        tile.tile_mapping = ((x * 3 + y * 11) % 8) as u16;
                    } else if elev < 0.50 {
                        // Mid-level plains
                        tile.cell_type = 1; // Iron-oxide dust
                        tile.is_wall_or_building = false;
                        tile.tile_mapping = (8 + ((x * 7 + y * 3) % 8)) as u16;
                    } else {
                        // Highland transition slope
                        tile.cell_type = 2; // Highland clay / rough slope
                        tile.is_wall_or_building = false;
                        tile.tile_mapping = (24 + ((x * 2 + y * 5) % 8)) as u16;
                    }

                    // Apply biome environmental hazards in secluded non-corridor areas
                    match biome {
                        TerrainBiome::VolcanicBasin => {
                            // Localized magma fissures in high thermal zones
                            if !tile.is_wall_or_building && elev < 0.28 {
                                let noise_val = sample_value_noise2d(
                                    x as f64 * 0.15,
                                    y as f64 * 0.15,
                                    seed.wrapping_add(555),
                                );
                                if noise_val > 0.82 {
                                    tile.cell_type = 4; // Liquid fissure
                                    tile.is_lava = true;
                                    tile.is_lava_possible = true;
                                } else if noise_val > 0.70 {
                                    tile.is_lava_possible = true;
                                }
                            }
                        }
                        TerrainBiome::BlightedBadlands => {
                            // Alien microbial blight infesting remote badland sectors
                            if !tile.is_wall_or_building && elev > 0.40 && elev < 0.60 {
                                let noise_val = sample_value_noise2d(
                                    x as f64 * 0.12,
                                    y as f64 * 0.12,
                                    seed.wrapping_add(777),
                                );
                                if noise_val > 0.80 {
                                    tile.cell_type = 5; // Microbial badlands
                                    tile.is_blight = true;
                                }
                            }
                        }
                        TerrainBiome::ObsidianRidge => {
                            if tile.cell_type == 1 && micro_rng.next_f64() < 0.15 {
                                tile.cell_type = 2; // Rough obsidian rock
                            }
                        }
                        TerrainBiome::CanyonsAndCrags => {
                            // Pure tactical sandstone and cliffs
                        }
                    }
                }

                tiles.push(tile);
            }
        }

        GameMap::new(width as u32, height as u32, tiles)
    }

    /// Place balanced local common/rare ore veins near bases and high-yield contested mineral caches in central valley
    fn place_mineral_deposits(
        width: usize,
        height: usize,
        spawn_points: &[(i32, i32)],
        abundance: OreAbundance,
        symmetry: MapSymmetry,
        tile_map: &mut GameMap,
        _rng: &mut SeededRng,
    ) -> Vec<OreDeposit> {
        let (local_common, local_rare, contested_common, contested_rare) = match abundance {
            OreAbundance::Scarce => (25_000, 15_000, 50_000, 30_000),
            OreAbundance::Normal => (50_000, 25_000, 80_000, 45_000),
            OreAbundance::Abundant => (90_000, 50_000, 120_000, 75_000),
        };

        let mut deposits = Vec::new();
        let mut deposit_id = 1u32;
        let mut occupied_tiles = HashSet::new();

        let cx = (width / 2) as i32;
        let cy = (height / 2) as i32;

        // 1. Symmetrical local Common and Rare deposits near each player base
        // Standard offset relative to spawn point: 7 tiles away from top-left
        let common_offset = (7, 1);
        let rare_offset = (1, 7);

        for &(sx, sy) in spawn_points {
            // Determine direction vector towards center to place ore along logistical base line
            let dir_x = if sx < cx { 1 } else { -1 };
            let dir_y = if sy < cy { 1 } else { -1 };

            let c_x = (sx + 3 + dir_x * common_offset.0).clamp(2, width as i32 - 3);
            let c_y = (sy + 3 + dir_y * common_offset.1).clamp(2, height as i32 - 3);

            let r_x = (sx + 3 + dir_x * rare_offset.0).clamp(2, width as i32 - 3);
            let r_y = (sy + 3 + dir_y * rare_offset.1).clamp(2, height as i32 - 3);

            // Place Common Ore deposit
            let mut dep_c = OreDeposit::new(deposit_id, OreType::Common, c_x, c_y, local_common);
            dep_c.yield_per_cycle = 50;
            deposits.push(dep_c);
            occupied_tiles.insert((c_x, c_y));
            deposit_id += 1;
            Self::clear_deposit_tile(tile_map, c_x, c_y);

            // Place Rare Ore deposit
            let mut dep_r = OreDeposit::new(deposit_id, OreType::Rare, r_x, r_y, local_rare);
            dep_r.yield_per_cycle = 45;
            deposits.push(dep_r);
            occupied_tiles.insert((r_x, r_y));
            deposit_id += 1;
            Self::clear_deposit_tile(tile_map, r_x, r_y);
        }

        // 2. High-yield contested mineral caches in central valley
        let contested_offsets = match symmetry {
            MapSymmetry::Rotational4Way => vec![
                (cx - 4, cy - 4, OreType::Common),
                (cx + 4, cy - 4, OreType::Rare),
                (cx + 4, cy + 4, OreType::Common),
                (cx - 4, cy + 4, OreType::Rare),
            ],
            MapSymmetry::MirroredX => vec![
                (cx - 5, cy, OreType::Common),
                (cx + 5, cy, OreType::Common),
                (cx - 3, cy - 3, OreType::Rare),
                (cx + 3, cy - 3, OreType::Rare),
            ],
            _ => vec![
                (cx - 4, cy - 3, OreType::Common),
                (cx + 4, cy + 3, OreType::Common),
                (cx + 3, cy - 4, OreType::Rare),
                (cx - 3, cy + 4, OreType::Rare),
            ],
        };

        for (tx, ty, ore_type) in contested_offsets {
            if !occupied_tiles.contains(&(tx, ty)) && tile_map.in_bounds(tx, ty) {
                let amount = if ore_type == OreType::Common {
                    contested_common
                } else {
                    contested_rare
                };
                let mut cache = OreDeposit::new(deposit_id, ore_type, tx, ty, amount);
                cache.yield_per_cycle = 75; // Rich extraction rate in central valley
                deposits.push(cache);
                occupied_tiles.insert((tx, ty));
                deposit_id += 1;
                Self::clear_deposit_tile(tile_map, tx, ty);
            }
        }

        deposits
    }

    /// Clear deposit tile and ensure it is passable
    fn clear_deposit_tile(tile_map: &mut GameMap, x: i32, y: i32) {
        if let Some(t) = tile_map.get_tile_mut(x as u32, y as u32) {
            t.is_wall_or_building = false;
            t.is_lava = false;
            t.is_lava_possible = false;
            t.cell_type = 2; // Highland clay / mineral bed
        }
    }

    /// Place hydrothermal fumaroles tagged for Geothermal Power Plants
    fn place_geothermal_vents(
        width: usize,
        height: usize,
        spawn_points: &[(i32, i32)],
        vent_count: usize,
        symmetry: MapSymmetry,
        tile_map: &mut GameMap,
        _rng: &mut SeededRng,
    ) -> Vec<(i32, i32)> {
        if vent_count == 0 {
            return Vec::new();
        }

        let cx = (width / 2) as i32;
        let cy = (height / 2) as i32;
        let mut vents = Vec::new();

        // 1. Give each player a local geothermal fumarole if count permits
        let local_alloc = vent_count.min(spawn_points.len());
        for i in 0..local_alloc {
            let (sx, sy) = spawn_points[i];
            let dir_x = if sx < cx { 1 } else { -1 };
            let dir_y = if sy < cy { 1 } else { -1 };

            // Place fumarole 8 tiles diagonal from base footprint
            let vx = (sx + 3 + dir_x * 6).clamp(3, width as i32 - 4);
            let vy = (sy + 3 + dir_y * 6).clamp(3, height as i32 - 4);

            vents.push((vx, vy));
            Self::clear_geothermal_footprint(tile_map, vx, vy);
        }

        // 2. Place remaining vents in central contested volcanic fault lines
        let remaining = vent_count.saturating_sub(local_alloc);
        if remaining > 0 {
            let central_candidates = match symmetry {
                MapSymmetry::Rotational4Way => vec![
                    (cx, cy - 6),
                    (cx + 6, cy),
                    (cx, cy + 6),
                    (cx - 6, cy),
                ],
                MapSymmetry::MirroredX => vec![
                    (cx - 6, cy - 5),
                    (cx + 6, cy - 5),
                    (cx - 6, cy + 5),
                    (cx + 6, cy + 5),
                ],
                _ => vec![
                    (cx - 5, cy),
                    (cx + 5, cy),
                    (cx, cy - 5),
                    (cx, cy + 5),
                ],
            };

            for (vx, vy) in central_candidates.into_iter().take(remaining) {
                if tile_map.in_bounds(vx, vy) && !vents.contains(&(vx, vy)) {
                    vents.push((vx, vy));
                    Self::clear_geothermal_footprint(tile_map, vx, vy);
                }
            }
        }

        vents
    }

    /// Clear 2x2 geothermal plant footprint ensuring passability and stability
    fn clear_geothermal_footprint(tile_map: &mut GameMap, vx: i32, vy: i32) {
        for dy in 0..2 {
            for dx in 0..2 {
                let tx = vx + dx;
                let ty = vy + dy;
                if let Some(t) = tile_map.get_tile_mut(tx as u32, ty as u32) {
                    t.is_wall_or_building = false;
                    t.is_lava = false;
                    t.is_lava_possible = false;
                    t.cell_type = 2; // Highland clay foundation
                }
            }
        }
    }

    /// Ensure 100% vehicle navigation connectivity between all spawn sites and map center
    fn ensure_connectivity(
        width: usize,
        height: usize,
        spawn_points: &[(i32, i32)],
        tile_map: &mut GameMap,
    ) {
        if spawn_points.is_empty() {
            return;
        }

        let cx = (width / 2) as i32;
        let cy = (height / 2) as i32;

        // Verify each spawn point can reach the center via BFS flood-fill
        for &(sx, sy) in spawn_points {
            let start = (sx + 3, sy + 3);
            let goal = (cx, cy);

            if !Self::check_connectivity(width, height, tile_map, start, goal) {
                // Punch passable 3-wide straight corridor directly to goal
                Self::carve_passable_channel(width, height, tile_map, start.0, start.1, goal.0, goal.1);
            }
        }

        // Verify all player spawn points can reach spawn_points[0]
        let root = (spawn_points[0].0 + 3, spawn_points[0].1 + 3);
        for &(sx, sy) in &spawn_points[1..] {
            let target = (sx + 3, sy + 3);
            if !Self::check_connectivity(width, height, tile_map, root, target) {
                Self::carve_passable_channel(width, height, tile_map, root.0, root.1, target.0, target.1);
            }
        }
    }

    /// Check if target is reachable from start using BFS across passable tiles
    pub fn check_connectivity(
        width: usize,
        height: usize,
        tile_map: &GameMap,
        start: (i32, i32),
        goal: (i32, i32),
    ) -> bool {
        if start == goal {
            return true;
        }

        let mut visited = vec![false; width * height];
        let mut queue = VecDeque::new();

        let start_idx = (start.1 as usize) * width + (start.0 as usize);
        visited[start_idx] = true;
        queue.push_back(start);

        let dirs = [
            (1, 0), (-1, 0), (0, 1), (0, -1),
            (1, 1), (-1, 1), (1, -1), (-1, -1),
        ];

        while let Some((x, y)) = queue.pop_front() {
            if x == goal.0 && y == goal.1 {
                return true;
            }

            for &(dx, dy) in &dirs {
                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let idx = (ny as usize) * width + (nx as usize);
                    if !visited[idx] {
                        visited[idx] = true;
                        if let Some(t) = tile_map.get_tile(nx as u32, ny as u32) {
                            if !t.is_wall_or_building && !t.is_lava {
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Carve a passable corridor directly through obstructions to guarantee path connectivity
    fn carve_passable_channel(
        width: usize,
        height: usize,
        tile_map: &mut GameMap,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    ) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut cur_x = x0;
        let mut cur_y = y0;

        loop {
            // Carve 3x3 footprint at each step along channel
            for oy in -1..=1 {
                for ox in -1..=1 {
                    let tx = cur_x + ox;
                    let ty = cur_y + oy;
                    if tx >= 0 && tx < width as i32 && ty >= 0 && ty < height as i32 {
                        if let Some(t) = tile_map.get_tile_mut(tx as u32, ty as u32) {
                            t.is_wall_or_building = false;
                            t.is_lava = false;
                            t.is_lava_possible = false;
                            t.cell_type = 0; // Smooth basalt corridor
                        }
                    }
                }
            }

            if cur_x == x1 && cur_y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                cur_x += sx;
            }
            if e2 <= dx {
                err += dx;
                cur_y += sy;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::pathfinding::PathFinder;

    #[test]
    fn test_deterministic_reproduction() {
        let config = MapGeneratorConfig {
            width: 64,
            height: 64,
            biome: TerrainBiome::CanyonsAndCrags,
            ore_abundance: OreAbundance::Normal,
            geothermal_vents: 4,
            symmetry: MapSymmetry::Rotational2Way,
            seed: 12345,
            player_count: 2,
        };

        let map1 = MapGenerator::generate(&config);
        let map2 = MapGenerator::generate(&config);

        assert_eq!(map1.spawn_points, map2.spawn_points);
        assert_eq!(map1.geothermal_vents, map2.geothermal_vents);
        assert_eq!(map1.ore_deposits.len(), map2.ore_deposits.len());
        for i in 0..map1.ore_deposits.len() {
            assert_eq!(map1.ore_deposits[i].tile_x, map2.ore_deposits[i].tile_x);
            assert_eq!(map1.ore_deposits[i].tile_y, map2.ore_deposits[i].tile_y);
            assert_eq!(map1.ore_deposits[i].remaining_amount, map2.ore_deposits[i].remaining_amount);
        }
        assert_eq!(map1.tile_map, map2.tile_map);
    }

    #[test]
    fn test_different_seeds_produce_different_maps() {
        let mut config1 = MapGeneratorConfig::default();
        config1.seed = 1001;

        let mut config2 = MapGeneratorConfig::default();
        config2.seed = 2002;

        let map1 = MapGenerator::generate(&config1);
        let map2 = MapGenerator::generate(&config2);

        assert_ne!(map1.tile_map.tiles, map2.tile_map.tiles);
    }

    #[test]
    fn test_base_footprint_clearance() {
        let biomes = [
            TerrainBiome::VolcanicBasin,
            TerrainBiome::CanyonsAndCrags,
            TerrainBiome::BlightedBadlands,
            TerrainBiome::ObsidianRidge,
        ];

        for &biome in &biomes {
            let config = MapGeneratorConfig {
                width: 64,
                height: 64,
                biome,
                ore_abundance: OreAbundance::Normal,
                geothermal_vents: 4,
                symmetry: MapSymmetry::Rotational2Way,
                seed: 9876,
                player_count: 2,
            };

            let map = MapGenerator::generate(&config);

            for &(sx, sy) in &map.spawn_points {
                for dy in 0..6 {
                    for dx in 0..6 {
                        let tx = (sx + dx) as u32;
                        let ty = (sy + dy) as u32;
                        let tile = map.tile_map.get_tile(tx, ty).expect("Base tile within bounds");

                        assert!(!tile.is_wall_or_building, "Base footprint at ({}, {}) has wall/cliff", tx, ty);
                        assert!(!tile.is_lava, "Base footprint at ({}, {}) has active lava", tx, ty);
                        assert!(!tile.is_blight, "Base footprint at ({}, {}) has microbial blight", tx, ty);
                    }
                }
            }
        }
    }

    #[test]
    fn test_ore_beacon_symmetry() {
        let config = MapGeneratorConfig {
            width: 64,
            height: 64,
            biome: TerrainBiome::CanyonsAndCrags,
            ore_abundance: OreAbundance::Normal,
            geothermal_vents: 4,
            symmetry: MapSymmetry::Rotational2Way,
            seed: 77777,
            player_count: 2,
        };

        let map = MapGenerator::generate(&config);
        assert_eq!(map.spawn_points.len(), 2);

        let p0_center = (map.spawn_points[0].0 + 3, map.spawn_points[0].1 + 3);
        let p1_center = (map.spawn_points[1].0 + 3, map.spawn_points[1].1 + 3);

        // Find local deposits closest to player 0 and player 1 base centers
        let p0_deposits: Vec<_> = map.ore_deposits.iter().filter(|d| {
            let dist = ((d.tile_x - p0_center.0).pow(2) + (d.tile_y - p0_center.1).pow(2)) as f64;
            dist <= 100.0
        }).collect();

        let p1_deposits: Vec<_> = map.ore_deposits.iter().filter(|d| {
            let dist = ((d.tile_x - p1_center.0).pow(2) + (d.tile_y - p1_center.1).pow(2)) as f64;
            dist <= 100.0
        }).collect();

        assert_eq!(p0_deposits.len(), p1_deposits.len(), "Each player has equal local mineral deposits");

        // Common and rare deposits per player
        let p0_common = p0_deposits.iter().filter(|d| d.ore_type == OreType::Common).count();
        let p1_common = p1_deposits.iter().filter(|d| d.ore_type == OreType::Common).count();
        assert_eq!(p0_common, p1_common);

        let p0_rare = p0_deposits.iter().filter(|d| d.ore_type == OreType::Rare).count();
        let p1_rare = p1_deposits.iter().filter(|d| d.ore_type == OreType::Rare).count();
        assert_eq!(p0_rare, p1_rare);

        let p0_dists: Vec<_> = p0_deposits.iter().map(|d| (d.tile_x - p0_center.0).pow(2) + (d.tile_y - p0_center.1).pow(2)).collect();
        let p1_dists: Vec<_> = p1_deposits.iter().map(|d| (d.tile_x - p1_center.0).pow(2) + (d.tile_y - p1_center.1).pow(2)).collect();
        assert_eq!(p0_dists, p1_dists);
    }

    #[test]
    fn test_base_connectivity_astar() {
        let config = MapGeneratorConfig {
            width: 64,
            height: 64,
            biome: TerrainBiome::VolcanicBasin,
            ore_abundance: OreAbundance::Normal,
            geothermal_vents: 4,
            symmetry: MapSymmetry::Rotational2Way,
            seed: 42,
            player_count: 2,
        };

        let map = MapGenerator::generate(&config);
        let start = (map.spawn_points[0].0 + 3, map.spawn_points[0].1 + 3);
        let goal = (map.spawn_points[1].0 + 3, map.spawn_points[1].1 + 3);

        let path = PathFinder::find_path(&map.tile_map, start, goal);
        assert!(path.is_some(), "A* path from base 0 to base 1 exists");

        let p = path.unwrap();
        assert!(!p.is_empty(), "Path has steps");

        // Verify all tiles along path are passable
        for &(tx, ty) in &p {
            let tile = map.tile_map.get_tile(tx as u32, ty as u32).expect("Path tile in bounds");
            assert!(!tile.is_wall_or_building, "Path step at ({}, {}) is blocked by wall", tx, ty);
            assert!(!tile.is_lava, "Path step at ({}, {}) is blocked by lava", tx, ty);
        }
    }

    #[test]
    fn test_four_player_rotational_symmetry() {
        let config = MapGeneratorConfig {
            width: 64,
            height: 64,
            biome: TerrainBiome::ObsidianRidge,
            ore_abundance: OreAbundance::Abundant,
            geothermal_vents: 4,
            symmetry: MapSymmetry::Rotational4Way,
            seed: 5555,
            player_count: 4,
        };

        let map = MapGenerator::generate(&config);
        assert_eq!(map.spawn_points.len(), 4);

        // Verify each base has clear footprint
        for &(sx, sy) in &map.spawn_points {
            for dy in 0..6 {
                for dx in 0..6 {
                    let tile = map.tile_map.get_tile((sx + dx) as u32, (sy + dy) as u32).unwrap();
                    assert!(!tile.is_wall_or_building);
                    assert!(!tile.is_lava);
                }
            }
        }

        // Verify geothermal vents count
        assert_eq!(map.geothermal_vents.len(), 4);
    }

    #[test]
    fn test_colony_state_from_generated_map() {
        use crate::sim::{ColonyState, Faction};

        let config = MapGeneratorConfig {
            width: 64,
            height: 64,
            biome: TerrainBiome::VolcanicBasin,
            ore_abundance: OreAbundance::Normal,
            geothermal_vents: 4,
            symmetry: MapSymmetry::Rotational2Way,
            seed: 9999,
            player_count: 2,
        };

        let gen_map = MapGenerator::generate(&config);
        let mut colony = ColonyState::from_generated_map(gen_map, Faction::Eden);

        assert_eq!(colony.faction, Faction::Eden);
        assert_eq!(colony.game_map.width, 64);
        assert_eq!(colony.game_map.height, 64);
        assert!(!colony.buildings.is_empty(), "Colony has initial buildings");
        assert!(!colony.units.is_empty(), "Colony has initial units");
        assert!(!colony.ai_players.is_empty(), "Colony has AI opponent");
        assert!(!colony.ore_deposits.is_empty(), "Colony has mineral deposits");

        // Verify simulation tick updates correctly
        let update_res = colony.update(0.1);
        assert!(update_res.mission_ended.is_none(), "Skirmish continues in progress");
    }

    #[test]
    fn test_new_skirmish_map_direct_helper() {
        use crate::sim::{ColonyState, Faction};

        let config = MapGeneratorConfig {
            width: 64,
            height: 64,
            biome: TerrainBiome::CanyonsAndCrags,
            ore_abundance: OreAbundance::Abundant,
            geothermal_vents: 3,
            symmetry: MapSymmetry::Rotational2Way,
            seed: 8888,
            player_count: 2,
        };

        let colony = ColonyState::new_skirmish_map(config, Faction::Plymouth);
        assert_eq!(colony.faction, Faction::Plymouth);
        assert_eq!(colony.population, 150);
        assert!(colony.power_produced >= colony.power_consumed);
    }

    #[test]
    fn test_mirrored_x_symmetry() {
        let config = MapGeneratorConfig {
            width: 64,
            height: 64,
            biome: TerrainBiome::CanyonsAndCrags,
            ore_abundance: OreAbundance::Normal,
            geothermal_vents: 2,
            symmetry: MapSymmetry::MirroredX,
            seed: 3333,
            player_count: 2,
        };

        let map = MapGenerator::generate(&config);
        assert_eq!(map.spawn_points.len(), 2);
        let p0 = map.spawn_points[0];
        let p1 = map.spawn_points[1];
        // Vertical Y coordinate matches in bilateral X mirror
        assert_eq!(p0.1, p1.1);
    }

    #[test]
    fn test_abundance_scaling() {
        let biomes = [TerrainBiome::CanyonsAndCrags];
        for b in biomes {
            let config_scarce = MapGeneratorConfig {
                width: 64,
                height: 64,
                biome: b,
                ore_abundance: OreAbundance::Scarce,
                geothermal_vents: 2,
                symmetry: MapSymmetry::Rotational2Way,
                seed: 111,
                player_count: 2,
            };
            let config_abundant = MapGeneratorConfig {
                width: 64,
                height: 64,
                biome: b,
                ore_abundance: OreAbundance::Abundant,
                geothermal_vents: 2,
                symmetry: MapSymmetry::Rotational2Way,
                seed: 111,
                player_count: 2,
            };

            let map_s = MapGenerator::generate(&config_scarce);
            let map_a = MapGenerator::generate(&config_abundant);

            let total_s: u32 = map_s.ore_deposits.iter().map(|d| d.remaining_amount).sum();
            let total_a: u32 = map_a.ore_deposits.iter().map(|d| d.remaining_amount).sum();

            assert!(total_a > total_s, "Abundant map has more total ore reserves than Scarce");
        }
    }

    #[test]
    fn test_prng_and_map_helpers() {
        let mut rng = SeededRng::new(42);
        let val_i = rng.range_i32(10, 20);
        assert!((10..=20).contains(&val_i));

        let val_u = rng.range_usize(5, 15);
        assert!((5..=15).contains(&val_u));

        let blank = GameMap::new_blank(32, 32);
        assert_eq!(blank.width, 32);
        assert_eq!(blank.height, 32);
        assert_eq!(blank.tiles.len(), 1024);
        assert!(blank.in_bounds(15, 15));
        assert!(!blank.in_bounds(-1, 0));
        assert!(!blank.in_bounds(32, 10));
    }
}
