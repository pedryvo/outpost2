//! In-Game Multilingual Internationalization (i18n) Engine
//!
//! Provides on-the-fly live language switching across all 6 canonical project languages:
//! - English (EN / "en")
//! - Portuguese - Brazil (PT-BR / "pt-BR")
//! - Spanish (ES / "es")
//! - German (DE / "de")
//! - Simplified Chinese (ZH-CN / "zh-CN")
//! - Russian (RU / "ru")
//!
//! Covers all UI domains:
//! - Main Menu navigation, options, and pauses
//! - Skirmish tactical parameters (factions, personalities, difficulties, abundances, disasters)
//! - Multiplayer WebRTC lobby (host, guest, SDP signaling, status)
//! - Campaign mission briefings (scenarios 0-6: Eden 1-3, Plymouth 1-3, Cadet Tutorial)
//! - In-game HUD & telemetry (gauges, rates, energy grid, sustenance, morale, factory, research)
//! - Mission debriefing evaluation (ranks, victory/defeat verdicts, stats labels)
//! - Unit chassis and colonial building designations

use serde::{Deserialize, Serialize};
use web_sys::Document;
use wasm_bindgen::JsCast;

// =========================================================================
// 1. Canonical Language Enum
// =========================================================================

/// Canonical project languages supported by the Outpost 2 engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Portuguese,
    Spanish,
    German,
    Chinese,
    Russian,
}

impl Language {
    /// Return slice of all 6 canonical project languages.
    #[allow(dead_code)]
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::Portuguese,
            Language::Spanish,
            Language::German,
            Language::Chinese,
            Language::Russian,
        ]
    }

    /// Primary 2-5 letter uppercase project code (e.g. "EN", "PT-BR").
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "EN",
            Language::Portuguese => "PT-BR",
            Language::Spanish => "ES",
            Language::German => "DE",
            Language::Chinese => "ZH-CN",
            Language::Russian => "RU",
        }
    }

    /// Standard BCP-47 lowercase/mixed locale tag (e.g. "en", "pt-BR").
    pub fn tag(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Portuguese => "pt-BR",
            Language::Spanish => "es",
            Language::German => "de",
            Language::Chinese => "zh-CN",
            Language::Russian => "ru",
        }
    }

    /// Endonym / native presentation name for the language.
    #[allow(dead_code)]
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Portuguese => "Português",
            Language::Spanish => "Español",
            Language::German => "Deutsch",
            Language::Chinese => "简体中文",
            Language::Russian => "Русский",
        }
    }

    /// Resolve string token (case-insensitive, tag or code) into canonical Language variant.
    pub fn from_code_or_tag(s: &str) -> Option<Self> {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "en" | "english" => Some(Language::English),
            "pt" | "pt-br" | "portuguese" | "português" => Some(Language::Portuguese),
            "es" | "spanish" | "español" => Some(Language::Spanish),
            "de" | "german" | "deutsch" => Some(Language::German),
            "zh" | "zh-cn" | "chinese" | "中文" => Some(Language::Chinese),
            "ru" | "russian" | "русский" => Some(Language::Russian),
            _ => None,
        }
    }

    /// Resolve string into Language, falling back to English upon unmapped token.
    pub fn from_code(s: &str) -> Self {
        Self::from_code_or_tag(s).unwrap_or(Language::English)
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

// =========================================================================
// 2. Domain-Specific Translation Dictionaries
// =========================================================================

/// Localization strings for Main Menu and Primary Navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainMenuDict {
    pub new_campaign: &'static str,
    pub campaign_missions: &'static str,
    pub skirmish: &'static str,
    pub skirmish_battle: &'static str,
    pub multiplayer: &'static str,
    pub multiplayer_lobby: &'static str,
    pub load_game: &'static str,
    pub load_archive: &'static str,
    pub options: &'static str,
    pub options_audio_velocity: &'static str,
    pub resume: &'static str,
    pub resume_operation: &'static str,
    pub quit: &'static str,
    pub back_to_menu: &'static str,
    pub back: &'static str,
    pub prev_op: &'static str,
    pub next_op: &'static str,
    pub begin_operation: &'static str,
    pub apply_and_return: &'static str,
    pub title_tagline: &'static str,
    pub subtitle: &'static str,
    pub select_theater: &'static str,
    pub select_theater_sub: &'static str,
    pub tutorial_button: &'static str,
    pub operational_directives_checklist: &'static str,
    pub tactical_operation_briefing: &'static str,
    pub directive_prefix: &'static str,
    pub secondary_prefix: &'static str,
}

/// Localization strings for Skirmish Match Configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkirmishDict {
    pub title: &'static str,
    pub map_selection: &'static str,
    pub player_faction: &'static str,
    pub bot_count: &'static str,
    pub personality_aggressive_rusher: &'static str,
    pub personality_defensive_fortifier: &'static str,
    pub personality_economic_tech_rusher: &'static str,
    pub difficulty_easy: &'static str,
    pub difficulty_medium: &'static str,
    pub difficulty_hard: &'static str,
    pub abundance: &'static str,
    pub abundance_sparse: &'static str,
    pub abundance_standard: &'static str,
    pub abundance_abundant: &'static str,
    pub abundance_infinite: &'static str,
    pub disaster: &'static str,
    pub disaster_disabled: &'static str,
    pub disaster_low: &'static str,
    pub disaster_standard: &'static str,
    pub disaster_catastrophic: &'static str,
    pub launch: &'static str,
    pub theater_params: &'static str,
    pub ai_slots_header: &'static str,
    pub bot_active: &'static str,
    pub bot_off: &'static str,
    pub status_summary: &'static str,
}

/// Localization strings for WebRTC Peer-to-Peer Multiplayer Lobby.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultiplayerDict {
    pub title: &'static str,
    pub host: &'static str,
    pub guest: &'static str,
    pub copy_offer: &'static str,
    pub paste_answer: &'static str,
    pub paste_offer: &'static str,
    pub generate_answer: &'static str,
    pub copy_answer: &'static str,
    pub lock_connection: &'static str,
    pub peer_connected: &'static str,
    pub peer_disconnected: &'static str,
    pub peer_connecting: &'static str,
    pub launch_match: &'static str,
    pub ready_toggle: &'static str,
    pub not_ready_toggle: &'static str,
    pub offer_ready_header: &'static str,
    pub awaiting_offer: &'static str,
    pub status_banner_label: &'static str,
}

/// Localization strings for Campaign Mission Briefings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BriefingScenarioDict {
    pub scenario_id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub transcript: &'static str,
    pub primary_directives: &'static [&'static str],
    pub secondary_directives: &'static [&'static str],
    pub hazard_summary: &'static str,
}

/// Localization strings for Real-Time HUD and Telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HudDict {
    pub common_ore: &'static str,
    pub rare_ore: &'static str,
    pub power: &'static str,
    pub food: &'static str,
    pub population: &'static str,
    pub sol: &'static str,
    pub sun: &'static str,
    pub starship: &'static str,
    pub tactical_radar: &'static str,
    pub radar_active: &'static str,
    // Energy grid operational conditions
    pub grid_optimal: &'static str,
    pub grid_brownout: &'static str,
    pub grid_blackout: &'static str,
    // Food security sustenance conditions
    pub food_surplus: &'static str,
    pub food_adequate: &'static str,
    pub food_rationing: &'static str,
    pub food_starvation: &'static str,
    // Morale tiers
    pub morale_excellent: &'static str,
    pub morale_good: &'static str,
    pub morale_fair: &'static str,
    pub morale_poor: &'static str,
    pub morale_rioting: &'static str,
    // Factory production UI
    pub factory_title: &'static str,
    pub factory_hull: &'static str,
    pub factory_cost: &'static str,
    pub factory_rally: &'static str,
    pub factory_queue: &'static str,
    pub factory_unassigned_rally: &'static str,
    pub factory_footer_hint: &'static str,
    // Research technology tree UI
    pub research_title: &'static str,
    pub research_active: &'static str,
    pub research_none: &'static str,
    pub research_footer_hint: &'static str,
}

/// Localization strings for Mission Debriefing and Performance Rankings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebriefingDict {
    pub rank_supreme_savior_title: &'static str,
    pub rank_supreme_savior_badge: &'static str,
    pub rank_supreme_savior_desc: &'static str,

    pub rank_distinguished_pioneer_title: &'static str,
    pub rank_distinguished_pioneer_badge: &'static str,
    pub rank_distinguished_pioneer_desc: &'static str,

    pub rank_tethys_survivor_title: &'static str,
    pub rank_tethys_survivor_badge: &'static str,
    pub rank_tethys_survivor_desc: &'static str,

    pub rank_colony_failure_title: &'static str,
    pub rank_colony_failure_badge: &'static str,
    pub rank_colony_failure_desc: &'static str,

    pub victory_title: &'static str,
    pub defeat_title: &'static str,
    pub victory_subtitle_template: &'static str,
    pub defeat_subtitle_template: &'static str,
    pub colony_duration: &'static str,
    pub evacuation_readiness: &'static str,
    pub survivors_morale: &'static str,
    pub refined_reserves: &'static str,
    pub score_label: &'static str,
    pub directives_checklist: &'static str,
    pub starship_escaped: &'static str,
    pub starship_assembled_template: &'static str,
}

/// Localization strings for Vehicle Chasses and Colonial Structures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitBuildingDict {
    // Structures
    pub command_center: &'static str,
    pub solar_array: &'static str,
    pub tokamak_reactor: &'static str,
    pub geothermal_plant: &'static str,
    pub common_ore_smelter: &'static str,
    pub rare_ore_smelter: &'static str,
    pub dirt_lab: &'static str,
    pub standard_lab: &'static str,
    pub residence: &'static str,
    pub agrodome: &'static str,
    pub vehicle_factory: &'static str,
    pub repair_facility: &'static str,
    pub guard_post: &'static str,
    pub spaceport: &'static str,
    pub terraformer: &'static str,
    // Units
    pub lynx: &'static str,
    pub panther: &'static str,
    pub tiger: &'static str,
    pub cargo_truck: &'static str,
    pub convec: &'static str,
    pub robo_miner: &'static str,
    pub robo_surveyor: &'static str,
    pub earthworker: &'static str,
    pub evacuation_transport: &'static str,
}

/// Localization strings for Options and Audio Configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionsDict {
    pub title: &'static str,
    pub master_volume: &'static str,
    pub sim_velocity: &'static str,
    pub crt_scanlines: &'static str,
    pub active_language: &'static str,
    pub enabled: &'static str,
    pub disabled: &'static str,
}

/// Localization strings for Tactical Pause Menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PauseMenuDict {
    pub title: &'static str,
    pub resume: &'static str,
    pub quicksave: &'static str,
    pub quickload: &'static str,
    pub save_load_manager: &'static str,
    pub options: &'static str,
    pub restart: &'static str,
    pub surrender: &'static str,
}

// =========================================================================
// 3. Complete Translation Package
// =========================================================================

/// Complete unified translation dictionary covering all subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Translation {
    // Legacy public fields for 100% backward compatibility
    pub faction: &'static str,
    pub sol: &'static str,
    pub sun: &'static str,
    pub pop: &'static str,
    pub power: &'static str,
    pub common_ore: &'static str,
    pub rare_ore: &'static str,
    pub food: &'static str,
    pub build_base: &'static str,
    pub units: &'static str,
    pub research: &'static str,
    pub save: &'static str,
    pub load: &'static str,
    pub radar: &'static str,
    pub briefing: &'static str,
    pub music: &'static str,

    // Strongly-typed sub-dictionaries
    pub language: Language,
    pub main_menu: MainMenuDict,
    pub skirmish: SkirmishDict,
    pub multiplayer: MultiplayerDict,
    pub hud: HudDict,
    pub debriefing: DebriefingDict,
    pub units_buildings: UnitBuildingDict,
    pub options: OptionsDict,
    pub pause_menu: PauseMenuDict,
    pub scenarios: [BriefingScenarioDict; 7],
}

impl Translation {
    /// Retrieve scenario briefing dictionary by internal scenario ID (0..=6), falling back to 0.
    #[allow(dead_code)]
    pub fn scenario(&self, id: usize) -> &BriefingScenarioDict {
        if id < self.scenarios.len() {
            &self.scenarios[id]
        } else {
            &self.scenarios[0]
        }
    }

    /// Retrieve localized structure name.
    pub fn structure_name(&self, st: &crate::sim::StructureType) -> &'static str {
        match st {
            crate::sim::StructureType::CommandCenter => self.units_buildings.command_center,
            crate::sim::StructureType::SolarArray => self.units_buildings.solar_array,
            crate::sim::StructureType::TokamakReactor => self.units_buildings.tokamak_reactor,
            crate::sim::StructureType::GeothermalPlant => self.units_buildings.geothermal_plant,
            crate::sim::StructureType::CommonOreSmelter => self.units_buildings.common_ore_smelter,
            crate::sim::StructureType::RareOreSmelter => self.units_buildings.rare_ore_smelter,
            crate::sim::StructureType::DirtLab => self.units_buildings.dirt_lab,
            crate::sim::StructureType::StandardLab => self.units_buildings.standard_lab,
            crate::sim::StructureType::Residence => self.units_buildings.residence,
            crate::sim::StructureType::Agrodome => self.units_buildings.agrodome,
            crate::sim::StructureType::VehicleFactory => self.units_buildings.vehicle_factory,
            crate::sim::StructureType::RepairFacility => self.units_buildings.repair_facility,
            crate::sim::StructureType::GuardPost => self.units_buildings.guard_post,
            crate::sim::StructureType::Spaceport => self.units_buildings.spaceport,
            crate::sim::StructureType::Terraformer => self.units_buildings.terraformer,
        }
    }

    /// Retrieve localized vehicle hull name.
    pub fn vehicle_name(&self, vh: &crate::render::hud::VehicleHull) -> &'static str {
        match vh {
            crate::render::hud::VehicleHull::Lynx => self.units_buildings.lynx,
            crate::render::hud::VehicleHull::Panther => self.units_buildings.panther,
            crate::render::hud::VehicleHull::Tiger => self.units_buildings.tiger,
            crate::render::hud::VehicleHull::CargoTruck => self.units_buildings.cargo_truck,
            crate::render::hud::VehicleHull::ConVec => self.units_buildings.convec,
            crate::render::hud::VehicleHull::RoboMiner => self.units_buildings.robo_miner,
            crate::render::hud::VehicleHull::RoboSurveyor => self.units_buildings.robo_surveyor,
            crate::render::hud::VehicleHull::Earthworker => self.units_buildings.earthworker,
        }
    }

    /// Retrieve localized energy grid alert label.
    pub fn energy_grid_status(&self, state: crate::render::hud::EnergyGridState) -> &'static str {
        match state {
            crate::render::hud::EnergyGridState::Normal => self.hud.grid_optimal,
            crate::render::hud::EnergyGridState::Brownout => self.hud.grid_brownout,
            crate::render::hud::EnergyGridState::Blackout => self.hud.grid_blackout,
        }
    }

    /// Retrieve localized morale tier label.
    pub fn morale_tier(&self, tier: crate::render::hud::MoraleTier) -> &'static str {
        match tier {
            crate::render::hud::MoraleTier::Excellent => self.hud.morale_excellent,
            crate::render::hud::MoraleTier::Good => self.hud.morale_good,
            crate::render::hud::MoraleTier::Fair => self.hud.morale_fair,
            crate::render::hud::MoraleTier::Poor => self.hud.morale_poor,
            crate::render::hud::MoraleTier::Rioting => self.hud.morale_rioting,
        }
    }

    /// Retrieve localized sustenance condition label.
    pub fn sustenance_state(&self, state: crate::render::hud::SustenanceState) -> &'static str {
        match state {
            crate::render::hud::SustenanceState::Surplus => self.hud.food_surplus,
            crate::render::hud::SustenanceState::Adequate => self.hud.food_adequate,
            crate::render::hud::SustenanceState::Deficit => self.hud.food_rationing,
            crate::render::hud::SustenanceState::Starvation => self.hud.food_starvation,
        }
    }

    /// Retrieve localized colonial commander rank title.
    pub fn colonial_rank_title(&self, rank: crate::sim::debrief::ColonialRank) -> &'static str {
        match rank {
            crate::sim::debrief::ColonialRank::SupremeColonySavior => self.debriefing.rank_supreme_savior_title,
            crate::sim::debrief::ColonialRank::DistinguishedPioneer => self.debriefing.rank_distinguished_pioneer_title,
            crate::sim::debrief::ColonialRank::TethysSurvivor => self.debriefing.rank_tethys_survivor_title,
            crate::sim::debrief::ColonialRank::ColonyFailure => self.debriefing.rank_colony_failure_title,
        }
    }

    /// Retrieve localized colonial commander rank badge string.
    pub fn colonial_rank_badge(&self, rank: crate::sim::debrief::ColonialRank) -> &'static str {
        match rank {
            crate::sim::debrief::ColonialRank::SupremeColonySavior => self.debriefing.rank_supreme_savior_badge,
            crate::sim::debrief::ColonialRank::DistinguishedPioneer => self.debriefing.rank_distinguished_pioneer_badge,
            crate::sim::debrief::ColonialRank::TethysSurvivor => self.debriefing.rank_tethys_survivor_badge,
            crate::sim::debrief::ColonialRank::ColonyFailure => self.debriefing.rank_colony_failure_badge,
        }
    }

    /// Retrieve localized colonial commander rank narrative evaluation.
    pub fn colonial_rank_description(&self, rank: crate::sim::debrief::ColonialRank) -> &'static str {
        match rank {
            crate::sim::debrief::ColonialRank::SupremeColonySavior => self.debriefing.rank_supreme_savior_desc,
            crate::sim::debrief::ColonialRank::DistinguishedPioneer => self.debriefing.rank_distinguished_pioneer_desc,
            crate::sim::debrief::ColonialRank::TethysSurvivor => self.debriefing.rank_tethys_survivor_desc,
            crate::sim::debrief::ColonialRank::ColonyFailure => self.debriefing.rank_colony_failure_desc,
        }
    }
}

// =========================================================================
// 4. Static Compiled Language Dictionaries
// =========================================================================

// --- English Dictionary ---
static TRANSLATION_EN: Translation = Translation {
    faction: "Faction:",
    sol: "Sol:",
    sun: "Sun:",
    pop: "Pop:",
    power: "Power:",
    common_ore: "Common Ore:",
    rare_ore: "Rare Ore:",
    food: "Food:",
    build_base: "🏗️ Build Base",
    units: "🚜 Units",
    research: "🔬 Research",
    save: "💾 Save",
    load: "📂 Load",
    radar: "Tactical Radar",
    briefing: "📋 Briefing",
    music: "🎵 Music",

    language: Language::English,
    main_menu: MainMenuDict {
        new_campaign: "New Campaign",
        campaign_missions: "🚀 CAMPAIGN MISSIONS (EDEN & PLYMOUTH)",
        skirmish: "Skirmish",
        skirmish_battle: "⚔️ SKIRMISH BATTLE (1-5 AI BOTS)",
        multiplayer: "Multiplayer",
        multiplayer_lobby: "📡 MULTIPLAYER P2P (WEBRTC LOBBY)",
        load_game: "Load Game",
        load_archive: "📂 LOAD MISSION ARCHIVE (.OP2SAVE)",
        options: "Options",
        options_audio_velocity: "⚙️ OPTIONS / AUDIO & VELOCITY",
        resume: "Resume",
        resume_operation: "▶️ RESUME CURRENT OPERATION",
        quit: "Quit",
        back_to_menu: "◀ BACK TO MENU",
        back: "◀ BACK",
        prev_op: "◀ PREV OP",
        next_op: "NEXT OP ▶",
        begin_operation: "🚀 BEGIN OPERATION",
        apply_and_return: "APPLY & RETURN",
        title_tagline: "SIERRA ON-LINE & DYNAMICS ENGINE // TETHYS EXPEDITION",
        subtitle: "—  D I V I D E D   D E S T I N Y  —",
        select_theater: "SELECT CAMPAIGN THEATER OF OPERATIONS",
        select_theater_sub: "Choose allegiance between the scientific Eden Foundation and Plymouth Brotherhood.",
        tutorial_button: "🎓 CADET PROVING GROUNDS (ACADEMY TRAINING SIMULATOR)",
        operational_directives_checklist: "OPERATIONAL DIRECTIVES CHECKLIST:",
        tactical_operation_briefing: "TACTICAL OPERATION BRIEFING",
        directive_prefix: "DIRECTIVE",
        secondary_prefix: "SECONDARY",
    },
    skirmish: SkirmishDict {
        title: "TACTICAL SKIRMISH LOBBY // SECTOR ENGAGEMENT",
        map_selection: "MAP",
        player_faction: "FACTION",
        bot_count: "Opponents",
        personality_aggressive_rusher: "Aggressive Rusher",
        personality_defensive_fortifier: "Defensive Fortifier",
        personality_economic_tech_rusher: "Economic Tech Rusher",
        difficulty_easy: "Easy",
        difficulty_medium: "Medium",
        difficulty_hard: "Hard",
        abundance: "RESOURCES",
        abundance_sparse: "Sparse (1,000 C / 200 R)",
        abundance_standard: "Standard (2,500 C / 600 R)",
        abundance_abundant: "Abundant (5,000 C / 1,500 R)",
        abundance_infinite: "Infinite Ore (50,000 C / 20,000 R)",
        disaster: "DISASTERS",
        disaster_disabled: "Disabled (Quiescent)",
        disaster_low: "Low (Occasional Tremors)",
        disaster_standard: "Standard (Normal Hazards)",
        disaster_catastrophic: "Catastrophic (Extreme)",
        launch: "⚔️ LAUNCH SKIRMISH",
        theater_params: "THEATER & SIMULATION PARAMETERS:",
        ai_slots_header: "COMBATANT AI SLOTS (1 TO 5 OPPONENTS):",
        bot_active: "[X] ACTIVE",
        bot_off: "[ ] OFF",
        status_summary: "Status",
    },
    multiplayer: MultiplayerDict {
        title: "TACTICAL WEBRTC PEER-TO-PEER LOBBY (SERVERLESS)",
        host: "HOST MATCH (OFFER)",
        guest: "JOIN AS GUEST (ANSWER)",
        copy_offer: "📋 COPY OFFER TO CLIPBOARD",
        paste_answer: "📥 PASTE GUEST ANSWER",
        paste_offer: "📥 PASTE HOST OFFER",
        generate_answer: "ACCEPT OFFER & GENERATE ANSWER",
        copy_answer: "📋 COPY ANSWER (SEND TO HOST)",
        lock_connection: "LOCK CONNECTION",
        peer_connected: "Peer Connected",
        peer_disconnected: "Disconnected",
        peer_connecting: "Connecting...",
        launch_match: "🚀 LAUNCH MATCH",
        ready_toggle: "READY: YES [TOGGLE]",
        not_ready_toggle: "READY: NO [TOGGLE]",
        offer_ready_header: "GENERATED BASE64 SDP OFFER (READY TO SHARE):",
        awaiting_offer: "Awaiting WebRTC offer generation...",
        status_banner_label: "CONNECTION STATUS",
    },
    hud: HudDict {
        common_ore: "Common Ore",
        rare_ore: "Rare Ore",
        power: "Power",
        food: "Food",
        population: "Population",
        sol: "Sol",
        sun: "Sunlight",
        starship: "Starship",
        tactical_radar: "Tactical Radar",
        radar_active: "ACTIVE",
        grid_optimal: "GRID OPTIMAL",
        grid_brownout: "BROWNOUT ALERT",
        grid_blackout: "BLACKOUT CRITICAL",
        food_surplus: "Surplus",
        food_adequate: "Adequate",
        food_rationing: "Rationing",
        food_starvation: "STARVATION",
        morale_excellent: "Excellent",
        morale_good: "Good",
        morale_fair: "Fair",
        morale_poor: "Poor",
        morale_rioting: "Rioting",
        factory_title: "FACTORY PRODUCTION & CONVEC KITS",
        factory_hull: "Hull",
        factory_cost: "Cost",
        factory_rally: "Rally Point",
        factory_queue: "Fabrication Queue:",
        factory_unassigned_rally: "Rally Point: Unassigned (Factory Bay)",
        factory_footer_hint: "Press 'F' to toggle panel | Right click to set Rally Point",
        research_title: "COLONIAL SCIENTIFIC RESEARCH MATRIX",
        research_active: "Active Project",
        research_none: "No Active Research Project",
        research_footer_hint: "Press 'R' to toggle research matrix",
    },
    debriefing: DebriefingDict {
        rank_supreme_savior_title: "Supreme Colony Savior",
        rank_supreme_savior_badge: "🎖️ Supreme Colony Savior",
        rank_supreme_savior_desc: "Flawless leadership: Minimal casualties, superior industrial output, and complete strategic dominance.",

        rank_distinguished_pioneer_title: "Distinguished Pioneer",
        rank_distinguished_pioneer_badge: "🌟 Distinguished Pioneer",
        rank_distinguished_pioneer_desc: "Commendable command: Colonial objectives secured with resilience despite hazardous planetary conditions.",

        rank_tethys_survivor_title: "Tethys Survivor",
        rank_tethys_survivor_badge: "🛡️ Tethys Survivor",
        rank_tethys_survivor_desc: "Pyrrhic survival: Colony survived through heavy adversity and significant resource expenditure.",

        rank_colony_failure_title: "Colony Failure",
        rank_colony_failure_badge: "☠️ Colony Failure",
        rank_colony_failure_desc: "Catastrophic collapse: Planetary outpost lost, civilian casualties critical, or primary directives failed.",

        victory_title: "MISSION ACCOMPLISHED: VICTORY",
        defeat_title: "COLONIAL DEFEAT: MISSION TERMINATED",
        victory_subtitle_template: "Congratulations, Commander! Primary directives have been successfully executed.",
        defeat_subtitle_template: "Critical failure. Colony Command Center has been obliterated.",
        colony_duration: "Colony Duration",
        evacuation_readiness: "Evacuation Readiness",
        survivors_morale: "Survivors / Morale",
        refined_reserves: "Refined Reserves",
        score_label: "Composite Colony Score",
        directives_checklist: "Directive Performance Checklist",
        starship_escaped: "🚀 Escaped into Orbit!",
        starship_assembled_template: "{:.0}% Assembled",
    },
    units_buildings: UnitBuildingDict {
        command_center: "Command Center",
        solar_array: "Solar Power Array",
        tokamak_reactor: "Tokamak Fusion Reactor",
        geothermal_plant: "Geothermal Power Plant",
        common_ore_smelter: "Common Ore Smelter",
        rare_ore_smelter: "Rare Ore Smelter",
        dirt_lab: "D.I.R.T. Laboratory",
        standard_lab: "Standard Research Lab",
        residence: "Colonial Residence",
        agrodome: "Bio-Dome Hydroponics",
        vehicle_factory: "Vehicle Factory",
        repair_facility: "Vehicle Repair Depot",
        guard_post: "Guard Post Turret",
        spaceport: "Starship Launch Gantry",
        terraformer: "Atmospheric Terraformer",

        lynx: "Lynx Scout",
        panther: "Panther Battle Tank",
        tiger: "Tiger Heavy Tank",
        cargo_truck: "Cargo Truck",
        convec: "ConVec Transport",
        robo_miner: "Robo-Miner",
        robo_surveyor: "Robo-Surveyor",
        earthworker: "Earthworker",
        evacuation_transport: "Evacuation Transport",
    },
    options: OptionsDict {
        title: "SYSTEM OPTIONS & AUDIO MATRIX",
        master_volume: "Master Audio Output Volume:",
        sim_velocity: "Simulation Velocity Scaling:",
        crt_scanlines: "CRT Retro Scanline Overlay:",
        active_language: "Active Language / Locale:",
        enabled: "ENABLED",
        disabled: "DISABLED",
    },
    pause_menu: PauseMenuDict {
        title: "TACTICAL SIMULATION PAUSED",
        resume: "▶️ RESUME MISSION (ESC)",
        quicksave: "💾 QUICKSAVE (F5)",
        quickload: "📂 QUICKLOAD (F9)",
        save_load_manager: "🗄️ SAVE / LOAD MANAGER",
        options: "⚙️ AUDIO & VELOCITY OPTIONS",
        restart: "🔄 RESTART MISSION",
        surrender: "💀 SURRENDER / ABORT TO MENU",
    },
    scenarios: [
        // Scenario 0: Eden 1 (Operation New Dawn)
        BriefingScenarioDict {
            scenario_id: 0,
            title: "Operation New Dawn",
            subtitle: "Sector Eden Prime // Planetary Inhabitation",
            transcript: "Colonists of the Eden Foundation: After the catastrophic failure of the Starship Conestoga, our landing site on New Terra must be secured immediately. Volcanic tremors indicate rising magma fissures nearby, and rogue Plymouth raiders have established aggressive perimeter posts.\n\nSecure colonial life support, automate freight ore hauling, commission defensive railgun armor, and advance planetary research to ensure the survival of our species.",
            primary_directives: &[
                "Stockpile 3,000t Common Ore and 600t Rare Ore",
                "Deploy and commission at least 2 armed combat tanks",
                "Complete scientific research in any primary discipline",
                "Protect the colony Command Center from hostile destruction",
            ],
            secondary_directives: &[
                "Establish a colony population of 120 colonists with positive morale",
            ],
            hazard_summary: "Localized dust vortices; minor seismic tremors.",
        },
        // Scenario 1: Plymouth 1 (Operation Badland Vanguard)
        BriefingScenarioDict {
            scenario_id: 1,
            title: "Operation Badland Vanguard",
            subtitle: "Sector Plymouth South // Territorial Reclamation",
            transcript: "Commanders of Plymouth: Rejecting Eden's authoritarian technocracy, our settlers carved a home in the jagged badland trenches. Eden scout patrols are encroaching on our geothermal vents.\n\nConstruct a Vehicle Factory, establish automated mining convoys, assemble a strike force of 3 combat tanks, and eliminate Eden raiders to secure our sovereign territory.",
            primary_directives: &[
                "Commission a Vehicle Factory to manufacture armored chassis",
                "Assemble a strike squadron of at least 3 combat tanks",
                "Mine 2,500t Common Ore to secure the canyon perimeter",
                "Defend the Plymouth Command Center against Eden assault",
            ],
            secondary_directives: &[
                "Maintain continuous power surplus throughout badland night cycles",
            ],
            hazard_summary: "High volcanic tremor risk; active geothermal gas vents.",
        },
        // Scenario 2: Tutorial (Cadet Proving Grounds)
        BriefingScenarioDict {
            scenario_id: 2,
            title: "Cadet Proving Grounds",
            subtitle: "Colonial Academy // Tactical Operations Simulator",
            transcript: "Cadet: Welcome to the New Terra Colonial Operations Simulator. In this trial, you will master the fundamentals of planetary colony management.\n\nVerify that base power generation exceeds consumption, deploy pressurized transport tubes between facilities, and oversee automated mineral transport to the smelter.",
            primary_directives: &[
                "Establish positive energy surplus (Power Produced > Consumed)",
                "Connect structures with at least 15 pressurized transport tubes",
                "Haul first freight load of Common Ore to the Smelter",
                "Maintain colonist morale above 90%",
            ],
            secondary_directives: &[
                "Commission an automated Robo-Miner on an ore beacon",
            ],
            hazard_summary: "Controlled simulator environment; low environmental hazard.",
        },
        // Scenario 3: Eden 2 (Operation Caldera Bastion)
        BriefingScenarioDict {
            scenario_id: 3,
            title: "Operation Caldera Bastion",
            subtitle: "Mount Vulcan Foothills // Molten Fissure Containment",
            transcript: "Eden Command Alert: Subsurface thermal sensors register catastrophic magma displacement along the northern fault lines. Molten fissures are rupturing the permafrost, threatening colonial survival dome sectors.\n\nStabilize base power generation above 400 kW using high-output arrays or geothermal conduits, construct a fortified perimeter of at least 2 defensive Guard Post Turrets overlooking fissure corridors, stockpile 2,500t Common Ore and 500t Rare Ore for emergency thermal barricades, and ensure the Eden Command Center remains secure from seismic fissures and hostile bombardment.",
            primary_directives: &[
                "Establish geothermal containment grid (Generate >= 400 kW power)",
                "Construct at least 2 defensive Guard Post Turrets guarding fissure rim",
                "Stockpile 2,500t Common Ore and 500t Rare Ore for reinforcement",
                "Protect the Eden Command Center from seismic or hostile annihilation",
            ],
            secondary_directives: &[
                "Maintain colonial population morale at or above 80%",
                "Complete scientific research in any advanced discipline",
            ],
            hazard_summary: "Spreading volcanic lava fissures; advancing magma flows.",
        },
        // Scenario 4: Plymouth 2 (Operation Iron Fang)
        BriefingScenarioDict {
            scenario_id: 4,
            title: "Operation Iron Fang",
            subtitle: "Mineral Ridge Chokepoint // Smelter Sabotage & Ambush",
            transcript: "Plymouth High Council Direct Action Dispatch: Eden technocrats have entrenched a heavily armed mineral extraction and smelting complex directly bordering our southern badland canyon passes, choking off Plymouth's mineral lifeline.\n\nAssemble an armored ambush strike squadron of at least 4 combat tanks equipped with railguns or lasers, erect a fortified Guard Post Turret to control the canyon choke point, hijack and stockpile at least 3,500t Common Ore and 800t Rare Ore, and defend the Plymouth Command Center against Eden's automated retaliatory counter-assault.",
            primary_directives: &[
                "Assemble an armored ambush strike squad of >= 4 armed tanks",
                "Fortify the ambush choke point with at least 1 Guard Post Turret",
                "Stockpile at least 3,500t Common Ore and 800t Rare Ore",
                "Protect Plymouth Command Center from Eden counter-offensive strikes",
            ],
            secondary_directives: &[
                "Maintain an operational Vehicle Factory for armored chassis production",
                "Sustain positive power generation surplus across the tactical grid",
            ],
            hazard_summary: "Fortified laser turrets; hazardous canyon bottlenecks.",
        },
        // Scenario 5: Eden 3 (Starship Evacuation)
        BriefingScenarioDict {
            scenario_id: 5,
            title: "Starship Evacuation",
            subtitle: "Mount Vulcan Caldera // Escape From Mount Vulcan",
            transcript: "Eden High Command Emergency Broadcast: Mount Vulcan's caldera is undergoing an apocalyptic supervolcanic eruption sequence. Molten fissures are severing the southern approaches, while desperate Plymouth raider divisions converge on our launch coordinates to seize our evacuation craft.\n\nCommission the Starship launch facility, fabricate all four primary evacuation starship modules (Command Module, Cryo Stasis Bay, Habitat Ring, and Ion Drive), transport at least 150 colonists via Evacuation Transports into the stasis berths, and hold the perimeter until orbital escape velocity is achieved.",
            primary_directives: &[
                "Assemble Starship modules (Command, Cryo Stasis, Habitat, Ion Drive)",
                "Evacuate at least 150 colonists via Evacuation Transports",
                "Defend colony against Plymouth raiders and magma fissure spread",
                "Protect the Eden Command Center from destruction",
            ],
            secondary_directives: &[
                "Maintain colonial morale above 85% during evacuation countdown",
                "Stockpile 3,000t Common Ore and 1,000t Rare Ore for fuel reserves",
            ],
            hazard_summary: "Cataclysmic volcanic eruption; fast-advancing magma rivers.",
        },
        // Scenario 6: Plymouth 3 (Operation Phoenix: Obsidian Crags)
        BriefingScenarioDict {
            scenario_id: 6,
            title: "Operation Phoenix: Obsidian Crags",
            subtitle: "Obsidian Crags // Citadel Assault & Terraformer Defense",
            transcript: "Plymouth Council of Free Settlers Dispatch: The terraforming atmospheric processor in the Obsidian Crags represents the final hope for breathable atmosphere on New Terra. However, Eden's fortified Obsidian Citadel has locked down the sector, and microscopic nanite blight contagion is propagating towards our processor vents.\n\nDeploy heavy assault tanks, safeguard the Atmospheric Terraformer from nanite contagion until 100% saturation is achieved, and breach the fortified Eden Citadel to neutralize their defensive guard posts and Tiger railgun divisions.",
            primary_directives: &[
                "Protect Atmospheric Terraformer from spreading nanite blight contagion",
                "Breaching assault against the fortified Eden Obsidian Citadel",
                "Achieve 100% Atmospheric Terraformer saturation",
                "Protect Plymouth Command Center from destruction",
            ],
            secondary_directives: &[
                "Eliminate all hostile Eden railgun Guard Post turrets in the crags",
            ],
            hazard_summary: "Nanite microbial blight contagion; heavy railgun fortresses.",
        },
    ],
};

// --- Portuguese (Brazil) Dictionary ---
static TRANSLATION_PT: Translation = Translation {
    faction: "Facção:",
    sol: "Sol:",
    sun: "Sol:",
    pop: "Pop:",
    power: "Energia:",
    common_ore: "Minério Comum:",
    rare_ore: "Minério Raro:",
    food: "Alimento:",
    build_base: "🏗️ Construir Base",
    units: "🚜 Unidades",
    research: "🔬 Pesquisar",
    save: "💾 Salvar",
    load: "📂 Carregar",
    radar: "Radar Tático",
    briefing: "📋 Diretiva",
    music: "🎵 Música",

    language: Language::Portuguese,
    main_menu: MainMenuDict {
        new_campaign: "Nova Campanha",
        campaign_missions: "🚀 MISSÕES DE CAMPANHA (EDEN & PLYMOUTH)",
        skirmish: "Confronto",
        skirmish_battle: "⚔️ BATALHA PERSONALIZADA (1-5 BOTS IA)",
        multiplayer: "Multijogador",
        multiplayer_lobby: "📡 MULTIJOGADOR P2P (LOBBY WEBRTC)",
        load_game: "Carregar Jogo",
        load_archive: "📂 CARREGAR ARQUIVO DE MISSÃO (.OP2SAVE)",
        options: "Opções",
        options_audio_velocity: "⚙️ OPÇÕES / ÁUDIO & VELOCIDADE",
        resume: "Continuar",
        resume_operation: "▶️ RETOMAR OPERAÇÃO ATUAL",
        quit: "Sair",
        back_to_menu: "◀ VOLTAR AO MENU",
        back: "◀ VOLTAR",
        prev_op: "◀ OP ANTERIOR",
        next_op: "PRÓXIMA OP ▶",
        begin_operation: "🚀 INICIAR OPERAÇÃO",
        apply_and_return: "APLICAR & RETORNAR",
        title_tagline: "SIERRA ON-LINE & MOTOR DYNAMICS // EXPEDIÇÃO TETHYS",
        subtitle: "—  D E S T I N O   D I V I D I D O  —",
        select_theater: "SELECIONAR TEATRO DE OPERAÇÕES DE CAMPANHA",
        select_theater_sub: "Escolha lealdade entre a científica Fundação Eden e a Irmandade de Plymouth.",
        tutorial_button: "🎓 CAMPO DE PROVAS PARA CADETES (SIMULADOR DE TREINAMENTO)",
        operational_directives_checklist: "LISTA DE DIRETIVAS OPERACIONAIS:",
        tactical_operation_briefing: "INSTRUÇÕES DA OPERAÇÃO TÁTICA",
        directive_prefix: "DIRETIVA",
        secondary_prefix: "SECUNDÁRIA",
    },
    skirmish: SkirmishDict {
        title: "LOBBY DE CONFRONTO TÁTICO // ENGAJAMENTO DE SETOR",
        map_selection: "MAPA",
        player_faction: "FACÇÃO",
        bot_count: "Oponentes",
        personality_aggressive_rusher: "Atacante Agressivo",
        personality_defensive_fortifier: "Fortificador Defensivo",
        personality_economic_tech_rusher: "Desenvolvedor Tecnológico",
        difficulty_easy: "Fácil",
        difficulty_medium: "Médio",
        difficulty_hard: "Difícil",
        abundance: "RECURSOS",
        abundance_sparse: "Escassa (1.000 C / 200 R)",
        abundance_standard: "Padrão (2.500 C / 600 R)",
        abundance_abundant: "Abundante (5.000 C / 1.500 R)",
        abundance_infinite: "Minério Infinito (50.000 C / 20.000 R)",
        disaster: "DESASTRES",
        disaster_disabled: "Desativado (Calmo)",
        disaster_low: "Baixa (Tremores Raros)",
        disaster_standard: "Padrão (Perigos Normais)",
        disaster_catastrophic: "Catastrófica (Extremo)",
        launch: "⚔️ INICIAR CONFRONTO",
        theater_params: "PARÂMETROS DO TEATRO & SIMULAÇÃO:",
        ai_slots_header: "SLOTS DE IA COMBATENTE (1 A 5 OPONENTES):",
        bot_active: "[X] ATIVO",
        bot_off: "[ ] DESL",
        status_summary: "Status",
    },
    multiplayer: MultiplayerDict {
        title: "LOBBY P2P TÁTICO WEBRTC (SEM SERVIDOR)",
        host: "CRIAR SALA (OFERTA)",
        guest: "ENTRAR COMO CONVIDADO (RESPOSTA)",
        copy_offer: "📋 COPIAR OFERTA PARA TRANSFERÊNCIA",
        paste_answer: "📥 COLAR RESPOSTA DO CONVIDADO",
        paste_offer: "📥 COLAR OFERTA DO ANFITRIÃO",
        generate_answer: "ACEITAR OFERTA & GERAR RESPOSTA",
        copy_answer: "📋 COPIAR RESPOSTA (ENVIAR AO ANFITRIÃO)",
        lock_connection: "TRAVAR CONEXÃO",
        peer_connected: "Par Conectado",
        peer_disconnected: "Desconectado",
        peer_connecting: "Conectando...",
        launch_match: "🚀 INICIAR PARTIDA",
        ready_toggle: "PRONTO: SIM [ALTERAR]",
        not_ready_toggle: "PRONTO: NÃO [ALTERAR]",
        offer_ready_header: "OFERTA SDP BASE64 GERADA (PRONTA PARA COMPARTILHAR):",
        awaiting_offer: "Aguardando geração de oferta WebRTC...",
        status_banner_label: "STATUS DA CONEXÃO",
    },
    hud: HudDict {
        common_ore: "Minério Comum",
        rare_ore: "Minério Raro",
        power: "Energia",
        food: "Alimento",
        population: "População",
        sol: "Sol",
        sun: "Luz Solar",
        starship: "Espaçonave",
        tactical_radar: "Radar Tático",
        radar_active: "ATIVO",
        grid_optimal: "REDE OTIMIZADA",
        grid_brownout: "ALERTA DE QUEDA",
        grid_blackout: "APAGÃO CRÍTICO",
        food_surplus: "Superávit",
        food_adequate: "Adequado",
        food_rationing: "Racionamento",
        food_starvation: "FOME CRÍTICA",
        morale_excellent: "Excelente",
        morale_good: "Bom",
        morale_fair: "Regular",
        morale_poor: "Ruim",
        morale_rioting: "Rebelião",
        factory_title: "PRODUÇÃO DA FÁBRICA & KITS CONVEC",
        factory_hull: "Chassi",
        factory_cost: "Custo",
        factory_rally: "Ponto de Encontro",
        factory_queue: "Fila de Fabricação:",
        factory_unassigned_rally: "Ponto de Encontro: Não Atribuído (Baia)",
        factory_footer_hint: "Pressione 'F' para alternar painel | Botão direito para definir Encontro",
        research_title: "MATRIZ DE PESQUISA CIENTÍFICA COLONIAL",
        research_active: "Projeto Ativo",
        research_none: "Nenhum Projeto de Pesquisa Ativo",
        research_footer_hint: "Pressione 'R' para alternar matriz de pesquisa",
    },
    debriefing: DebriefingDict {
        rank_supreme_savior_title: "Salvador Supremo da Colônia",
        rank_supreme_savior_badge: "🎖️ Salvador Supremo da Colônia",
        rank_supreme_savior_desc: "Liderança impecável: Mínimas baixas, produção industrial superior e domínio estratégico absoluto.",

        rank_distinguished_pioneer_title: "Pioneiro Ilustre",
        rank_distinguished_pioneer_badge: "🌟 Pioneiro Ilustre",
        rank_distinguished_pioneer_desc: "Comando louvável: Objetivos coloniais alcançados com bravura sob condições hostis.",

        rank_tethys_survivor_title: "Sobrevivente de Tethys",
        rank_tethys_survivor_badge: "🛡️ Sobrevivente de Tethys",
        rank_tethys_survivor_desc: "Vitória de Pirro: A colônia sobreviveu através de severa adversidade e pesado consumo de recursos.",

        rank_colony_failure_title: "Colapso Colonial",
        rank_colony_failure_badge: "☠️ Colapso Colonial",
        rank_colony_failure_desc: "Colapso catastrófico: Posto avançado perdido, baixas civis críticas ou diretivas primárias fracassadas.",

        victory_title: "MISSÃO CUMPRIDA: VITÓRIA",
        defeat_title: "DERROTA COLONIAL: MISSÃO ABORTADA",
        victory_subtitle_template: "Parabéns, Comandante! As diretivas primárias foram executadas com sucesso.",
        defeat_subtitle_template: "Falha crítica. O Centro de Comando colonial foi completamente destruído.",
        colony_duration: "Duração da Colônia",
        evacuation_readiness: "Prontidão de Evacuação",
        survivors_morale: "Sobreviventes / Moral",
        refined_reserves: "Reservas Refinadas",
        score_label: "Pontuação Composta da Colônia",
        directives_checklist: "Verificação das Diretivas",
        starship_escaped: "🚀 Em Órbita com Sucesso!",
        starship_assembled_template: "{:.0}% Montada",
    },
    units_buildings: UnitBuildingDict {
        command_center: "Centro de Comando",
        solar_array: "Matriz de Energia Solar",
        tokamak_reactor: "Reator de Fusão Tokamak",
        geothermal_plant: "Usina Geotérmica",
        common_ore_smelter: "Fundição de Minério Comum",
        rare_ore_smelter: "Fundição de Minério Raro",
        dirt_lab: "Laboratório D.I.R.T.",
        standard_lab: "Laboratório Padrão",
        residence: "Residência Colonial",
        agrodome: "Hidroponia Bio-Domo",
        vehicle_factory: "Fábrica de Veículos",
        repair_facility: "Oficina de Reparos",
        guard_post: "Torre de Guarda",
        spaceport: "Plataforma de Lançamento",
        terraformer: "Terraformador Atmosférico",

        lynx: "Batedor Lynx",
        panther: "Tanque de Combate Panther",
        tiger: "Tanque Pesado Tiger",
        cargo_truck: "Caminhão de Carga",
        convec: "Transporte ConVec",
        robo_miner: "Robô-Minerador",
        robo_surveyor: "Robô-Topógrafo",
        earthworker: "Terraplanador",
        evacuation_transport: "Transporte de Evacuação",
    },
    options: OptionsDict {
        title: "OPÇÕES DO SISTEMA & MATRIZ DE ÁUDIO",
        master_volume: "Volume Principal de Saída de Áudio:",
        sim_velocity: "Escala de Velocidade da Simulação:",
        crt_scanlines: "Filtro Retrô de Linhas CRT:",
        active_language: "Idioma Ativo / Localização:",
        enabled: "ATIVADO",
        disabled: "DESATIVADO",
    },
    pause_menu: PauseMenuDict {
        title: "SIMULAÇÃO TÁTICA PAUSADA",
        resume: "▶️ RETOMAR MISSÃO (ESC)",
        quicksave: "💾 SALVAMENTO RÁPIDO (F5)",
        quickload: "📂 CARREGAMENTO RÁPIDO (F9)",
        save_load_manager: "🗄️ GERENCIADOR DE SAVES",
        options: "⚙️ OPÇÕES DE ÁUDIO & VELOCIDADE",
        restart: "🔄 REINICIAR MISSÃO",
        surrender: "💀 RENDER-SE / VOLTAR AO MENU",
    },
    scenarios: [
        // Scenario 0: Eden 1
        BriefingScenarioDict {
            scenario_id: 0,
            title: "Operação Novo Amanhecer",
            subtitle: "Setor Eden Prime // Colonização Planetária",
            transcript: "Colonizadores da Fundação Eden: Após a falha catastrófica da espaçonave Conestoga, nosso local de pouso em Nova Terra deve ser protegido imediatamente. Tremores vulcânicos indicam fissuras de magma crescentes nas proximidades, e invasores renegados de Plymouth estabeleceram postos perimetrais agressivos.\n\nProtejam o suporte de vida colonial, automatizem o transporte de minério, mobilizem blindados com canhões de trilho e avancem a pesquisa científica para garantir a sobrevivência de nossa espécie.",
            primary_directives: &[
                "Armazenar 3.000t de Minério Comum e 600t de Minério Raro",
                "Construir e mobilizar pelo menos 2 tanques de combate armados",
                "Concluir pesquisa científica em qualquer disciplina primária",
                "Proteger o Centro de Comando colonial contra destruição inimiga",
            ],
            secondary_directives: &[
                "Estabelecer uma população de 120 colonos com moral positivo",
            ],
            hazard_summary: "Vórtices de poeira locais; tremores sísmicos leves.",
        },
        // Scenario 1: Plymouth 1
        BriefingScenarioDict {
            scenario_id: 1,
            title: "Operação Vanguarda dos Ermos",
            subtitle: "Setor Plymouth Sul // Reivindicação Territorial",
            transcript: "Comandantes de Plymouth: Rejeitando a tecnocracia autoritária de Eden, nossos colonos construíram um lar nas trincheiras acidentadas dos ermos. Patrulhas de reconhecimento de Eden estão avançando sobre nossas fontes geotérmicas.\n\nConstruam uma Fábrica de Veículos, estabeleçam comboios de mineração automatizados, reúnam uma força de ataque de 3 tanques de combate e eliminem os invasores de Eden para garantir nosso território soberano.",
            primary_directives: &[
                "Comissionar uma Fábrica de Veículos para fabricar chassis blindados",
                "Reunir um esquadrão de ataque com pelo menos 3 tanques de combate",
                "Extrair 2.500t de Minério Comum para proteger o desfiladeiro",
                "Defender o Centro de Comando de Plymouth contra ataques de Eden",
            ],
            secondary_directives: &[
                "Manter excedente contínuo de energia durante os ciclos noturnos dos ermos",
            ],
            hazard_summary: "Alto risco de tremores vulcânicos; respiradouros de gás geotérmico ativos.",
        },
        // Scenario 2: Tutorial
        BriefingScenarioDict {
            scenario_id: 2,
            title: "Campo de Provas para Cadetes",
            subtitle: "Academia Colonial // Simulador de Operações Táticas",
            transcript: "Cadete: Bem-vindo ao Simulador de Operações Coloniais de Nova Terra. Neste teste, você dominará os fundamentos do gerenciamento de uma colônia planetária.\n\nVerifique se a geração de energia supera o consumo, instale tubos pressurizados de transporte entre as instalações e supervisione o transporte automatizado de minério até a fundição.",
            primary_directives: &[
                "Estabelecer superávit de energia positivo (Produção > Consumo)",
                "Conectar estruturas com pelo menos 15 tubos de transporte pressurizados",
                "Transportar a primeira carga de Minério Comum para a Fundição",
                "Manter o moral dos colonos acima de 90%",
            ],
            secondary_directives: &[
                "Comissionar um Robô-Minerador automatizado em um sinalizador de minério",
            ],
            hazard_summary: "Ambiente de simulador controlado; baixo risco ambiental.",
        },
        // Scenario 3: Eden 2
        BriefingScenarioDict {
            scenario_id: 3,
            title: "Operação Bastião da Caldeira",
            subtitle: "Encostas do Monte Vulcan // Contenção de Fissuras Magmáticas",
            transcript: "Alerta do Comando Eden: Sensores térmicos subterrâneos registram deslocamento catastrófico de magma nas falhas geológicas ao norte. Fissuras incandescentes estão rompendo o permafrost e ameaçam os domos coloniais.\n\nEstabilizem a energia da base acima de 400 kW com matrizes de alta potência ou usinas geotérmicas, construam um perímetro fortificado com pelo menos 2 Torres de Guarda vigiando o corredor de lava, estoquem 2.500t de Minério Comum e 500t de Minério Raro para barreiras térmicas emergenciais e garantam a integridade do Centro de Comando contra fendas e bombardeios hostis.",
            primary_directives: &[
                "Estabelecer rede geotérmica de contenção (Gerar >= 400 kW de energia)",
                "Construir pelo menos 2 Torres de Guarda vigiando a borda da fissura",
                "Armazenar 2.500t de Minério Comum e 500t de Minério Raro para reforço",
                "Proteger o Centro de Comando de Eden contra destruição sísmica ou inimiga",
            ],
            secondary_directives: &[
                "Manter o moral da população colonial em 80% ou mais",
                "Concluir pesquisa científica em qualquer disciplina avançada",
            ],
            hazard_summary: "Fissuras de lava vulcânica em expansão; fluxos de magma ativos.",
        },
        // Scenario 4: Plymouth 2
        BriefingScenarioDict {
            scenario_id: 4,
            title: "Operação Presa de Ferro",
            subtitle: "Desfiladeiro da Crista Mineral // Emboscada e Sabotagem da Fundição",
            transcript: "Despacho do Alto Conselho de Plymouth: Tecnocratas de Eden fortificaram um complexo de extração mineral e fundição na entrada dos nossos desfiladeiros ao sul, estrangulando o suprimento vital de Plymouth.\n\nReúnam um esquadrão de emboscada com pelo menos 4 tanques de combate equipados com canhões de trilho ou lasers, ergam uma Torre de Guarda fortificada no gargalo do cânion, interceptem e estoquem 3.500t de Minério Comum e 800t de Minério Raro, e defendam o Centro de Comando contra o contra-ataque de Eden.",
            primary_directives: &[
                "Reunir um esquadrão blindado de emboscada de >= 4 tanques armados",
                "Fortificar o ponto de afunilamento com pelo menos 1 Torre de Guarda",
                "Armazenar pelo menos 3.500t de Minério Comum e 800t de Minério Raro",
                "Proteger o Centro de Comando de Plymouth contra contra-ataques de Eden",
            ],
            secondary_directives: &[
                "Manter uma Fábrica de Veículos operacional para blindados",
                "Sustentar excedente de energia positivo em toda a rede tática",
            ],
            hazard_summary: "Torres de laser fortificadas; gargalos perigosos no cânion.",
        },
        // Scenario 5: Eden 3
        BriefingScenarioDict {
            scenario_id: 5,
            title: "Evacuação da Espaçonave",
            subtitle: "Caldeira do Monte Vulcan // Fuga do Monte Vulcan",
            transcript: "Transmissão de Emergência do Alto Comando Eden: A caldeira do Monte Vulcan entrou em erupção supervulcânica apocalíptica. Fissuras de lava estão destruindo as rotas ao sul, enquanto divisões desesperadas de incursores de Plymouth convergem para capturar nossa nave de evacuação.\n\nComissionem a plataforma de lançamento, fabriquem todos os quatro módulos da espaçonave (Módulo de Comando, Baía Criogênica, Anel Habitacional e Motor Iônico), transportem pelo menos 150 colonos em Transportes de Evacuação para as cápsulas de êxtase e defendam o perímetro até atingirmos a velocidade de escape orbital.",
            primary_directives: &[
                "Montar módulos da espaçonave (Comando, Criogenia, Habitat, Motor Iônico)",
                "Evacuar pelo menos 150 colonos através de Transportes de Evacuação",
                "Defender a colônia contra invasores de Plymouth e avanço do magma",
                "Proteger o Centro de Comando de Eden contra destruição",
            ],
            secondary_directives: &[
                "Manter o moral colonial acima de 85% durante a contagem regressiva",
                "Armazenar 3.000t de Minério Comum e 1.000t de Minério Raro para combustível",
            ],
            hazard_summary: "Erupção vulcânica cataclísmica; rios de magma em avanço rápido.",
        },
        // Scenario 6: Plymouth 3
        BriefingScenarioDict {
            scenario_id: 6,
            title: "Operação Fênix: Penhascos de Obsidiana",
            subtitle: "Penhascos de Obsidiana // Assalto à Cidadela e Defesa do Terraformador",
            transcript: "Despacho do Conselho de Colonos Livres de Plymouth: O processador atmosférico nos Penhascos de Obsidiana é a última esperança de ar respirável em Nova Terra. Contudo, a Cidadela de Eden bloqueou o setor, e a praga microscópica de nanites está se propagando em direção às turbinas do processador.\n\nImplantem tanques pesados, protejam o Terraformador Atmosférico contra o contágio de nanites até atingir 100% de saturação e destruam a Cidadela de Eden para neutralizar suas torres de guarda e tanques Tiger com canhões de trilho.",
            primary_directives: &[
                "Proteger o Terraformador Atmosférico contra a contaminação de nanites",
                "Realizar assalto de brecha contra a Cidadela fortificada de Eden",
                "Alcançar 100% de saturação do Terraformador Atmosférico",
                "Proteger o Centro de Comando de Plymouth contra destruição",
            ],
            secondary_directives: &[
                "Eliminar todas as Torres de Guarda com canhões de trilho de Eden nos penhascos",
            ],
            hazard_summary: "Praga microbiana de nanites; fortalezas com canhões de trilho pesados.",
        },
    ],
};

// --- Spanish Dictionary ---
static TRANSLATION_ES: Translation = Translation {
    faction: "Facción:",
    sol: "Sol:",
    sun: "Sol:",
    pop: "Población:",
    power: "Energía:",
    common_ore: "Mineral Común:",
    rare_ore: "Mineral Raro:",
    food: "Comida:",
    build_base: "🏗️ Construir Base",
    units: "🚜 Unidades",
    research: "🔬 Investigar",
    save: "💾 Guardar",
    load: "📂 Cargar",
    radar: "Radar Táctico",
    briefing: "📋 Directiva",
    music: "🎵 Música",

    language: Language::Spanish,
    main_menu: MainMenuDict {
        new_campaign: "Nueva Campaña",
        campaign_missions: "🚀 MISIONES DE CAMPAÑA (EDEN Y PLYMOUTH)",
        skirmish: "Escaramuza",
        skirmish_battle: "⚔️ BATALLA DE ESCARAMUZA (1-5 BOTS IA)",
        multiplayer: "Multijugador",
        multiplayer_lobby: "📡 MULTIJUGADOR P2P (SALA WEBRTC)",
        load_game: "Cargar Partida",
        load_archive: "📂 CARGAR ARCHIVO DE MISIÓN (.OP2SAVE)",
        options: "Opciones",
        options_audio_velocity: "⚙️ OPCIONES / AUDIO Y VELOCIDAD",
        resume: "Reanudar",
        resume_operation: "▶️ REANUDAR OPERACIÓN ACTUAL",
        quit: "Salir",
        back_to_menu: "◀ VOLVER AL MENÚ",
        back: "◀ VOLVER",
        prev_op: "◀ OP PREVIA",
        next_op: "SIGUIENTE OP ▶",
        begin_operation: "🚀 INICIAR OPERACIÓN",
        apply_and_return: "APLICAR Y VOLVER",
        title_tagline: "SIERRA ON-LINE Y MOTOR DYNAMICS // EXPEDICIÓN TETHYS",
        subtitle: "—  D E S T I N O   D I V I D I D O  —",
        select_theater: "SELECCIONAR TEATRO DE OPERACIONES DE CAMPAÑA",
        select_theater_sub: "Elija lealtad entre la científica Fundación Eden y la Hermandad de Plymouth.",
        tutorial_button: "🎓 CAMPO DE PRUEBAS DE CADETES (SIMULADOR DE ENTRENAMIENTO)",
        operational_directives_checklist: "LISTA DE DIRECTIVAS OPERACIONALES:",
        tactical_operation_briefing: "INSTRUCCIONES DE LA OPERACIÓN TÁCTICA",
        directive_prefix: "DIRECTIVA",
        secondary_prefix: "SECUNDARIA",
    },
    skirmish: SkirmishDict {
        title: "SALA DE ESCARAMUZA TÁCTICA // ENFRENTAMIENTO DE SECTOR",
        map_selection: "MAPA",
        player_faction: "FACCIÓN",
        bot_count: "Oponentes",
        personality_aggressive_rusher: "Atacante Agresivo",
        personality_defensive_fortifier: "Fortificador Defensivo",
        personality_economic_tech_rusher: "Desarrollador Tecnológico",
        difficulty_easy: "Fácil",
        difficulty_medium: "Medio",
        difficulty_hard: "Difícil",
        abundance: "RECURSOS",
        abundance_sparse: "Escasa (1.000 C / 200 R)",
        abundance_standard: "Estándar (2.500 C / 600 R)",
        abundance_abundant: "Abundante (5.000 C / 1.500 R)",
        abundance_infinite: "Mineral Infinito (50.000 C / 20.000 R)",
        disaster: "DESASTRES",
        disaster_disabled: "Desactivado (Tranquilo)",
        disaster_low: "Baja (Temblores Ocasionales)",
        disaster_standard: "Estándar (Peligros Normales)",
        disaster_catastrophic: "Catastrófica (Extremo)",
        launch: "⚔️ INICIAR ESCARAMUZA",
        theater_params: "PARÁMETROS DEL TEATRO Y SIMULACIÓN:",
        ai_slots_header: "RANURAS DE IA DE COMBATE (1 A 5 OPONENTES):",
        bot_active: "[X] ACTIVO",
        bot_off: "[ ] DESAC",
        status_summary: "Estado",
    },
    multiplayer: MultiplayerDict {
        title: "SALA P2P TÁCTICA WEBRTC (SIN SERVIDOR)",
        host: "CREAR SALA (OFERTA)",
        guest: "UNIRSE COMO INVITADO (RESPUESTA)",
        copy_offer: "📋 COPIAR OFERTA AL PORTAPAPELES",
        paste_answer: "📥 PEGAR RESPUESTA DEL INVITADO",
        paste_offer: "📥 PEGAR OFERTA DEL ANFITRIÓN",
        generate_answer: "ACEPTAR OFERTA Y GENERAR RESPUESTA",
        copy_answer: "📋 COPIAR RESPUESTA (ENVIAR AL ANFITRIÓN)",
        lock_connection: "BLOQUEAR CONEXIÓN",
        peer_connected: "Par Conectado",
        peer_disconnected: "Desconectado",
        peer_connecting: "Conectando...",
        launch_match: "🚀 INICIAR PARTIDA",
        ready_toggle: "LISTO: SÍ [CAMBIAR]",
        not_ready_toggle: "LISTO: NO [CAMBIAR]",
        offer_ready_header: "OFERTA SDP BASE64 GENERADA (LISTA PARA COMPARTIR):",
        awaiting_offer: "Esperando generación de oferta WebRTC...",
        status_banner_label: "ESTADO DE CONEXIÓN",
    },
    hud: HudDict {
        common_ore: "Mineral Común",
        rare_ore: "Mineral Raro",
        power: "Energía",
        food: "Comida",
        population: "Población",
        sol: "Sol",
        sun: "Luz Solar",
        starship: "Nave Espacial",
        tactical_radar: "Radar Táctico",
        radar_active: "ACTIVO",
        grid_optimal: "RED ÓPTIMA",
        grid_brownout: "ALERTA DE CAÍDA",
        grid_blackout: "APAGÓN CRÍTICO",
        food_surplus: "Superávit",
        food_adequate: "Adecuado",
        food_rationing: "Racionamiento",
        food_starvation: "HAMBRUNA",
        morale_excellent: "Excelente",
        morale_good: "Bueno",
        morale_fair: "Regular",
        morale_poor: "Malo",
        morale_rioting: "Disturbios",
        factory_title: "PRODUCCIÓN DE FÁBRICA Y KITS CONVEC",
        factory_hull: "Chasis",
        factory_cost: "Coste",
        factory_rally: "Punto de Reunión",
        factory_queue: "Cola de Fabricación:",
        factory_unassigned_rally: "Punto de Reunión: Sin Asignar (Bahía)",
        factory_footer_hint: "Pulse 'F' para alternar panel | Clic derecho para punto de Reunión",
        research_title: "MATRIZ DE INVESTIGACIÓN CIENTÍFICA",
        research_active: "Proyecto Activo",
        research_none: "Sin Proyecto de Investigación Activo",
        research_footer_hint: "Pulse 'R' para alternar matriz de investigación",
    },
    debriefing: DebriefingDict {
        rank_supreme_savior_title: "Salvador Supremo de la Colonia",
        rank_supreme_savior_badge: "🎖️ Salvador Supremo de la Colonia",
        rank_supreme_savior_desc: "Liderazgo impecable: Mínimas bajas, producción industrial superior y dominio estratégico absoluto.",

        rank_distinguished_pioneer_title: "Pionero Distinguido",
        rank_distinguished_pioneer_badge: "🌟 Pionero Distinguido",
        rank_distinguished_pioneer_desc: "Mando encomiable: Objetivos coloniales asegurados con firmeza bajo condiciones hostiles.",

        rank_tethys_survivor_title: "Superviviente de Tetis",
        rank_tethys_survivor_badge: "🛡️ Superviviente de Tetis",
        rank_tethys_survivor_desc: "Victoria pírrica: La colonia sobrevivió a duras penas con gran consumo de recursos.",

        rank_colony_failure_title: "Fracaso Colonial",
        rank_colony_failure_badge: "☠️ Fracaso Colonial",
        rank_colony_failure_desc: "Colapso catastrófico: Puesto avanzado perdido, bajas civiles críticas o directivas principales fallidas.",

        victory_title: "MISIÓN CUMPLIDA: VICTORIA",
        defeat_title: "DERROTA COLONIAL: MISIÓN ABORTADA",
        victory_subtitle_template: "¡Enhorabuena, Comandante! Las directivas primarias se han ejecutado con éxito.",
        defeat_subtitle_template: "Fallo crítico. El Centro de Mando colonial ha sido completamente destruido.",
        colony_duration: "Duración de la Colonia",
        evacuation_readiness: "Preparación para Evacuación",
        survivors_morale: "Supervivientes / Moral",
        refined_reserves: "Reservas Refinadas",
        score_label: "Puntuación de la Colonia",
        directives_checklist: "Lista de Directivas",
        starship_escaped: "🚀 ¡En Órbita con Éxito!",
        starship_assembled_template: "{:.0}% Ensamblada",
    },
    units_buildings: UnitBuildingDict {
        command_center: "Centro de Mando",
        solar_array: "Matriz de Energía Solar",
        tokamak_reactor: "Reactor de Fusión Tokamak",
        geothermal_plant: "Planta Geotérmica",
        common_ore_smelter: "Fundición de Mineral Común",
        rare_ore_smelter: "Fundición de Mineral Raro",
        dirt_lab: "Laboratorio D.I.R.T.",
        standard_lab: "Laboratorio Estándar",
        residence: "Residencia Colonial",
        agrodome: "Hidroponía Bio-Domo",
        vehicle_factory: "Fábrica de Vehículos",
        repair_facility: "Taller de Reparación",
        guard_post: "Torreta de Guardia",
        spaceport: "Plataforma de Lanzamiento",
        terraformer: "Terraformador Atmosférico",

        lynx: "Explorador Lynx",
        panther: "Tanque Panther",
        tiger: "Tanque Pesado Tiger",
        cargo_truck: "Camión de Carga",
        convec: "Transporte ConVec",
        robo_miner: "Robo-Minero",
        robo_surveyor: "Robo-Topógrafo",
        earthworker: "Movetierras",
        evacuation_transport: "Transporte de Evacuación",
    },
    options: OptionsDict {
        title: "OPCIONES DEL SISTEMA Y MATRIZ DE AUDIO",
        master_volume: "Volumen Maestro de Salida de Audio:",
        sim_velocity: "Escala de Velocidad de la Simulación:",
        crt_scanlines: "Filtro Retro de Líneas de Escaneo CRT:",
        active_language: "Idioma Activo / Configuración Regional:",
        enabled: "ACTIVADO",
        disabled: "DESACTIVADO",
    },
    pause_menu: PauseMenuDict {
        title: "SIMULACIÓN TÁCTICA PAUSADA",
        resume: "▶️ REANUDAR MISIÓN (ESC)",
        quicksave: "💾 GUARDADO RÁPIDO (F5)",
        quickload: "📂 CARGA RÁPIDA (F9)",
        save_load_manager: "🗄️ GESTOR DE PARTIDAS",
        options: "⚙️ OPCIONES DE AUDIO Y VELOCIDAD",
        restart: "🔄 REINICIAR MISIÓN",
        surrender: "💀 RENDIRSE / SALIR AL MENÚ",
    },
    scenarios: [
        // Scenario 0: Eden 1
        BriefingScenarioDict {
            scenario_id: 0,
            title: "Operación Nuevo Amanecer",
            subtitle: "Sector Eden Prime // Asentamiento Planetario",
            transcript: "Colonizadores de la Fundación Eden: Tras el fallo catastrófico de la nave espacial Conestoga, nuestro lugar de aterrizaje en Nueva Terra debe asegurarse de inmediato. Los temblores volcánicos indican fisuras de magma activas en las cercanías, e invasores de Plymouth han levantado puestos de avanzada hostiles.\n\nAseguren el soporte vital colonial, automaticen el transporte de mineral, desplieguen blindados de combate con cañones de riel y avancen en la investigación científica para asegurar la supervivencia de nuestra especie.",
            primary_directives: &[
                "Almacenar 3.000t de Mineral Común y 600t de Mineral Raro",
                "Construir y desplegar al menos 2 tanques de combate armados",
                "Completar investigación científica en cualquier disciplina principal",
                "Proteger el Centro de Mando colonial de la destrucción hostil",
            ],
            secondary_directives: &[
                "Establecer una población de 120 colonos con moral positiva",
            ],
            hazard_summary: "Torbellinos de polvo locales; temblores sísmicos menores.",
        },
        // Scenario 1: Plymouth 1
        BriefingScenarioDict {
            scenario_id: 1,
            title: "Operación Vanguardia de las Tierras Yermas",
            subtitle: "Sector Plymouth Sur // Reclamación Territorial",
            transcript: "Comandantes de Plymouth: Rechazando la tecnocracia autoritaria de Eden, nuestros colonos forjaron un hogar en las escarpadas trincheras de las tierras yermas. Patrullas de reconocimiento de Eden avanzan sobre nuestros respiraderos geotérmicos.\n\nConstruyan una Fábrica de Vehículos, establezcan convoyes mineros automatizados, reúnan una fuerza de asalto de 3 tanques de combate y eliminen a los invasores de Eden para defender nuestro territorio soberano.",
            primary_directives: &[
                "Encargar una Fábrica de Vehículos para producir chasis blindados",
                "Reunir un escuadrón de ataque de al menos 3 tanques de combate",
                "Extraer 2.500t de Mineral Común para asegurar el cañón perimetral",
                "Defender el Centro de Mando de Plymouth del asalto de Eden",
            ],
            secondary_directives: &[
                "Mantener superávit continuo de energía durante la noche de las tierras yermas",
            ],
            hazard_summary: "Alto riesgo de temblores volcánicos; respiraderos de gas geotérmico activos.",
        },
        // Scenario 2: Tutorial
        BriefingScenarioDict {
            scenario_id: 2,
            title: "Campo de Pruebas de Cadetes",
            subtitle: "Academia Colonial // Simulador de Operaciones Tácticas",
            transcript: "Cadete: Bienvenido al Simulador de Operaciones Coloniales de Nueva Terra. En esta prueba, dominarás los fundamentos de la administración colonial planetaria.\n\nVerifica que la generación de energía supere el consumo, instala tubos presurizados entre instalaciones y supervisa el transporte automatizado de mineral a la fundición.",
            primary_directives: &[
                "Establecer superávit energético positivo (Producción > Consumo)",
                "Conectar estructuras con al menos 15 tubos presurizados",
                "Transportar la primera carga de Mineral Común a la Fundición",
                "Mantener la moral de los colonos por encima del 90%",
            ],
            secondary_directives: &[
                "Poner en servicio un Robo-Minero automatizado en una baliza de mineral",
            ],
            hazard_summary: "Entorno de simulación controlado; peligro ambiental bajo.",
        },
        // Scenario 3: Eden 2
        BriefingScenarioDict {
            scenario_id: 3,
            title: "Operación Bastión de la Caldera",
            subtitle: "Laderas del Monte Vulcan // Contención de Fisuras de Magma",
            transcript: "Alerta de Mando de Eden: Los sensores térmicos subterráneos detectan desplazamientos catastróficos de magma en las fallas norteñas. Las fisuras incandescentes están rompiendo el permafrost, amenazando los domos de supervivencia.\n\nEstabilicen la energía base por encima de 400 kW con paneles solares o conductos geotérmicos, construyan un perímetro de al menos 2 Torretas de Guardia sobre el desfiladero de lava, almacenen 2.500t de Mineral Común y 500t de Mineral Raro para barreras de emergencia y protejan el Centro de Mando.",
            primary_directives: &[
                "Establecer red de contención geotérmica (Generar >= 400 kW de energía)",
                "Construir al menos 2 Torretas de Guardia vigilando el borde de la fisura",
                "Almacenar 2.500t de Mineral Común y 500t de Mineral Raro para refuerzo",
                "Proteger el Centro de Mando de Eden de la destrucción sísmica u hostil",
            ],
            secondary_directives: &[
                "Mantener la moral de la población en un 80% o más",
                "Completar investigación científica en cualquier disciplina avanzada",
            ],
            hazard_summary: "Fisuras volcánicas en propagación; ríos de magma incandescentes.",
        },
        // Scenario 4: Plymouth 2
        BriefingScenarioDict {
            scenario_id: 4,
            title: "Operación Colmillo de Hierro",
            subtitle: "Garganta de Cresta Mineral // Sabotaje de Fundición y Emboscada",
            transcript: "Despacho de Acción Directa de Plymouth: Los tecnócratas de Eden han atrincherado un complejo minero y de fundición fuertemente artillado en las rutas del sur, asfixiando los suministros de Plymouth.\n\nReúnan un escuadrón de emboscada blindado de al menos 4 tanques de combate con láseres o cañones de riel, levanten una Torreta de Guardia para controlar el paso estrecho, almacenen al menos 3.500t de Mineral Común y 800t de Mineral Raro, y defiendan el Centro de Mando.",
            primary_directives: &[
                "Reunir un escuadrón blindado de emboscada de >= 4 tanques de combate",
                "Fortificar el paso estrecho con al menos 1 Torreta de Guardia",
                "Almacenar al menos 3.500t de Mineral Común y 800t de Mineral Raro",
                "Proteger el Centro de Mando de Plymouth de la contraofensiva de Eden",
            ],
            secondary_directives: &[
                "Mantener una Fábrica de Vehículos operativa para producción de chasis",
                "Mantener superávit energético positivo en toda la red táctica",
            ],
            hazard_summary: "Torretas láser fortificadas; cuellos de botella peligrosos en el cañón.",
        },
        // Scenario 5: Eden 3
        BriefingScenarioDict {
            scenario_id: 5,
            title: "Evacuación de la Nave Espacial",
            subtitle: "Caldera del Monte Vulcan // Huida del Monte Vulcan",
            transcript: "Transmisión de Emergencia del Alto Mando de Eden: La caldera del Monte Vulcan está sufriendo una erupción supervolcánica apocalíptica. Las fisuras de lava cortan los accesos al sur, mientras divisiones hostiles de Plymouth se dirigen a nuestras coordenadas para capturar la nave de escape.\n\nActiven la plataforma de lanzamiento, fabriquen los cuatro módulos de la nave espacial (Módulo de Mando, Bahía Criogénica, Anillo de Hábitat y Motor Iónico), trasladen al menos a 150 colonos mediante Transportes de Evacuación a las cápsulas y aseguren el perímetro hasta alcanzar la velocidad de escape orbital.",
            primary_directives: &[
                "Ensamblar módulos de la nave (Mando, Criogenia, Hábitat, Motor Iónico)",
                "Evacuar al menos a 150 colonos mediante Transportes de Evacuación",
                "Defender la colonia contra atacantes de Plymouth y el avance de lava",
                "Proteger el Centro de Mando de Eden contra la destrucción",
            ],
            secondary_directives: &[
                "Mantener la moral colonial por encima del 85% durante la cuenta atrás",
                "Almacenar 3.000t de Mineral Común y 1.000t de Mineral Raro para combustible",
            ],
            hazard_summary: "Erupción volcánica cataclísmica; ríos de magma de rápido avance.",
        },
        // Scenario 6: Plymouth 3
        BriefingScenarioDict {
            scenario_id: 6,
            title: "Operación Fénix: Riscos de Obsidiana",
            subtitle: "Riscos de Obsidiana // Asalto a la Ciudadela y Defensa del Terraformador",
            transcript: "Despacho del Consejo de Colonos Libres de Plymouth: El procesador atmosférico en los Riscos de Obsidiana es la última esperanza de una atmósfera respirable en Nueva Terra. Sin embargo, la Ciudadela fortificada de Eden ha bloqueado el sector y una plaga de nanobots avanza hacia los conductos.\n\nDesplieguen tanques pesados de asalto, protejan el Terraformador Atmosférico del contagio de nanobots hasta alcanzar el 100% de saturación y penetren en la Ciudadela de Eden para anular sus defensas y divisiones de tanques Tiger.",
            primary_directives: &[
                "Proteger el Terraformador Atmosférico de la plaga de nanobots",
                "Asalto de ruptura contra la Ciudadela fortificada de Eden",
                "Alcanzar el 100% de saturación en el Terraformador Atmosférico",
                "Proteger el Centro de Mando de Plymouth de la destrucción",
            ],
            secondary_directives: &[
                "Eliminar todas las Torretas de Guardia de cañones de riel en los riscos",
            ],
            hazard_summary: "Contagio biológico de nanobots; fortalezas con cañones de riel pesados.",
        },
    ],
};

// --- German Dictionary ---
static TRANSLATION_DE: Translation = Translation {
    faction: "Fraktion:",
    sol: "Sol:",
    sun: "Sonne:",
    pop: "Bevölkerung:",
    power: "Energie:",
    common_ore: "Erz:",
    rare_ore: "Seltenerz:",
    food: "Nahrung:",
    build_base: "🏗️ Basis Bauen",
    units: "🚜 Einheiten",
    research: "🔬 Forschung",
    save: "💾 Speichern",
    load: "📂 Laden",
    radar: "Taktisches Radar",
    briefing: "📋 Lagebericht",
    music: "🎵 Musik",

    language: Language::German,
    main_menu: MainMenuDict {
        new_campaign: "Neue Kampagne",
        campaign_missions: "🚀 KAMPAGNEN-MISSIONEN (EDEN & PLYMOUTH)",
        skirmish: "Gefecht",
        skirmish_battle: "⚔️ GEFECHTSCHLACHT (1-5 KI-BOTS)",
        multiplayer: "Mehrspieler",
        multiplayer_lobby: "📡 MEHRSPIELER P2P (WEBRTC-LOBBY)",
        load_game: "Spiel Laden",
        load_archive: "📂 MISSIONSARCHIV LADEN (.OP2SAVE)",
        options: "Optionen",
        options_audio_velocity: "⚙️ OPTIONEN / AUDIO & GESCHWINDIGKEIT",
        resume: "Fortsetzen",
        resume_operation: "▶️ AKTUELLE MISSION FORTSETZEN",
        quit: "Beenden",
        back_to_menu: "◀ ZURÜCK ZUM MENÜ",
        back: "◀ ZURÜCK",
        prev_op: "◀ VORHERIGE OP",
        next_op: "NÄCHSTE OP ▶",
        begin_operation: "🚀 OPERATION STARTEN",
        apply_and_return: "ANWENDEN & ZURÜCK",
        title_tagline: "SIERRA ON-LINE & DYNAMICS ENGINE // TETHYS-EXPEDITION",
        subtitle: "—  G E T E I L T E S   S C H I C K S A L  —",
        select_theater: "KAMPAGNEN-EINSATZGEBIET WÄHLEN",
        select_theater_sub: "Wählen Sie Ihre Loyalität zwischen der wissenschaftlichen Eden-Stiftung und der Plymouth-Bruderschaft.",
        tutorial_button: "🎓 KADETTEN-ÜBUNGSGELÄNDE (AKADEMIE-TRAININGSSIMULATOR)",
        operational_directives_checklist: "CHECKLISTE DER EINSATZDIREKTIVEN:",
        tactical_operation_briefing: "TAKTIK-EINSATZBESPRECHUNG",
        directive_prefix: "DIREKTIVE",
        secondary_prefix: "SEKUNDÄR",
    },
    skirmish: SkirmishDict {
        title: "TAKTISCHE GEFECHTS-LOBBY // SEKTOR-EINSATZ",
        map_selection: "KARTE",
        player_faction: "FRAKTION",
        bot_count: "Gegner",
        personality_aggressive_rusher: "Aggressiver Rusher",
        personality_defensive_fortifier: "Defensiver Festungsbauer",
        personality_economic_tech_rusher: "Wirtschafts- und Tech-Rusher",
        difficulty_easy: "Einfach",
        difficulty_medium: "Mittel",
        difficulty_hard: "Schwer",
        abundance: "RESSOURCEN",
        abundance_sparse: "Karg (1.000 Erz / 200 Selten)",
        abundance_standard: "Standard (2.500 Erz / 600 Selten)",
        abundance_abundant: "Reichlich (5.000 Erz / 1.500 Selten)",
        abundance_infinite: "Unendlich Erz (50.000 Erz / 20.000 Selten)",
        disaster: "KATASTROPHEN",
        disaster_disabled: "Deaktiviert (Ruhig)",
        disaster_low: "Niedrig (Gelegentliche Beben)",
        disaster_standard: "Standard (Normale Gefahren)",
        disaster_catastrophic: "Katastrophal (Extrem)",
        launch: "⚔️ GEFECHT STARTEN",
        theater_params: "EINSATZGEBIET- & SIMULATIONSPARAMETER:",
        ai_slots_header: "KAMPF-KI-SLOTS (1 BIS 5 GEGNER):",
        bot_active: "[X] AKTIV",
        bot_off: "[ ] AUS",
        status_summary: "Status",
    },
    multiplayer: MultiplayerDict {
        title: "TAKTISCHE WEBRTC-PEER-TO-PEER-LOBBY (SERVERLOS)",
        host: "SPIEL ERSTELLEN (ANGEBOT)",
        guest: "ALS GAST BEITRETEN (ANTWORT)",
        copy_offer: "📋 ANGEBOT IN ZWISCHENABLAGE KOPIEREN",
        paste_answer: "📥 GAST-ANTWORT EINFÜGEN",
        paste_offer: "📥 HOST-ANGEBOT EINFÜGEN",
        generate_answer: "ANGEBOT ANNEHMEN & ANTWORT GENERIEREN",
        copy_answer: "📋 ANTWORT KOPIEREN (AN HOST SENDEN)",
        lock_connection: "VERBINDUNG FESTLEGEN",
        peer_connected: "Gegenstelle Verbunden",
        peer_disconnected: "Getrennt",
        peer_connecting: "Verbindungsaufbau...",
        launch_match: "🚀 MATCH STARTEN",
        ready_toggle: "BEREIT: JA [UMSCHALTEN]",
        not_ready_toggle: "BEREIT: NEIN [UMSCHALTEN]",
        offer_ready_header: "GENERIERTE BASE64-SDP-OFFERTE (BEREIT ZUM TEILEN):",
        awaiting_offer: "Warten auf WebRTC-Angebotserstellung...",
        status_banner_label: "VERBINDUNGSSTATUS",
    },
    hud: HudDict {
        common_ore: "Erz",
        rare_ore: "Seltenerz",
        power: "Energie",
        food: "Nahrung",
        population: "Bevölkerung",
        sol: "Sol",
        sun: "Sonnenlicht",
        starship: "Raumschiff",
        tactical_radar: "Taktisches Radar",
        radar_active: "AKTIV",
        grid_optimal: "NETZ OPTIMAL",
        grid_brownout: "TEILAUSFALL-WARNUNG",
        grid_blackout: "TOTALAUSFALL KRITISCH",
        food_surplus: "Überschuss",
        food_adequate: "Ausreichend",
        food_rationing: "Rationierung",
        food_starvation: "HUNGERNOT",
        morale_excellent: "Ausgezeichnet",
        morale_good: "Gut",
        morale_fair: "Mäßig",
        morale_poor: "Schlecht",
        morale_rioting: "Aufruhr",
        factory_title: "FABRIKPRODUKTION & CONVEC-BAUSÄTZE",
        factory_hull: "Chassis",
        factory_cost: "Kosten",
        factory_rally: "Sammelpunkt",
        factory_queue: "Fertigungswarteschlange:",
        factory_unassigned_rally: "Sammelpunkt: Nicht zugewiesen (Fabrikhalle)",
        factory_footer_hint: "Drücken Sie 'F' für Panel | Rechtsklick setzt Sammelpunkt",
        research_title: "KOLONIALE FORSCHUNGSMATRIX",
        research_active: "Aktives Projekt",
        research_none: "Kein aktives Forschungsprojekt",
        research_footer_hint: "Drücken Sie 'R' für Forschungsmatrix",
    },
    debriefing: DebriefingDict {
        rank_supreme_savior_title: "Oberster Kolonieretter",
        rank_supreme_savior_badge: "🎖️ Oberster Kolonieretter",
        rank_supreme_savior_desc: "Tadellose Führung: Minimale Verluste, überlegene Industrieproduktion und vollständige strategische Dominanz.",

        rank_distinguished_pioneer_title: "Ausgezeichneter Pionier",
        rank_distinguished_pioneer_badge: "🌟 Ausgezeichneter Pionier",
        rank_distinguished_pioneer_desc: "Lobenswertes Kommando: Koloniale Ziele trotz gefährlicher Umweltbedingungen mit Entschlossenheit gesichert.",

        rank_tethys_survivor_title: "Tethys-Überlebender",
        rank_tethys_survivor_badge: "🛡️ Tethys-Überlebender",
        rank_tethys_survivor_desc: "Pyrrhussieg: Die Kolonie überlebte schwere Rückschläge unter extrem hohem Ressourcenaufwand.",

        rank_colony_failure_title: "Koloniekollaps",
        rank_colony_failure_badge: "☠️ Koloniekollaps",
        rank_colony_failure_desc: "Katastrophales Scheitern: Außenposten verloren, zivile Verluste kritisch oder Primärdirektiven verfehlt.",

        victory_title: "MISSION ERFÜLLT: SIEG",
        defeat_title: "KOLONIALE NIEDERLAGE: MISSION ABGEBROCHEN",
        victory_subtitle_template: "Glückwunsch, Kommandeur! Die Primärdirektiven wurden erfolgreich ausgeführt.",
        defeat_subtitle_template: "Kritischer Fehler. Die Kommandozentrale der Kolonie wurde vollständig vernichtet.",
        colony_duration: "Koloniedauer",
        evacuation_readiness: "Evakuierungsbereitschaft",
        survivors_morale: "Überlebende / Moral",
        refined_reserves: "Raffinierte Reserven",
        score_label: "Kolonie-Gesamtpunktzahl",
        directives_checklist: "Direktiven-Erfüllungsliste",
        starship_escaped: "🚀 Erfolgreich in den Orbit entkommen!",
        starship_assembled_template: "{:.0}% Montiert",
    },
    units_buildings: UnitBuildingDict {
        command_center: "Kommandozentrale",
        solar_array: "Solaranlage",
        tokamak_reactor: "Tokamak-Fusionsreaktor",
        geothermal_plant: "Geothermiekraftwerk",
        common_ore_smelter: "Erzschmelze",
        rare_ore_smelter: "Seltenerzschmelze",
        dirt_lab: "D.I.R.T.-Labor",
        standard_lab: "Standardforschungslabor",
        residence: "Koloniale Wohnunterkunft",
        agrodome: "Biodom-Hydrokultur",
        vehicle_factory: "Fahrzeugfabrik",
        repair_facility: "Reparaturwerkstatt",
        guard_post: "Wachturm-Geschütz",
        spaceport: "Raumschiff-Startrampe",
        terraformer: "Atmosphären-Terraformer",

        lynx: "Lynx-Späher",
        panther: "Panther-Kampfpanzer",
        tiger: "Tiger-Schwerpanzer",
        cargo_truck: "Frachttransporter",
        convec: "ConVec-Transporter",
        robo_miner: "Robo-Bergbaufahrzeug",
        robo_surveyor: "Robo-Vermesser",
        earthworker: "Erdarbeiter",
        evacuation_transport: "Evakuierungstransporter",
    },
    options: OptionsDict {
        title: "SYSTEMOPTIONEN & AUDIOMATRIX",
        master_volume: "Hauptlautstärke der Audioausgabe:",
        sim_velocity: "Skalierung der Simulationsgeschwindigkeit:",
        crt_scanlines: "Retro-CRT-Abtastzeilenüberlagerung:",
        active_language: "Aktive Sprache / Gebietsschema:",
        enabled: "AKTIVIERT",
        disabled: "DEAKTIVIERT",
    },
    pause_menu: PauseMenuDict {
        title: "TAKTIKSIMULATION PAUSIERT",
        resume: "▶️ MISSION FORTSETZEN (ESC)",
        quicksave: "💾 SCHNELLSPEICHERN (F5)",
        quickload: "📂 SCHNELLLADEN (F9)",
        save_load_manager: "🗄️ SPEICHER-/LADEVERWALTUNG",
        options: "⚙️ AUDIO- & GESCHWINDIGKEITSOPTIONEN",
        restart: "🔄 MISSION NEU STARTEN",
        surrender: "💀 KAPITULIEREN / ZUM MENÜ",
    },
    scenarios: [
        // Scenario 0: Eden 1
        BriefingScenarioDict {
            scenario_id: 0,
            title: "Operation Neue Dämmerung",
            subtitle: "Sektor Eden Prime // Planetare Kolonisation",
            transcript: "Kolonisten der Eden-Stiftung: Nach dem katastrophalen Verlust des Raumschiffs Conestoga muss unser Landeplatz auf New Terra unverzüglich gesichert werden. Vulkanische Erschütterungen deuten auf aufsteigende Magmaspalten in der Nähe hin, und abtrünnige Plymouth-Angreifer haben aggressive Außenposten errichtet.\n\nSichern Sie die Lebenserhaltung der Kolonie, automatisieren Sie den Erztransport, stellen Sie gepanzerte Railgun-Fahrzeuge in Dienst und treiben Sie die planetare Forschung voran, um das Überleben unserer Spezies zu sichern.",
            primary_directives: &[
                "3.000 t Erz und 600 t Seltenerz anhäufen",
                "Mindestens 2 bewaffnete Kampfpanzer in Dienst stellen",
                "Wissenschaftliche Forschung in einer Primärdisziplin abschließen",
                "Kommandozentrale der Kolonie vor feindlicher Zerstörung schützen",
            ],
            secondary_directives: &[
                "Koloniebevölkerung von 120 Kolonisten mit positiver Moral aufbauen",
            ],
            hazard_summary: "Lokale Staubwirbel; leichte seismische Erschütterungen.",
        },
        // Scenario 1: Plymouth 1
        BriefingScenarioDict {
            scenario_id: 1,
            title: "Operation Ödland-Vorhut",
            subtitle: "Sektor Plymouth Süd // Territoriale Rückeroberung",
            transcript: "Kommandeure von Plymouth: Im Widerstand gegen Edens autoritäre Technokratie haben unsere Siedler in den schroffen Schluchten des Ödlands eine Heimat geschaffen. Eden-Späher dringen nun in unsere Geothermalgebiete vor.\n\nErrichten Sie eine Fahrzeugfabrik, richten Sie automatisierte Erzkonvois ein, stellen Sie eine Streitmacht von 3 Kampfpanzern zusammen und vernichten Sie die Eden-Angreifer, um unser Territorium zu sichern.",
            primary_directives: &[
                "Fahrzeugfabrik zur Produktion gepanzerter Fahrgestelle in Betrieb nehmen",
                "Angriffsgeschwader von mindestens 3 Kampfpanzern zusammenstellen",
                "2.500 t Erz abbauen, um die Canyongrenze zu sichern",
                "Kommandozentrale von Plymouth gegen Eden-Angriffe verteidigen",
            ],
            secondary_directives: &[
                "Kontinuierlichen Energieüberschuss während der Ödland-Nachtzyklen aufrechterhalten",
            ],
            hazard_summary: "Hohes Risiko vulkanischer Beben; aktive Geothermaldämpfe.",
        },
        // Scenario 2: Tutorial
        BriefingScenarioDict {
            scenario_id: 2,
            title: "Kadetten-Übungsgelände",
            subtitle: "Kolonialakademie // Taktischer Operationssimulator",
            transcript: "Kadett: Willkommen im Kolonialen Operationssimulator von New Terra. In dieser Prüfung lernen Sie die Grundlagen der planetaren Kolonieleitung kennen.\n\nStellen Sie sicher, dass die Stromerzeugung den Verbrauch übersteigt, verlegen Sie Drucktransportröhren zwischen Gebäuden und überwachen Sie den automatischen Erztransport zur Schmelze.",
            primary_directives: &[
                "Positiven Energieüberschuss erzielen (Erzeugung > Verbrauch)",
                "Gebäude mit mindestens 15 Drucktransportröhren verbinden",
                "Erste Frachtladung Erz zur Schmelze transportieren",
                "Kolonistenmoral über 90% halten",
            ],
            secondary_directives: &[
                "Automatisiertes Robo-Bergbaufahrzeug auf einem Erzsignal platzieren",
            ],
            hazard_summary: "Kontrollierte Simulatorumgebung; geringe Umweltgefahren.",
        },
        // Scenario 3: Eden 2
        BriefingScenarioDict {
            scenario_id: 3,
            title: "Operation Caldera-Bastion",
            subtitle: "Ausläufer des Mount Vulcan // Eindämmung der Magmaspalten",
            transcript: "Eden-Kommando-Alarm: Unterirdische Thermosensoren registrieren katastrophale Magmabewegungen entlang der nördlichen Verwerfungslinien. Glühende Spalten reißen den Permafrost auf und bedrohen die Kuppelsektoren.\n\nStabilisieren Sie die Energieversorgung über 400 kW mittels Hochleistungsanlagen oder Geothermieleitungen, errichten Sie mindestens 2 Wachturm-Geschütze am Spaltenrand, lagern Sie 2.500 t Erz und 500 t Seltenerz für Notbarrikaden ein und sichern Sie die Kommandozentrale.",
            primary_directives: &[
                "Geothermales Eindämmungsnetz errichten (>= 400 kW Energie erzeugen)",
                "Mindestens 2 Wachturm-Geschütze am Rand der Magmaspalte bauen",
                "2.500 t Erz und 500 t Seltenerz als Reserve einlagern",
                "Kommandozentrale vor seismischer oder feindlicher Vernichtung schützen",
            ],
            secondary_directives: &[
                "Koloniemoral bei oder über 80% halten",
                "Wissenschaftliche Forschung in einer fortgeschrittenen Disziplin abschließen",
            ],
            hazard_summary: "Sich ausbreitende Lavaspalten; vorrückende Magmaströme.",
        },
        // Scenario 4: Plymouth 2
        BriefingScenarioDict {
            scenario_id: 4,
            title: "Operation Eisenklaue",
            subtitle: "Mineralgrat-Engpass // Schmelzensabotage & Hinterhalt",
            transcript: "Depesche des Hohen Rats von Plymouth: Die Technokraten von Eden haben einen schwer bewaffneten Erzgewinnungs- und Schmelzkomplex an unseren südlichen Passagen verschanzt und schneiden uns von den Ressourcen ab.\n\nStellen Sie ein gepanzertes Hinterhaltsgeschwader von mindestens 4 Kampfpanzern zusammen, errichten Sie ein befestigtes Wachturm-Geschütz am Engpass, lagern Sie 3.500 t Erz und 800 t Seltenerz ein und verteidigen Sie die Kommandozentrale.",
            primary_directives: &[
                "Gepanzertes Hinterhaltsgeschwader von >= 4 Panzern aufstellen",
                "Engpass mit mindestens 1 Wachturm-Geschütz befestigen",
                "Mindestens 3.500 t Erz und 800 t Seltenerz einlagern",
                "Kommandozentrale von Plymouth vor Edens Gegenangriffen schützen",
            ],
            secondary_directives: &[
                "Fahrzeugfabrik für gepanzerte Fahrgestelle betriebsbereit halten",
                "Positiven Energieüberschuss im taktischen Netz aufrechterhalten",
            ],
            hazard_summary: "Befestigte Lasergeschütze; gefährliche Canyon-Engpässe.",
        },
        // Scenario 5: Eden 3
        BriefingScenarioDict {
            scenario_id: 5,
            title: "Raumschiff-Evakuierung",
            subtitle: "Caldera des Mount Vulcan // Flucht vom Mount Vulcan",
            transcript: "Notfallübertragung des Eden-Oberkommandos: Die Caldera des Mount Vulcan durchläuft eine apokalyptische Supervulkanausbruchs-Sequenz. Lavaströme schneiden die Südzugänge ab, während Plymouth-Verbände anrücken, um unser Evakuierungsschiff zu kapern.\n\nNehmen Sie die Startrampe in Betrieb, fertigen Sie alle vier Raumschiffmodule (Kommandomodul, Kryokammer, Habitatring, Ionenantrieb), transportieren Sie mindestens 150 Kolonisten in Evakuierungstransportern zu den Stasiskammern und halten Sie den Außenbereich bis zum Orbitalstart.",
            primary_directives: &[
                "Raumschiffmodule montieren (Kommando, Kryostase, Habitat, Ionenantrieb)",
                "Mindestens 150 Kolonisten mit Evakuierungstransportern evakuieren",
                "Kolonie gegen Plymouth-Angreifer und vorrückende Lavamassen verteidigen",
                "Kommandozentrale von Eden vor Zerstörung schützen",
            ],
            secondary_directives: &[
                "Koloniemoral während des Start-Countdowns über 85% halten",
                "3.000 t Erz und 1.000 t Seltenerz als Treibstoffreserve einlagern",
            ],
            hazard_summary: "Kataklysmischer Vulkanausbruch; schnell vorrückende Lavaströme.",
        },
        // Scenario 6: Plymouth 3
        BriefingScenarioDict {
            scenario_id: 6,
            title: "Operation Phönix: Obsidian-Klippen",
            subtitle: "Obsidian-Klippen // Zitadellensturm & Terraformer-Verteidigung",
            transcript: "Depesche des Rats freier Siedler von Plymouth: Der Atmosphären-Prozessor in den Obsidian-Klippen ist unsere letzte Hoffnung auf atembare Luft auf New Terra. Doch Edens Obsidian-Zitadelle riegelt das Gebiet ab, und mikroskopische Naniten-Fäulnis breitet sich in Richtung der Prozessor-Ansaugstutzen aus.\n\nSetzen Sie schwere Angriffspanzer ein, verteidigen Sie den Atmosphären-Terraformer vor der Naniten-Verseuchung bis 100% Sättigung erreicht sind, und stürmen Sie Edens Zitadelle, um deren Railgun-Stellungen und Tiger-Panzerverbände auszuschalten.",
            primary_directives: &[
                "Atmosphären-Terraformer vor der Naniten-Fäulnis schützen",
                "Durchbruchsangriff auf Edens befestigte Obsidian-Zitadelle",
                "100% Sättigung des Atmosphären-Terraformers erreichen",
                "Kommandozentrale von Plymouth vor Zerstörung schützen",
            ],
            secondary_directives: &[
                "Alle feindlichen Railgun-Wachtürme in den Klippen vernichten",
            ],
            hazard_summary: "Mikrobielle Naniten-Seuche; schwere Railgun-Festungen.",
        },
    ],
};

// --- Simplified Chinese Dictionary ---
static TRANSLATION_ZH: Translation = Translation {
    faction: "阵营:",
    sol: "火星日:",
    sun: "光照:",
    pop: "人口:",
    power: "电力:",
    common_ore: "普通矿石:",
    rare_ore: "稀有矿石:",
    food: "食物:",
    build_base: "🏗️ 建造基地",
    units: "🚜 作战单位",
    research: "🔬 科技研发",
    save: "💾 存档",
    load: "📂 读档",
    radar: "战术雷达",
    briefing: "📋 任务简报",
    music: "🎵 音乐",

    language: Language::Chinese,
    main_menu: MainMenuDict {
        new_campaign: "新战役",
        campaign_missions: "🚀 战役任务 (伊甸与普利茅斯)",
        skirmish: "遭遇战",
        skirmish_battle: "⚔️ 遭遇战对决 (1-5 电脑AI)",
        multiplayer: "多人游戏",
        multiplayer_lobby: "📡 多人对战 P2P (WEBRTC 大厅)",
        load_game: "读取存档",
        load_archive: "📂 读取任务存档 (.OP2SAVE)",
        options: "系统设置",
        options_audio_velocity: "⚙️ 系统设置 / 音频与模拟速率",
        resume: "继续游戏",
        resume_operation: "▶️ 恢复当前作战行动",
        quit: "退出",
        back_to_menu: "◀ 返回主菜单",
        back: "◀ 返回",
        prev_op: "◀ 上一任务",
        next_op: "下一任务 ▶",
        begin_operation: "🚀 启动作战行动",
        apply_and_return: "保存并返回",
        title_tagline: "雪乐山在线与 DYNAMICS 经典引擎 // 忒堤斯远征纪元",
        subtitle: "—  命 运 分 歧  —",
        select_theater: "请选择战役作战战区",
        select_theater_sub: "在崇尚科技的伊甸基金会与崇尚自由的普利茅斯兄弟会之间选择阵营效忠。",
        tutorial_button: "🎓 学员实战演练场 (殖民地军官学院训练中心)",
        operational_directives_checklist: "作战行动指令核查清单:",
        tactical_operation_briefing: "战术作战任务指令简报",
        directive_prefix: "主线指令",
        secondary_prefix: "次要目标",
    },
    skirmish: SkirmishDict {
        title: "战术遭遇战大厅 // 战区交战指令",
        map_selection: "作战地图",
        player_faction: "玩家阵营",
        bot_count: "电脑敌军数",
        personality_aggressive_rusher: "激进突袭者",
        personality_defensive_fortifier: "坚固防守者",
        personality_economic_tech_rusher: "经济科技专精",
        difficulty_easy: "简单",
        difficulty_medium: "中等",
        difficulty_hard: "困难",
        abundance: "资源丰度",
        abundance_sparse: "贫瘠 (1,000 普通 / 200 稀有)",
        abundance_standard: "标准 (2,500 普通 / 600 稀有)",
        abundance_abundant: "丰富 (5,000 普通 / 1,500 稀有)",
        abundance_infinite: "无限矿石 (50,000 普通 / 20,000 稀有)",
        disaster: "自然灾害",
        disaster_disabled: "已禁用 (平静安全)",
        disaster_low: "极少 (偶有轻震)",
        disaster_standard: "正常 (常规地质危机)",
        disaster_catastrophic: "灾难性 (极度危险)",
        launch: "⚔️ 开始遭遇战",
        theater_params: "战区地形与演习模拟参数:",
        ai_slots_header: "对抗 AI 席位 (1 至 5 名电脑对手):",
        bot_active: "[X] 参战",
        bot_off: "[ ] 关闭",
        status_summary: "对局状态",
    },
    multiplayer: MultiplayerDict {
        title: "战术 WEBRTC 点对点联机大厅 (无服务器)",
        host: "创建主机 (OFFER)",
        guest: "客机加入 (ANSWER)",
        copy_offer: "📋 复制提议代码 (OFFER)",
        paste_answer: "📥 粘贴客机应答代码 (ANSWER)",
        paste_offer: "📥 粘贴主机提议代码 (OFFER)",
        generate_answer: "接受提议并生成应答 (ANSWER)",
        copy_answer: "📋 复制应答代码 (发给主机)",
        lock_connection: "锁定建立连接",
        peer_connected: "对等端已连接",
        peer_disconnected: "已断开连接",
        peer_connecting: "正在建立通信连接...",
        launch_match: "🚀 启动多人比赛",
        ready_toggle: "准备就绪: 是 [切换]",
        not_ready_toggle: "准备就绪: 否 [切换]",
        offer_ready_header: "已生成的 BASE64 SDP 提议代码 (可直接共享):",
        awaiting_offer: "等待生成 WebRTC 提议代码...",
        status_banner_label: "网络通信状态",
    },
    hud: HudDict {
        common_ore: "普通矿石",
        rare_ore: "稀有矿石",
        power: "基地电力",
        food: "食物储备",
        population: "殖民人口",
        sol: "火星日",
        sun: "太阳光照",
        starship: "逃亡星舰",
        tactical_radar: "战术雷达",
        radar_active: "扫描中",
        grid_optimal: "电网最优",
        grid_brownout: "供电不足警告",
        grid_blackout: "电网全线崩溃",
        food_surplus: "食物充裕",
        food_adequate: "供应正常",
        food_rationing: "配给调控",
        food_starvation: "严重饥荒",
        morale_excellent: "极高",
        morale_good: "良好",
        morale_fair: "一般",
        morale_poor: "低落",
        morale_rioting: "暴动骚乱",
        factory_title: "工厂制造与工程载具组件",
        factory_hull: "装甲底盘",
        factory_cost: "造价成本",
        factory_rally: "集结集结点",
        factory_queue: "装配线建造队列:",
        factory_unassigned_rally: "集结点: 默认未分配 (工厂出厂坞)",
        factory_footer_hint: "按 'F' 键开闭控制面板 | 鼠标右键指派集结点",
        research_title: "殖民地科研矩阵",
        research_active: "当前进行中研发项目",
        research_none: "当前无进行中的科研课题",
        research_footer_hint: "按 'R' 键开闭科技树研发面板",
    },
    debriefing: DebriefingDict {
        rank_supreme_savior_title: "至高殖民地救世主",
        rank_supreme_savior_badge: "🎖️ 至高殖民地救世主",
        rank_supreme_savior_desc: "无懈可击的卓越统帅：极低伤亡、超强工业产出与彻底的战术制霸。",

        rank_distinguished_pioneer_title: "杰出开拓先驱",
        rank_distinguished_pioneer_badge: "🌟 杰出开拓先驱",
        rank_distinguished_pioneer_desc: "值得嘉奖的指挥风范：在险恶严苛的行星环境下沉着达成全盘殖民目标。",

        rank_tethys_survivor_title: "忒堤斯幸存者",
        rank_tethys_survivor_badge: "🛡️ 忒堤斯幸存者",
        rank_tethys_survivor_desc: "代价惨痛的惨胜：历经惨烈死伤与物资空前消耗，勉强保全殖民地火种。",

        rank_colony_failure_title: "殖民彻底失控",
        rank_colony_failure_badge: "☠️ 殖民彻底失控",
        rank_colony_failure_desc: "毁灭性溃败：外星拓殖据点沦陷，平民伤亡惨重，核心战略指令彻底失败。",

        victory_title: "任务完成：胜利",
        defeat_title: "殖民失败：任务终止",
        victory_subtitle_template: "祝贺你，指挥官！所有核心战术目标指令均已圆满完成。",
        defeat_subtitle_template: "灾难性溃败。殖民地核心指挥中心已被敌方或灾难彻底摧毁。",
        colony_duration: "生存周期",
        evacuation_readiness: "撤离完备度",
        survivors_morale: "幸存者与士气",
        refined_reserves: "精炼储备资源",
        score_label: "殖民地综合评分",
        directives_checklist: "作战指令完成度核查",
        starship_escaped: "🚀 成功脱离引力进入轨道！",
        starship_assembled_template: "已组装 {:.0}%",
    },
    units_buildings: UnitBuildingDict {
        command_center: "指挥中心",
        solar_array: "太阳能发电阵列",
        tokamak_reactor: "托卡马克聚变反应堆",
        geothermal_plant: "地热发电厂",
        common_ore_smelter: "普通矿石冶炼厂",
        rare_ore_smelter: "稀有矿石冶炼厂",
        dirt_lab: "D.I.R.T. 基础实验室",
        standard_lab: "标准科研实验室",
        residence: "殖民居所",
        agrodome: "生态水培温室",
        vehicle_factory: "战车工厂",
        repair_facility: "装甲维修车间",
        guard_post: "警戒炮塔",
        spaceport: "星舰发射台",
        terraformer: "大气地球化改造仪",

        lynx: "山猫侦察车 (Lynx)",
        panther: "黑豹主战坦克 (Panther)",
        tiger: "虎式重型战车 (Tiger)",
        cargo_truck: "货运卡车",
        convec: "工程建造车 (ConVec)",
        robo_miner: "全自动采矿机 (Robo-Miner)",
        robo_surveyor: "地质勘探仪 (Robo-Surveyor)",
        earthworker: "土方工程车 (Earthworker)",
        evacuation_transport: "撤离人员运输机",
    },
    options: OptionsDict {
        title: "系统选项与音频矩阵",
        master_volume: "主音频输出音量大小:",
        sim_velocity: "模拟演进速率倍率:",
        crt_scanlines: "CRT 复古光栅扫描线效果:",
        active_language: "当前激活语言 / 区域格式:",
        enabled: "已开启",
        disabled: "已关闭",
    },
    pause_menu: PauseMenuDict {
        title: "战术战况模拟已暂停",
        resume: "▶️ 继续任务执行 (ESC)",
        quicksave: "💾 快速存档 (F5)",
        quickload: "📂 快速读档 (F9)",
        save_load_manager: "🗄️ 存档 / 读档管理中心",
        options: "⚙️ 音频与流速选项",
        restart: "🔄 重新开始本次任务",
        surrender: "💀 弃守放弃 / 返回主菜单",
    },
    scenarios: [
        // Scenario 0: Eden 1
        BriefingScenarioDict {
            scenario_id: 0,
            title: "新曙光行动",
            subtitle: "伊甸主基地防区 // 行星拓殖",
            transcript: "伊甸基金会的拓殖者们：在康内斯托加号星舰遭遇灾难性事故后，我们必须立即巩固新泰拉行星的着陆点。地质监测显示附近火山岩浆裂隙正在加剧，普利茅斯的武装掠夺小队已在周边建立前哨站。\n\n保障殖民地维生系统运转，建立自动化矿石货运网络，制造电磁轨道炮装甲战车，并推进科学研发以确保人类文明的延续。",
            primary_directives: &[
                "储备 3,000 吨普通矿石和 600 吨稀有矿石",
                "制造并部署至少 2 辆武装主战坦克",
                "在任意基础学科完成一项科学技术研发",
                "保卫殖民地指挥中心免遭敌军摧毁",
            ],
            secondary_directives: &[
                "维持 120 名殖民者定居人口且保持正向士气",
            ],
            hazard_summary: "局部沙尘旋风；轻微地质震颤。",
        },
        // Scenario 1: Plymouth 1
        BriefingScenarioDict {
            scenario_id: 1,
            title: "荒原先锋行动",
            subtitle: "普利茅斯南区防线 // 领地光复",
            transcript: "普利茅斯的指挥官们：为了摆脱伊甸基金会的极权科技统治，我们的拓殖者在崎岖的荒原峡谷中开辟了家园。伊甸的侦察分队正在逼近我们的地热排气口。\n\n建造战车工厂，建立全自动采矿运输车队，集结由至少 3 辆主战坦克组成的突击装甲编队，歼灭伊甸侵略者以捍卫我们神圣的主权领地。",
            primary_directives: &[
                "建造并投产战车工厂以生产重型装甲底盘",
                "集结一支至少拥有 3 辆作战坦克的装甲突击中队",
                "开采 2,500 吨普通矿石以巩固大峡谷防御前沿",
                "保卫普利茅斯指挥中心击退伊甸部队的猛烈进攻",
            ],
            secondary_directives: &[
                "在荒原严寒的黑夜周期中保持电网始终处于盈余状态",
            ],
            hazard_summary: "高烈度火山地震风险；剧烈地热毒气排气孔。",
        },
        // Scenario 2: Tutorial
        BriefingScenarioDict {
            scenario_id: 2,
            title: "学员实战训练场",
            subtitle: "殖民地军官学院 // 战术模拟演练中心",
            transcript: "军校学员：欢迎进入新泰拉殖民地战术模拟演习系统。在本次考核中，你将掌握行星基地建立与资源调配的核心基础准则。\n\n确保基地总发电量超越总耗电负荷，在各建筑设施之间铺设密闭增压交通管道，并调度矿石货运车将原矿输送至冶炼厂。",
            primary_directives: &[
                "建立正向电力结余 (发电总量 > 耗电负载)",
                "使用至少 15 条增压密封通道连接各殖民设施",
                "将首批开采的普通矿石顺利运抵冶炼厂",
                "将殖民地居民整体士气维持在 90% 以上",
            ],
            secondary_directives: &[
                "在矿藏信标上部署并投产一台全自动采矿机",
            ],
            hazard_summary: "受控模拟演习环境；环境危害极低。",
        },
        // Scenario 3: Eden 2
        BriefingScenarioDict {
            scenario_id: 3,
            title: "破火山口要塞行动",
            subtitle: "沃尔肯火山山麓 // 熔岩地裂阻断战",
            transcript: "伊甸最高司令部紧急战报：地下热能传感器检测到北部断裂带正发生毁灭性岩浆挤压。滚烫的熔岩裂隙撕裂了冻土带，直接威胁到生态防护穹顶的安全。\n\n通过高能发电阵列或地热发电机将供电稳定在 400 千瓦以上，在熔岩通道边缘建立至少 2 座重型警戒炮塔防御工事，储备 2,500 吨普通矿石和 500 吨稀有矿石加固热能防线，确保伊甸指挥中心免遭地裂与敌火摧毁。",
            primary_directives: &[
                "建立地热能源阻断电网 (总发电量 >= 400 kW)",
                "在熔岩裂缝边缘建造至少 2 座防御警戒炮塔",
                "储备 2,500 吨普通矿石和 500 吨稀有矿石作为应急加固建材",
                "保护伊甸指挥中心免遭火山地震或敌军炮火破坏",
            ],
            secondary_directives: &[
                "将殖民地居民的民心士气维持在 80% 或更高",
                "完成任意高级学科领域的科学技术研发",
            ],
            hazard_summary: "持续扩散的火山熔岩裂隙；不断推进的高温岩浆流。",
        },
        // Scenario 4: Plymouth 2
        BriefingScenarioDict {
            scenario_id: 4,
            title: "铁牙行动",
            subtitle: "矿脊要隘咽喉 // 冶炼厂破坏与伏击战",
            transcript: "普利茅斯最高理事会行动令：伊甸的技术官僚在扼守我们南部荒原峡谷通路的要冲地带筑起了重兵把守的采矿冶炼综合体，企图切断普利茅斯的矿石生命线。\n\n集结一支装备磁轨炮或强激光的至少 4 辆主战坦克装甲伏击连队，在峡谷隘口修筑重装警戒炮塔，抢运并储备至少 3,500 吨普通矿石和 800 吨稀有矿石，保卫普利茅斯指挥中心粉碎伊甸的反扑反击。",
            primary_directives: &[
                "集结一支至少由 4 辆武装主战坦克组成的装甲突击编队",
                "在峡谷咽喉锁钥位置构筑至少 1 座重装防御警戒炮塔",
                "储备至少 3,500 吨普通矿石和 800 吨稀有矿石",
                "保卫普利茅斯指挥中心击退伊甸自动反击部队的猛攻",
            ],
            secondary_directives: &[
                "确保战车工厂持续处于满负荷战车运转生产状态",
                "维持整个战术电网的发电量持续处于充裕结余状态",
            ],
            hazard_summary: "重装敌军激光防御炮塔；危险的峡谷狭窄隘口。",
        },
        // Scenario 5: Eden 3
        BriefingScenarioDict {
            scenario_id: 5,
            title: "星舰紧急撤离行动",
            subtitle: "沃尔肯破火山口 // 逃离沃尔肯火山灾厄",
            transcript: "伊甸最高司令部特级紧急广播：沃尔肯火山口正处于毁灭性的超级火山大爆发临界点。滚滚岩浆已切断了南部通道，失去理智的普利茅斯叛军正向发射场疯狂合围以抢夺撤离星舰。\n\n启动星舰发射龙门台，制造并组装四大逃生核心模块（指挥模块、低温冷冻休眠舱、居住环带与离子推进引擎），派遣撤离运输机将至少 150 名殖民者送入休眠舱，坚守防线直至星舰达到飞离轨道的第二宇宙速度。",
            primary_directives: &[
                "制造并组装全部星舰模块 (指挥舱、冷冻舱、居住环、离子引擎)",
                "调度人员运输车辆安全撤离至少 150 名核心殖民人员",
                "击退普利茅斯袭击者并阻断炽热岩浆裂缝的吞噬蔓延",
                "严密保护伊甸核心指挥中心免遭任何毁灭打击",
            ],
            secondary_directives: &[
                "在逃亡星舰点火倒计时期间将居民士气稳定在 85% 以上",
                "储备 3,000 吨普通矿石和 1,000 吨稀有矿石作为逃生推进燃料储备",
            ],
            hazard_summary: "末日级大灾变火山爆发；高速漫延的炽热岩浆长河。",
        },
        // Scenario 6: Plymouth 3
        BriefingScenarioDict {
            scenario_id: 6,
            title: "凤凰行动：黑曜石峭壁",
            subtitle: "黑曜石峭壁要塞防区 // 突击要塞与大气地球化改造仪保卫战",
            transcript: "普利茅斯自由开拓者最高委员会绝密战报：矗立在黑曜石峭壁的大气行星改造处理器是人类在新泰拉重塑可呼吸大气的最后生机。然而，伊甸坚固的黑曜石要塞封锁了整个峡谷，失控的纳米微生物枯萎瘟疫正向处理器核心排气口加速扩散。\n\n部署重型装甲突击集群，死守大气地球化改造仪免遭纳米瘟疫彻底感染直至达到 100% 行星大气饱和度，攻破坚不可摧的伊甸黑曜石要塞，摧毁其前沿防御炮塔和虎式重型磁轨炮装甲军团。",
            primary_directives: &[
                "保卫大气地球化改造仪免受不断扩散的纳米枯萎病菌毒害",
                "强行突破并攻破伊甸防御森严的黑曜石重装核心要塞",
                "使大气地球化改造仪顺利达到 100% 满负荷行星饱和度",
                "全力保卫普利茅斯指挥中心免遭任何形式的彻底毁灭",
            ],
            secondary_directives: &[
                "彻底摧毁峭壁走廊周围所有伊甸重型磁轨炮防御警戒炮塔",
            ],
            hazard_summary: "致命微型纳米噬菌体疫病；重装磁轨炮高地要塞群。",
        },
    ],
};

// --- Russian Dictionary ---
static TRANSLATION_RU: Translation = Translation {
    faction: "Фракция:",
    sol: "Сол:",
    sun: "Солнце:",
    pop: "Население:",
    power: "Энергия:",
    common_ore: "Обычная Руда:",
    rare_ore: "Редкая Руда:",
    food: "Пища:",
    build_base: "🏗️ Строить Базу",
    units: "🚜 Юниты",
    research: "🔬 Исследования",
    save: "💾 Сохранить",
    load: "📂 Загрузить",
    radar: "Тактический Радар",
    briefing: "📋 Брифинг",
    music: "🎵 Музыка",

    language: Language::Russian,
    main_menu: MainMenuDict {
        new_campaign: "Новая кампания",
        campaign_missions: "🚀 КАМПАНИЯ (ЭДЕМ И ПЛИМУТ)",
        skirmish: "Схватка",
        skirmish_battle: "⚔️ СХВАТКА (1-5 ИИ БОТОВ)",
        multiplayer: "Сетевая игра",
        multiplayer_lobby: "📡 СЕТЕВАЯ ИГРА P2P (WEBRTC)",
        load_game: "Загрузить игру",
        load_archive: "📂 ЗАГРУЗИТЬ АРХИВ (.OP2SAVE)",
        options: "Настройки",
        options_audio_velocity: "⚙️ НАСТРОЙКИ / ЗВУК И СКОРОСТЬ",
        resume: "Продолжить",
        resume_operation: "▶️ ПРОДОЛЖИТЬ ТЕКУЩУЮ МИССИЮ",
        quit: "Выход",
        back_to_menu: "◀ В ГЛАВНОЕ МЕНЮ",
        back: "◀ НАЗАД",
        prev_op: "◀ ПРЕД. ОП",
        next_op: "СЛЕД. ОП ▶",
        begin_operation: "🚀 НАЧАТЬ ОПЕРАЦИЮ",
        apply_and_return: "ПРИМЕНИТЬ И НАЗАД",
        title_tagline: "SIERRA ON-LINE И ДВИЖОК DYNAMICS // ЭКСПЕДИЦИЯ НА ТЕТИС",
        subtitle: "—  Р А З Д Е Л Е Н Н А Я   С У Д Ь Б А  —",
        select_theater: "ВЫБЕРИТЕ ТЕАТР ВОЕННЫХ ДЕЙСТВИЙ",
        select_theater_sub: "Выберите сторону: научный Фонд Эдем или Братство Плимута.",
        tutorial_button: "🎓 ПОЛИГОН КУРСАНТОВ (СИМУЛЯТОР АКАДЕМИИ)",
        operational_directives_checklist: "СПИСОК ОПЕРАТИВНЫХ ДИРЕКТИВ:",
        tactical_operation_briefing: "БРИФИНГ ТАКТИЧЕСКОЙ ОПЕРАЦИИ",
        directive_prefix: "ДИРЕКТИВА",
        secondary_prefix: "ВТОРИЧНАЯ",
    },
    skirmish: SkirmishDict {
        title: "ТАКТИЧЕСКОЕ ЛОББИ СХВАТКИ // БОЙ В СЕКТОРЕ",
        map_selection: "КАРТА",
        player_faction: "ФРАКЦИЯ",
        bot_count: "Противники",
        personality_aggressive_rusher: "Агрессивный штурмовик",
        personality_defensive_fortifier: "Оборонительный укрепитель",
        personality_economic_tech_rusher: "Экономический технократ",
        difficulty_easy: "Легко",
        difficulty_medium: "Средне",
        difficulty_hard: "Сложно",
        abundance: "РЕСУРСЫ",
        abundance_sparse: "Скудные (1 000 Руды / 200 Редкой)",
        abundance_standard: "Стандартные (2 500 Руды / 600 Редкой)",
        abundance_abundant: "Богатые (5 000 Руды / 1 500 Редкой)",
        abundance_infinite: "Бесконечная руда (50 000 / 20 000)",
        disaster: "КАТАСТРОФЫ",
        disaster_disabled: "Отключены (Спокойно)",
        disaster_low: "Низкая (Редкие толчки)",
        disaster_standard: "Стандартная (Обычные угрозы)",
        disaster_catastrophic: "Катастрофическая (Экстрим)",
        launch: "⚔️ НАЧАТЬ СХВАТКУ",
        theater_params: "ПАРАМЕТРЫ ТЕАТРА И СИМУЛЯЦИИ:",
        ai_slots_header: "СЛОТЫ БОЕВЫХ ИИ (1-5 ПРОТИВНИКОВ):",
        bot_active: "[X] АКТИВЕН",
        bot_off: "[ ] ВЫКЛ",
        status_summary: "Статус",
    },
    multiplayer: MultiplayerDict {
        title: "ТАКТИЧЕСКОЕ ЛОББИ P2P WEBRTC (БЕЗ СЕРВЕРА)",
        host: "СОЗДАТЬ ИГРУ (OFFER)",
        guest: "ВОЙТИ КАК ГОСТЬ (ANSWER)",
        copy_offer: "📋 КОПИРОВАТЬ OFFER В БУФЕР",
        paste_answer: "📥 ВСТАВИТЬ ANSWER ГОСТЯ",
        paste_offer: "📥 ВСТАВИТЬ OFFER ХОСТА",
        generate_answer: "ПРИНЯТЬ OFFER И СОЗДАТЬ ANSWER",
        copy_answer: "📋 КОПИРОВАТЬ ANSWER (ОТПРАВИТЬ ХОСТУ)",
        lock_connection: "ЗАКРЕПИТЬ СОЕДИНЕНИЕ",
        peer_connected: "Пир подключен",
        peer_disconnected: "Отключено",
        peer_connecting: "Подключение...",
        launch_match: "🚀 ЗАПУСТИТЬ МАТЧ",
        ready_toggle: "ГОТОВ: ДА [ПЕРЕКЛЮЧИТЬ]",
        not_ready_toggle: "ГОТОВ: НЕТ [ПЕРЕКЛЮЧИТЬ]",
        offer_ready_header: "СФОРМИРОВАННЫЙ BASE64 SDP OFFER (ГОТОВ К ОТПРАВКЕ):",
        awaiting_offer: "Ожидание генерации WebRTC Offer...",
        status_banner_label: "СТАТУС СОЕДИНЕНИЯ",
    },
    hud: HudDict {
        common_ore: "Обычная Руда",
        rare_ore: "Редкая Руда",
        power: "Энергия",
        food: "Пища",
        population: "Население",
        sol: "Сол",
        sun: "Солнечный свет",
        starship: "Звездолет",
        tactical_radar: "Тактический Радар",
        radar_active: "АКТИВЕН",
        grid_optimal: "СЕТЬ В НОРМЕ",
        grid_brownout: "СБОЙ ПИТАНИЯ",
        grid_blackout: "КРИТИЧЕСКИЙ СБОЙ",
        food_surplus: "Избыток",
        food_adequate: "Достаточно",
        food_rationing: "Нормирование",
        food_starvation: "ГОЛОД",
        morale_excellent: "Отличный",
        morale_good: "Хороший",
        morale_fair: "Средний",
        morale_poor: "Плохой",
        morale_rioting: "Бунт",
        factory_title: "ПРОИЗВОДСТВО ФАБРИКИ И КИТЫ CONVEC",
        factory_hull: "Шасси",
        factory_cost: "Стоимость",
        factory_rally: "Точка сбора",
        factory_queue: "Очередь производства:",
        factory_unassigned_rally: "Точка сбора: Не назначена (Цех)",
        factory_footer_hint: "Нажмите 'F' для открытия панели | Правый клик — точка сбора",
        research_title: "МАТРИЦА НАУЧНЫХ ИССЛЕДОВАНИЙ",
        research_active: "Текущий проект",
        research_none: "Нет активного проекта исследований",
        research_footer_hint: "Нажмите 'R' для открытия матрицы исследований",
    },
    debriefing: DebriefingDict {
        rank_supreme_savior_title: "Великий Спаситель Колонии",
        rank_supreme_savior_badge: "🎖️ Великий Спаситель Колонии",
        rank_supreme_savior_desc: "Безупречное командование: Минимальные потери, превосходное производство и абсолютное стратегическое превосходство.",

        rank_distinguished_pioneer_title: "Заслуженный Пионер",
        rank_distinguished_pioneer_badge: "🌟 Заслуженный Пионер",
        rank_distinguished_pioneer_desc: "Достойное руководство: Цели колонии выполнены с честью в опаснейших условиях.",

        rank_tethys_survivor_title: "Выживший на Тетисе",
        rank_tethys_survivor_badge: "🛡️ Выживший на Тетисе",
        rank_tethys_survivor_desc: "Пиррова победа: Колония выжила ценой огромных потерь и колоссальных затрат ресурсов.",

        rank_colony_failure_title: "Крах Колонии",
        rank_colony_failure_badge: "☠️ Крах Колонии",
        rank_colony_failure_desc: "Катастрофический провал: Аванпост потерян, критические потери среди населения, директивы провалены.",

        victory_title: "МИССИЯ ВЫПОЛНЕНА: ПОБЕДА",
        defeat_title: "ПОРАЖЕНИЕ: МИССИЯ ПРОВАЛЕНА",
        victory_subtitle_template: "Поздравляем, Командир! Основные директивы успешно выполнены.",
        defeat_subtitle_template: "Критический провал. Командный Центр колонии полностью уничтожен.",
        colony_duration: "Время существования",
        evacuation_readiness: "Готовность к эвакуации",
        survivors_morale: "Выжившие / Мораль",
        refined_reserves: "Запасы ресурсов",
        score_label: "Итоговый счет колонии",
        directives_checklist: "Проверка выполнения директив",
        starship_escaped: "🚀 Успешный выход на орбиту!",
        starship_assembled_template: "{:.0}% Собрано",
    },
    units_buildings: UnitBuildingDict {
        command_center: "Командный Центр",
        solar_array: "Солнечная Электростанция",
        tokamak_reactor: "Термоядерный Реактор Токамак",
        geothermal_plant: "Геотермальная Станция",
        common_ore_smelter: "Плавильня Обычной Руды",
        rare_ore_smelter: "Плавильня Редкой Руды",
        dirt_lab: "Лаборатория D.I.R.T.",
        standard_lab: "Стандартная Лаборатория",
        residence: "Жилой Комплекс",
        agrodome: "Агрокупол Гидропоники",
        vehicle_factory: "Завод Машин",
        repair_facility: "Ремонтная Мастерская",
        guard_post: "Сторожевая Турель",
        spaceport: "Космодром",
        terraformer: "Атмосферный Терраформер",

        lynx: "Разведчик Lynx",
        panther: "Боевой танк Panther",
        tiger: "Тяжелый танк Tiger",
        cargo_truck: "Грузовик",
        convec: "Транспорт ConVec",
        robo_miner: "Робот-Шахтер",
        robo_surveyor: "Геодезист",
        earthworker: "Землерой",
        evacuation_transport: "Эвакуационный Транспорт",
    },
    options: OptionsDict {
        title: "СИСТЕМНЫЕ НАСТРОЙКИ И ЗВУКОВАЯ МАТРИЦА",
        master_volume: "Общая громкость звукового вывода:",
        sim_velocity: "Масштабирование скорости симуляции:",
        crt_scanlines: "Ретро-эффект CRT-сканлайнов:",
        active_language: "Текущий язык / Локализация:",
        enabled: "ВКЛЮЧЕНО",
        disabled: "ОТКЛЮЧЕНО",
    },
    pause_menu: PauseMenuDict {
        title: "ТАКТИЧЕСКАЯ СИМУЛЯЦИЯ ПРИОСТАНОВЛЕНА",
        resume: "▶️ ПРОДОЛЖИТЬ МИССИЮ (ESC)",
        quicksave: "💾 БЫСТРОЕ СОХРАНЕНИЕ (F5)",
        quickload: "📂 БЫСТРАЯ ЗАГРУЗКА (F9)",
        save_load_manager: "🗄️ УПРАВЛЕНИЕ СОХРАНЕНИЯМИ",
        options: "⚙️ НАСТРОЙКИ ЗВУКА И СКОРОСТИ",
        restart: "🔄 НАЧАТЬ МИССИЮ ЗАНОВО",
        surrender: "💀 СДАТЬСЯ / ВЫЙТИ В МЕНЮ",
    },
    scenarios: [
        // Scenario 0: Eden 1
        BriefingScenarioDict {
            scenario_id: 0,
            title: "Операция «Новый рассвет»",
            subtitle: "Сектор Эдем Прайм // Заселение планеты",
            transcript: "Колонисты Фонда Эдем! После катастрофического крушения звездолета «Конестога» наш район высадки на Новой Терре должен быть немедленно укреплен. Вулканические датчики фиксируют приближение лавовых трещин, а рейдеры Плимута развернули агрессивные передовые заставы.\n\nОбеспечьте системы жизнеобеспечения, автоматизируйте перевозку руды, разверните рельсотронные танки и продвигайте научные исследования во имя выживания нашего вида.",
            primary_directives: &[
                "Накопить 3 000 т обычной руды и 600 т редкой руды",
                "Произвести и ввести в строй минимум 2 боевых танка",
                "Завершить научное исследование в любой ключевой дисциплине",
                "Защитить Командный Центр колонии от вражеского уничтожения",
            ],
            secondary_directives: &[
                "Обеспечить численность колонии в 120 жителей с высоким моральным духом",
            ],
            hazard_summary: "Локальные пылевые вихри; слабые сейсмические толчки.",
        },
        // Scenario 1: Plymouth 1
        BriefingScenarioDict {
            scenario_id: 1,
            title: "Операция «Авангард пустошей»",
            subtitle: "Сектор Плимут-Юг // Возвращение территорий",
            transcript: "Командиры Плимута! Отвергнув технократическую тиранию Эдема, наши колонисты создали дом в изрезанных ущельях пустошей. Разведывательные патрули Эдема посягают на наши геотермальные источники.\n\nПостройте Завод Машин, наладьте автоматические конвои рудовозов, соберите ударную группировку из 3 боевых танков и уничтожьте рейдеров Эдема для защиты нашей территории.",
            primary_directives: &[
                "Построить Завод Машин для выпуска бронированных шасси",
                "Сформировать ударный эскадрон минимум из 3 танков",
                "Добыть 2 500 т обычной руды для укрепления каньона",
                "Защитить Командный Центр Плимута от нападения Эдема",
            ],
            secondary_directives: &[
                "Поддерживать постоянный избыток энергии во время ночных циклов пустошей",
            ],
            hazard_summary: "Высокий риск вулканических толчков; выбросы геотермального газа.",
        },
        // Scenario 2: Tutorial
        BriefingScenarioDict {
            scenario_id: 2,
            title: "Полигон курсантов",
            subtitle: "Колониальная академия // Симулятор тактических операций",
            transcript: "Курсант! Добро пожаловать в Симулятор колониальных операций Новой Терры. В этом испытании вы освоите базовые основы управления планетарной базой.\n\nУбедитесь, что выработка энергии превышает потребление, проложите герметичные транспортные трубы между объектами и организуйте доставку руды на плавильню.",
            primary_directives: &[
                "Обеспечить положительный энергобаланс (Производство > Потребление)",
                "Соединить сооружения минимум 15 герметичными трубами",
                "Доставить первый груз обычной руды на Плавильню",
                "Удерживать моральный дух колонистов выше 90%",
            ],
            secondary_directives: &[
                "Развернуть автоматического Робота-Шахтера на рудном маяке",
            ],
            hazard_summary: "Контролируемая среда симулятора; низкая угроза.",
        },
        // Scenario 3: Eden 2
        BriefingScenarioDict {
            scenario_id: 3,
            title: "Операция «Бастион Кальдеры»",
            subtitle: "Подножие вулкана Вулкан // Локализация лавовых разломов",
            transcript: "Тревога командования Эдема! Подземные термодатчики фиксируют опасное смещение магмы вдоль северных разломов. Раскаленные трещины разрывают вечную мерзлоту, угрожая куполам колонии.\n\nСтабилизируйте энергосеть выше 400 кВт, постройте оборонительный периметр минимум из 2 Сторожевых Турелей вдоль разлома, накопите 2 500 т обычной руды и 500 т редкой руды для термозащитных баррикад и защитите Командный Центр.",
            primary_directives: &[
                "Создать геотермальную сеть сдерживания (Мощность >= 400 кВт)",
                "Построить минимум 2 Сторожевые Турели для охраны края разлома",
                "Накопить 2 500 т обычной руды и 500 т редкой руды для укреплений",
                "Защитить Командный Центр Эдема от сейсмического и вражеского разрушения",
            ],
            secondary_directives: &[
                "Поддерживать моральный дух населения на уровне не ниже 80%",
                "Завершить научное исследование в передовой дисциплине",
            ],
            hazard_summary: "Расширяющиеся лавовые разломы; наступающие потоки магмы.",
        },
        // Scenario 4: Plymouth 2
        BriefingScenarioDict {
            scenario_id: 4,
            title: "Операция «Железный клык»",
            subtitle: "Узловой перевал Рудного хребта // Саботаж плавильни и засада",
            transcript: "Депеша Высшего Совета Плимута: Технократы Эдема укрепили добывающий комплекс на границе наших южных каньонов, перекрыв поставки минералов для Плимута.\n\nСоберите бронетанковую группу минимум из 4 боевых танков с лазерами или рельсотронами, возведите Сторожевую Турель для контроля перевала, захватите и сохраните не менее 3 500 т обычной руды и 800 т редкой руды, и защитите Командный Центр.",
            primary_directives: &[
                "Сформировать бронегруппу для засады минимум из 4 танков",
                "Укрепить узкий проход каньона минимум 1 Сторожевой Турелью",
                "Накопить минимум 3 500 т обычной руды и 800 т редкой руды",
                "Защитить Командный Центр Плимута от контратак войск Эдема",
            ],
            secondary_directives: &[
                "Поддерживать работу Завода Машин для выпуска бронетехники",
                "Обеспечить постоянный избыток выработки энергии в сети",
            ],
            hazard_summary: "Укрепленные лазерные турели; опасные узкие каньоны.",
        },
        // Scenario 5: Eden 3
        BriefingScenarioDict {
            scenario_id: 5,
            title: "Эвакуация на звездолете",
            subtitle: "Кальдера вулкана Вулкан // Побег с вулкана Вулкан",
            transcript: "Экстренная трансляция Верховного командования Эдема! Кальдера вулкана переживает катастрофическое супервулканическое извержение. Потоки магмы отрезали пути к югу, а отряды Плимута наступают на космодром, чтобы захватить эвакуационный корабль.\n\nЗапустите пусковой комплекс, соберите все 4 модуля звездолета (Командный, Криостазиса, Жилой отсек, Ионный двигатель), перевезите не менее 150 колонистов на транспортах эвакуации и удерживайте оборону до старта корабля.",
            primary_directives: &[
                "Собрать модули звездолета (Командный, Криостазис, Жилой, Ионный двигатель)",
                "Эвакуировать не менее 150 жителей на Эвакуационных Транспортах",
                "Защитить колонию от атак рейдеров Плимута и наступающей лавы",
                "Защитить Командный Центр Эдема от уничтожения",
            ],
            secondary_directives: &[
                "Удерживать моральный дух выше 85% до момента старта",
                "Накопить 3 000 т обычной руды и 1 000 т редкой руды для запасов топлива",
            ],
            hazard_summary: "Катаклизмическое извержение вулкана; быстротекущие реки магмы.",
        },
        // Scenario 6: Plymouth 3
        BriefingScenarioDict {
            scenario_id: 6,
            title: "Операция «Феникс: Обсидиановые скалы»",
            subtitle: "Обсидиановые скалы // Штурм цитадели и оборона терраформера",
            transcript: "Депеша Совета Свободных Поселенцев Плимута: Атмосферный процессор в Обсидиановых скалах — последняя надежда на пригодный для дыхания воздух Новой Терры. Однако Цитадель Эдема заблокировала сектор, а нанитовая зараза подбирается к вентиляционным шахтам процессора.\n\nРазверните тяжелые танки, защитите Атмосферный Терраформер от нанитовой порчи до достижения 100% насыщения и штурмуйте Цитадель Эдема, чтобы нейтрализовать их башни с рельсотронами и дивизии танков Tiger.",
            primary_directives: &[
                "Защитить Атмосферный Терраформер от распространения нанитовой заразы",
                "Осуществить штурм укрепленной Обсидиановой Цитадели Эдема",
                "Достичь 100% насыщения атмосферы через Терраформер",
                "Защитить Командный Центр Плимута от уничтожения",
            ],
            secondary_directives: &[
                "Уничтожить все вражеские Сторожевые Турели с рельсотронами в скалах",
            ],
            hazard_summary: "Микробиологическая нанитовая зараза; мощные крепости с рельсотронами.",
        },
    ],
};

// =========================================================================
// 5. Public Resolution & Switching API
// =========================================================================

/// Retrieve static reference to the compiled dictionary for the given Language.
pub fn get_dictionary(lang: Language) -> &'static Translation {
    match lang {
        Language::English => &TRANSLATION_EN,
        Language::Portuguese => &TRANSLATION_PT,
        Language::Spanish => &TRANSLATION_ES,
        Language::German => &TRANSLATION_DE,
        Language::Chinese => &TRANSLATION_ZH,
        Language::Russian => &TRANSLATION_RU,
    }
}

/// Retrieve translation by string code/tag, with automatic fallback to English.
#[allow(dead_code)]
pub fn get_translation(lang_code: &str) -> Translation {
    let lang = Language::from_code(lang_code);
    *get_dictionary(lang)
}

/// Retrieve static reference to a scenario briefing by scenario ID and language.
#[allow(dead_code)]
pub fn get_scenario_briefing(scenario_id: usize, lang: Language) -> &'static BriefingScenarioDict {
    get_dictionary(lang).scenario(scenario_id)
}

/// Apply localized text across all DOM chrome headers, status gauges, action buttons, and modals.
pub fn apply_language(lang_code: &str, doc: &Document) {
    let lang = Language::from_code(lang_code);
    let t = get_dictionary(lang);

    // Top telemetry chrome labels
    if let Some(el) = doc.get_element_by_id("lbl-faction") { el.set_text_content(Some(t.faction)); }
    if let Some(el) = doc.get_element_by_id("lbl-sol") { el.set_text_content(Some(t.sol)); }
    if let Some(el) = doc.get_element_by_id("lbl-sun") { el.set_text_content(Some(t.sun)); }
    if let Some(el) = doc.get_element_by_id("lbl-pop") { el.set_text_content(Some(t.pop)); }
    if let Some(el) = doc.get_element_by_id("lbl-power") { el.set_text_content(Some(t.power)); }
    if let Some(el) = doc.get_element_by_id("lbl-ore") { el.set_text_content(Some(t.common_ore)); }
    if let Some(el) = doc.get_element_by_id("lbl-rare") { el.set_text_content(Some(t.rare_ore)); }
    if let Some(el) = doc.get_element_by_id("lbl-food") { el.set_text_content(Some(t.food)); }
    if let Some(el) = doc.get_element_by_id("lbl-starship") { el.set_text_content(Some(&format!("{}:", t.hud.starship))); }

    // Bottom action bar controls
    if let Some(el) = doc.get_element_by_id("btn-build") { el.set_text_content(Some(t.build_base)); }
    if let Some(el) = doc.get_element_by_id("btn-units") { el.set_text_content(Some(t.units)); }
    if let Some(el) = doc.get_element_by_id("btn-research") { el.set_text_content(Some(t.research)); }
    if let Some(el) = doc.get_element_by_id("btn-multiplayer") { el.set_text_content(Some(&format!("📡 {}", t.main_menu.multiplayer))); }
    if let Some(el) = doc.get_element_by_id("btn-save") { el.set_text_content(Some(t.save)); }
    if let Some(el) = doc.get_element_by_id("btn-load") { el.set_text_content(Some(t.load)); }
    if let Some(el) = doc.get_element_by_id("btn-briefing") { el.set_text_content(Some(t.briefing)); }
    if let Some(el) = doc.get_element_by_id("btn-music") { el.set_text_content(Some(t.music)); }
    if let Some(el) = doc.get_element_by_id("btn-pause") { el.set_text_content(Some(&format!("⏸️ {}", t.main_menu.options))); }

    // Tactical Radar Minimap
    if let Some(el) = doc.get_element_by_id("minimap-title") { el.set_text_content(Some(t.radar)); }
    if let Some(el) = doc.get_element_by_id("radar-sweep") { el.set_text_content(Some(t.hud.radar_active)); }

    // Briefing Modal buttons
    if let Some(el) = doc.get_element_by_id("btn-ack-briefing") { el.set_text_content(Some(t.main_menu.begin_operation)); }

    // Multiplayer Modal buttons
    if let Some(el) = doc.get_element_by_id("btn-net-host") { el.set_text_content(Some(t.multiplayer.host)); }
    if let Some(el) = doc.get_element_by_id("btn-copy-offer") { el.set_text_content(Some(t.multiplayer.copy_offer)); }
    if let Some(el) = doc.get_element_by_id("btn-copy-answer") { el.set_text_content(Some(t.multiplayer.copy_answer)); }
    if let Some(el) = doc.get_element_by_id("btn-accept-answer") { el.set_text_content(Some(t.multiplayer.lock_connection)); }
    if let Some(el) = doc.get_element_by_id("btn-net-join") { el.set_text_content(Some(t.multiplayer.generate_answer)); }

    // Synchronize HTML <select id="lang-select"> value if needed
    if let Some(el) = doc.get_element_by_id("lang-select") {
        if let Ok(sel) = el.dyn_into::<web_sys::HtmlSelectElement>() {
            let tag = lang.tag();
            if sel.value() != tag {
                sel.set_value(tag);
            }
        }
    }
}

// =========================================================================
// 6. Exhaustive Unit Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_enum_variants_and_codes() {
        let langs = Language::all();
        assert_eq!(langs.len(), 6);

        assert_eq!(Language::English.code(), "EN");
        assert_eq!(Language::Portuguese.code(), "PT-BR");
        assert_eq!(Language::Spanish.code(), "ES");
        assert_eq!(Language::German.code(), "DE");
        assert_eq!(Language::Chinese.code(), "ZH-CN");
        assert_eq!(Language::Russian.code(), "RU");

        assert_eq!(Language::English.tag(), "en");
        assert_eq!(Language::Portuguese.tag(), "pt-BR");
        assert_eq!(Language::Spanish.tag(), "es");
        assert_eq!(Language::German.tag(), "de");
        assert_eq!(Language::Chinese.tag(), "zh-CN");
        assert_eq!(Language::Russian.tag(), "ru");

        assert_eq!(Language::English.native_name(), "English");
        assert_eq!(Language::Portuguese.native_name(), "Português");
        assert_eq!(Language::Spanish.native_name(), "Español");
        assert_eq!(Language::German.native_name(), "Deutsch");
        assert_eq!(Language::Chinese.native_name(), "简体中文");
        assert_eq!(Language::Russian.native_name(), "Русский");
    }

    #[test]
    fn test_language_code_and_tag_parsing() {
        assert_eq!(Language::from_code_or_tag("en"), Some(Language::English));
        assert_eq!(Language::from_code_or_tag("EN"), Some(Language::English));
        assert_eq!(Language::from_code_or_tag("English"), Some(Language::English));

        assert_eq!(Language::from_code_or_tag("pt-BR"), Some(Language::Portuguese));
        assert_eq!(Language::from_code_or_tag("pt"), Some(Language::Portuguese));
        assert_eq!(Language::from_code_or_tag("português"), Some(Language::Portuguese));

        assert_eq!(Language::from_code_or_tag("es"), Some(Language::Spanish));
        assert_eq!(Language::from_code_or_tag("ES"), Some(Language::Spanish));
        assert_eq!(Language::from_code_or_tag("español"), Some(Language::Spanish));

        assert_eq!(Language::from_code_or_tag("de"), Some(Language::German));
        assert_eq!(Language::from_code_or_tag("DE"), Some(Language::German));
        assert_eq!(Language::from_code_or_tag("deutsch"), Some(Language::German));

        assert_eq!(Language::from_code_or_tag("zh-CN"), Some(Language::Chinese));
        assert_eq!(Language::from_code_or_tag("zh"), Some(Language::Chinese));
        assert_eq!(Language::from_code_or_tag("中文"), Some(Language::Chinese));

        assert_eq!(Language::from_code_or_tag("ru"), Some(Language::Russian));
        assert_eq!(Language::from_code_or_tag("RU"), Some(Language::Russian));
        assert_eq!(Language::from_code_or_tag("русский"), Some(Language::Russian));

        assert_eq!(Language::from_code_or_tag("invalid"), None);
        assert_eq!(Language::from_code_or_tag(""), None);
    }

    #[test]
    fn test_language_fallback_resolution() {
        assert_eq!(Language::from_code("klingon"), Language::English);
        assert_eq!(Language::from_code(""), Language::English);
        assert_eq!(Language::from_code("de"), Language::German);

        let t_invalid = get_translation("unknown_locale");
        assert_eq!(t_invalid.language, Language::English);

        let t_valid = get_translation("ru");
        assert_eq!(t_valid.language, Language::Russian);
    }

    #[test]
    fn test_dictionary_completeness_all_languages() {
        for &lang in Language::all() {
            let d = get_dictionary(lang);

            // Legacy top-level fields
            assert!(!d.faction.is_empty(), "Missing faction in {:?}", lang);
            assert!(!d.sol.is_empty(), "Missing sol in {:?}", lang);
            assert!(!d.sun.is_empty(), "Missing sun in {:?}", lang);
            assert!(!d.pop.is_empty(), "Missing pop in {:?}", lang);
            assert!(!d.power.is_empty(), "Missing power in {:?}", lang);
            assert!(!d.common_ore.is_empty(), "Missing common_ore in {:?}", lang);
            assert!(!d.rare_ore.is_empty(), "Missing rare_ore in {:?}", lang);
            assert!(!d.food.is_empty(), "Missing food in {:?}", lang);
            assert!(!d.build_base.is_empty(), "Missing build_base in {:?}", lang);
            assert!(!d.units.is_empty(), "Missing units in {:?}", lang);
            assert!(!d.research.is_empty(), "Missing research in {:?}", lang);
            assert!(!d.save.is_empty(), "Missing save in {:?}", lang);
            assert!(!d.load.is_empty(), "Missing load in {:?}", lang);
            assert!(!d.radar.is_empty(), "Missing radar in {:?}", lang);
            assert!(!d.briefing.is_empty(), "Missing briefing in {:?}", lang);
            assert!(!d.music.is_empty(), "Missing music in {:?}", lang);

            // Main Menu
            assert!(!d.main_menu.new_campaign.is_empty());
            assert!(!d.main_menu.campaign_missions.is_empty());
            assert!(!d.main_menu.skirmish.is_empty());
            assert!(!d.main_menu.skirmish_battle.is_empty());
            assert!(!d.main_menu.multiplayer.is_empty());
            assert!(!d.main_menu.multiplayer_lobby.is_empty());
            assert!(!d.main_menu.load_game.is_empty());
            assert!(!d.main_menu.load_archive.is_empty());
            assert!(!d.main_menu.options.is_empty());
            assert!(!d.main_menu.options_audio_velocity.is_empty());
            assert!(!d.main_menu.resume.is_empty());
            assert!(!d.main_menu.resume_operation.is_empty());
            assert!(!d.main_menu.quit.is_empty());
            assert!(!d.main_menu.back.is_empty());
            assert!(!d.main_menu.begin_operation.is_empty());
            assert!(!d.main_menu.apply_and_return.is_empty());

            // Skirmish
            assert!(!d.skirmish.title.is_empty());
            assert!(!d.skirmish.map_selection.is_empty());
            assert!(!d.skirmish.player_faction.is_empty());
            assert!(!d.skirmish.personality_aggressive_rusher.is_empty());
            assert!(!d.skirmish.personality_defensive_fortifier.is_empty());
            assert!(!d.skirmish.personality_economic_tech_rusher.is_empty());
            assert!(!d.skirmish.difficulty_easy.is_empty());
            assert!(!d.skirmish.difficulty_medium.is_empty());
            assert!(!d.skirmish.difficulty_hard.is_empty());
            assert!(!d.skirmish.abundance_sparse.is_empty());
            assert!(!d.skirmish.abundance_standard.is_empty());
            assert!(!d.skirmish.abundance_abundant.is_empty());
            assert!(!d.skirmish.abundance_infinite.is_empty());
            assert!(!d.skirmish.disaster_disabled.is_empty());
            assert!(!d.skirmish.disaster_low.is_empty());
            assert!(!d.skirmish.disaster_standard.is_empty());
            assert!(!d.skirmish.disaster_catastrophic.is_empty());
            assert!(!d.skirmish.launch.is_empty());

            // Multiplayer
            assert!(!d.multiplayer.title.is_empty());
            assert!(!d.multiplayer.host.is_empty());
            assert!(!d.multiplayer.guest.is_empty());
            assert!(!d.multiplayer.copy_offer.is_empty());
            assert!(!d.multiplayer.paste_answer.is_empty());
            assert!(!d.multiplayer.generate_answer.is_empty());
            assert!(!d.multiplayer.peer_connected.is_empty());
            assert!(!d.multiplayer.peer_disconnected.is_empty());
            assert!(!d.multiplayer.launch_match.is_empty());

            // HUD
            assert!(!d.hud.grid_optimal.is_empty());
            assert!(!d.hud.grid_brownout.is_empty());
            assert!(!d.hud.grid_blackout.is_empty());
            assert!(!d.hud.food_surplus.is_empty());
            assert!(!d.hud.food_adequate.is_empty());
            assert!(!d.hud.food_rationing.is_empty());
            assert!(!d.hud.food_starvation.is_empty());
            assert!(!d.hud.morale_excellent.is_empty());
            assert!(!d.hud.morale_good.is_empty());
            assert!(!d.hud.morale_fair.is_empty());
            assert!(!d.hud.morale_poor.is_empty());
            assert!(!d.hud.morale_rioting.is_empty());
            assert!(!d.hud.factory_title.is_empty());
            assert!(!d.hud.research_title.is_empty());

            // Debriefing
            assert!(!d.debriefing.rank_supreme_savior_title.is_empty());
            assert!(!d.debriefing.rank_supreme_savior_badge.is_empty());
            assert!(!d.debriefing.rank_supreme_savior_desc.is_empty());
            assert!(!d.debriefing.rank_distinguished_pioneer_title.is_empty());
            assert!(!d.debriefing.rank_distinguished_pioneer_badge.is_empty());
            assert!(!d.debriefing.rank_distinguished_pioneer_desc.is_empty());
            assert!(!d.debriefing.rank_tethys_survivor_title.is_empty());
            assert!(!d.debriefing.rank_tethys_survivor_badge.is_empty());
            assert!(!d.debriefing.rank_tethys_survivor_desc.is_empty());
            assert!(!d.debriefing.rank_colony_failure_title.is_empty());
            assert!(!d.debriefing.rank_colony_failure_badge.is_empty());
            assert!(!d.debriefing.rank_colony_failure_desc.is_empty());
            assert!(!d.debriefing.victory_title.is_empty());
            assert!(!d.debriefing.defeat_title.is_empty());
            assert!(!d.debriefing.colony_duration.is_empty());
            assert!(!d.debriefing.evacuation_readiness.is_empty());
            assert!(!d.debriefing.survivors_morale.is_empty());
            assert!(!d.debriefing.refined_reserves.is_empty());
            assert!(!d.debriefing.score_label.is_empty());
            assert!(!d.debriefing.directives_checklist.is_empty());

            // Unit & Building designations
            assert!(!d.units_buildings.command_center.is_empty());
            assert!(!d.units_buildings.solar_array.is_empty());
            assert!(!d.units_buildings.tokamak_reactor.is_empty());
            assert!(!d.units_buildings.geothermal_plant.is_empty());
            assert!(!d.units_buildings.common_ore_smelter.is_empty());
            assert!(!d.units_buildings.rare_ore_smelter.is_empty());
            assert!(!d.units_buildings.dirt_lab.is_empty());
            assert!(!d.units_buildings.standard_lab.is_empty());
            assert!(!d.units_buildings.residence.is_empty());
            assert!(!d.units_buildings.agrodome.is_empty());
            assert!(!d.units_buildings.vehicle_factory.is_empty());
            assert!(!d.units_buildings.repair_facility.is_empty());
            assert!(!d.units_buildings.guard_post.is_empty());
            assert!(!d.units_buildings.spaceport.is_empty());
            assert!(!d.units_buildings.terraformer.is_empty());

            assert!(!d.units_buildings.lynx.is_empty());
            assert!(!d.units_buildings.panther.is_empty());
            assert!(!d.units_buildings.tiger.is_empty());
            assert!(!d.units_buildings.cargo_truck.is_empty());
            assert!(!d.units_buildings.convec.is_empty());
            assert!(!d.units_buildings.robo_miner.is_empty());
            assert!(!d.units_buildings.robo_surveyor.is_empty());
            assert!(!d.units_buildings.earthworker.is_empty());
            assert!(!d.units_buildings.evacuation_transport.is_empty());

            // Options & Pause Menu
            assert!(!d.options.title.is_empty());
            assert!(!d.options.master_volume.is_empty());
            assert!(!d.pause_menu.title.is_empty());
            assert!(!d.pause_menu.resume.is_empty());
            assert!(!d.pause_menu.quicksave.is_empty());
            assert!(!d.pause_menu.quickload.is_empty());
            assert!(!d.pause_menu.restart.is_empty());
            assert!(!d.pause_menu.surrender.is_empty());

            // All 7 Scenarios
            assert_eq!(d.scenarios.len(), 7);
            for sc in &d.scenarios {
                assert!(!sc.title.is_empty(), "Empty title in sc {} lang {:?}", sc.scenario_id, lang);
                assert!(!sc.subtitle.is_empty(), "Empty subtitle in sc {} lang {:?}", sc.scenario_id, lang);
                assert!(!sc.transcript.is_empty(), "Empty transcript in sc {} lang {:?}", sc.scenario_id, lang);
                assert!(!sc.hazard_summary.is_empty(), "Empty hazard in sc {} lang {:?}", sc.scenario_id, lang);
                assert!(!sc.primary_directives.is_empty(), "No primary directives in sc {} lang {:?}", sc.scenario_id, lang);
                for dir in sc.primary_directives {
                    assert!(!dir.is_empty(), "Empty directive in sc {} lang {:?}", sc.scenario_id, lang);
                }
            }
        }
    }

    #[test]
    fn test_briefing_scenario_retrieval_all_languages() {
        for &lang in Language::all() {
            for sc_id in 0..7 {
                let sc = get_scenario_briefing(sc_id, lang);
                assert_eq!(sc.scenario_id, sc_id);
                assert!(!sc.title.is_empty());
                assert!(sc.primary_directives.len() >= 3);
            }
        }
    }

    #[test]
    fn test_structure_and_vehicle_translation_integrity() {
        for &lang in Language::all() {
            let d = get_dictionary(lang);

            let cc = d.structure_name(&crate::sim::StructureType::CommandCenter);
            assert!(!cc.is_empty());

            let sp = d.structure_name(&crate::sim::StructureType::Spaceport);
            assert!(!sp.is_empty());

            let lynx = d.vehicle_name(&crate::render::hud::VehicleHull::Lynx);
            assert!(!lynx.is_empty());

            let tiger = d.vehicle_name(&crate::render::hud::VehicleHull::Tiger);
            assert!(!tiger.is_empty());
        }
    }

    #[test]
    fn test_hud_and_debriefing_rank_translations() {
        for &lang in Language::all() {
            let d = get_dictionary(lang);

            let grid_opt = d.energy_grid_status(crate::render::hud::EnergyGridState::Normal);
            assert!(!grid_opt.is_empty());

            let morale_good = d.morale_tier(crate::render::hud::MoraleTier::Good);
            assert!(!morale_good.is_empty());

            let sust_surplus = d.sustenance_state(crate::render::hud::SustenanceState::Surplus);
            assert!(!sust_surplus.is_empty());

            let rank_title = d.colonial_rank_title(crate::sim::debrief::ColonialRank::SupremeColonySavior);
            assert!(!rank_title.is_empty());

            let rank_badge = d.colonial_rank_badge(crate::sim::debrief::ColonialRank::SupremeColonySavior);
            assert!(!rank_badge.is_empty());

            let rank_desc = d.colonial_rank_description(crate::sim::debrief::ColonialRank::ColonyFailure);
            assert!(!rank_desc.is_empty());
        }
    }
}
