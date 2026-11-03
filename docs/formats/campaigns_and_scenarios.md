# Outpost 2 — Campaign Scenarios & Multi-Map Architecture

## 1. Overview
The Outpost 2 modern WebAssembly remake features a native multi-campaign selector architecture capable of instantiating distinct colonial operational theaters from authentic `.map` files extracted from `maps.vol`.

## 2. Supported Operational Sectors

| Scenario ID | Codename | Map Asset | Faction | Starting Threat | Primary Directives |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **0** | **Operation New Dawn** | `eden01.map` (48 KB, 64x64) | **Eden** | Plymouth raiders & magma fissures | Ore stockpiles, Combat Tanks, Scientific Research, Protect Command Center |
| **1** | **Operation Badland Vanguard** | `ply01.map` (48 KB, 64x64) | **Plymouth** | Eden vanguard incursions | Commission Vehicle Factory, Assemble 3 Lynx tanks, Stockpile 2,000t Common Ore |
| **2** | **Cadet Proving Grounds** | `tutorial.map` (48 KB, 64x64) | **Eden** | Academy holographic trials | Energy surplus, Connect 15 tubes, Freight mineral transport, Morale >90% |
| **3** | **Operation Caldera Bastion** | `eden02.map` (64 KB, 128x64) | **Eden** | Molten magma fissures & Plymouth raiders | Geothermal grid >= 400 kW, 2 Guard Posts, Stockpile 2,500t Common / 500t Rare Ore, Protect Command Center |
| **4** | **Operation Iron Fang** | `ply02.map` (48 KB, 64x64) | **Plymouth** | Eden armored counter-patrols & heavy automated defense | 4 Armed Combat Tanks, Fortify Choke Point with Guard Post, Stockpile 3,500t Common / 800t Rare Ore, Protect Command Center |

---

## 3. Map Header & Cellular Layout
Every Outpost 2 binary `.map` file begins with a 20-byte header:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapHeader {
    pub version_tag: u32,  // 0x1011 (Outpost 2 map format)
    pub b_saved: i32,      // Save flag indicator (0=raw map, 1=savegame)
    pub lg_width: u32,     // Log2 of grid width (e.g. 6 -> 64 tiles, 7 -> 128 tiles)
    pub height: u32,       // Grid height (e.g. 64 tiles)
    pub tilesets: u32,     // Number of associated tileset clips (512)
}
```

Each grid cell is packed into a 32-bit little-endian bitfield:
- **Bits 0..4 (5 bits):** `cell_type` (0=Smooth Basalt/Road, 1=Iron-oxide Dust, 2=Cliff Slope, 3=Impassable Ridge, 4=Liquid Fissure, 5=Rough Badlands, 7=Canyon Trench).
- **Bits 5..15 (11 bits):** `tile_mapping` (Reference index into tileset sprites).
- **Bits 16..26 (11 bits):** `unit_index` (Active unit occupying cell).
- **Bit 27 (1 bit):** `is_lava` (Active molten magma hazard).
- **Bit 28 (1 bit):** `is_lava_possible` (Dormant volcanic fissure prone to rupture).
- **Bit 30 (1 bit):** `is_blight` (Microscopic nanite Blight hazard).
- **Bit 31 (1 bit):** `is_wall_or_building` (Solid structure footprint / sheer cliff barrier).

---

## 4. Objective Evaluation Pipeline
During each simulation tick, `Mission::evaluate` processes live colony telemetry:
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
Defeat triggers immediately if the Command Center is destroyed. Victory triggers when all primary directives are achieved (`is_primary: true` and `is_completed: true`). Secondary objectives reward optional goals without halting scenario progression.

---

## 5. Detailed Campaign Dossiers: Expanded Scenarios

### 5.1 Scenario 3: Eden Mission 2 — "Molten Fissure Containment"
- **Operational Theater:** Sector Caldera Ridge (`assets/eden02.map`, 128x64 tiles, 64,725 bytes)
- **Faction:** Eden Foundation
- **Tactical Codename:** `Operation Caldera Bastion — Molten Fissure Containment`

#### Narrative Briefing
> *"Eden Command Alert: Subsurface thermal sensors register catastrophic magma displacement along the northern fault lines. Molten fissures are rupturing the permafrost, threatening colonial survival dome sectors.*
>
> *Stabilize base power generation above 400 kW using high-output arrays or geothermal conduits, construct a fortified perimeter of at least 2 defensive Guard Post Turrets overlooking fissure corridors, stockpile 2,500t Common Ore and 500t Rare Ore for emergency thermal barricades, and ensure the Eden Command Center remains secure from seismic fissures and hostile bombardment."*

#### Objectives Architecture
- **Primary Directives:**
  1. `[id: 1]` **Power Grid Stabilization:** Generate at least 400 kW total base power (`power_produced >= 400`).
  2. `[id: 2]` **Perimeter Defense:** Deploy and activate at least 2 Guard Post Turrets guarding the fissure rim.
  3. `[id: 3]` **Resource Stockpile:** Accumulate 2,500 tonnes of Common Ore and 500 tonnes of Rare Ore for thermal barrier reinforcement.
  4. `[id: 4]` **Command Survival:** Protect the Eden Command Center from destruction.
- **Secondary Directives:**
  1. `[id: 5]` **Population Morale:** Maintain colonial morale at or above 80% despite volcanic tremor alarms.
  2. `[id: 6]` **Technological Advancement:** Complete scientific research in any primary or advanced discipline.

#### Environmental & Combat Threats
- **Magma Fissure Shearing:** Subterranean magma chambers active across 675 liquid fissure tiles (cell type 4) along an extended 128-tile lateral fault.
- **Seismic Shockwaves:** Volcanic tremors threaten structural integrity and trigger emergency evacuation alerts.
- **Plymouth Skirmish Incursions:** Aggressive Plymouth raiders maneuver along elevated ridge corridors to sabotage solar collector lines and mining convoys.

---

### 5.2 Scenario 4: Plymouth Mission 2 — "Smelter Sabotage & Ambush"
- **Operational Theater:** Sector Plymouth South Badlands (`assets/ply02.map`, 64x64 tiles, 48,341 bytes)
- **Faction:** Plymouth Colony
- **Tactical Codename:** `Operation Iron Fang — Smelter Sabotage & Ambush`

#### Narrative Briefing
> *"Plymouth High Council Direct Action Dispatch: Eden technocrats have entrenched a heavily armed mineral extraction and smelting complex directly bordering our southern badland canyon passes, choking off Plymouth's mineral lifeline.*
>
> *Assemble an armored ambush strike squadron of at least 4 combat tanks equipped with railguns or lasers, erect a fortified Guard Post Turret to control the canyon choke point, hijack and stockpile at least 3,500t Common Ore and 800t Rare Ore, and defend the Plymouth Command Center against Eden's automated retaliatory counter-assault."*

#### Objectives Architecture
- **Primary Directives:**
  1. `[id: 1]` **Strike Squadron Assembly:** Commission an armored strike squad of at least 4 combat tanks (Panther or Lynx chassis armed with Railguns/Lasers).
  2. `[id: 2]` **Choke Point Fortification:** Deploy at least 1 fortified Guard Post Turret controlling the ambush gorge.
  3. `[id: 3]` **Mineral Seizure:** Stockpile at least 3,500 tonnes of Common Ore and 800 tonnes of Rare Ore.
  4. `[id: 4]` **Command Survival:** Protect the Plymouth Command Center against retaliatory counter-strikes.
- **Secondary Directives:**
  1. `[id: 5]` **Vehicle Factory Readiness:** Maintain an active Vehicle Factory for continuous armored chassis production.
  2. `[id: 6]` **Energy Autonomy:** Sustain a positive energy surplus (`power_produced > power_consumed`) across all ambush posts.

#### Environmental & Combat Threats
- **Rugged Canyon Choke Points:** Over 2,195 tiles of rough badlands terrain (cell type 5) restricting vehicle convoy transit to narrow gorges.
- **Eden Heavy Railgun Patrols:** High-tech Eden mechanized patrols escorting ore shipments with heavy railguns.
- **Logistical Vulnerability:** Staging bases operate far from central geothermal deposits, making power distribution networks vulnerable to interdiction.
