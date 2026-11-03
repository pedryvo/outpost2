<div align="center">

# outpost2

**Reverse Engineering und moderner Web-Port (Rust, WebAssembly, HTML5 Canvas & Web Audio API) von _Outpost 2: Divided Destiny_ (Sierra On-Line / Dynamix, 1997)**

[![Rust](https://img.shields.io/badge/rust-1.80+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-webassembly-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Status](https://img.shields.io/badge/status-abgeschlossen-brightgreen)](#-fortschritt)
[![Phase](https://img.shields.io/badge/phase-7_%E2%80%94_kampagnen_und_raumschiff-blue)](docs/PLAN.md)
[![Plattformen](https://img.shields.io/badge/plattformen-Web_%7C_Vercel_%7C_Mobile-lightgrey)](#-spiel-starten)
[![Lizenz: GPL-3.0](https://img.shields.io/badge/lizenz-GPL--3.0--or--later-blue)](LICENSE)

_Das Hard-Sci-Fi-Echtzeit-Strategiespiel-Meisterwerk von Sierra und Dynamix,<br>nativ für moderne Webbrowser, Desktops und mobile Geräte neu entwickelt._

[English](README.md) · [Português](README.pt-BR.md) · [Español](README.es.md) · **Deutsch** · [中文](README.zh-CN.md) · [Русский](README.ru.md)

</div>

---

## 🚀 Über das Projekt

**outpost2** analysiert das Originalspiel _Outpost 2: Divided Destiny_ (`Outpost2.exe`) und dessen Datenarchive (`art.vol`, `maps.vol`, `sound.vol`, `tech.vol`) von 1997 mittels **REA** (Reverse Engineer Anything) und Ghidra, um eine moderne, saubere Web-Implementierung in **Rust und WebAssembly** bereitzustellen.

Ziel ist es, den Strategieklassiker in modernen Webbrowsern mit nativer Leistung, futuristischer HUD-Benutzeroberfläche, positionalem Audio und Sofort-Deployment auf Vercel Edge erlebbar zu machen.

---

## ✨ Features

- 🦀 **Reine Rust- & WebAssembly-Engine**: Keine schweren Emulatoren oder Windows-Kompatibilitätsschichten. Kompakter Standalone-WASM-Build (~370 KB).
- 🗺️ **Hochauflösende isometrische 2D-Engine**: Flüssiges Zoomen, Subpixel-Kamerabewegung, Tag-/Nacht-Zyklen und 2.5D-Geländeklippen.
- ⚙️ **Modernes Sci-Fi-HUD**: Echtzeit-Kolonietelemetrie, Energiestatus, Lebenserhaltung, Erzvorräte und interaktives Radar-Minimap.
- 🚀 **Evakuierungsraumschiff-Bau**: 4-stufiger Bau des Kolonie-Fluchtschiffs (Kommandobrücke, Kälteschlaf-Module, Fusionsantrieb, Strahlungsschild) vor der planetaren Zerstörung durch Magma und Seuche.
- ⚔️ **Authentische Kampagnen & Missionen**: Missionsauswahl mit echten Karten (`eden01.map`, `ply01.map`, `tutorial.map`) und rivalisierender KI.
- 🏭 **Fabrik-Warteschlangen & Schweißeffekte**: Animierte Plasmaschweiß-Lichtbögen und HUD-Fortschrittsbalken über der Fahrzeugfabrik.
- 📱 **Mobile & Touch-Steuerung**: Optimiert für Smartphones und Tablets mit Touch-Panning und Pinch-to-Zoom.
- 🔊 **Web Audio & Militärfunk**: Synthesizer für Soundeffekte, prozedurale Musik und Web Speech API für Warnfunkdurchsagen.
- 🌍 **Mehrsprachig (i18n)**: Umschalten der Sprache im laufenden Spiel zwischen 6 Sprachen (**English**, **Português do Brasil**, **Español**, **Deutsch**, **简体中文** und **Русский**).

---

## 📊 Fortschritt

| Bereich | Status | Details |
|---|:---:|---|
| **Planung & Architektur** | ✅ | Umfassende Roadmap · [`docs/PLAN.md`](docs/PLAN.md) |
| **REA Reverse Engineering** | ✅ | PE-Analyse von `Outpost2.exe`, 509 C++-Engine-APIs · [`re/engine_symbols.json`](re/engine_symbols.json) |
| **`.vol`-Archiv-Reader** | ✅ | Streaming-Parser für Outpost-2-Container · [`docs/formats/vol_archive.md`](docs/formats/vol_archive.md) |
| **Farbpaletten- & Sprite-Decoder** | ✅ | 256-Farben-Palette und 5.390 Sprite-Definitionen · [`docs/formats/prt_format.md`](docs/formats/prt_format.md) |
| **Isometrischer Renderer** | ✅ | Isometrisches Gitter, 2.5D-Klippenkanten und Tag/Nacht-Licht · [`docs/formats/palette_and_tiles.md`](docs/formats/palette_and_tiles.md) |
| **Nebel des Krieges & Radar** | ✅ | 3-stufiges Sichtbarkeitssystem (Unerforscht, Erforscht, Sichtbar) · [`docs/formats/fog_of_war.md`](docs/formats/fog_of_war.md) |
| **Kolonie-Simulationskern** | ✅ | Eden- & Plymouth-Logistik: Strom, Lebenserhaltung, Erz, Moral und Katastrophen · [`docs/formats/disasters.md`](docs/formats/disasters.md) |
| **Gebäude & Röhrennetzwerk** | ✅ | Modulare Baukits & druckbelüftete Transportröhren · [`docs/formats/structures_and_tubes.md`](docs/formats/structures_and_tubes.md) |
| **Interaktiver Bauhof** | ✅ | Ausziehbare Bauplan-Schublade (11 Gebäude), holografische Geister-Platzierung & Röhrenprojektion |
| **Gebäude-Recycling & Rückbau** | ✅ | Abbruch von Strukturen für 50% Erz-Rückerstattung und automatische Stromgitter-Neubalancierung |
| **Einheitensteuerung & 8-Wege-A\*** | ✅ | A*-Pfadfindung auf isometrischem Gitter mit Rechtsklick-Befehlen · [`docs/ENGINE_ARCHITECTURE.md`](docs/ENGINE_ARCHITECTURE.md) |
| **Taktische Rahmen-Mehrfachauswahl**| ✅ | Auswahlrahmen mit Sci-Fi-Winkelretikeln und Shift-Gruppierung |
| **Fahrzeug-Reparatur & Felddepots** | ✅ | Mobile Reparaturfahrzeuge und Depots mit smaragdgrünen Nanoschweiß-Strahlen |
| **Earthworker-Pioniertechnik** | ✅ | Verstärkte Keramik-Schutzmauern, Verlegung von Feldleitungen und Biogefahren-Bereinigung |
| **ConVec-Bausatz-Entfaltung** | ✅ | Baufahrzeuge mit vorgefertigten Gebäudekits, Vor-Ort-Entfaltung und Fundamentintegration |
| **Wach- & Patrouillen-Taktik** | ✅ | Automatische Sektoren-Wachanker, bidirektionale Patrouillenrouten und reaktive Zielerfassung |
| **Dynamische Pest & Staubwirbel** | ✅ | Kriechende mikrobielle Sporen-Gefahr, ätzender Hüllenschaden und wirbelnde atmosphärische Tornados |
| **Wissenschaftliche Forschungsmatrix**| ✅ | Interaktives Tech-Modal mit Kategorien-Taxonomie und passiven Kolonie-Boni (Panzerung, Laser, Erzschmelze) · [`docs/formats/tech_tree.md`](docs/formats/tech_tree.md) |
| **Multi-Karten-Kampagnen** | ✅ | Authentische Schauplätze (Eden 1, Plymouth 1, Akademie-Tutorial) · [`docs/formats/campaigns_and_scenarios.md`](docs/formats/campaigns_and_scenarios.md) |
| **Fabrikwarteschlange & Schweißen** | ✅ | Fahrzeugbau mit Plasma-Schweißeffekt und Fortschrittsbalken |
| **Raumschiff-Evakuierung** | ✅ | Bau in 4 Phasen und dynamische Lebenserhaltungsmechanik · [`docs/formats/starship_assembly.md`](docs/formats/starship_assembly.md) |
| **Persistenz & Touch-Gesten** | ✅ | Quicksave/Load im Browserspeicher und mobile Gestensteuerung |
| **Audio-Engine & Kampf-SFX** | ✅ | Web Audio Synthesizer (Laser, Explosionen, Signaltöne) und klassische Audio-Samples |
| **Lokalisierung (i18n)** | ✅ | Mehrsprachig: EN, PT-BR, ES, DE, ZH-CN, RU |
| **Vercel & WASM-Deployment** | ✅ | Trunk-Build-Pipeline mit Release-LTO-Optimierung (`vercel.json`) |

---

## 🎮 Spiel starten

```bash
# Werkzeuge mit mise installieren
mise install

# Lokalen Entwicklungsserver starten
mise exec -- trunk serve

# Produktions-Build kompilieren
mise exec -- trunk build --release
```

---

## 📜 Lizenz

Der Quellcode von **outpost2** ist unter der **GNU General Public License v3.0 oder neuer** lizenziert ([LICENSE](LICENSE)).
