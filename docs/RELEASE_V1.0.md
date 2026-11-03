# Outpost 2: Divided Destiny — Version 1.0.0 Production Release Announcement

**Release Tag:** `v1.0.0`  
**Target Architecture:** Rust 1.80+ · WebAssembly (`wasm32-unknown-unknown`) · HTML5 Canvas · Web Audio API · WebRTC  
**License:** GNU General Public License v3.0 or later ([LICENSE](../LICENSE))  
**Date:** November 2026  
**Status:** Production Ready · 112/112 Automated Tests Passing · 0 Warnings · 0 Errors

---

## 🌌 Executive Summary

The **outpost2** development team is proud to announce the official **v1.0.0 Production Release** of *Outpost 2: Divided Destiny* for the modern web.

Originally published by Sierra On-Line and Dynamix in 1997, *Outpost 2* is celebrated as one of the most intellectually rigorous hard science-fiction real-time strategy games in computing history. Set on the hostile, dying colony world of Planet Tethys following the catastrophic loss of Earth, players lead one of two opposing factions—the scientific purists of **Eden** or the rugged, independent survivors of **Plymouth**—in a desperate race to establish sustainable human habitation and escape impending planetary extinction.

Through clean-room reverse engineering, zero-copy Rust binary parsers, and a WebAssembly-native simulation pipeline, **outpost2 v1.0.0** delivers a native web port that runs instantly in any modern browser without plugins, emulators, or legacy Windows installations.

---

## 🏛️ Reverse Engineering Methodology & Clean-Room Architecture

```
+-----------------------------------------------------------------------------------+
|                        REA & Ghidra Reverse Engineering                           |
|  - 3,068 procedures decompiled & analyzed                                         |
|  - 2,498 text literals cataloged                                                  |
|  - 509 exported C++ engine APIs mapped (re/engine_symbols.json)                   |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|                        Pure Rust WebAssembly Engine (outpost2)                    |
|  +--------------------+  +--------------------+  +-----------------------------+  |
|  |   Archive Parser   |  | Colony Simulation  |  |   Procedural Map Generator  |  |
|  |  (src/archive/)    |  |    (src/sim/)      |  |     (src/sim/generator.rs)  |  |
|  +--------------------+  +--------------------+  +-----------------------------+  |
|  +--------------------+  +--------------------+  +-----------------------------+  |
|  |  Isometric Canvas  |  |   WebAudio Mixer   |  |   WebRTC P2P DataChannels   |  |
|  |   (src/render/)    |  |   (src/audio/)     |  |       (src/net/)            |  |
|  +--------------------+  +--------------------+  +-----------------------------+  |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|                            Modern Client Runtime                                  |
|   HTML5 Canvas (2D) · Web Audio Gain Matrix · PWA Service Worker (sw.js) · i18n   |
+-----------------------------------------------------------------------------------+
```

### 1. Static Analysis via REA (Reverse Engineer Anything)
Using the REA MCP framework and Ghidra, the 32-bit Windows PE binary `Outpost2.exe` was fully analyzed:
- Decompiled **3,068 procedures** across core simulation, graphics rendering, network dispatch, and file I/O.
- Cataloged **2,498 text strings and diagnostic literals**.
- Mapped **509 exported C++ class interfaces** spanning `TApp`, `GameMap`, `Unit`, `Building`, `Research`, and `Packet` hierarchies into [`re/engine_symbols.json`](../re/engine_symbols.json).

### 2. Zero-Copy Binary Containers
The original Sierra resource containers were reverse-engineered and re-implemented in safe, high-performance Rust:
- **`.vol` Archive Reader (`src/archive/mod.rs`)**: Streaming zero-copy volume parser supporting uncompressed, LZH, and RLE streams across `art.vol`, `maps.vol`, `sound.vol`, `voices.vol`, and `tech.vol` ([`docs/formats/vol_archive.md`](formats/vol_archive.md)).
- **Sprite and Palette Decoders (`src/render/tiles.rs`, `src/render/sprites.rs`)**: Master 256-color palette parser and `.prt` sprite descriptor loader decoding all 5,390 original visual assets ([`docs/formats/prt_format.md`](formats/prt_format.md)).
- **Binary Map Formats (`src/sim/map.rs`)**: Direct ingestion of original `.map` cell matrices, cliff topologies, passability bitfields, and ore beacons ([`docs/formats/map_format.md`](formats/map_format.md)).

### 3. Emulation-Free WebAssembly Pipeline
Rather than bundling DOSBox, Wine, or x86 instruction emulators, all game subsystems are written in native, memory-safe Rust compiling to `wasm32-unknown-unknown`. Simulation logic executes deterministically on the client CPU with zero virtualization overhead and instantaneous load times.

---

## 🎨 Retro Fidelity & Audio-Visual Immersion

### High-DPI Isometric Viewport & Canvas Rendering
- **2:1 Diamond Isometric Projection**: Recreates the classic 64x32 pixel tile layout with sub-pixel camera scrolling, zoom scaling ($0.4\times$ to $2.5\times$), and pixel-art sharpness via integer boundary snapping ([`docs/formats/palette_and_tiles.md`](formats/palette_and_tiles.md)).
- **2.5D Cliffs & Elevation Drop-Shadows**: Multi-tier terrain topology with vertical cliff rims and authentic directional ambient shadows.
- **Diurnal Planetary Lighting**: Continuous solar flux ambient transitions between Day, Twilight, and Night, dynamically affecting solar power generation.
- **Fog of War & Sensor Radar Matrix**: 3-tier visibility (Unexplored, Explored, Visible) calculated dynamically from building sight corridors and vehicle sensor suites ([`docs/formats/fog_of_war.md`](formats/fog_of_war.md)).

### Authentic SoundBank & Kathy Cullis Voice Dispatches
- **252-Slot Legacy Audio Table**: Complete integration of original Sierra sound effects and unit confirmations ([`docs/formats/sound_and_voices.md`](formats/sound_and_voices.md)).
- **Tactical Radio Chatter**: Decoded 8-bit mono unsigned PCM waveforms featuring original tactical dispatcher Kathy Cullis (*"Cargo truck selected"*, *"We are under attack!"*, *"Danger: Volcanic eruption detected"*).
- **Procedural Audio Synthesizer Fallback**: Multi-oscillator Web Audio synthesizer ensuring rich acoustic feedback even in asset-constrained offline environments.

### Retro Main Menu & Mission Briefing Cinematics
- **Procedural Canvas Title Splash**: Authentic 1997 visual aesthetic featuring animated drifting starfields, Planet Tethys horizon illumination, retro perspective grid ground planes, and optional CRT scanline filters ([`docs/formats/main_menu_and_briefing.md`](formats/main_menu_and_briefing.md)).
- **Scenario Briefing Modal**: Comprehensive pre-mission briefings with historical narrative transcripts, operational directives, victory conditions, and voice dispatches across all campaign scenarios.

---

## ⚙️ Colony Simulation & RTS Mechanics

### Faction Logistics & Modular Construction
- **Eden vs. Plymouth Parameters**: Faction-specific starting kits, technology paths, and unit profiles ([`docs/formats/campaigns_and_scenarios.md`](formats/campaigns_and_scenarios.md)).
- **13 Modular Base Structures**: Command Center, Geothermal Power Plants (+450 kW baseload with twin cooling towers and steam plumes), Tokamak Fusion Reactors, Solar Arrays, Smelters with molten crucible glow cycles, Structure Factories, Vehicle Factories, and perimeter Guard Posts with automated twin cannons ([`docs/formats/structures_and_tubes.md`](formats/structures_and_tubes.md)).
- **Pressurized Transport Tube Network**: Arterial life-support conduits distributing electrical power and oxygen between interconnected modules.
- **Structure Salvage & Demolition**: Deconstruct damaged or obsolete facilities to reclaim 50% ore investment and rebalance colony power grids.

### Mining Logistics & Unit Control
- **Subterranean Mine Heads**: Robo-Miners excavate surface mineral beacons, deploying automated mining derricks for Common and Rare Ore extraction ([`docs/formats/mining_and_logistics.md`](formats/mining_and_logistics.md)).
- **Autonomous Haulage Convoys**: Cargo Trucks automatically establish loop routes between active mine heads and Smelters.
- **8-Directional Isometric A\* Pathfinding**: Vector pathfinding with dynamic hazard avoidance (lava flows, seismic fissures) and friendly unit collision avoidance ([`docs/formats/pathfinding.md`](formats/pathfinding.md)).
- **Combat Units & Tactical Stances**: Tiger heavy assault tanks equipped with hyper-velocity railguns and magnetic acceleration coils, Lynx light scout tanks, mobile Repair Vehicles with emerald nano-welding beams, and Earthworker engineering units with ceramic defense wall construction and obstacle bulldozing.
- **Tactical Stances**: Automated sector guard anchors, bidirectional patrol routes, and drag-box multi-unit selection with high-tech corner brackets and Shift-click aggregation.

### Planetary Hazards & Disaster Engine
- **Creeping Microbial Blight**: Self-propagating biological hazard consuming biomass and degrading structures ([`docs/formats/disasters.md`](formats/disasters.md)).
- **Volcanic Eruptions & Magma Fronts**: Glowing lava streams spilling into valleys and consuming anything in their path.
- **Seismic Fissures**: Sudden geological fractures shearing transport tubes and severing power grids.
- **Atmospheric Dust Vortices**: Whistling cyclonic storms roaming the battlefield and damaging vehicle hulls.
- **Kinetic Meteor Showers**: Ballistic orbital strikes causing devastating localized craters and explosive shockwaves.

### University Tech Tree & Starship Escape
- **11-Node Scientific Research Matrix**: Academic disciplines spanning Vulcanology, Laser Defense Systems, Supersonic Railguns, Geothermal Tapping, and High-Yield Agriculture ([`docs/formats/tech_tree.md`](formats/tech_tree.md)).
- **Evacuation Starship Multi-Stage Assembly**: Four fabrication phases (Command Bridge, Cryo-Stasis Chamber, Fusion Thruster Array, Life Support Shield) necessary to complete the ultimate planetary escape ([`docs/formats/starship_assembly.md`](formats/starship_assembly.md)).
- **6 Canonical Campaign Missions**: Full campaign progression covering Eden 1–3 (*Operation New Dawn*, *Eden Retreat*, *Escape from Mount Vulcan*) and Plymouth 1–3 (*Badland Vanguard*, *The Rift Defense*, *Obsidian Crags Assault*), plus Cadet Training Academy ([`docs/formats/campaign_progression_and_debrief.md`](formats/campaign_progression_and_debrief.md)).
- **Post-Mission Debriefing**: Rigorous statistical evaluation scoring Sol completion time, civilian casualty rates, and resource efficiency, awarding authentic military merits (*Supreme Colony Savior*, *Distinguished Pioneer*, *Tethys Survivor*).

---

## 🗺️ Procedural Skirmish Map Generator

Integrated in `src/sim/generator.rs`, the procedural map generator creates balanced, competitive maps for endless replayability ([`docs/formats/procedural_map_generator.md`](formats/procedural_map_generator.md)):
- **4 Terrain Biomes**:
  - *Volcanic Basin*: Magma rivers, basalt fields, and geothermal fumaroles.
  - *Canyons and Crags*: Rocky choke points, elevated mesas, and labyrinthine passes.
  - *Blighted Badlands*: Microbial hazard fields, toxic mudflats, and high-risk ore caches.
  - *Obsidian Ridge*: High-altitude obsidian plateaus and fortified vantage points.
- **Rigorous Multiplayer Symmetries**: Supports Rotational 2-Way, Rotational 4-Way, and Bilateral Mirrored-X symmetries to guarantee absolute competitive fairness.
- **Guaranteed Base Clearance & A\* Connectivity**: Deterministic 6x6 footprint clearance around spawn locations and automated A* validation ensuring all player bases and contested mineral beacons remain traversable.
- **3 Tactical AI Personalities**: Aggressive Rusher, Defensive Fortifier, and Economic Tech Rusher driven by autonomous expansion logic and coordinated assault squads ([`docs/formats/ai_and_personalities.md`](formats/ai_and_personalities.md)).

---

## 🌐 Multiplayer WebRTC P2P & Lockstep Protocol

Outpost 2 features serverless, peer-to-peer multiplayer running over WebRTC DataChannels ([`docs/formats/multiplayer_protocol.md`](formats/multiplayer_protocol.md), [`docs/formats/multiplayer_matchmaking_and_telemetry.md`](formats/multiplayer_matchmaking_and_telemetry.md)):
- **Zero-Server Manual Signaling**: Players establish direct browser-to-browser connections by exchanging compact Base64-encoded Session Description Protocol (SDP) payloads via a single-click clipboard copy-paste interface.
- **Deterministic Turn-Buffered Lockstep**: Both players execute identical simulation steps governed by synchronized PRNG seeds, buffered command queues, and automatic desynchronization detection.
- **Live In-Game HUD Telemetry**: Top-bar network pill displaying real-time round-trip latency (RTT in ms), turn buffer status, and peer connectivity indicators.
- **Color-Coded Faction Tactical Chat**: In-game real-time messaging with faction color branding (`[Eden]` cyan, `[Plymouth]` coral/red) and keyboard dispatch via `Enter`.

---

## 📱 Progressive Web App (PWA) & Offline Runtime

Engineered as a modern Progressive Web App for desktop, tablet, and smartphone browsers ([`docs/formats/pwa_and_offline_deployment.md`](formats/pwa_and_offline_deployment.md)):
- **Offline Service Worker (`sw.js`)**: Cache-First caching strategy storing all application assets, WebAssembly binaries, map data, and audio banks for 100% offline gameplay.
- **Installable PWA Manifest (`manifest.json`)**: Standalone application window mode, retro `#0c0e12` branding, and Retina 192x192 / 512x512 maskable app icons.
- **Multi-Touch Ergonomics (`src/render/touch.rs`)**: Single-tap unit selection, double-tap orders, two-finger panning, and smooth pinch-to-zoom.
- **Multi-Channel Audio Mixer Matrix (`src/audio/mod.rs`)**: Dedicated Web Audio `GainNode` routing for Master Volume, Sound Effects, and Music Synthesizer channels with instantaneous, pop-free slider adjustments.
- **Simulation Velocity Scaling**: Real-time game speed multipliers ($0.5\times, 1.0\times, 2.0\times, 4.0\times$) available via the in-game pause menu.
- **Comprehensive State Persistence (`src/sim/save.rs`)**: 5 browser `LocalStorage` save slots, F5 QuickSave, F9 QuickLoad, 100-Sol AutoSave, and standalone `.op2save` file export/import ([`docs/formats/save_games_and_lobby.md`](formats/save_games_and_lobby.md)).

---

## 🌍 In-Game 6-Language Localization Engine

The Outpost 2 internationalization engine (`src/i18n.rs`) provides comprehensive, strongly-typed multilingual coverage across all 6 supported languages ([`docs/formats/i18n_and_localization.md`](formats/i18n_and_localization.md)):
- **English** (`en`)
- **Português do Brasil** (`pt-BR`)
- **Español** (`es`)
- **Deutsch** (`de`)
- **简体中文** (`zh-CN`)
- **Русский** (`ru`)

Coverage encompasses the Canvas retro main menu, campaign theater selectors, narrative briefing transcripts, in-game HUD telemetry, production queue tooltips, debriefing rank merits, and DOM accessibility elements. Locale resolution automatically parses browser tags (`navigator.language`) and gracefully falls back to English for unknown locales with zero panics.

---

## 🧪 Quality Assurance & Test Metrics

To guarantee flawless simulation determinism, network parity, and campaign integrity, the engine is validated by an exhaustive 112-test QA suite ([`docs/formats/integration_testing_and_qa.md`](formats/integration_testing_and_qa.md)):

```
running 108 tests (unit tests in src/lib.rs)
................................................................................
............................
test result: ok. 108 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

running 4 tests (end-to-end integration tests in tests/integration_test.rs)
test test_full_campaign_mission_lifecycle ... ok
test test_multi_bot_skirmish_autonomy_simulation ... ok
test test_webrtc_p2p_lockstep_turn_synchronization ... ok
test test_state_persistence_save_load_roundtrip_integrity ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

### Test Suite Breakdown:
| Subsystem Module | Tests | Validation Scope |
|---|:---:|---|
| **Audio & Legacy Bank** | 7 | 252-slot table integrity, unsigned 8-bit PCM parsing, gain node routing |
| **Localization (i18n)** | 7 | Dictionary completeness, IETF BCP 47 parsing, fallback resolution across 6 languages |
| **Multiplayer & Protocol** | 10 | SDP Base64 roundtrips, handshake serialization, turn buffer orders, lobby state transitions |
| **HUD & Tactical Chat** | 10 | Radar projection math, hotkey dispatch, energy grid states, chat queue bounds |
| **Menu & PWA Runtime** | 11 | Screen navigation, options clamping, velocity multipliers, button hitboxes, cache manifests |
| **Sprite & Visual Effects** | 5 | 8-directional chassis orientation, turret aiming, explosion sequence stages |
| **Touch & High-DPI Scaling**| 7 | Pinch-to-zoom math, two-finger panning, Retina snapping, tap recognition |
| **Tactical AI & Autonomous Bot**| 9 | 3 AI personalities, building placement selection, factory quotas, retreat logic |
| **Dynamic Events & Scripting**| 5 | Sol timers, spatial triggers, casualty thresholds, dynamic reinforcements |
| **Map Generator & Symmetries**| 10 | 4 biomes, rotational/bilateral symmetries, A* base connectivity, deterministic seeds |
| **Campaign Scenarios** | 10 | Eden 1-3, Plymouth 1-3, victory/defeat evaluation, terraformer defense |
| **State Persistence & Save/Load**| 9 | LocalStorage slots, `.op2save` file export/import, metadata integrity |
| **Simulation Core & Hazards** | 8 | Meteor destruction, power routing, mining extraction loops |
| **System Integration Tests** | 4 | Full campaign lifecycle, 4-player AI skirmish autonomy, WebRTC lockstep sync, save roundtrip |
| **Total Test Count** | **112** | **100% Pass Rate · 0 Failures · 0 Compiler Warnings** |

---

## 🚀 Quick Start & Deployment Guide

### Local Development
```bash
# Clone the repository
git clone https://github.com/pedryvo/outpost2.git
cd outpost2

# Install dependencies via mise
mise install

# Run local development server with hot-reload
trunk serve
```
Open `http://127.0.0.1:8080` in your browser.

### Production Release Build
```bash
# Compile optimized standalone WebAssembly distribution
trunk build --release
```
The output directory `dist/` contains all optimized static assets (~370 KB compressed WASM) ready for edge deployment.

### Edge Deployment on Vercel
Connect your GitHub repository to [Vercel](https://vercel.com/) or run:
```bash
vercel --prod
```
The included [`vercel.json`](../vercel.json) automatically configures streaming WebAssembly MIME types (`application/wasm`) and HTTP edge caching headers.

---

## 📜 Acknowledgments & License

- **Original Creators**: *Outpost 2: Divided Destiny* is copyright © 1997 Sierra On-Line, Dynamix, and Activision Blizzard.
- **Preservation Effort**: This modern clean-room implementation was developed as an educational open-source software preservation project.
- **License**: The source code of **outpost2** is licensed under the **GNU General Public License v3.0 or later** ([LICENSE](../LICENSE)).
