# Outpost 2 — Campaign Scenarios & Multi-Map Architecture

## 1. Overview
The Outpost 2 modern WebAssembly remake features a native multi-campaign selector architecture capable of instantiating distinct colonial operational theaters from authentic `.map` files extracted from `maps.vol`.

## 2. Supported Operational Sectors

| Scenario ID | Codename | Map Asset | Faction | Starting Threat | Primary Directives |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **0** | **Operation New Dawn** | `eden01.map` (48 KB, 64x64) | **Eden** | Plymouth raiders & magma fissures | Ore stockpiles, Combat Tanks, Scientific Research, Protect Command Center |
| **1** | **Operation Badland Vanguard** | `ply01.map` (48 KB, 64x64) | **Plymouth** | Eden vanguard incursions | Commission Vehicle Factory, Assemble 3 Lynx tanks, Stockpile 2,000t Common Ore |
| **2** | **Cadet Proving Grounds** | `tutorial.map` (48 KB, 64x64) | **Eden** | Academy holographic trials | Energy surplus, Connect 15 tubes, Freight mineral transport, Morale >90% |

## 3. Map Header & Cellular Layout
Every Outpost 2 binary `.map` file begins with a 20-byte header:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapHeader {
    pub version_tag: u32,  // 0x1011 (Outpost 2 map format)
    pub b_saved: i32,      // Save flag indicator
    pub lg_width: u32,     // Log2 of grid width (e.g. 6 -> 64 tiles)
    pub height: u32,       // Grid height (e.g. 64 tiles)
    pub tilesets: u32,     // Number of associated tileset clips (512)
}
```

Each grid cell is packed into a 32-bit unsigned integer bitfield:
- **Bits 0..4 (5 bits):** `cell_type` (0=Smooth Basalt/Road, 1=Iron-oxide Dust, 2=Cliff Slope, 3=Impassable Ridge, 4=Liquid Fissure).
- **Bits 5..15 (11 bits):** `tile_mapping` (Reference index into tileset sprites).
- **Bits 16..26 (11 bits):** `unit_index` (Active unit occupying cell).
- **Bit 27 (1 bit):** `is_lava` (Active molten magma hazard).
- **Bit 28 (1 bit):** `is_lava_possible` (Dormant volcanic fissure).
- **Bit 30 (1 bit):** `is_blight` (Microscopic nanite Blight hazard).
- **Bit 31 (1 bit):** `is_wall_or_building` (Solid structure footprint / sheer cliff barrier).

## 4. Objective Evaluation Pipeline
During the simulation tick, `Mission::evaluate` processes live colony telemetry:
```rust
pub fn evaluate(
    &mut self,
    common_ore: u32,
    rare_ore: u32,
    power_produced: u32,
    power_consumed: u32,
    tubes_count: usize,
    morale_pct: u32,
    buildings: &[Building],
    units: &[Unit],
    tech_tree: &TechTree,
) -> Option<&'static str>
```
Defeat triggers immediately if the Command Center is destroyed. Victory triggers when all primary directives are achieved, broadcasting high-priority audio announcements through the tactical computer radio dispatch.
