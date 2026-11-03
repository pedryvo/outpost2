# Development Plan: Outpost 2 Divided Destiny Web Port (Rust + WASM)

Porting Sierra / Dynamix's 1997 hard sci-fi real-time strategy classic *Outpost 2: Divided Destiny* to the modern Web using **Rust, WebAssembly (WASM), HTML5 Canvas, and Web Audio API**.

## Phases

### Phase 1: REA Reverse Engineering & Binary Architecture Mapping (Completed)
- [x] Analyze `Outpost2.exe` with REA MCP & Ghidra (extract PE inventory, memory segments, 3,068 procedures, and 2,498 text literals).
- [x] Document the 509 exported C++ engine symbols and class hierarchies (`re/engine_symbols.json`).
- [x] Analyze resource containers (`art.vol`, `maps.vol`, `sound.vol`, `voices.vol`, `tech.vol`).
- [x] Map legacy Win32 dependencies (`GDI32`, `USER32`, `KERNEL32`) for native SDL2/Canvas/WebAudio abstraction.
- [x] Map the `.vol` format header and compression algorithms (uncompressed, LZH, RLE).

### Phase 2: Binary Parsers & Asset Pipeline in Rust (In Progress)
- [x] Zero-copy `.vol` volume archive reader in Rust (`VolArchive`).
- [x] 256-color palette parser and master color translation tables.
- [x] Bitmap tile decoder (`OP2_ART.BMP` and well archives).
- [ ] Map file parser (`.map` files from `maps.vol` containing cell types, cliffs, and ore deposits).
- [ ] Web Audio sound manager for PCM effects and ambient voices.

### Phase 3: Isometric Map Engine & Viewport
- [ ] High-performance 2D isometric tile grid renderer with WebAssembly memory buffers.
- [ ] Multi-layer terrain rendering: base terrain, elevation/cliffs, lava/microbial blight paths, and structure kit overlays.
- [ ] Viewport scrolling, edge panning, keyboard WASD, and touch drag gestures.
- [ ] Authentic fog-of-war and dynamic day/night light cycle.
- [ ] Smooth zoom (1x, 2x, 3x, Fit-to-Screen) with crisp pixel-art preservation.

### Phase 4: Colony Simulation & RTS Core Mechanics
- [ ] Colony factions: Eden and Plymouth colony parameters.
- [ ] Structure grid placement: Command Center, Solar Panels, Smelters, Garages, Nurseries, and connecting tubes.
- [ ] Logistics & Life Support: Food, power grid connectivity, metal ore extraction, and colonist morale.
- [ ] Unit movement & pathfinding (A* on isometric grid): ConVec, Cargo Truck, Robo-Miner, Earthworker.
- [ ] Construction cycle: Deploying structure kits from ConVecs.

### Phase 5: Modern Sci-Fi Tactical UI & Accessibility
- [ ] Modernized tactical HUD:
  - Top command bar (Colony health, population, power status, ore reserves, game speed).
  - Bottom interactive command console (Unit orders, factory queues, building stats).
  - Real-time radar minimap with interactive jump-to-location.
- [ ] Responsive design supporting ultrawide desktop displays, laptops, and mobile screens.
- [ ] Touch virtual controls for mobile tablets and smartphones.
- [ ] Multi-language support (EN, PT-BR, ES, DE, ZH-CN, RU).

### Phase 6: Vercel Deployment & Persistence
- [ ] Automated Trunk release build with LTO and binary optimization.
- [ ] Zero-latency edge deployment on Vercel with streaming WebAssembly instantiation.
- [ ] IndexedDB save state manager to persist campaign progress across browser sessions.
