# Outpost 2 Procedural Skirmish Map Generator & Terrain Topology Specification

The Procedural Map Generator (`src/sim/generator.rs`) produces balanced, deterministic, competitive multiplayer and skirmish maps for *Outpost 2: Divided Destiny*. It generates natural choke points, canyon corridors, volcanic basins, mineral caches, and geothermal vents with strict multiplayer symmetry and guaranteed vehicle traversability.

---

## 1. Architectural Overview & Configuration

Procedural map generation is governed by `MapGeneratorConfig` and produces a `GeneratedMap`:

```rust
pub struct MapGeneratorConfig {
    pub width: usize,               // 64, 96, 128 (standard Outpost 2 square maps)
    pub height: usize,              // 64, 96, 128
    pub biome: TerrainBiome,        // VolcanicBasin, CanyonsAndCrags, BlightedBadlands, ObsidianRidge
    pub ore_abundance: OreAbundance,// Scarce, Normal, Abundant
    pub geothermal_vents: usize,    // Hydrothermal fumaroles for Geothermal Power Plants
    pub symmetry: MapSymmetry,      // None, Rotational2Way, Rotational4Way, MirroredX
    pub seed: u64,                  // Deterministic multi-client lockstep synchronization
    pub player_count: usize,        // 2 to 6 spawn points
}
```

### Resulting Artifact: `GeneratedMap`
```rust
pub struct GeneratedMap {
    pub tile_map: TileMap,             // GameMap tile bitfields and cell types
    pub spawn_points: Vec<(i32, i32)>, // Top-left coordinates of guaranteed 6x6 base footprints
    pub ore_deposits: Vec<OreDeposit>, // Symmetrically balanced local & contested ore beacons
    pub geothermal_vents: Vec<(i32, i32)>, // Hydrothermal fumarole coordinates
    pub biome: TerrainBiome,           // Biome environment tag
}
```

---

## 2. Terrain Biomes

| Biome | Visual Aesthetic | Cell Type Mapping | Environmental Hazards |
|---|---|---|---|
| **`VolcanicBasin`** | Basalt plains, molten caldera fault lines | `0` (Basalt rock), `2` (Highland clay), `4` (Liquid fissure) | Active magma pools (`is_lava = true`, `is_lava_possible = true`) in thermal basins |
| **`CanyonsAndCrags`** | Iron-oxide badlands, steep sandstone plateaus | `1` (Iron-oxide dust), `3` (Ridge/cliff), `0` (Canyon bed) | Steep cliff faces (`is_wall_or_building = true`) forming strategic choke points |
| **`BlightedBadlands`** | Alien-corroded wasteland | `5` (Blighted earth), `1` (Barren dust), `3` (Crags) | Alien Microbial Blight (`is_blight = true`) pockets in contested outer sectors |
| **`ObsidianRidge`** | Vitrified volcanic spires, razor crags | `0` (Smooth basalt), `2` (Obsidian rock), `3` (Spire wall) | Rich mineral deposits locked behind narrow obsidian canyon passes |

---

## 3. Mathematical Terrain Topology & Algorithms

### 3.1 Multi-Octave Fractal Brownian Motion (fBm)
Elevation $E(x, y) \in [0.0, 1.0]$ is sculpted using 4-octave value noise with quintic S-curve interpolation ($6t^5 - 15t^4 + 10t^3$):

$$E(x, y) = \frac{\sum_{i=0}^{N-1} A_i \cdot \text{Noise}(x \cdot f_i, y \cdot f_i, \text{seed}_i)}{\sum_{i=0}^{N-1} A_i}$$

Where:
- Frequency $f_i = f_0 \cdot 2^i$ ($f_0 = 0.045$)
- Amplitude $A_i = A_0 \cdot 0.5^i$ (persistence $p = 0.5$, lacunarity $l = 2.0$)
- Hash function: 64-bit SplitMix64 avalanche hashing over integer coordinates and seed.

### 3.2 Symmetry Operators
To guarantee absolute multiplayer fairness:
- **`Rotational2Way` ($C_2$ symmetry):** Heightmap, corridors, and resources satisfy:
  $$E(x, y) = \frac{1}{2}\big[E(x, y) + E(W - 1 - x, H - 1 - y)\big]$$
  Opposing player bases and mineral beacons are 180° rotationally symmetric across the map center.
- **`Rotational4Way` ($C_4$ symmetry):** Quadrant symmetry for 4-player matches on square maps:
  $$E(x, y) = \frac{1}{4}\big[E(x, y) + E(W - 1 - x, H - 1 - y) + E(y, W - 1 - x) + E(W - 1 - y, x)\big]$$
- **`MirroredX` ($\sigma_v$ reflection):** Bilateral mirror across central vertical axis:
  $$E(x, y) = \frac{1}{2}\big[E(x, y) + E(W - 1 - x, y)\big]$$
- **`None`:** Asymmetric organic procedural terrain for narrative bot skirmishes.

---

## 4. Choke Points, Corridors & Plateaus

### 4.1 Canyon Corridor & Contested Valley Carving
1. **Central Contested Valley:** A circular basin of radius $R_{valley} \approx 0.12 \cdot \min(W, H)$ is carved at map center $(cx, cy)$, lowering elevation to passable floor level ($E \approx 0.25$).
2. **Canyon Corridors:** Bresenham line carving lowers a 3-to-4 tile wide corridor connecting each player base center $(sx + 3, sy + 3)$ to $(cx, cy)$.
3. **Flank Perimeter Routes:** Symmetrical corridors connect adjacent spawn bases $(P_i \leftrightarrow P_{i+1})$, creating flanking opportunities around central choke points.
4. **Natural Choke Point Formation:**
   Tiles with elevation $E \ge 0.68$ or local gradient $\max |\Delta E| \ge 0.16$ are classified as impassable cliff ridges (`cell_type = 3`, `is_wall_or_building = true`). Where carved corridors cut through elevated plateaus, the bordering walls naturally form narrow, high-ground choke points ideal for defensive Guard Post Turrets.

---

## 5. Guaranteed Base Clearance & Logistics

### 5.1 $6 \times 6$ Clear Base Footprint
For every spawn point $(sx, sy)$, the entire $[sx - 1, sx + 7] \times [sy - 1, sy + 7]$ bounding box is flattened and purified:
- `cell_type = 0` or `1` (Smooth basalt / Iron-oxide dust)
- `is_wall_or_building = false` (No cliff walls or obstacles)
- `is_lava = false` and `is_lava_possible = false` (No magma hazards)
- `is_blight = false` (No microbial blight infestation)

This guarantees immediate unhindered construction of:
1. Command Center ($3 \times 3$) at $(sx, sy)$
2. Solar Power Array ($2 \times 2$) at $(sx + 4, sy)$
3. Common Ore Smelter ($2 \times 2$) at $(sx, sy + 4)$
4. Vehicle Factory ($3 \times 3$) at $(sx + 4, sy + 4)$
5. Connecting pressurized life-support tubes without terrain collisions.

### 5.2 Mineral Beacon Distribution (`OreDeposit`)
Resources are split into local veins and central contested caches:

| Resource Tier | Abundance: Scarce | Abundance: Normal | Abundance: Abundant | Extraction Yield | Placement |
|---|---|---|---|---|---|
| **Local Common Ore** | 25,000 t | 50,000 t | 90,000 t | 50 t/cycle | Symmetrical offset (dist $\sqrt{50}$ from base center) |
| **Local Rare Ore** | 15,000 t | 25,000 t | 50,000 t | 45 t/cycle | Symmetrical offset (dist $\sqrt{50}$ from base center) |
| **Contested Central Cache** | 50,000 / 30,000 t | 80,000 / 45,000 t | 120,000 / 75,000 t | **75 t/cycle** | Neutral central valley $(cx \pm 4, cy \pm 4)$ |

### 5.3 Hydrothermal Fumaroles (Geothermal Vents)
- Hydrothermal vents are placed on stable highland bedrock (`cell_type = 2`, `is_wall_or_building = false`, `is_lava = false`).
- Guaranteed $2 \times 2$ footprint clearance permits immediate construction of Geothermal Power Plants (+450 kW continuous clean power).
- Symmetrically allocated: 1 local vent per player (if $V \ge N$), with remaining vents placed along contested volcanic fault lines.

---

## 6. Connectivity Verification Engine

To prevent map seeds with vehicle-blocking bottlenecks:
1. **BFS Flood-Fill Check:** Computes reachable tile components from each spawn point $(sx + 3, sy + 3)$ across passable tiles (`!is_wall_or_building && !is_lava`).
2. **Auto-Corridor Piercing:** If any base cannot reach the central valley $(cx, cy)$ or other bases, the generator punches a 3-wide straight passable corridor directly connecting the isolated components.
3. **A\* Unit Traversal Validation:** Confirms vehicle navigation paths exist using `PathFinder::find_path`.

---

## 7. Skirmish Engine Integration

Procedurally generated maps integrate seamlessly into `ColonyState`:

```rust
// Generate and launch a procedural skirmish match
let config = MapGeneratorConfig {
    width: 64,
    height: 64,
    biome: TerrainBiome::CanyonsAndCrags,
    ore_abundance: OreAbundance::Normal,
    geothermal_vents: 4,
    symmetry: MapSymmetry::Rotational2Way,
    seed: 42,
    player_count: 2,
};

let colony = ColonyState::new_skirmish_map(config, Faction::Eden);
```

- **Player 1 Base:** Placed at `spawn_points[0]` with initial Command Center, Solar Array, Smelter, Vehicle Factory, ConVec, Cargo Truck, Robo-Miner, and Scout.
- **Rival AI Opponents:** Spawned at `spawn_points[1..N]` with configured `AiPlayer` controllers (`AggressiveRusher`, `DefensiveFortifier`, `EconomicTechRusher`), independent production queues, and tactical squad assault managers.
- **Mission Objectives:** Instantiates `Mission::new_skirmish` (Scenario ID 100) tracking Command Center preservation, elimination of rival bases, and secondary geothermal energy milestones.
