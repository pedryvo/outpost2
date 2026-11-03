# Outpost 2 Technology Tree Specification (`tech.vol`)

Technology trees in *Outpost 2: Divided Destiny* determine the scientific progress of the human colonies (Eden and Plymouth), unlocking new buildings, weapons, chassis modifications, and planetary survival protections.

## File Format (`basictek.txt`, `edentek.txt`, `ply_tek.txt`)

Research files in `tech.vol` are plain-text declarative scripts structured around `BEGIN_TECH` blocks:

```ini
; Example Tech Node definition from basictek.txt
BEGIN_TECH "Astronomy" 02701
    CATEGORY        1
    COST            400
    DESCRIPTION     "Our astronomical database has been rebuilt, giving us increased knowledge of the possible resources and hazards of New Terra."
END_TECH
```

## Tech Categories

| Category ID | Field of Study | Typical Unlocks |
|---|---|---|
| `0` | Base Infrastructure | Command Centers, Laboratories, Solar Power Arrays |
| `1` | Planetary Science | Satellite telemetry, seismic sensors, ore survey scanners |
| `2` | Life Support & Bio | Hydroponics automation, nursery habitats, medical clinics |
| `3` | Metallurgy & Materials | High-temperature smelters, rare metal refinement |
| `4` | Disaster Defense | Lava containment walls, disaster warning beacons |
| `5` | Military Engineering | Lynx light tanks, Panther medium tanks, laser cannons |

## Canonical WebAssembly Research Matrix

| ID | Technology Node | Category | Cost (C) | Prerequisite Unlocks & Tactical Benefits |
|:---:|---|:---:|:---:|---|
| `2106` | Basic Laboratory | `0` Core Science | 0 | Unlocked at start. Required for high-tier research execution. |
| `2701` | Astronomy | `1` Space & Orbital | 400 | Rebuilds astronomical satellite network, mapping orbital hazards. |
| `3301` | Hydroponics Automation | `2` Life Support | 650 | Automated nutrient feeding arrays, $+50\%$ food output per Agridome. |
| `3402` | High-Temperature Smelting | `3` Metallurgy | 800 | Plasma-arc furnaces, $+25\%$ refined ore yield from smelters. |
| `4201` | Lava Containment Walls | `4` Engineering | 1100 | Fortified ceramic thermal barriers to resist lava flows and secure choke points. |
| `4301` | Vulcanology & Hydrothermal Extraction | `4` Engineering | 750 | Unlocks continuous $+450\text{ kW}$ Geothermal Power Plant construction. |
| `5104` | Lynx Chassis Reinforced Armor | `5` Military Robotics | 950 | Alloy composite plates granting $+50$ durability to light scout chassis. |
| `5202` | Pulse Laser Optics | `5` Military Robotics | 1200 | Refined optical focusing crystals granting $+15$ beam intensity to laser arms. |
| `5301` | Magnetic Acceleration & Railgun Armaments | `5` Military Robotics | 1400 | Unlocks Tiger Heavy Tanks armed with supersonic kinetic Railguns. |
| `5401` | Automated Point Defense Turrets | `5` Military Robotics | 600 | Unlocks automated perimeter Guard Post defense turrets with dual rapid cannons. |
| `6101` | Cryogenic Evacuation Modules | `1` Space & Orbital | 1500 | Advanced passenger stasis modules for the Evacuation Starship payload. |

## Game Engine Mechanics

1. **Laboratories:** Research is conducted inside active Laboratories with assigned scientists.
2. **Ore Investment:** Advancing research deducts Common Ore from the colony stockpile per Sol tick.
3. **Prerequisites:** Advanced military armor, railgun armaments, geothermal power plants, and perimeter defense turrets actively check technology state before permitting vehicle queuing, ConVec kit loading, or terrain placement.
