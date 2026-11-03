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

## Game Engine Mechanics

1. **Laboratories:** Research is conducted inside active Laboratories with assigned scientists.
2. **Ore Investment:** Advancing research deducts Common Ore from the colony stockpile per Sol tick.
3. **Prerequisites:** Higher-tier technologies require completion of parent tech nodes before becoming available in the Research command interface.
