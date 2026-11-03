<div align="center">

# outpost2

**Engenharia reversa e porte web moderno (Rust, WebAssembly, HTML5 Canvas & Web Audio API) de _Outpost 2: Divided Destiny_ (Sierra On-Line / Dynamix, 1997)**

[![Rust](https://img.shields.io/badge/rust-1.80+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-webassembly-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Status](https://img.shields.io/badge/status-em--progresso-yellow)](#-progresso)
[![Fase](https://img.shields.io/badge/fase-1_%26_2_%E2%80%94_re_%26_engine_rust-blue)](docs/PLAN.md)
[![Plataformas](https://img.shields.io/badge/plataformas-Web_%7C_Vercel_%7C_Mobile-lightgrey)](#-executando-o-jogo)
[![Licença: GPL-3.0](https://img.shields.io/badge/licen%C3%A7a-GPL--3.0--or--later-blue)](LICENSE)

_A obra-prima de estratégia em tempo real de ficção científica da Sierra & Dynamix,<br>reengenheirada para rodar nativamente em navegadores modernos, desktop e mobile._

[English](README.md) · **Português** · [Español](README.es.md) · [Deutsch](README.de.md) · [中文](README.zh-CN.md) · [Русский](README.ru.md)

</div>

---

## 🚀 Sobre

O **outpost2** estuda o executável original de _Outpost 2: Divided Destiny_ (`Outpost2.exe`) e seus arquivos de dados (`art.vol`, `maps.vol`, `sound.vol`, `tech.vol`) (Sierra On-Line / Dynamix, 1997) utilizando **REA** (Reverse Engineer Anything) e Ghidra para mapear cada hierarquia de classes, estruturas de dados e mecânica de jogo, entregando uma implementação web moderna e limpa.

Diferente de máquinas virtuais pesadas ou emuladores x86, este porte roda nativamente em **Rust compilado para WebAssembly (WASM)** com HTML5 Canvas e Web Audio:

- 🌐 **Simulação Nativa Rust / WebAssembly**: Simulação 100% no cliente com segurança de memória, zero sobrecarga de emulação e alto desempenho.
- 🗺️ **Engine Isométrica de Alta Resolução**: Viewport em grade isométrica 2D autêntica com zoom suave, rolagem sub-pixel de câmera, ciclo dia/noite e camadas dinâmicas de terreno (fluxos de lava, praga microbiana e trilhas de desastre).
- ⚙️ **HUD Sci-Fi Moderno**: Interface de comando tática de alta tecnologia com telemetria da colônia em tempo real, balanceamento de rede elétrica, suporte de vida, estoques de minério e minimapa de radar interativo.
- 📱 **Mobile & Touch-Ready**: Layout otimizado para desktop, tablets e smartphones, com suporte a arrastar com o dedo, pinça para zoom e barras de ação rápida.
- 🔊 **Áudio Espacial Web**: Reprodução nativa de notificações de voz clássicas, confirmações de unidades, ruídos de maquinário e trilhas sonoras.
- 🌍 **Localização Automática (i18n)**: Suporte multilíngue com troca de idioma em tempo real (**Inglês**, **Português do Brasil**, **Espanhol**, **Alemão**, **Chinês Simplificado** e **Russo**).
- ⚡ **Deploy Instantâneo na Vercel**: Empacotado como uma aplicação WebAssembly estática ultrarrápida, pronta para deploy contínuo na Vercel ou qualquer CDN de ponta.

---

## 📊 Progresso

| Área | Status | Detalhes |
|---|:---:|---|
| **Plano & Arquitetura** | ✅ | Estrutura completa do projeto e roadmap · [`docs/PLAN.md`](docs/PLAN.md) |
| **Engenharia Reversa com REA** | ✅ | Análise PE de `Outpost2.exe`, 509 APIs C++ exportadas · [`re/engine_symbols.json`](re/engine_symbols.json) |
| **Leitor de Volume `.vol`** | ✅ | Parser puro em Rust para arquivos de volume · [`docs/formats/vol_archive.md`](docs/formats/vol_archive.md) |
| **Decodificadores de Paleta & Sprites** | ✅ | Paleta de 256 cores, 5.390 definições de sprite em `.prt` · [`docs/formats/prt_format.md`](docs/formats/prt_format.md) |
| **Renderizador Isométrico de Tiles** | ✅ | Grade isométrica dinâmica, iluminação diurna dia/noite e culling · [`docs/formats/palette_and_tiles.md`](docs/formats/palette_and_tiles.md) |
| **Névoa de Guerra & Radar** | ✅ | Grid sensorial de visibilidade em 3 níveis (Inexplorado, Explorado, Visível) · [`docs/formats/fog_of_war.md`](docs/formats/fog_of_war.md) |
| **Núcleo de Simulação da Colônia** | ✅ | Logística Eden e Plymouth: fluxo de energia, suporte à vida, minério, moral e desastres · [`docs/formats/disasters.md`](docs/formats/disasters.md) |
| **Estruturas & Rede de Tubos** | ✅ | Kits modulares de construção e rede arterial de tubos pressurizados · [`docs/formats/structures_and_tubes.md`](docs/formats/structures_and_tubes.md) |
| **Controle de Unidades & 8-Way A\*** | ✅ | Pathfinding A* isométrico em 8 direções, evasão de lava e ordens com botão direito · [`docs/ENGINE_ARCHITECTURE.md`](docs/ENGINE_ARCHITECTURE.md) |
| **Árvore de Pesquisa Tecnológica** | ✅ | 12 disciplinas científicas com investimento dinâmico de laboratório · [`docs/formats/tech_tree.md`](docs/formats/tech_tree.md) |
| **Persistência de Estado & Touch** | ✅ | Salvamento/carregamento rápido no navegador, gestos touch e pinch-to-zoom |
| **Engine de Áudio** | ✅ | Sintetizador Web Audio API para beeps de interface, chimes de seleção e ordens |
| **HUD Tático Sci-Fi** | ✅ | Telemetria em tempo real, minimapa de radar, filas de produção e console de unidades |
| **Localização (i18n)** | ✅ | Esquema multilíngue: EN, PT-BR, ES, DE, ZH-CN, RU |
| **Pronto para Vercel & WASM** | ✅ | Pipeline do Trunk com otimização LTO em modo release (`vercel.json`) |

---

## 🎮 Executando o Jogo

### Pré-requisitos
- [Rust](https://www.rust-lang.org/) (versão estável)
- Gerenciador de ferramentas [mise](https://mise.jdx.dev/)
- [Trunk](https://trunkrs.dev/) (empacotador de aplicações web WASM em Rust)

### Configuração com o `mise`
```bash
# Clone o repositório
git clone https://github.com/pedryvo/outpost2.git
cd outpost2

# Ative as ferramentas do projeto via mise
mise install
```

### Desenvolvimento Local
```bash
# Inicie o servidor de desenvolvimento do Trunk com recarregamento rápido
trunk serve
```

Abra o navegador em `http://127.0.0.1:8080` para testar imediatamente!

### Compilação de Produção
```bash
# Gere a distribuição estática otimizada para WebAssembly
trunk build --release
```

A pasta `dist/` resultante contém arquivos estáticos independentes prontos para publicação na Vercel, Netlify ou GitHub Pages.

---

## 🚀 Deploy na Vercel

Publique com um único comando pela Vercel CLI:
```bash
vercel
```

Ou conecte o repositório `pedryvo/outpost2` ao seu painel da [Vercel](https://vercel.com/) para deploys automáticos a cada `git push`.

---

## ⚖️ Nota Legal

_Outpost 2: Divided Destiny_, suas logomarcas, artes, áudios e universo são marcas registradas e copyright © Sierra On-Line, Dynamix e Activision Blizzard.

Este é um projeto não comercial, educacional e de código aberto feito por fãs para preservação e interoperabilidade de software. Não possui afiliação, endosso ou patrocínio de Sierra, Dynamix ou Activision.

O código-fonte de **outpost2** está licenciado sob a **GNU General Public License v3.0 ou posterior** ([LICENSE](LICENSE)).
