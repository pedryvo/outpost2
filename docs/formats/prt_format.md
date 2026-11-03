# Outpost 2 PRT (`op2_art.prt`) Specification

The `op2_art.prt` file (contained inside `art.vol`) is the **Palette and Resource Table** of *Outpost 2: Divided Destiny*. It defines all 5,390 game sprites, animations, secondary palettes, and coordinate mappings that decode the pixel data stored in `OP2_ART.BMP`.

## Binary Structure

A `.prt` file contains two primary sections in sequential order:

```
+---------------------------------------+
| CPAL (Palette Container)              |
|  - Palette count (typically 9)        |
|  - 9x PPAL chunks (256 colors each)   |
+---------------------------------------+
| ImageMetadata Array                   |
|  - Sprite count (5,390 sprites)       |
|  - 5,390x ImageMeta records (20 bytes)|
+---------------------------------------+
| Animation Sequences                   |
|  - Frame indices and layer offsets    |
+---------------------------------------+
```

## 1. Palette Section (`CPAL`)

- `tag`: Fixed 4 bytes `"CPAL"`.
- `palette_count`: 32-bit unsigned integer (e.g. `9`).
- Followed by `palette_count` blocks, each containing:
  - `PaletteHeader`: 28 bytes (containing `PPAL`, `head`, tag count, `data` headers).
  - 256 colors $\times$ 4 bytes (`B, G, R, 0`).

## 2. Sprite Metadata Record (`ImageMeta` - 20 Bytes)

Immediately following the palettes, a 32-bit integer specifies the sprite count (e.g. `5,390`). Each sprite entry is exactly 20 bytes:

| Offset | Type | Field | Description |
|---|---|---|---|
| `0x00` | `uint32` | `scanLineByteWidth` | Width of each scanline in bytes (aligned to 4-byte boundaries) |
| `0x04` | `uint32` | `pixelDataOffset` | Byte offset of raw pixel data inside `OP2_ART.BMP` |
| `0x08` | `uint32` | `height` | Sprite height in pixels |
| `0x0C` | `uint32` | `width` | Sprite width in pixels |
| `0x10` | `uint16` | `imageType` | Bitfield flags (Bit 0 = Game vs Menu graphic, Bit 2 = Shadow mask 1-bit) |
| `0x12` | `uint16` | `paletteIndex` | Index of the color palette (0..=8) used to shade this sprite |

## 3. Pixel Decoding Algorithm

To extract and render sprite ID $N$ onto HTML5 Canvas / WebGL:
1. Lookup `ImageMeta` record at index $N$.
2. Seek `OP2_ART.BMP` to offset `pixelDataOffset`.
3. If `imageType.isShadow` is `1`:
   - Read 1-bit monochrome mask (each byte contains 8 binary pixels).
   - Render transparent black shadow overlay on terrain.
4. If `imageType.isShadow` is `0`:
   - Read 8-bit indexed bytes across `height` scanlines with stride `scanLineByteWidth`.
   - Index pixels through `CPAL.palettes[paletteIndex]`.
   - Palette color 0 is treated as transparent alpha.
