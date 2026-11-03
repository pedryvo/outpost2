# Outpost 2 Sprite Animation & Texture Atlas Architecture

This document specifies the sprite decoding, orientation calculation, animation state machine, and pixel rendering pipeline implemented in `src/render/sprites.rs` for the Outpost 2 Divided Destiny Rust/WASM engine.

---

## 1. 256-Color Palette Index Mapping

Outpost 2 graphics rely on an 8-bit indexed color model decoded from `OP2_ART.BMP` using palette tables stored in `op2_art.prt`.

### Palette Table Hierarchy
- **Master Palette (`MASTER_PALETTE`):** Standard fallback 256-color palette extracted from `color.bmp` inside `art.vol`.
- **CPAL Block:** Contains 9 specialized palettes (256 colors $\times$ 4 bytes `B, G, R, 0`):
  - `Palette 0`: Standard UI and terrain palette.
  - `Palette 1`: Eden faction baseline colors (cyan, cobalt blue, steel grey).
  - `Palette 2`: Plymouth faction baseline colors (crimson, ember orange, dark slate).
  - `Palette 3`: Eden unit chassis & vehicle highlights.
  - `Palette 5`: Explosion fireballs, burning debris, lava, and high-energy projectiles.
  - `Palette 6`: Vehicle turret mounts and heavy energy weapons.
  - `Palette 7`: Plymouth unit chassis & vehicle highlights.
  - `Palette 8`: Subterranean and mining equipment overlays.

### Pixel Shading & Transparency Rules
1. **Color Index 0 (Transparent):**
   - Palette index `0x00` is strictly reserved as transparent alpha (`rgba(0, 0, 0, 0)`).
   - The rasterizer skips blending when encountering index 0.
2. **Color Indices 1..=255 (Opaque):**
   - Mapped to full opacity `[R, G, B, 255]` using the assigned palette.
3. **Shadow Mask Mode (`imageType & 4 != 0`):**
   - 1-bit monochrome mask packed into scanlines.
   - Each set bit (`1`) renders semi-transparent black shadow (`rgba(0, 0, 0, 0.43)` / alpha `110`) cast onto terrain.
4. **Faction Color Remapping:**
   - Eden units dynamically override the palette index to `Palette 3`.
   - Plymouth units dynamically override the palette index to `Palette 7`.

---

## 2. 8-Direction Orientation Frame Calculation

Vehicles, mobile chassis, and turrets support 8 discrete rotational facings:
`North (0)`, `NorthEast (1)`, `East (2)`, `SouthEast (3)`, `South (4)`, `SouthWest (5)`, `West (6)`, `NorthWest (7)`.

```
                  North (0)
                     ^
       NorthWest (7) |  NorthEast (1)
               \     |     /
                \    |    /
       West (6) <----+----> East (2)
                /    |    \
               /     |     \
       SouthWest (5) |  SouthEast (3)
                     v
                  South (4)
```

### Algorithm

#### 1. Screen Angle Conversion
Given a movement velocity or aiming vector $(\Delta x, \Delta y)$ in 2D screen coordinates (+X = East, +Y = South):

$$\theta = \text{atan2}(\Delta y, \Delta x)$$

$$\theta_{\text{norm}} = (\theta + 2\pi) \pmod{2\pi}$$

Each compass slice occupies $45^\circ$ ($\pi / 4$ radians). To center the primary compass axes, the angle is shifted by half a sector ($22.5^\circ$ or $\pi / 8$ radians):

$$\text{sector} = \left\lfloor \frac{\theta_{\text{norm}} + \frac{\pi}{8}}{\frac{\pi}{4}} \right\rfloor \pmod 8$$

Mapping from sector to `Direction8`:
- `0`: **East**
- `1`: **SouthEast**
- `2`: **South**
- `3`: **SouthWest**
- `4`: **West**
- `5`: **NorthWest**
- `6`: **North**
- `7`: **NorthEast**

#### 2. Isometric Tile Delta Conversion
For discrete unit tile movement from $(tx_1, ty_1)$ to $(tx_2, ty_2)$ on a 2:1 isometric diamond grid:

$$\Delta sx = (dtx - dty) \cdot \frac{W_{\text{tile}}}{2}$$

$$\Delta sy = (dtx + dty) \cdot \frac{H_{\text{tile}}}{2}$$

$$\text{Direction8} = \text{Direction8::from\_screen\_vector}(\Delta sx, \Delta sy)$$

---

## 3. Vehicle Chassis & Turret Sprite Mappings

Each unit type maps to a base sprite frame index in `op2_art.prt`:

| Unit Type | Eden Base ID | Plymouth Base ID | Dimensions | Notes |
|---|---|---|---|---|
| **Lynx Scout** | `264` | `3128` | $33 \times 26$ px | Fast light chassis with twin treads |
| **Panther Tank** | `1200` | `3112` | $36 \times 24$ px | Medium battle tank chassis |
| **Tiger Heavy** | `1592` | `1608` | $47 \times 26$ px | Heavy siege chassis with elongated hull |
| **Cargo Truck** | `4081` | `4097` | $29 \times 35$ px | Resource hauler (empty / ore loaded) |
| **Robo-Miner** | `4320` | `4320` | $58 \times 36$ px | Subterranean mobile mining excavator |
| **Earthworker** | `4776` | `4776` | $50 \times 38$ px | Bulldozer with terraforming blade |
| **ConVec** | `4344` | `4344` | $28 \times 24$ px | Structure kit transporter |
| **Robo-Surveyor** | `944` | `944` | $15 \times 19$ px | Automated recon surveyor |
| **Repair Vehicle**| `808` | `808` | $22 \times 16$ px | Mobile field engineering van |

### Frame Calculation Formula

$$\text{Frame ID} = \text{BaseID} + (\text{tread\_frame} \times 8) + \text{direction.frame\_offset}()$$

- When stationary, $\text{tread\_frame} = 0$.
- When moving, $\text{tread\_frame}$ toggles between `0` and `1` every $120\text{ ms}$, creating continuous caterpillar track animation.

### Independent Turrets & Weapon Mounts
Armed vehicles calculate turret orientation independently from the chassis heading:
- **Eden Turrets:** Base frame `4052` + $\text{direction.frame\_offset}()$.
- **Plymouth Turrets:** Base frame `4060` + $\text{direction.frame\_offset}()$.
- **Heavy Tiger Railgun:** Base frame `4068` + $\text{direction.frame\_offset}()$.

### Robo-Miner Drill Cycle
When active on an ore beacon (`is_drilling == true`), the drill cycles through 8 rotary cutting frames (`drill_frame` 0..=7) every $80\text{ ms}$.

---

## 4. Multi-Stage Explosion Fireball Sequences

Vehicle destruction and ordnance detonations play a 12-frame multi-stage explosion sequence starting at sprite ID `4449` ($48 \times 31$ px, `Palette 5`):

```
+------------------------------------------------------------------------+
| Frame 0..2  | Ignition Stage    | Intense white-hot detonation core    |
| Frame 3..5  | Expansion Stage   | Blooming orange/yellow fireball      |
| Frame 6..8  | Peak Stage        | Shockwave ring & black soot smoke    |
| Frame 9..11 | Dissipation Stage | Billowing smoke & dying embers       |
+------------------------------------------------------------------------+
```

### Stage Timing
- **Frame Duration:** $50\text{ ms}$ per frame ($600\text{ ms}$ total animation lifetime).
- **Stage Progression:**
  1. `Ignition (0..=2)`: Blinding flash with high energy radius ($r \approx 6\text{ px}$ to $12\text{ px}$).
  2. `Expansion (3..=5)`: Burning hydrocarbon fireball ($r \approx 14\text{ px}$ to $22\text{ px}$).
  3. `Peak (6..=8)`: Shockwave ring detonation ($r \approx 24\text{ px}$ to $32\text{ px}$).
  4. `Dissipation (9..=11)`: Thinning smoke clouds and cooling embers with fading alpha.

---

## 5. Rendering Methods

### 1. Memory RGBA Pixel Buffer (`draw_to_rgba_buffer`)
Blits decoded sprite pixels directly into raw memory byte slices `&mut [u8]` (e.g. software framebuffers, offscreen textures):
- Supports full viewport bounding box clipping (handles negative $(x, y)$ coordinates and border overflow).
- Fast-path for opaque pixels ($\alpha = 255$).
- Integer alpha blending for semi-transparent shadow masks:

$$\text{dest}_c = \frac{\text{src}_c \cdot \alpha_{\text{src}} + \text{dest}_c \cdot (255 - \alpha_{\text{src}})}{255}$$

### 2. HTML5 Canvas Rendering (`draw_to_canvas`)
Blits sprites to HTML5 `CanvasRenderingContext2d` using `web_sys::ImageData` and `put_image_data`.

### 3. Procedural Vector Sprite Fallback
When running without external art assets extracted, `UnitSpriteManager` uses clean procedural vector rendering with full 8-directional rotation, animated caterpillar tracks, independent weapon turrets, drilling bits, and multi-stage fireball shockwaves.
