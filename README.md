<div align="center">

# outpost2

**Reverse engineering and modern web port (Rust, WebAssembly, HTML5 Canvas & Web Audio API) of _Outpost 2: Divided Destiny_ (Sierra On-Line / Dynamix, 1997)**

[![Rust](https://img.shields.io/badge/rust-1.80+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-webassembly-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![CI](https://github.com/pedryvo/outpost2/actions/workflows/ci.yml/badge.svg)](https://github.com/pedryvo/outpost2/actions/workflows/ci.yml)
[![Status](https://img.shields.io/badge/status-active-brightgreen)](#-progress)
[![Phase](https://img.shields.io/badge/phase-1_%26_2_%E2%80%94_re_%26_rust_engine-blue)](docs/PLAN.md)
[![Platforms](https://img.shields.io/badge/platforms-Web_%7C_Vercel_%7C_Mobile-lightgrey)](#-running-the-game)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0--or--later-blue)](LICENSE)

_The 1997 hard sci-fi real-time strategy masterpiece by Sierra & Dynamix,<br>re-engineered to run natively in modern web browsers, desktop, and mobile._

**English** · [Português](README.pt-BR.md) · [Español](README.es.md) · [Deutsch](README.de.md) · [中文](README.zh-CN.md) · [Русский](README.ru.md)

</div>

---

## 🚀 About

**outpost2** studies the original _Outpost 2: Divided Destiny_ executable (`Outpost2.exe`) and data archives (`art.vol`, `maps.vol`, `sound.vol`, `tech.vol`) (Sierra On-Line / Dynamix, 1997) using **REA** (Reverse Engineer Anything) and Ghidra to map every class hierarchy, data structure, and game mechanic, delivering a modern, clean web implementation.

Unlike heavyweight virtual machines or x86 emulators, this port runs natively in **Rust compiled to WebAssembly (WASM)** with HTML5 Canvas and Web Audio:

- 🌐 **Native Rust / WebAssembly Simulation**: Pure client-side simulation with memory safety, zero emulation overhead, and high-performance rendering.
- 🗺️ **High-Resolution Isometric Engine**: Authentic 2D isometric grid viewport with smooth zooming, sub-pixel camera scrolling, day/night cycles, and dynamic terrain overlays (lava flows, microbial blight, and disaster tracks).
- ⚙️ **Modern Sci-Fi Colony HUD**: A sleek, high-tech command interface with real-time colony telemetry, power grid balancing, life support monitors, ore stockpiles, and an interactive radar minimap.
- 📱 **Mobile & Touch-Ready**: Optimized layout for desktop, tablets, and smartphones, featuring touch panning, pinch-to-zoom, and quick action bars.
- 🔊 **Web Audio Soundscape**: Native playback of classic voice notifications, unit audio confirmations, machinery ambiances, and musical tracks.
- 🌍 **Automatic Localization (i18n)**: Multilingual support with live in-game language switching (**English**, **Português do Brasil**, **Español**, **Deutsch**, **简体中文**, and **Русский**).
- ⚡ **Instant Vercel Deployment**: Bundled as an ultra-fast static WebAssembly single-page application ready for continuous deployment on Vercel or any modern edge CDN.

---

## 📊 Progress

| Area | Status | Details |
|---|:---:|---|
| **Plan & Architecture** | ✅ | Full project breakdown and roadmap · [`docs/PLAN.md`](docs/PLAN.md) |
| **REA Reverse Engineering** | ✅ | PE analysis of `Outpost2.exe`, 509 exported C++ engine APIs · [`re/engine_symbols.json`](re/engine_symbols.json) |
| **`.vol` Volume Reader** | ✅ | Pure Rust streaming parser for archive containers · [`docs/formats/vol_archive.md`](docs/formats/vol_archive.md) |
| **Palette & Sprite Decoders** | ✅ | 256-color palette parser, `.prt` 5,390 sprite definitions · [`docs/formats/prt_format.md`](docs/formats/prt_format.md) |
| **Isometric Tile Renderer** | ✅ | Dynamic isometric grid, day/night diurnal lighting, viewport culling · [`docs/formats/palette_and_tiles.md`](docs/formats/palette_and_tiles.md) |
| **Fog of War & Tactical Radar** | ✅ | 3-tier sensor grid with live radar telemetry (mines, laser/railgun tracers, impact shockwaves) · [`docs/formats/fog_of_war.md`](docs/formats/fog_of_war.md) |
| **Colony Simulation Core** | ✅ | Eden & Plymouth logistics: power flux, life support, ore, morale, disasters · [`docs/formats/disasters.md`](docs/formats/disasters.md) |
| **Structures & Tube Network** | ✅ | Modular base kits & pressurized transport tube grid · [`docs/formats/structures_and_tubes.md`](docs/formats/structures_and_tubes.md) |
| **Interactive Construction Yard**| ✅ | Slide-out blueprint drawer (13 buildings), holographic ghost placement & tube projections |
| **Geothermal Power Plants**      | ✅ | Continuous baseload hydrothermal energy (+450 kW) with twin cooling towers and animated steam plumes |
| **Guard Post Defense Turrets**   | ✅ | Automated perimeter defense turrets with twin rapid-fire defensive cannons and laser targeting |
| **Ore Smelting & Exhaust Plumes**| ✅ | Animated crucible molten heat cycles, glowing vat cores, and rising atmospheric exhaust smoke |
| **Structure Recycling & Demolition**| ✅ | Salvage obsolete or damaged buildings to reclaim 50% ore investment & rebalance power grids |
| **Unit Control & 8-Way A\*** | ✅ | A* isometric pathfinding, lava avoidance, right-click orders · [`docs/ENGINE_ARCHITECTURE.md`](docs/ENGINE_ARCHITECTURE.md) |
| **Tactical Drag Selection** | ✅ | Multi-unit drag-box with corner bracket reticles and Shift-click grouping |
| **Tiger Heavy Tanks & Railguns**  | ✅ | Heavy dual-tread assault tanks with supersonic hyper-velocity railguns and magnetic coil visuals |
| **Robo-Miner Mine Head Construction**| ✅ | Subterranean mine head excavation over mineral beacons, activating automated cargo extraction · [`docs/formats/mining_and_logistics.md`](docs/formats/mining_and_logistics.md) |
| **Vehicle Repair & Field Triage**| ✅ | Mobile Repair Vehicles and stationary Depots projecting emerald nano-welding beams |
| **Earthworker Engineering & Bulldoze**| ✅ | Fortified ceramic defense walls, field conduit laying, hazard purging, and obstruction bulldozing |
| **ConVec Modular Kit Deployment** | ✅ | Construction Vehicles hauling pre-fabricated kits, unfolding installations, and foundation lockdown |
| **Tactical Guard & Patrol Stances** | ✅ | Automated sector guard anchors, bidirectional patrol lanes, and reactive target acquisition |
| **Planetary Hazards & Disasters** | ✅ | Creeping microbial Blight, volcanic fissures, atmospheric dust vortices, and kinetic meteor showers |
| **Scientific Research Matrix** | ✅ | Interactive tech modal with category taxonomy and passive colony bonuses (armor, laser, smelting) · [`docs/formats/tech_tree.md`](docs/formats/tech_tree.md) |
| **Multi-Map Campaigns** | ✅ | Authentic operational theaters (Eden 1, Plymouth 1, Academy Tutorial) · [`docs/formats/campaigns_and_scenarios.md`](docs/formats/campaigns_and_scenarios.md) |
| **Factory Queues & Animation** | ✅ | Production queue scheduling, welding plasma arcs, HUD progress bars |
| **Starship Evacuation & Launch** | ✅ | 4-stage starship assembly, orbital launch animation with billowing exhaust plumes, and escape fanfare · [`docs/formats/starship_assembly.md`](docs/formats/starship_assembly.md) |
| **Mission Debriefing & Directives**| ✅ | Post-mission operational evaluation, survivor demographics, directive checklist, and campaign progression |
| **State Persistence & Touch** | ✅ | Browser storage quicksave/load, mobile pinch-to-zoom & gesture controls |
| **Audio Engine & Combat SFX** | ✅ | Web Audio procedural synthesizer (lasers, explosions, build hums) & classic audio samples |
| **Modern Sci-Fi HUD** | ✅ | Real-time telemetry, radar minimap, build queues, and unit command console |
| **Localization (i18n)** | ✅ | Multi-language schema: EN, PT-BR, ES, DE, ZH-CN, RU |
| **Vercel & WASM Ready** | ✅ | Trunk pipeline with release LTO optimization (`vercel.json`) |

---

## 🎮 Running the Game

### Prerequisites
- [Rust](https://www.rust-lang.org/) (stable)
- [mise](https://mise.jdx.dev/) tool manager
- [Trunk](https://trunkrs.dev/) (WASM web application bundler)

### Environment Setup with `mise`
```bash
# Clone the repository
git clone https://github.com/pedryvo/outpost2.git
cd outpost2

# Activate project tools via mise
mise install
```

### Local Development
```bash
# Start Trunk local development server with hot-reload
trunk serve
```

Open your browser at `http://127.0.0.1:8080` to play immediately!

### Production Build
```bash
# Build optimized WebAssembly static distribution
trunk build --release
```

The resulting `dist/` directory contains standalone static assets ready for direct upload to Vercel, Netlify, or GitHub Pages.

---

## 🚀 Deploying to Vercel

Deploy with a single command via the Vercel CLI:
```bash
vercel
```

Or connect the repository `pedryvo/outpost2` to your [Vercel Dashboard](https://vercel.com/) for automated continuous deployment on every `git push`.

---

## 🕹️ Controls

| Action | Desktop Keyboard / Mouse | Mobile / Tablet Touch |
|---|---|---|
| **Pan Camera** | Arrow Keys or `W`, `A`, `S`, `D` / Mouse Drag | Drag finger across screen |
| **Zoom Viewport** | Mouse Wheel / `+`, `-` | Pinch to zoom |
| **Select Unit / Building** | Left Click | Tap |
| **Order Move / Action** | Right Click | Context Button / Double Tap |
| **Toggle Radar Minimap** | `M` or Radar Button | Minimap Corner Tap |
| **Pause Simulation** | `Spacebar` or `P` | Pause Button |
| **Toggle Sound Effects** | `Ctrl + S` | Sound Icon |
| **Toggle Radio Chatter** | `Ctrl + V` | Voice Icon |

---

## 🗂️ Project Layout

```
docs/
  PLAN.md              Development roadmap and task milestones
  formats/
    vol_archive.md     Outpost 2 .vol container specification
re/
  engine_symbols.json  509 exported C++ symbols extracted from Outpost2.exe
src/
  lib.rs               WebAssembly entry point and game lifecycle
  archive/             Pure Rust .vol binary parser
  render/              Isometric tile grid and Canvas2D viewport
  sim/                 Colony logistics, power, life support, and units
  ui/                  Modernized sci-fi HUD and tactical minimap
  audio/               Web Audio manager
index.html             Web application shell and canvas viewport
Trunk.toml             Trunk bundler configuration
vercel.json            High-performance edge headers and WASM mime-types
mise.toml              Tool version pinning (Rust, Trunk)
```

---

## 🔬 Methodology

1. **Static Analysis & Ghidra Extraction**: The Windows PE executable `Outpost2.exe` was analyzed with Ghidra via the **REA (Reverse Engineer Anything)** MCP server to discover memory block boundaries, 3,068 procedures, and 509 exported C++ class interfaces.
2. **Container & Asset Decoders**: The `.vol` format specifications and `OP2_ART.BMP` palette layouts were reconstructed into zero-copy, memory-safe Rust readers.
3. **Clean-Room Implementation**: All colony simulation, tile rendering, and unit logic are written in modern Rust compiling directly to WebAssembly without legacy Windows binaries.
4. **Accessible Web HUD**: Rather than trapping menus inside an opaque retro window, command interfaces use accessible, high-contrast, responsive web components styled after classic sci-fi telemetry.

---

## ⚖️ Legal Notice

_Outpost 2: Divided Destiny_, its logos, artwork, audio, and game lore are trademarks and copyright © Sierra On-Line, Dynamix, and Activision Blizzard.

This is an open-source, non-profit educational fan project developed for software preservation and cross-platform interoperability. This project is not affiliated with, endorsed by, or sponsored by Sierra, Dynamix, or Activision.

The source code of **outpost2** is licensed under the **GNU General Public License v3.0 or later** ([LICENSE](LICENSE)).
