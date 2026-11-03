# Development Plan: Outpost 2 Divided Destiny Web Port (Rust + WASM)

Porting Sierra / Dynamix's 1997 hard sci-fi real-time strategy classic *Outpost 2: Divided Destiny* to the modern Web using **Rust, WebAssembly (WASM), HTML5 Canvas, and Web Audio API**.

---

## Roadmap & Milestone Completion

### Phase 1: REA Reverse Engineering & Binary Architecture Mapping (Completed)
- [x] **Static PE Analysis & Disassembly**: Disassembled `Outpost2.exe` via REA MCP server and Ghidra, identifying memory segments, 3,068 procedures, and 2,498 string literals.
  - *Artifacts & Specs*: [`re/engine_symbols.json`](../re/engine_symbols.json), [`docs/formats/vol_archive.md`](formats/vol_archive.md)
  - *Code References*: `src/archive/mod.rs`
- [x] **C++ Engine Symbol Documentation**: Cataloged 509 exported symbols across dynamic classes (`TApp`, `GameMap`, `Unit`, `Building`, `Research`, `Packet`).
  - *Artifacts & Specs*: [`re/engine_symbols.json`](../re/engine_symbols.json), [`docs/ENGINE_ARCHITECTURE.md`](ENGINE_ARCHITECTURE.md)
- [x] **Archive Container Mapping**: Reversed the `.vol` archive format used across `art.vol`, `maps.vol`, `sound.vol`, `voices.vol`, and `tech.vol`.
  - *Artifacts & Specs*: [`docs/formats/vol_archive.md`](formats/vol_archive.md)
  - *Code References*: `src/archive/mod.rs`
- [x] **Legacy Win32 Subsystem Decoupling**: Identified and abstracted `GDI32`, `USER32`, and `KERNEL32` subsystems into modern HTML5 Canvas, Web Audio, and WebRTC layers.
  - *Artifacts & Specs*: [`docs/ENGINE_ARCHITECTURE.md`](ENGINE_ARCHITECTURE.md)
  - *Code References*: `src/lib.rs`, `src/render/mod.rs`

### Phase 2: Binary Parsers & Asset Pipeline in Rust (Completed)
- [x] **Zero-Copy Volume Container Reader**: Implemented high-performance streaming `.vol` parser in pure Rust.
  - *Artifacts & Specs*: [`docs/formats/vol_archive.md`](formats/vol_archive.md)
  - *Code References*: `src/archive/mod.rs`
- [x] **256-Color Palette Decoders**: Extracted master color table and faction translation remap tables (Eden cyan, Plymouth red).
  - *Artifacts & Specs*: [`docs/formats/palette_and_tiles.md`](formats/palette_and_tiles.md)
  - *Code References*: `src/render/palette.rs`
- [x] **Sprite Definition & Bitmap Decoders**: Built decoders for `OP2_ART.BMP` and `.prt` sprite metadata (5,390 frames).
  - *Artifacts & Specs*: [`docs/formats/prt_format.md`](formats/prt_format.md), [`docs/formats/sprites_and_animation.md`](formats/sprites_and_animation.md)
  - *Code References*: `src/render/tiles.rs`, `src/render/sprites.rs`
- [x] **Binary Map Parsers**: Reversed `.map` format containing cell types, cliff geometry, cell flags, and mineral beacon locations.
  - *Artifacts & Specs*: [`docs/formats/map_format.md`](formats/map_format.md)
  - *Code References*: `src/sim/map.rs`
- [x] **WASM Sound Bank & Voice Player**: Integrated Web Audio manager and 252-entry sound table with Kathy Cullis tactical radio speech dispatches.
  - *Artifacts & Specs*: [`docs/formats/sound_and_voices.md`](formats/sound_and_voices.md)
  - *Code References*: `src/audio/mod.rs`, `src/audio/bank.rs`

### Phase 3: Isometric Map Engine & Viewport (Completed)
- [x] **High-Performance Isometric Grid Renderer**: 2D diamond-grid isometric renderer blitting directly to HTML5 Canvas.
  - *Artifacts & Specs*: [`docs/formats/palette_and_tiles.md`](formats/palette_and_tiles.md)
  - *Code References*: `src/render/mod.rs`, `src/render/tiles.rs`
- [x] **Multi-Layer Terrain & 2.5D Cliffs**: Elevated plateau rendering with drop-shadows, cliff rims, and dynamic surface hazards (lava, microbial blight).
  - *Artifacts & Specs*: [`docs/formats/palette_and_tiles.md`](formats/palette_and_tiles.md), [`docs/formats/disasters.md`](formats/disasters.md)
  - *Code References*: `src/render/mod.rs`, `src/sim/map.rs`
- [x] **Viewport Camera Controls & Touch Ergonomics**: Sub-pixel panning, edge scrolling, keyboard navigation, and mobile pinch-to-zoom / two-finger drag.
  - *Artifacts & Specs*: [`docs/formats/wasm_packaging_and_web_runtime.md`](formats/wasm_packaging_and_web_runtime.md)
  - *Code References*: `src/render/touch.rs`, `src/render/mod.rs`
- [x] **Fog of War & Sensor Matrix**: 3-tier visibility (Unexplored, Explored, Visible) updated in real-time by structure and unit sight radii.
  - *Artifacts & Specs*: [`docs/formats/fog_of_war.md`](formats/fog_of_war.md)
  - *Code References*: `src/sim/map.rs`, `src/render/mod.rs`
- [x] **Diurnal Planetary Day/Night Light Cycle**: Smooth solar flux ambient shading transitioning between Day, Twilight, and Night.
  - *Artifacts & Specs*: [`docs/formats/palette_and_tiles.md`](formats/palette_and_tiles.md)
  - *Code References*: `src/render/mod.rs`, `src/sim/mod.rs`

### Phase 4: Colony Simulation & RTS Core Mechanics (Completed)
- [x] **Colony Logistics Core**: Full Eden and Plymouth resource economics: Common Ore, Rare Ore, Food, Energy Grid balancing, and Colonist Morale tiers.
  - *Artifacts & Specs*: [`docs/formats/campaigns_and_scenarios.md`](formats/campaigns_and_scenarios.md), [`docs/formats/mining_and_logistics.md`](formats/mining_and_logistics.md)
  - *Code References*: `src/sim/mod.rs`, `src/sim/structures.rs`
- [x] **Modular Base Structures & Transport Tubes**: 13 structure types (Command Center, Smelters, Geothermal Plants, Tokamaks, Factories, Guard Posts) and pressurized tube connectivity.
  - *Artifacts & Specs*: [`docs/formats/structures_and_tubes.md`](formats/structures_and_tubes.md)
  - *Code References*: `src/sim/structures.rs`
- [x] **8-Directional Isometric A\* Pathfinding**: Vector navigation with hazard avoidance (lava, blight) and dynamic vehicle collision.
  - *Artifacts & Specs*: [`docs/formats/pathfinding.md`](formats/pathfinding.md), [`docs/ENGINE_ARCHITECTURE.md`](ENGINE_ARCHITECTURE.md)
  - *Code References*: `src/sim/pathfinding.rs`, `src/sim/mod.rs`
- [x] **Automated Mining Logistics**: Robo-Miner subterranean mine head excavation, mineral beacon yield scaling, and automated Cargo Truck hauling loops.
  - *Artifacts & Specs*: [`docs/formats/mining_and_logistics.md`](formats/mining_and_logistics.md)
  - *Code References*: `src/sim/mining.rs`, `src/sim/mod.rs`
- [x] **Unit Combat & Tactical Stances**: Tiger heavy tanks, supersonic railguns, Lynxes, laser targeting, guard anchors, and patrol routes.
  - *Artifacts & Specs*: [`docs/formats/combat_and_weapons.md`](formats/combat_and_weapons.md)
  - *Code References*: `src/sim/mod.rs`, `src/render/sprites.rs`
- [x] **Scientific Research Matrix**: 11-discipline university tech tree with prerequisite unlocks and passive colony benefits.
  - *Artifacts & Specs*: [`docs/formats/tech_tree.md`](formats/tech_tree.md)
  - *Code References*: `src/sim/tech.rs`, `src/render/hud.rs`

### Phase 5: Modern Sci-Fi Tactical UI & Accessibility (Completed)
- [x] **Responsive High-Tech HUD**: Live colony telemetry bar, minimap radar with jump-to-location, production queue drawer, and research browser.
  - *Artifacts & Specs*: [`docs/formats/ui_and_hud.md`](formats/ui_and_hud.md)
  - *Code References*: `src/render/hud.rs`
- [x] **Mobile Touch Controls**: Multi-touch pinch-to-zoom, two-finger panning, single-tap selection, and long-press action recognition.
  - *Artifacts & Specs*: [`docs/formats/wasm_packaging_and_web_runtime.md`](formats/wasm_packaging_and_web_runtime.md)
  - *Code References*: `src/render/touch.rs`
- [x] **6-Language Localization Engine**: Multi-language dictionary (EN, PT-BR, ES, DE, ZH-CN, RU) with runtime locale detection and English fallback.
  - *Artifacts & Specs*: [`docs/formats/i18n_and_localization.md`](formats/i18n_and_localization.md)
  - *Code References*: `src/i18n.rs`

### Phase 6: Vercel Deployment & Persistence (Completed)
- [x] **Optimized WebAssembly Pipeline**: Standalone ~370 KB WASM release build with Link-Time Optimization (LTO).
  - *Artifacts & Specs*: [`docs/formats/wasm_packaging_and_web_runtime.md`](formats/wasm_packaging_and_web_runtime.md)
  - *Code References*: `Trunk.toml`, `Cargo.toml`
- [x] **Edge Deployment Configuration**: High-performance HTTP caching and streaming WASM headers for Vercel and modern CDNs.
  - *Artifacts & Specs*: [`docs/formats/pwa_and_offline_deployment.md`](formats/pwa_and_offline_deployment.md)
  - *Code References*: `vercel.json`
- [x] **State Persistence & Standalone `.op2save` Archives**: LocalStorage 5-slot persistence, F5 QuickSave, F9 QuickLoad, 100-Sol AutoSave, and standalone `.op2save` file export/import.
  - *Artifacts & Specs*: [`docs/formats/save_games_and_lobby.md`](formats/save_games_and_lobby.md)
  - *Code References*: `src/sim/save.rs`

### Phase 7: Authentic Campaigns, Starship Assembly & Production Queues (Completed)
- [x] **6 Canonical Campaign Theaters**: Full Eden 1–3 and Plymouth 1–3 missions with authentic `.map` terrain layouts.
  - *Artifacts & Specs*: [`docs/formats/campaign_progression_and_debrief.md`](formats/campaign_progression_and_debrief.md)
  - *Code References*: `src/sim/mission.rs`, `src/sim/map.rs`
- [x] **Evacuation Starship Multi-Stage Assembly**: 4-stage escape vessel fabrication (Command Bridge, Cryo-Stasis, Fusion Thruster, Life Support Shield).
  - *Artifacts & Specs*: [`docs/formats/starship_assembly.md`](formats/starship_assembly.md)
  - *Code References*: `src/sim/mod.rs`, `src/render/hud.rs`
- [x] **Mission Debriefing & Merit Rankings**: Post-mission statistical evaluation across casualties, time elapsed, and Starship readiness, awarding authentic military merits.
  - *Artifacts & Specs*: [`docs/formats/campaign_progression_and_debrief.md`](formats/campaign_progression_and_debrief.md)
  - *Code References*: `src/sim/debrief.rs`

### Phase 8: Multiplayer WebRTC, Lockstep Protocol & Tactical Chat (Completed)
- [x] **Serverless WebRTC DataChannels**: Zero-server peer-to-peer data transport using compact Base64 SDP exchange.
  - *Artifacts & Specs*: [`docs/formats/multiplayer_protocol.md`](formats/multiplayer_protocol.md), [`docs/formats/multiplayer_matchmaking_and_telemetry.md`](formats/multiplayer_matchmaking_and_telemetry.md)
  - *Code References*: `src/net/mod.rs`, `src/net/protocol.rs`, `src/net/session.rs`
- [x] **Turn-Buffered Deterministic Lockstep Protocol**: Synchronized PRNG seed handshakes, input turn queues, and packet validation.
  - *Artifacts & Specs*: [`docs/formats/multiplayer_protocol.md`](formats/multiplayer_protocol.md)
  - *Code References*: `src/net/session.rs`, `src/sim/mod.rs`
- [x] **Live Multiplayer HUD Telemetry**: In-game latency pills (RTT), lockstep buffer status, and peer connectivity indicators.
  - *Artifacts & Specs*: [`docs/formats/multiplayer_matchmaking_and_telemetry.md`](formats/multiplayer_matchmaking_and_telemetry.md)
  - *Code References*: `src/render/hud.rs`
- [x] **In-Game Faction Tactical Chat**: Color-coded real-time communication (`[Eden]` cyan, `[Plymouth]` red) with keyboard dispatch via `Enter`.
  - *Artifacts & Specs*: [`docs/formats/multiplayer_matchmaking_and_telemetry.md`](formats/multiplayer_matchmaking_and_telemetry.md)
  - *Code References*: `src/render/hud.rs`, `src/net/protocol.rs`
- [x] **Interactive Match Lobby State Machine**: Integrated Host and Guest lobby screen with single-click clipboard workflow and map cycling.
  - *Artifacts & Specs*: [`docs/formats/save_games_and_lobby.md`](formats/save_games_and_lobby.md)
  - *Code References*: `src/net/lobby.rs`, `src/render/menu.rs`

### Phase 9: Tactical AI Commanders & Dynamic Scripted Events (Completed)
- [x] **3 Strategic AI Personalities**: Aggressive Rusher, Defensive Fortifier, and Economic Tech Rusher with autonomous base planning and factory quotas.
  - *Artifacts & Specs*: [`docs/formats/ai_and_personalities.md`](formats/ai_and_personalities.md)
  - *Code References*: `src/sim/ai.rs`
- [x] **Dynamic Scripted Event Engine**: Sol-timed triggers, spatial proximity ambush zones, structure casualty thresholds, dynamic reinforcement drops, and planetary disaster triggers.
  - *Artifacts & Specs*: [`docs/formats/dynamic_events_and_scripting.md`](formats/dynamic_events_and_scripting.md)
  - *Code References*: `src/sim/events.rs`, `src/sim/mod.rs`

### Phase 10: Retro Main Menu, Briefing Cinematics & Audio Experience (Completed)
- [x] **Retro Main Menu System**: Pure Canvas title splash, procedural starfield animation, planetary horizon arc, and CRT scanline filter.
  - *Artifacts & Specs*: [`docs/formats/main_menu_and_briefing.md`](formats/main_menu_and_briefing.md)
  - *Code References*: `src/render/menu.rs`
- [x] **Scenario Briefing Modal**: Narrative briefing screen with operational objectives, background transcripts, and Kathy Cullis voice dispatch playback.
  - *Artifacts & Specs*: [`docs/formats/main_menu_and_briefing.md`](formats/main_menu_and_briefing.md)
  - *Code References*: `src/render/menu.rs`, `src/audio/bank.rs`
- [x] **Multi-Channel Volume Routing & Speed Controls**: Independent Master, SFX, and Music gain control via Web Audio `GainNode`s, plus 0.5x–4.0x game speed multipliers.
  - *Artifacts & Specs*: [`docs/formats/pwa_and_offline_deployment.md`](formats/pwa_and_offline_deployment.md)
  - *Code References*: `src/audio/mod.rs`, `src/render/menu.rs`, `src/lib.rs`

### Phase 11: Procedural Skirmish Map Generator & PWA Offline Deployment (Completed)
- [x] **Procedural Map Generator**: Balanced skirmish generator with 4 biomes (Volcanic Basin, Canyons & Crags, Blighted Badlands, Obsidian Ridge), rotational/bilateral symmetries, and A* base connectivity.
  - *Artifacts & Specs*: [`docs/formats/procedural_map_generator.md`](formats/procedural_map_generator.md)
  - *Code References*: `src/sim/generator.rs`
- [x] **PWA Offline Service Worker & Manifest**: Cache-First offline caching via `sw.js`, Retina maskable icons, and installable standalone display configuration.
  - *Artifacts & Specs*: [`docs/formats/pwa_and_offline_deployment.md`](formats/pwa_and_offline_deployment.md)
  - *Code References*: `sw.js`, `manifest.json`, `index.html`

### Phase 12: Exhaustive Quality Assurance & Production Release (Completed)
- [x] **108 Unit Tests**: Exhaustive test coverage verifying audio parsing, i18n locales, WebRTC signaling, HUD calculations, touch gestures, AI behaviors, events, generator symmetries, missions, and save/load serialization.
  - *Artifacts & Specs*: [`docs/formats/integration_testing_and_qa.md`](formats/integration_testing_and_qa.md)
  - *Code References*: `src/lib.rs`
- [x] **4 End-to-End Integration Tests**: System tests validating full campaign lifecycle, multi-bot skirmish autonomy, WebRTC lockstep synchronization, and state persistence.
  - *Artifacts & Specs*: [`docs/formats/integration_testing_and_qa.md`](formats/integration_testing_and_qa.md)
  - *Code References*: `tests/integration_test.rs`
- [x] **Production Release Deliverables**: Version 1.0.0 announcement, architecture report, multi-language documentation synchronization across all 6 language editions.
  - *Artifacts & Specs*: [`docs/RELEASE_V1.0.md`](RELEASE_V1.0.md), [`docs/formats/game_polish_and_release.md`](formats/game_polish_and_release.md)
- [x] **Zero-Warning & Zero-Error Compilation**: Verified clean under `cargo check` and `cargo test`.
