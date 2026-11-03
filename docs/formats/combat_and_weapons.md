# Outpost 2: Divided Destiny — Combat Simulation & Tactical Weaponry Specification

## 1. Overview
In *Outpost 2: Divided Destiny*, survival on New Terra requires active defense against rival faction skirmishers (Eden vs Plymouth) and planetary microbial threats. Vehicles and stationary defensive emplacements mount distinct weapon systems with specific ballistics, ranges, and tactical counters.

---

## 2. Weapon Systems Matrix

| Weapon | Range | Damage | Refire Rate | Tracer Glow | Tactical Role |
|:---|:---:|:---:|:---:|:---:|:---|
| **High-Energy Pulse Laser** | 5.5 tiles | 24 | 0.8 s | Cyan (`#00d4ff`) | Instantaneous armor-piercing beam, optimal against light vehicles and scouts. |
| **Microwave Thermal Emitter**| 4.0 tiles | 18 | 0.6 s | Amber (`#facc15`) | Cooks electronic microcircuits, debilitating mining equipment and cargo trucks. |
| **Hyper-Velocity Railgun** | 7.0 tiles | 45 | 1.8 s | White/Blue (`#e2e8f0`) | Long-range kinetic penetrator firing depleted tungsten rounds. Heavy tank artillery. |
| **Rocket Propelled Grenade (RPG)** | 6.0 tiles | 35 | 1.4 s | Fiery Orange (`#f97316`) | Concussive explosive ordinance delivering blast radius damage to clusters. |
| **EMP Disruption Arc** | 4.5 tiles | 12 | 2.0 s | Purple (`#c084fc`) | Disables drive motors and sensor grids temporarily. |
| **Microbial Acid Cloud** | 5.0 tiles | 28 | 1.2 s | Green (`#22c55e`) | Plymouth biological weapon spraying corrosive blight spores that eat chassis armor. |
| **Guard Post Dual Cannons**| 6.5 tiles | 25* | 1.1 s | Crimson (`#ef4444`) | Automated perimeter bunker turret (*enhanced by Laser Research). |

---

## 3. Combat Mechanics & Faction AI
1. **Target Acquisition:**
   - **Manual Order:** Right-clicking directly on a hostile unit orders selected friendly units to `Attack { target_unit_id }`.
   - **Autonomous Stance:** Units with active weapons continuously sweep a radial arc within weapon range. If a hostile unit is detected, the unit engages autonomously.
   - **Guard Post Defense Grid:** Stationary bunkers linked via pressurized tubes autonomously lock onto hostile units within 6.5 tiles and fire twin defensive beams.
2. **Tactical Stances:**
   - **Guard Stance:** Anchors units to a tactical perimeter coordinate. Units pursue intruders within 6 tiles and automatically retreat back to their anchor once threats are eliminated.
   - **Patrol Stance:** Establishes bidirectional patrol routes between two coordinates, engaging targets of opportunity along the route.
   - **Halt Stance:** Clears all queued movement and targeting waypoints, immediately holding position.
3. **Vehicle Repair & Field Triage:**
   - **Repair Vehicles:** Mobile engineering units that home in on damaged friendly units ($HP < MaxHP$) and project emerald nano-repair beams (`#10b981`), restoring armor at 25 HP/s.
   - **Repair Depots:** Base installations generating a continuous healing diagnostic aura for stationed vehicles.
4. **Visual & Audio FX:**
   - Laser beams and kinetic slugs are rendered as multi-layer glowing energy lines with white-hot cores.
   - Upon target destruction ($HP \le 0$), a multi-stage explosive detonation shockwave expands and dissipates into planetary ash.
   - The tactical radar minimap blips hostile Plymouth forces in bright combat red (`#ef4444`).
