# Outpost 2 WebAssembly Engine Architecture

This document provides a comprehensive technical overview of the Rust / WebAssembly port of *Outpost 2: Divided Destiny*, detailing the responsibilities of each module, data flow, memory model, and instructions for future development.

## 🏗️ Architecture Overview

```
                                  ┌───────────────────────────────┐
                                  │      Browser Host / DOM       │
                                  │   (HTML5 Canvas + Web Audio)  │
                                  └──────────────▲────────────────┘
                                                 │
                                           wasm-bindgen
                                                 │
                                  ┌──────────────▼────────────────┐
                                  │          src/lib.rs           │
                                  │  (GameApp Loop & DOM Events)  │
                                  └───┬───────┬───────┬───────┬───┘
                                      │       │       │       │
             ┌────────────────────────┘       │       │       └────────────────────────┐
             ▼                                ▼       ▼                                ▼
   ┌──────────────────┐               ┌───────────────┐               ┌──────────────────┐
   │    src/render/   │               │   src/sim/    │               │   src/archive/   │
   │  Isometric Grid  │               │ Colony State, │               │   .vol Binary    │
   │ & Canvas 2D View │               │ Units & .map  │               │ Streaming Parser │
   └─────────┬────────┘               └───────┬───────┘               └──────────────────┘
             │                                │
             ▼                                ▼
   ┌──────────────────┐               ┌───────────────┐
   │ src/render/      │               │  src/audio/   │
   │ palette.rs       │               │ Web Audio API │
   │ 256 Master Color │               │ Sound Manager │
   └──────────────────┘               └───────────────┘
```

---

## 📦 Module Breakdown

### 1. `src/lib.rs` (WASM Entry Point & Event Dispatcher)
- **Lifecycle:** Initialized on page load via `#[wasm_bindgen(start)]`.
- **Game Loop:** Runs via `requestAnimationFrame` passing exact delta time (`dt`) to the simulation update.
- **Input Handling:**
  - Mouse left-click: Unit selection (checks bounding distance in world space) or camera drag pan.
  - Mouse middle/right-click: Camera dragging.
  - Keyboard: `W`, `A`, `S`, `D` and arrow keys pan the camera; `+` and `-` zoom.
  - Command Buttons: `btn-build`, `btn-units`, `btn-research`, `btn-zoom-in`, `btn-zoom-out`.
- **HUD Telemetry:** Writes live colony statistics directly into DOM nodes (`#colony-sol`, `#colony-pop`, `#colony-power`, `#colony-ore`, `#colony-rare`, `#colony-food`) on each frame tick.

### 2. `src/archive/mod.rs` (Binary Archive Container Reader)
- **Responsibilities:** Parses Outpost 2 `.vol` containers (`art.vol`, `maps.vol`, `sound.vol`, `tech.vol`).
- **Section Parsing:**
  - Reads `VOL ` section header with 31-bit length and 1-bit padding mask (`raw_len & 0x7FFFFFFF`).
  - Reads `volh` (volume header metadata).
  - Reads `vols` (string table containing null-terminated file names).
  - Reads `voli` (index entries consisting of `filenameOffset`, `dataBlockOffset`, `fileSize`, `compressionType`).
- **File Retrieval:** `get_file(name)` locates internal assets in $O(N)$ with case-insensitive matching.

### 3. `src/sim/mod.rs` & `src/sim/map.rs` (Colony Simulation & Maps)
- **Colony State:** Tracks faction (Eden vs Plymouth), current Sol day, population, morale percentage, metal ore reserves, and power grid balance.
- **Unit Management:** Entities with `id`, `name`, `tile_x`, `tile_y`, `hp`, `max_hp`, and selection rings.
- **Map Reader (`map.rs`):**
  - Reads 20-byte `MapHeader` (`versionTag >= 0x1010`, `lgWidthInTiles`, `heightInTiles`, `tilesetCount`).
  - Decodes 32-bit tile bitfields into `MapTile` structs (`cell_type`, `tile_mapping`, `unit_index`, `is_lava`, `is_blight`, `is_wall_or_building`).
  - Embeds the official `eden01.map` (48 KB) as default playable terrain.

### 4. `src/render/mod.rs` & `src/render/palette.rs` (Isometric Graphics)
- **Coordinate Conversion:**
  - World Tile `(tx, ty)` to Screen `(sx, sy)`:
    $$\text{sx} = (\text{tx} - \text{ty}) \times (\text{tile\_width} \times 0.5 \times \text{zoom}) + \text{camera\_x}$$
    $$\text{sy} = (\text{tx} + \text{ty}) \times (\text{tile\_height} \times 0.5 \times \text{zoom}) + \text{camera\_y}$$
- **Viewport Culling:** Skips tiles outside screen bounds $[-100, \text{width} + 100]$.
- **Palette Shading:** Uses `MASTER_PALETTE: [[u8; 3]; 256]` extracted from `color.bmp` to render terrain, molten lava flows, and microbial blight trails.
- **Radar Minimap:** Renders an overview radar canvas blipping active structures and mobile units in real time.

### 5. `src/audio/mod.rs` (Web Audio Subsystem)
- **Browser Autoplay Compliance:** Lazy-initializes the `AudioContext` upon the first user interaction (canvas click or button tap).
- **Audio Feedback:** Synthesizes low-latency responsive sci-fi chimes and acknowledges unit selection and building placement orders.

---

## 🛠️ Tooling & Build Pipeline

| Tool | Purpose | Configuration |
|---|---|---|
| **mise** | Runtime tool pinning | `mise.toml` pins Rust `stable` and `github:thedodd/trunk@0.22.1` |
| **Trunk** | WASM application bundler | `Trunk.toml` bundles `index.html` and outputs to `dist/` |
| **wasm-opt** | Binary size reduction | Release builds run with `-Os` producing a ~59 KB `.wasm` binary |
| **Vercel** | Edge deployment | `vercel.json` provides `Content-Type: application/wasm` and cache headers |

---

## 🚀 How to Resume Work

1. Ensure tool shims are active:
   ```bash
   mise install
   ```
2. Start the local development server:
   ```bash
   trunk serve
   ```
3. Open `http://127.0.0.1:8080`.
4. Review the roadmap milestones in [`docs/PLAN.md`](PLAN.md).
