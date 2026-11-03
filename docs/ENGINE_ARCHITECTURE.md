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

### 5. `src/sim/disasters.rs` & `src/sim/fog.rs` (Planetary Hazards & Sensor Grid)
- **Disaster Simulation:** Volcanic fissures erupt along geological fault lines, converting solid basalt into impassable molten lava (`is_lava = true`). Generates seismic screen tremors decaying exponentially over time.
- **Fog of War:** 3-tier visibility model (`Unexplored`, `Explored`, `Visible`). Units radiate Euclidean sensor circles ($R = 6-8$ tiles). Unexplored tiles are shrouded in deep space black; explored tiles retain terrain topography with muted luminance (0.35×); active sensor tiles render full vitality and live mobile units.
- **Diurnal Solar Cycle:** Planetary day/night cycle modulates ambient surface illumination (0.2× to 1.0×) and governs solar power generation from operational solar arrays.

### 6. `src/sim/structures.rs` & Transport Tube Grid
- **Colony Structures:** Defines structural footprint, common/rare ore costs, and power balances for Command Centers, Solar Power Arrays, Tokamak Reactors, Ore Smelters, DIRT Labs, Standard Labs, Bio-Domes, and Vehicle Factories.
- **Pressurized Tubes:** Buildings are linked via automated orthogonal titanium conduits with internal glowing cyan life-support arteries.

### 7. State Persistence & Mobile Touch Controls
- **Progress Persistence:** `SaveGameData` captures colony metadata, ore reserves, units, buildings, transport tubes, and researched tech nodes into a JSON schema persisted directly to `window.localStorage("op2_colony_save")`.
- **Mobile Gestures:** Full touch event support (`touchstart`, `touchmove`, `touchend`) with single-finger viewport drag panning and multi-touch pinch-to-zoom.

### 8. `src/sim/combat.rs` & `src/sim/ai.rs` (Tactical Warfare & Rival AI)
- **Combat Simulation:** Real-time engagement mechanics for pulse lasers, microwave emitters, railguns, RPGs, EMP arcs, and acid clouds. Firing units project multi-layer glowing energy beams (`CombatBeam`) and generate fiery concussive shockwaves (`ExplosionEffect`) on target destruction.
- **Rival Faction AI Director:** Governs hostile Plymouth outpost operations, patrol waypoints, territorial defense, and automated wave reinforcements.

### 9. `src/audio/radio.rs` & `src/audio/mod.rs` (Audio & Speech Dispatch)
- **Radio Dispatcher:** Employs the Web Speech API with pitched military cadences to speak critical alerts ("Disaster warning", "Hostile Plymouth reinforcements deployed", "Research completed").
- **Web Audio FX:** Synthesizes low-latency responsive sci-fi chimes for selection, move acknowledgments, and building placement.

### 10. `src/i18n.rs` (Live In-Game Localization)
- Real-time language switching without page reload across English, Portuguese (BR), Spanish, German, Simplified Chinese, and Russian.

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
