<div align="center">

# outpost2

**Ingeniería inversa y adaptación web moderna (Rust, WebAssembly, HTML5 Canvas y Web Audio API) de _Outpost 2: Divided Destiny_ (Sierra On-Line / Dynamix, 1997)**

[![Rust](https://img.shields.io/badge/rust-1.80+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-webassembly-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![CI](https://github.com/pedryvo/outpost2/actions/workflows/ci.yml/badge.svg)](https://github.com/pedryvo/outpost2/actions/workflows/ci.yml)
[![Versión](https://img.shields.io/badge/versi%C3%B3n-v1.0.0-blue)](docs/RELEASE_V1.0.md)
[![Estado](https://img.shields.io/badge/estado-completado-brightgreen)](#-progreso)
[![Pruebas](https://img.shields.io/badge/pruebas-112%20pasadas-brightgreen)](docs/formats/integration_testing_and_qa.md)
[![Fase](https://img.shields.io/badge/fase-v1.0.0_%E2%80%94_listo_para_producci%C3%B3n-brightgreen)](docs/PLAN.md)
[![Plataformas](https://img.shields.io/badge/plataformas-Web_%7C_PWA_%7C_Vercel_%7C_Mobile-lightgrey)](#-ejecutar-el-juego)
[![Licencia: GPL-3.0](https://img.shields.io/badge/licencia-GPL--3.0--or--later-blue)](LICENSE)

_La obra maestra de estrategia en tiempo real y ciencia ficción de Sierra y Dynamix,<br>reconstruida para ejecutarse de forma nativa en navegadores web modernos, ordenadores y móviles._

[English](README.md) · [Português](README.pt-BR.md) · **Español** · [Deutsch](README.de.md) · [中文](README.zh-CN.md) · [Русский](README.ru.md)

</div>

---

## 🚀 Acerca de

**outpost2** estudia el ejecutable original de _Outpost 2: Divided Destiny_ (`Outpost2.exe`) y sus archivos de datos (`art.vol`, `maps.vol`, `sound.vol`, `tech.vol`) (Sierra On-Line / Dynamix, 1997) utilizando **REA** (Reverse Engineer Anything) y Ghidra para reconstruir sus estructuras de clases y mecánicas, ofreciendo una implementación web moderna y optimizada en **Rust y WebAssembly**.

El objetivo es preservar y modernizar este clásico juego de estrategia en tiempo real para navegadores modernos, ordenadores de escritorio y dispositivos táctiles, con simulación en tiempo real, interfaz HUD futurista, audio posicional y despliegue instantáneo en Vercel Edge.

---

## ✨ Características

- 🦀 **Motor en Rust Puro y WebAssembly**: Sin emuladores lentos ni capas pesadas de compatibilidad de Windows. Código Rust seguro compilado en un binario WebAssembly compacto (~370 KB).
- 🗺️ **Motor Isométrico de Alta Resolución**: Renderizado de cuadrícula isométrica en 2D con zoom suave, desplazamiento sub-píxel, ciclos de día y noche e iluminación diurna dinámica.
- ⚙️ **HUD Táctico Moderno**: Interfaz futurista con telemetría en tiempo real, balance de energía, soporte vital, reservas de mineral y minimapa de radar interactivo.
- 🚀 **Montaje de Nave de Evacuación**: Carrera estelar para fabricar los 4 módulos de la nave espacial (Puente de Mando, Cámaras Criogénicas, Propulsores de Fusión y Escudo de Radiación) antes de que el magma y la Plaga destruyan la colonia.
- ⚔️ **Campañas y Teatros de Operación**: Selector de campañas con escenarios auténticos (`eden01.map`, `ply01.map`, `tutorial.map`) y dirección de IA rival.
- 🏭 **Filas de Fábrica y Soldadura por Plasma**: Animación visual de arcos de plasma y barras de progreso del HUD durante la construcción de tanques y vehículos.
- 📱 **Preparado para Móviles y Táctil**: Control mediante gestos táctiles, arrastre y pellizco para zoom (pinch-to-zoom).
- 🔊 **Paisaje Sonoro Web Audio & Radio**: Sintetizador de efectos de sonido, música ambiental y sintetizador de voz para anuncios de emergencia.
- 🌍 **Localización Multilingüe (i18n)**: Cambio de idioma al instante entre 6 idiomas (**English**, **Português do Brasil**, **Español**, **Deutsch**, **简体中文** y **Русский**).

---

## 📊 Progreso

| Área | Estado | Detalles |
|---|:---:|---|
| **Plan y Arquitectura** | ✅ | Desglose y mapa de ruta del proyecto · [`docs/PLAN.md`](docs/PLAN.md) |
| **Ingeniería Inversa con REA** | ✅ | Análisis PE de `Outpost2.exe`, 509 APIs exportadas de C++ · [`re/engine_symbols.json`](re/engine_symbols.json) |
| **Lector de Archivos `.vol`** | ✅ | Parser en streaming para contenedores de archivos · [`docs/formats/vol_archive.md`](docs/formats/vol_archive.md) |
| **Decodificadores de Paleta y Sprites** | ✅ | Paleta de 256 colores y 5.390 definiciones de sprites · [`docs/formats/prt_format.md`](docs/formats/prt_format.md) |
| **Renderizador Isométrico** | ✅ | Cuadrícula isométrica, sombreado de riscos 2.5D y ciclos solares · [`docs/formats/palette_and_tiles.md`](docs/formats/palette_and_tiles.md) |
| **Niebla de Guerra y Radar Táctico**| ✅ | Rejilla sensorial de 3 niveles con telemetría en radar (minas, trazadores láser/railgun y ondas de choque) · [`docs/formats/fog_of_war.md`](docs/formats/fog_of_war.md) |
| **Núcleo de Simulación Colonial** | ✅ | Logística Eden y Plymouth: energía, soporte vital, minerales, moral y desastres · [`docs/formats/disasters.md`](docs/formats/disasters.md) |
| **Estructuras y Red de Tubos** | ✅ | Kits modulares de construcción y red de tubos presurizados · [`docs/formats/structures_and_tubes.md`](docs/formats/structures_and_tubes.md) |
| **Astillero de Construcción Interactivo**| ✅ | Cajón deslizable de planos (13 edificios), colocación fantasma holográfica y tubos proyectados |
| **Plantas de Energía Geotérmica**| ✅ | Energía hidrotermal continua (+450 kW) con torres dobles de refrigeración y penachos de vapor |
| **Torretas Defensivas Guard Post**| ✅ | Puestos de guardia perimetrales con cañones dobles automáticos y puntero láser |
| **Fundición y Penachos de Humo**   | ✅ | Ciclos incandescentes de metal fundido en crisoles y columnas de humo de chimenea |
| **Reciclaje y Demolición Colonial**| ✅ | Desmantelamiento de edificios para recuperar 50% de minerales y equilibrar la red energética |
| **Control de Unidades y A\* 8-Vías** | ✅ | Pathfinding A* isométrico en 8 direcciones y órdenes tácticas · [`docs/ENGINE_ARCHITECTURE.md`](docs/ENGINE_ARCHITECTURE.md) |
| **Selección Táctica por Arrastre** | ✅ | Caja de selección multi-unidad con retículas angulares sci-fi y agrupación con Shift |
| **Tanques Pesados Tiger & Cañón Rail**| ✅ | Blindados pesados de doble oruga con cañones cinéticos de hipervelocidad y bobinas magnéticas |
| **Construcción de Minas con Robo-Miner**| ✅ | Excavación de cabezas de pozo sobre balizas de minerales, activando logística con camiones · [`docs/formats/mining_and_logistics.md`](docs/formats/mining_and_logistics.md) |
| **Reparación de Vehículos & Triage**| ✅ | Vehículos de Reparación móviles y Talleres con rayos de nanosoldadura esmeralda |
| **Ingeniería Earthworker & Bulldozer**| ✅ | Muros defensivos de cerámica, tubos de campo, purga de biopeligros y demolición con bulldozer |
| **Despliegue de Kits con ConVec** | ✅ | Vehículos de Construcción transportando kits modulares, montaje en terreno y anclaje |
| **Tácticas de Guardia y Patrulla** | ✅ | Puntos de anclaje de defensa, rutas bidireccionales de patrulla y combate reactivo |
| **Peligros Planetarios y Desastres**| ✅ | Plaga microbiana reptante, fisuras volcánicas, vórtices de polvo y lluvias de meteoritos cinéticos |
| **Sirena de Alerta Roja & Incursiones**| ✅ | Sirena bitonal de emergencia del Centro de Mando y asaltos coordinados de escuadrones Plymouth |
| **Matriz Científica de Investigación** | ✅ | Árbol de 11 tecnologías con prerrequisitos obligatorios (Vulcanología, Cañón Rail, Defensas) y bonos pasivos · [`docs/formats/tech_tree.md`](docs/formats/tech_tree.md) |
| **Multijugador P2P y Lobby WebRTC**| ✅ | Red punto a punto sin servidores (intercambio SDP Base64, órdenes lockstep, chat táctico y máquina de estados de lobby) · [`docs/formats/save_games_and_lobby.md`](docs/formats/save_games_and_lobby.md) |
| **IA Táctica y Autonomía Skirmish** | ✅ | 3 doctrinas estratégicas de IA (Rusher Agresivo, Fortificador Defensivo, Rusher Tecnológico), planificación de base y asaltos coordinados · [`docs/formats/ai_and_personalities.md`](docs/formats/ai_and_personalities.md) |
| **Eventos Dinámicos y Scripting**   | ✅ | Disparadores por Sols, emboscadas por proximidad espacial, refuerzos dinámicos y desastres planetarios · [`docs/formats/dynamic_events_and_scripting.md`](docs/formats/dynamic_events_and_scripting.md) |
| **Campaña Completa (Eden 1-3 & Plymouth 1-3)**| ✅ | 6 teatros de operación incluyendo Eden 3 (*Escape del Monte Vulcan*) y Plymouth 3 (*Asalto a los Riscos de Obsidiana*) con mapas `.map` · [`docs/formats/campaign_progression_and_debrief.md`](docs/formats/campaign_progression_and_debrief.md) |
| **Motor de Sprites y Animación**    | ✅ | Orientación de chasis en 8 direcciones ($45^\circ$), torretas, taladros rotatorios del Robo-Miner y detonaciones de fuego en 12 fases · [`docs/formats/sprites_and_animation.md`](docs/formats/sprites_and_animation.md) |
| **SoundBank Auténtico y Voces**     | ✅ | Tabla clásica de 252 registros, transmisiones de radio tácticas de Kathy Cullis y SFX precargados en WASM con cero latencia · [`docs/formats/sound_and_voices.md`](docs/formats/sound_and_voices.md) |
| **WebHUD Moderno y Radar Minimapa** | ✅ | Minimapa con niebla de guerra, alertas de desastres (lava, vórtices, meteoros), paneles de fábrica/tecnología y atajos de teclado · [`docs/formats/ui_and_hud.md`](docs/formats/ui_and_hud.md) |
| **Persistencia y Archivos .op2save** | ✅ | Ranuras en `LocalStorage` (1-5, F5 QuickSave, F9 QuickLoad, AutoSave a 100 Sols) y exportación/importación de `.op2save` · [`docs/formats/save_games_and_lobby.md`](docs/formats/save_games_and_lobby.md) |
| **Empaquetado WASM y Táctil Móvil** | ✅ | Escalado Retina High-DPI, desbloqueo de audio por interacción, gestos táctiles (zoom por pellizco y arrastre) y paquete optimizado · [`docs/formats/wasm_packaging_and_web_runtime.md`](docs/formats/wasm_packaging_and_web_runtime.md) |
| **Menú Principal Retro Interactivo y Sistema de Briefing** | ✅ | Menú principal clásico en Canvas, selector de escenarios, cinemáticas narrativas con voz de Kathy Cullis y filtro scanline CRT · [`docs/formats/main_menu_and_briefing.md`](docs/formats/main_menu_and_briefing.md) |
| **Generador Procedimental de Mapas de Escaramuza (4 Biomas, Simetrías, Conectividad A\*)** | ✅ | Generador equilibrado para escaramuza: 4 biomas (Cuenca Volcánica, Cañones y Riscos, Yermos Corrompidos, Cresta de Obsidiana), simetrías 2 vías/4 vías/espejo y conectividad vehicular A* · [`docs/formats/procedural_map_generator.md`](docs/formats/procedural_map_generator.md) |
| **PWA Offline con Service Worker y Matriz de Ganancia de Audio** | ✅ | Service Worker Cache-First (`sw.js`), manifiesto e iconos PWA, nodos de ganancia Web Audio (Maestro/SFX/Música) y multiplicadores de velocidad 0.5x–4.0x · [`docs/formats/pwa_and_offline_deployment.md`](docs/formats/pwa_and_offline_deployment.md) |
| **Telemetría Multijugador WebRTC y Chat Táctico en Partida** | ✅ | Monitor de latencia RTT en tiempo real, estado de sincronización lockstep y chat táctico entre facciones con colores de bando (`[Eden]` / `[Plymouth]`) · [`docs/formats/multiplayer_matchmaking_and_telemetry.md`](docs/formats/multiplayer_matchmaking_and_telemetry.md) |
| **Motor de Localización en 6 Idiomas en el Juego** | ✅ | Motor de traducción nativo integral para EN, PT-BR, ES, DE, ZH-CN y RU en todos los menús, telemetría del HUD, briefings de campaña y evaluaciones de debriefing · [`docs/formats/i18n_and_localization.md`](docs/formats/i18n_and_localization.md) |
| **Exhaustiva Suite de QA con 112 Tests Unitarios y de Integración** | ✅ | 108 pruebas unitarias + 4 pruebas de integración extremo a extremo verificando campañas completas, bots de escaramuza, sincronización lockstep WebRTC y persistencia de partidas · [`docs/formats/integration_testing_and_qa.md`](docs/formats/integration_testing_and_qa.md) |
| **Despliegue Vercel & WASM** | ✅ | Pipeline de Trunk con optimización LTO release (`vercel.json`) |

---

## 🎮 Ejecutar el Juego

```bash
# Instalar herramientas con mise
mise install

# Iniciar servidor de desarrollo local
mise exec -- trunk serve

# O compilar versión optimizada para producción
mise exec -- trunk build --release
```

---

## 📜 Licencia

El código fuente de **outpost2** está licenciado bajo la **GNU General Public License v3.0 o posterior** ([LICENSE](LICENSE)).
