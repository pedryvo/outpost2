//! Outpost 2 Natural Disasters & Dynamic Hazard Simulator
//! Simulates volcanic magma expansion, seismic tremors, and Microbial Blight contagion.

use crate::sim::map::GameMap;

#[derive(Debug, Clone)]
pub struct AtmosphericVortex {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
    pub lifetime: f64,
    pub max_lifetime: f64,
}

#[derive(Debug, Clone)]
pub struct DisasterSimulator {
    pub sol_timer: f64,
    pub blight_timer: f64,
    pub vortex_timer: f64,
    pub active_lava_tiles: Vec<(u32, u32)>,
    pub active_blight_tiles: Vec<(u32, u32)>,
    pub active_vortices: Vec<AtmosphericVortex>,
    pub eruption_count: u32,
    pub quake_shake_offset: (f64, f64),
    pub quake_duration: f64,
}

impl DisasterSimulator {
    pub fn new() -> Self {
        Self {
            sol_timer: 0.0,
            blight_timer: 0.0,
            vortex_timer: 0.0,
            active_lava_tiles: Vec::new(),
            active_blight_tiles: Vec::new(),
            active_vortices: Vec::new(),
            eruption_count: 0,
            quake_shake_offset: (0.0, 0.0),
            quake_duration: 0.0,
        }
    }

    /// Advance planetary natural disasters according to New Terra volcanic cycles
    pub fn update(&mut self, map: &mut GameMap, dt_secs: f64) -> Option<&'static str> {
        self.sol_timer += dt_secs;
        self.blight_timer += dt_secs;
        self.vortex_timer += dt_secs;

        // Advance active atmospheric vortices
        self.active_vortices.retain_mut(|v| {
            v.lifetime -= dt_secs;
            v.x += v.vx * dt_secs;
            v.y += v.vy * dt_secs;
            v.lifetime > 0.0
        });

        // Spawn rotary dust vortex every ~50 seconds
        let mut vortex_notice = None;
        if self.vortex_timer > 50.0 {
            self.vortex_timer = 0.0;
            let start_x = 10.0 + (self.eruption_count as f64 * 7.0) % 35.0;
            let start_y = 5.0 + (self.eruption_count as f64 * 5.0) % 25.0;
            let angle = (self.sol_timer * 1.7) % std::f64::consts::TAU;
            self.active_vortices.push(AtmosphericVortex {
                x: start_x,
                y: start_y,
                vx: angle.cos() * 2.2,
                vy: angle.sin() * 2.2,
                radius: 3.5,
                lifetime: 24.0,
                max_lifetime: 24.0,
            });
            vortex_notice = Some("METEOROLOGICAL ALERT: Rotary dust vortex formed on New Terra! Secure transport conduits.");
        }

        // Quake camera shake dissipation
        if self.quake_duration > 0.0 {
            self.quake_duration -= dt_secs;
            let magnitude = (self.quake_duration * 12.0).min(6.0);
            self.quake_shake_offset = (
                (self.sol_timer * 40.0).sin() * magnitude,
                (self.sol_timer * 35.0).cos() * magnitude,
            );
        } else {
            self.quake_shake_offset = (0.0, 0.0);
        }

        // Initialize blight node if empty
        if self.active_blight_tiles.is_empty() && self.blight_timer > 15.0 {
            let seed_x = 38.min(map.width - 2);
            let seed_y = 36.min(map.height - 2);
            let idx = (seed_y * map.width + seed_x) as usize;
            if idx < map.tiles.len() {
                map.tiles[idx].is_blight = true;
                self.active_blight_tiles.push((seed_x, seed_y));
            }
        }

        // Creep Microbial Blight spores every ~35 seconds
        if self.blight_timer > 35.0 {
            self.blight_timer = 0.0;
            if self.trigger_blight_creep(map) {
                return Some("BIOHAZARD WARNING: Microbial Blight contagion detected! Creeping spores advancing.");
            }
        }

        // Trigger volcanic fissure event every ~45 seconds
        if self.sol_timer > 45.0 {
            self.sol_timer = 0.0;
            self.eruption_count += 1;
            self.trigger_fissure(map);
            self.quake_duration = 3.5; // 3.5 seconds of planetary tremor
            return Some("SEISMIC WARNING: Volcanic fissure erupted on New Terra! Magma is advancing.");
        }

        vortex_notice
    }

    /// Creep microbial blight to neighboring tiles
    fn trigger_blight_creep(&mut self, map: &mut GameMap) -> bool {
        if self.active_blight_tiles.is_empty() {
            return false;
        }

        let mut new_spores = Vec::new();
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1)];

        for &(bx, by) in self.active_blight_tiles.iter().rev().take(6) {
            for &(dx, dy) in &dirs {
                let nx = bx as i32 + dx;
                let ny = by as i32 + dy;
                if nx >= 0 && nx < map.width as i32 && ny >= 0 && ny < map.height as i32 {
                    let u_nx = nx as u32;
                    let u_ny = ny as u32;
                    let idx = (u_ny * map.width + u_nx) as usize;
                    if idx < map.tiles.len() && !map.tiles[idx].is_lava && !map.tiles[idx].is_blight {
                        map.tiles[idx].is_blight = true;
                        new_spores.push((u_nx, u_ny));
                        break;
                    }
                }
            }
            if new_spores.len() >= 3 {
                break;
            }
        }

        let added = !new_spores.is_empty();
        self.active_blight_tiles.extend(new_spores);
        if self.active_blight_tiles.len() > 100 {
            self.active_blight_tiles.drain(0..40);
        }
        added
    }

    /// Spread magma along valid lava channels
    fn trigger_fissure(&mut self, map: &mut GameMap) {
        // Eruption epicenter near northern volcanic ridge (x: 15..25, y: 12..18)
        let center_x = 18 + (self.eruption_count * 3) % 15;
        let center_y = 14 + (self.eruption_count * 2) % 10;

        for dy in -2..=2 {
            for dx in -2..=2 {
                let tx = (center_x as i32 + dx).clamp(0, map.width as i32 - 1) as u32;
                let ty = (center_y as i32 + dy).clamp(0, map.height as i32 - 1) as u32;
                let idx = (ty * map.width + tx) as usize;

                if idx < map.tiles.len() {
                    // Ignite tile as active molten lava
                    map.tiles[idx].is_lava = true;
                    self.active_lava_tiles.push((tx, ty));
                }
            }
        }
    }
}
