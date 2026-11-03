<div align="center">

# outpost2

**Reverse Engineering und moderner Web-Port (Rust, WebAssembly, HTML5 Canvas & Web Audio API) von _Outpost 2: Divided Destiny_ (Sierra On-Line / Dynamix, 1997)**

[![Rust](https://img.shields.io/badge/rust-1.80+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-webassembly-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![CI](https://github.com/pedryvo/outpost2/actions/workflows/ci.yml/badge.svg)](https://github.com/pedryvo/outpost2/actions/workflows/ci.yml)
[![Version](https://img.shields.io/badge/version-v1.0.0-blue)](docs/RELEASE_V1.0.md)
[![Status](https://img.shields.io/badge/status-abgeschlossen-brightgreen)](#-fortschritt)
[![Tests](https://img.shields.io/badge/tests-112%20bestanden-brightgreen)](docs/formats/integration_testing_and_qa.md)
[![Phase](https://img.shields.io/badge/phase-v1.0.0_%E2%80%94_produktionsbereit-brightgreen)](docs/PLAN.md)
[![Plattformen](https://img.shields.io/badge/plattformen-Web_%7C_PWA_%7C_Vercel_%7C_Mobile-lightgrey)](#-spiel-starten)
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
| **Nebel des Krieges & Taktik-Radar**| ✅ | 3-stufiges Sichtbarkeitssystem mit Live-Radar (Minen, Laser-/Railgun-Leuchtspuren, Detonationswellen) · [`docs/formats/fog_of_war.md`](docs/formats/fog_of_war.md) |
| **Kolonie-Simulationskern** | ✅ | Eden- & Plymouth-Logistik: Strom, Lebenserhaltung, Erz, Moral und Katastrophen · [`docs/formats/disasters.md`](docs/formats/disasters.md) |
| **Gebäude & Röhrennetzwerk** | ✅ | Modulare Baukits & druckbelüftete Transportröhren · [`docs/formats/structures_and_tubes.md`](docs/formats/structures_and_tubes.md) |
| **Interaktiver Bauhof** | ✅ | Ausziehbare Bauplan-Schublade (13 Gebäude), holografische Geister-Platzierung & Röhrenprojektion |
| **Geothermie-Kraftwerke**        | ✅ | Kontinuierliche Grundlast-Hydrothermalenergie (+450 kW) mit Doppel-Kühltürmen und Dampfschwaden |
| **Wachposten-Verteidigungstürme** | ✅ | Automatische Verteidigungstürme mit dualen Schnellfeuer-Kanonen und Laserzielerfassung |
| **Erzschmelze & Rauchschwaden** | ✅ | Animierte Glühzyklen flüssigen Metalls in Schmelztiegeln und aufsteigende Rauchfahnen |
| **Gebäude-Recycling & Rückbau** | ✅ | Abbruch von Strukturen für 50% Erz-Rückerstattung und automatische Stromgitter-Neubalancierung |
| **Einheitensteuerung & 8-Wege-A\*** | ✅ | A*-Pfadfindung auf isometrischem Gitter mit Rechtsklick-Befehlen · [`docs/ENGINE_ARCHITECTURE.md`](docs/ENGINE_ARCHITECTURE.md) |
| **Taktische Rahmen-Mehrfachauswahl**| ✅ | Auswahlrahmen mit Sci-Fi-Winkelretikeln und Shift-Gruppierung |
| **Schwere Tiger-Panzer & Railgun**| ✅ | Schwere Doppelkettenpanzer mit Überschall-Railguns und magnetischen Beschleunigungsspulen |
| **Minenkopf-Bau mit Robo-Miner**  | ✅ | Unterirdischer Förderturm-Bau über Mineralien-Baken, der Frachttransport für Trucks aktiviert · [`docs/formats/mining_and_logistics.md`](docs/formats/mining_and_logistics.md) |
| **Fahrzeug-Reparatur & Felddepots** | ✅ | Mobile Reparaturfahrzeuge und Depots mit smaragdgrünen Nanoschweiß-Strahlen |
| **Earthworker-Pioniertechnik & Planieren**| ✅ | Keramik-Schutzmauern, Feldleitungen, Biogefahren-Bereinigung und Planieren von Hindernissen |
| **ConVec-Bausatz-Entfaltung** | ✅ | Baufahrzeuge mit vorgefertigten Gebäudekits, Vor-Ort-Entfaltung und Fundamentintegration |
| **Wach- & Patrouillen-Taktik** | ✅ | Automatische Sektoren-Wachanker, bidirektionale Patrouillenrouten und reaktive Zielerfassung |
| **Planetare Gefahren & Katastrophen**| ✅ | Kriechende mikrobielle Pest, Vulkanspalten, Staubwirbel-Tornados und kinetische Meteorschauer |
| **Roter Alarm Sirene & Überfälle**| ✅ | Zweiton-Notfallsirene der Kommandozentrale und koordinierte Angriffsstaffeln feindlicher Plymouth-Einheiten |
| **Wissenschaftliche Forschungsmatrix**| ✅ | 11-Knoten-Tech-Baum mit Pflichtvoraussetzungen (Vulkanologie, Railguns, Punktverteidigung) und passiven Boni · [`docs/formats/tech_tree.md`](docs/formats/tech_tree.md) |
| **P2P-WebRTC-Multiplayer & Lobby** | ✅ | Serverloses Peer-to-Peer-Netzwerk (SDP-Base64-Austausch, Lockstep-Befehle, Chat und Lobby-Zustandsautomat) · [`docs/formats/save_games_and_lobby.md`](docs/formats/save_games_and_lobby.md) |
| **Taktische KI & Scharmützel-Autonomie**| ✅ | 3 strategische KI-Doktrinen (Aggressiver Rusher, Defensiver Festungsbauer, Tech-Rusher), Basisplanung und koordinierte Angriffe · [`docs/formats/ai_and_personalities.md`](docs/formats/ai_and_personalities.md) |
| **Dynamische Ereignisse & Skripte** | ✅ | Sol-Timer-Trigger, räumliche Hinterhalte, dynamische Verstärkungen und Auslöser für Naturkatastrophen · [`docs/formats/dynamic_events_and_scripting.md`](docs/formats/dynamic_events_and_scripting.md) |
| **Vollständige Kampagne (Eden 1-3 & Plymouth 1-3)**| ✅ | 6 Schauplätze einschließlich Eden 3 (*Flucht vom Mount Vulcan*) und Plymouth 3 (*Sturm auf die Obsidian-Klippen*) mit extrahierten Karten · [`docs/formats/campaign_progression_and_debrief.md`](docs/formats/campaign_progression_and_debrief.md) |
| **Sprite- & Animations-Engine**     | ✅ | 8-Wege-Fahrgestellorientierung ($45^\circ$), Geschütztürme, rotierende Minenbohrer und 12-stufige Feuerbälle · [`docs/formats/sprites_and_animation.md`](docs/formats/sprites_and_animation.md) |
| **Authentische SoundBank & Funksprüche**| ✅ | 252-Slot-Legendentabelle, Kathy Cullis Taktik-Funksprüche und verzögerungsfreie WASM-Vorladung von Kampf-SFX · [`docs/formats/sound_and_voices.md`](docs/formats/sound_and_voices.md) |
| **Modernes WebHUD & Radar-Minimap** | ✅ | Radar mit Kriegsnebel, Katastrophen-Warnpunkten (Lava, Tornados, Meteore), Fabrik-/Technik-Panels und Tastatur-Shortcuts · [`docs/formats/ui_and_hud.md`](docs/formats/ui_and_hud.md) |
| **Spielstand-Persistenz & .op2save**| ✅ | `LocalStorage`-Slots (1-5, F5 Quicksave, F9 Quickload, 100-Sol Autosave) sowie Export/Import von `.op2save`-Dateien · [`docs/formats/save_games_and_lobby.md`](docs/formats/save_games_and_lobby.md) |
| **WASM-Paket & Mobile Touch-Gesten**| ✅ | High-DPI-Retina-Skalierung, Audio-Freischaltung per Touch, 2-Finger-Pinch/Pan und optimiertes Release-Paket · [`docs/formats/wasm_packaging_and_web_runtime.md`](docs/formats/wasm_packaging_and_web_runtime.md) |
| **Interaktives Retro-Hauptmenü & Einsatz-Briefing-System** | ✅ | Authentisches Canvas-Hauptmenü, Szenario-Wähler, narrative Briefing-Videosequenz mit Kathy-Cullis-Sprachausgabe und CRT-Scanline-Filter · [`docs/formats/main_menu_and_briefing.md`](docs/formats/main_menu_and_briefing.md) |
| **Prozeduraler Gefechts-Kartengenerator (4 Biome, Symmetrien, A\*-Konnektivität)** | ✅ | Ausgewogener prozeduraler Generator: 4 Biome (Vulkanbecken, Schluchten & Klippen, Verseuchte Ödlande, Obsidian-Kamm), 2-Wege-/4-Wege-/Spiegelsymmetrien und A*-Fahrzeugdurchgängigkeit · [`docs/formats/procedural_map_generator.md`](docs/formats/procedural_map_generator.md) |
| **Offline-PWA mit Service Worker & Audiomischer-Verstärkung** | ✅ | Cache-First Service Worker (`sw.js`), PWA-Manifest und App-Icons, Web-Audio-Verstärkerknoten (Master/SFX/Musik) sowie Simulationsmultiplikatoren 0.5x–4.0x · [`docs/formats/pwa_and_offline_deployment.md`](docs/formats/pwa_and_offline_deployment.md) |
| **WebRTC-Mehrspieler-Telemetrie & Taktischer Chat im Spiel** | ✅ | RTT-Echtzeit-Latenzanzeige, Turn-gepufferter Lockstep-Synchronisationsstatus und farbcodierter Fraktionschat (`[Eden]` / `[Plymouth]`) · [`docs/formats/multiplayer_matchmaking_and_telemetry.md`](docs/formats/multiplayer_matchmaking_and_telemetry.md) |
| **Integriertes 6-Sprachen-Lokalisierungssystem** | ✅ | Vollständige native Übersetzung für EN, PT-BR, ES, DE, ZH-CN und RU über alle Menüs, HUD-Telemetrie, Kampagnen-Briefings und Einsatzbewertungen hinweg · [`docs/formats/i18n_and_localization.md`](docs/formats/i18n_and_localization.md) |
| **Umfassende QA-Suite mit 112 Unit- & Integrationstests** | ✅ | 108 Unit-Tests + 4 End-to-End-Integrationstests zur Validierung von Kampagnenverläufen, Multi-Bot-Autonomie, WebRTC-Lockstep und Speicherstands-Integrität · [`docs/formats/integration_testing_and_qa.md`](docs/formats/integration_testing_and_qa.md) |
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
