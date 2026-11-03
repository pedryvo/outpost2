# Outpost 2: Divided Destiny — Campaign Scenarios & Mission Directives Specification

## 1. Overview
In *Outpost 2: Divided Destiny*, players command either the Eden Foundation or the Plymouth Colony across narrative-driven survival scenarios on the dying world of New Terra. Every scenario establishes primary objectives, planetary hazards, and victory/evacuation failure conditions.

---

## 2. Scenario 01: "Operation New Dawn — Sector Eden Prime"

### 2.1 Narrative Context
Following the crash of the Starship Conestoga, Eden survivors establish an initial foothold on the rugged basalt flats of Sector Eden Prime. Unstable subterranean volcanic fault lines and rogue Plymouth scouting squads threaten the nascent colony's existence.

### 2.2 Objective Architecture
- **Primary Directives:**
  1. **Ore Stockpiles:** Accumulate a strategic reserve of at least 3,000 tonnes of Common Ore and 600 tonnes of Rare Ore.
  2. **Tactical Armor:** Commission and deploy at least 2 armed combat vehicles (Panther Battle Tanks or Lynx Scouts) to repel hostile raiders.
  3. **Scientific Progress:** Complete at least one breakthrough research project in the technological tree.
  4. **Command Core Survival:** Prevent the destruction of the central Command Center structure.
- **Victory Condition:** All primary directives evaluated as `is_completed: true`.
- **Defeat Condition:** Total destruction of the Command Center or complete colony evacuation failure.

---

## 3. Mission Briefing UI & Atmospheric Audio
- **In-Game Directive Modal:** Accessible via the bottom tactical command console (`📋 Briefing`).
- **Interactive Tactical Minimap:** Direct camera centering by clicking anywhere on the tactical radar, displaying a live camera frustum box tracking viewport boundaries.
- **Procedural Ambient Music Engine:** Synthesizes low-frequency sub-bass drones and space arpeggios via the Web Audio API with dedicated mute toggle (`🎵 Music`).
