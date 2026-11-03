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

## 3. Kinetic Meteor Showers & Impact Detonations

- **Atmospheric Entry:** Hypersonic kinetic orbital fragments plummet across Martian coordinates, impacting surface terrain with blast radius detonations.
- **Infrastructure Damage:** Severed transport conduit tubes, structure health degradation, and lethal area-of-effect blast damage to unshielded vehicles.
- **Seismic Tremor:** Generates localized ground quakes and visual shockwave rings registered across tactical radar sensors.

## 4. Red Alert Siren & Plymouth Incursion Raids

- **Emergency Alert Acoustic System:** Command Center synthesizes a dual-tone sawtooth red alert siren (alternating between 587.33 Hz and 880.0 Hz) upon incoming impact detonations or hostile tactical strike waves.
- **Hostile Plymouth Assault Squads:** Rival faction AI directors organize periodic coordinated combat incursions every 2nd reinforcement cycle, deploying Tiger Railgun tanks and Raider escorts with march-to-contact orders targeting player perimeter structures and ore haulers.
- **Voice Radio Dispatch:** Synthesized computerized tactical voice dispatches broadcast urgent audio status advisories to player commanders in real-time.
