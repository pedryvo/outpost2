# Outpost 2 Natural Disasters & Planetary Hazards

New Terra is a geologically volatile world in active crustal tectonic transition. Colonies face severe environmental hazards that threaten life support systems, structures, and ground units.

## 1. Volcanic Eruptions & Fissures

- **Trigger Cycle:** Planetary mantle pressure builds over time, rupturing the crust in volcanic fissures every ~45 Sol seconds.
- **Surface Impact:** Fractures the terrain, transforming stable rock and clay cells into active molten lava tiles (`MapTile.is_lava = true`).
- **Seismic Shockwaves:** Tremors produce an authentic screen-shake camera offset that decays exponentially over $3.5$ seconds.
- **Unit Danger:** Any mobile unit or structure kit trapped within an advancing lava channel is immediately incinerated.

## 2. The Microbial Blight

- **Nature:** A synthetic bio-weaponized alien spore organism that feeds on copper alloys, polymer insulation, and colonial biological matter.
- **Spread Vector:** Creeps outward from infected epicenter tiles into adjacent cells marked `bMicrobePossible`.
- **Mitigation:** Colonists must research bio-containment walls or deploy Earthworker burn barriers to sterilize contaminated soil.
