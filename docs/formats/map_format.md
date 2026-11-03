# Outpost 2 Map File (`.map`) Specification

The `.map` file format stores terrain elevation, cell movement passability, lava flow paths, microbial blight infestation, and tile graphics mappings for *Outpost 2: Divided Destiny*.

## Map Header Structure (`MapHeader`)

The first 20 bytes of every `.map` file form the binary header:

| Offset | Type | Field | Description |
|---|---|---|---|
| `0x00` | `uint32` | `versionTag` | Map version number (must be `>= 0x1010`, typically `0x1011`) |
| `0x04` | `int32` | `bSavedGame` | `0` for raw map files, `1` for saved game states (`.op2`) |
| `0x08` | `uint32` | `lgWidthInTiles` | Base-2 logarithm of width (Width in tiles = `1 << lgWidthInTiles`) |
| `0x0C` | `uint32` | `heightInTiles` | Height of the map grid in tiles (e.g. 64, 128, 256, 512) |
| `0x10` | `uint32` | `tilesetCount` | Number of tile sets referenced by this map (e.g. 512) |

## Tile Grid Cell Bitfield (`Tile` - 4 Bytes)

Immediately following the 20-byte header, the map stores `WidthInTiles * HeightInTiles` cells as 32-bit little-endian bitfields:

```
Bit:  31  30  29  28  27  26       16 15           5 4       0
     [ W | M | E |LP | L | Unit Index | Tile Mapping | CellType ]
```

| Bits | Width | Field | Description |
|---|---|---|---|
| `0..=4` | 5 | `cellType` | Terrain movement speed / surface status (flat, rough, rock, bulldozed, walled, tubed, rubble) |
| `5..=15` | 11 | `tileMappingIndex` | Graphic index referencing the tileset source ("well") |
| `16..=26` | 11 | `unitIndex` | ID of the active unit occupying this tile (0 if unoccupied) |
| `27` | 1 | `bLava` | `1` if molten lava is currently active on this tile |
| `28` | 1 | `bLavaPossible` | `1` if lava can legally spread into this tile |
| `29` | 1 | `bExpansion` | Used internally by disaster simulator for lava/blight advance |
| `30` | 1 | `bMicrobe` | `1` if the deadly alien Microbial Blight infests this tile |
| `31` | 1 | `bWallOrBuilding` | `1` if a fortified wall or structure occupies the tile |

## Clipping and Tileset Source Blocks

Following the raw tile grid:
1. `clipRect`: 16 bytes (`left`, `top`, `right`, `bottom` in tiles) defining the playable bounds.
2. `tilesetSources`: Array of tileset filenames (up to 8 characters + tile count) specifying which `well` graphics archives are needed.
3. Signature marker: Fixed 10-byte string `"TILE SET\x1a"`.
4. `tileMappings`: Animation frames and graphic lookup tables.
5. `terrainTypes`: Movement passability matrix.
