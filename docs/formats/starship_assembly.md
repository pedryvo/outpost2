# Outpost 2 — Evacuation Starship Assembly & Demographics

## 1. Lore & Objective Context
In Outpost 2: Divided Destiny, humanity's ultimate survival relies on escaping New Terra before volcanic catastrophes and the spreading Blight engulf the planet. Both the Eden Foundation and Plymouth faction race to construct an interstellar evacuation vessel.

## 2. Starship Modules
Fabrication occurs in colonial factories and orbital launch facilities:
1. **Orbital Command Bridge:** Primary navigation computers and sub-light sensor arrays (25% assembly).
2. **Cryo-Stasis Habitat:** Deep-sleep chambers for hundreds of civilian colonists (50% assembly).
3. **Ionized Fusion Thruster:** High-impulse reaction drives for interplanetary escape (75% assembly).
4. **Life Support & Radiation Shield:** Closed-loop oxygen recycle and planetary magnetic shielding (100% Launch Readiness).

## 3. Demographics & Life Support Grid
Colony morale and population health are strictly bound to life support energy:
- **Positive Energy Surplus (`Power Produced >= Power Consumed`):** Life support operates normally, morale steadily climbs toward 100%.
- **Negative Energy Deficit (`Power Produced < Power Consumed`):** Critical life-support failure. Morale drops. If morale falls below 50%, civilian casualties occur periodically.

## 4. Factory Queue & Fabrication Animation
- Vehicles and Starship components are ordered via `queue_production(building_id, item)`.
- During active manufacturing, the factory displays an animated electromagnetic welding spark beam (`rgba(56, 189, 248, pulse)`) and a progress bar above the structure roof.
- Upon reaching 100%, units roll directly out of the factory bay into service.
