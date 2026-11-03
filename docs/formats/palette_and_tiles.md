# Outpost 2 Palette & Tileset ("Wells") Specification

*Outpost 2* uses an 8-bit indexed 256-color graphic subsystem optimized for 1997 PC hardware.

## Master Palette (`color.bmp`)

The authoritative 256 RGB colors used by Outpost 2 are packed in `color.bmp` inside `art.vol`:

- **Container:** Windows 3.x Bitmap header (54 bytes) + 256 `[B, G, R, 0]` palette entries (1024 bytes).
- **Extracted Array:** Encoded directly into [`src/render/palette.rs`](../../src/render/palette.rs) as `MASTER_PALETTE: [[u8; 3]; 256]`.
- **Key Palette Ranges:**
  - `0`: Transparent / Key color.
  - `1..=15`: Interface gray tones, steel bevels, and shadows.
  - `16..=31`: Basalt, crater rim, and volcanic rock dark hues.
  - `32..=63`: Martian iron-oxide red soils and New Terra clay dust.
  - `64..=95`: Ore deposits, copper veining, and rare earth minerals.
  - `96..=127`: Eden faction cyan/sky-blue neon highlights.
  - `128..=159`: Plymouth faction industrial safety-orange/gold highlights.
  - `240..=255`: Molten magma glow, weapon plasma, and laser beams.

## Tileset Well Archives (`well0000.bmp` .. `well0012.bmp`)

Terrain tiles are grouped into "wells" inside `art.vol`:

| File | Width | Height | Tile Count | Description |
|---|---|---|---|---|
| `well0000.bmp` | 32 px | 32 px | 1 | Base bedrock tile |
| `well0001.bmp` | 32 px | 8,608 px | 269 | Red desert terrain transitions |
| `well0002.bmp` | 32 px | 5,216 px | 163 | Basalt craters and rock plateaus |
| `well0003.bmp` | 32 px | 192 px | 6 | Flat foundations |
| `well0004.bmp` | 32 px | 11,488 px | 359 | Cliff edges and elevation ramps |
| `well0005.bmp` | 32 px | 4,704 px | 147 | Ore deposit veins (common & rare) |
| `well0007.bmp` | 32 px | 6,624 px | 207 | Volcanic fissures and lava beds |
| `well0008.bmp` | 32 px | 11,104 px | 347 | Roads, tube connectors, and bulldozed pads |

Each well is formatted as a vertical strip of 32×32 pixel tiles (each tile takes 1,024 bytes of pixel data).
