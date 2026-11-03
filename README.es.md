<div align="center">

# outpost2

**Ingeniería inversa y adaptación web moderna (Rust, WebAssembly, HTML5 Canvas y Web Audio API) de _Outpost 2: Divided Destiny_ (Sierra On-Line / Dynamix, 1997)**

[![Rust](https://img.shields.io/badge/rust-1.80+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-webassembly-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Estado](https://img.shields.io/badge/estado-completado-brightgreen)](#-progreso)
[![Fase](https://img.shields.io/badge/fase-7_%E2%80%94_campañas_y_nave-blue)](docs/PLAN.md)
[![Plataformas](https://img.shields.io/badge/plataformas-Web_%7C_Vercel_%7C_Mobile-lightgrey)](#-ejecutar-el-juego)
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
| **Niebla de Guerra y Radar** | ✅ | Rejilla de sensores de 3 niveles (Inexplorado, Explorado, Visible) · [`docs/formats/fog_of_war.md`](docs/formats/fog_of_war.md) |
| **Núcleo de Simulación Colonial** | ✅ | Logística Eden y Plymouth: energía, soporte vital, minerales, moral y desastres · [`docs/formats/disasters.md`](docs/formats/disasters.md) |
| **Estructuras y Red de Tubos** | ✅ | Kits modulares de construcción y red de tubos presurizados · [`docs/formats/structures_and_tubes.md`](docs/formats/structures_and_tubes.md) |
| **Control de Unidades y A\* 8-Vías** | ✅ | Pathfinding A* isométrico en 8 direcciones y órdenes tácticas · [`docs/ENGINE_ARCHITECTURE.md`](docs/ENGINE_ARCHITECTURE.md) |
| **Árbol de Investigación** | ✅ | 12 disciplinas científicas con inversión en laboratorios · [`docs/formats/tech_tree.md`](docs/formats/tech_tree.md) |
| **Campañas Multi-Mapa** | ✅ | Teatros de operación auténticos (Eden 1, Plymouth 1, Academia) · [`docs/formats/campaigns_and_scenarios.md`](docs/formats/campaigns_and_scenarios.md) |
| **Filas de Fábrica y Soldadura** | ✅ | Planificación de producción de vehículos con arcos de plasma y barras de progreso |
| **Evacuación en Nave Espacial** | ✅ | Fabricación en 4 etapas y mecánica de soporte vital · [`docs/formats/starship_assembly.md`](docs/formats/starship_assembly.md) |
| **Persistencia y Control Táctil** | ✅ | Guardado/carga rápida en navegador y gestos táctiles |
| **Motor de Audio y Radio** | ✅ | Sintetizador Web Audio, música procedural y radio militar |
| **Localización (i18n)** | ✅ | Soporte multilíngue para EN, PT-BR, ES, DE, ZH-CN, RU |
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
