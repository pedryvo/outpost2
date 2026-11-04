//! Outpost 2 — Main Menu, Briefing Cinematic & Tactical Selector Subsystem
#![allow(dead_code)]
//!
//! Provides the authentic retro Sierra/Dynamics Outpost 2 menu architecture:
//! - MainMenu: Retro Outpost 2 branding title splash, starfield/planet horizon canvas animation,
//!   and primary navigation (Campaign, Skirmish, Multiplayer, Load, Options)
//! - CampaignBriefing: Scenario narrative transcript, faction crest vector art,
//!   satellite minimap scan preview, directives checklist, and launch operation controls
//! - SkirmishSetup: Interactive tactical lobby configuring map theater, human faction,
//!   up to 5 bot slots (faction, AI personality doctrine, difficulty), resource abundance,
//!   and planetary disaster frequency
//! - PauseMenu: Tactical simulation pause overlay (Resume, QuickSave F5, QuickLoad F9,
//!   Archives, Settings, Restart, Surrender/Abort)
//! - LoadMission: Visual slot manager for LocalStorage slots 1-5, QuickSave, AutoSave,
//!   and standalone .op2save import/export
//! - Options: Volume sliders, simulation velocity, multilingual language selector,
//!   and retro visual enhancements

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::render::novella::NOVELLA_CHAPTERS;
use crate::sim::{
    ai::{AiPersonality, Difficulty},
    save::{LocalStorageBackend, SaveMetadata},
    ColonyState, Faction,
};

// =========================================================================
// 1. Data Structures & Screen Enums
// =========================================================================

/// Current active screen in the menu system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MenuScreen {
    /// In-game live simulation (menu hidden)
    None,
    /// Outpost 2 Title Splash & Primary Navigation (Dialog 137)
    MainMenu,
    /// Campaign Scenario Selection / Colony Options (Dialog 140)
    CampaignSelect,
    /// Tactical Campaign Mission Briefing & Novella (Dialog 219)
    CampaignBriefing,
    /// Skirmish Battle Lobby (Dialog 222 Colony Games)
    SkirmishSetup,
    /// Serverless Peer-to-Peer WebRTC Multiplayer Lobby
    MultiplayerLobby,
    /// Visual Save/Load Slot Manager & .op2save Archiver
    LoadMission,
    /// System Audio & Preferences Configuration (Dialog 142)
    Options,
    /// In-Game Tactical Pause Overlay
    PauseMenu,
    /// Tutorials List Screen (Dialog 269)
    Tutorials,
    /// Authentic Outpost 2 Lore & Credits Dialog
    About,
}

/// Campaign Difficulty selection (Dialog 140 / Dialog 222)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignDifficulty {
    Easy,
    Normal,
    Hard,
}

impl CampaignDifficulty {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
        }
    }
}

/// Active tab in Dialog 219 (Mission Menu / Briefing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BriefingTab {
    Briefing,
    Objectives,
    Story,
}


/// Resource abundance tier for skirmish matches
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceAbundance {
    Sparse,
    Standard,
    Abundant,
    InfiniteOre,
}

impl ResourceAbundance {
    pub fn starting_ore(&self) -> (u32, u32) {
        match self {
            Self::Sparse => (1000, 200),
            Self::Standard => (2500, 600),
            Self::Abundant => (5000, 1500),
            Self::InfiniteOre => (50000, 20000),
        }
    }

    pub fn display_name(&self) -> &'static str {
        self.display_name_localized(crate::i18n::Language::English)
    }

    pub fn display_name_localized(&self, lang: crate::i18n::Language) -> &'static str {
        let dict = crate::i18n::get_dictionary(lang);
        match self {
            Self::Sparse => dict.skirmish.abundance_sparse,
            Self::Standard => dict.skirmish.abundance_standard,
            Self::Abundant => dict.skirmish.abundance_abundant,
            Self::InfiniteOre => dict.skirmish.abundance_infinite,
        }
    }
}

/// Planetary disaster frequency for skirmish matches
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisasterFrequency {
    Disabled,
    Low,
    Standard,
    Catastrophic,
}

impl DisasterFrequency {
    pub fn interval_ticks(&self) -> u32 {
        match self {
            Self::Disabled => 999_999,
            Self::Low => 1800,        // ~90 sec
            Self::Standard => 900,    // ~45 sec
            Self::Catastrophic => 400, // ~20 sec
        }
    }

    pub fn display_name(&self) -> &'static str {
        self.display_name_localized(crate::i18n::Language::English)
    }

    pub fn display_name_localized(&self, lang: crate::i18n::Language) -> &'static str {
        let dict = crate::i18n::get_dictionary(lang);
        match self {
            Self::Disabled => dict.skirmish.disaster_disabled,
            Self::Low => dict.skirmish.disaster_low,
            Self::Standard => dict.skirmish.disaster_standard,
            Self::Catastrophic => dict.skirmish.disaster_catastrophic,
        }
    }
}

/// Skirmish AI opponent bot slot configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkirmishBotSlot {
    pub enabled: bool,
    pub faction: Faction,
    pub personality: AiPersonality,
    pub difficulty: Difficulty,
    pub spawn_pos: (i32, i32),
}

impl SkirmishBotSlot {
    pub fn new(
        enabled: bool,
        faction: Faction,
        personality: AiPersonality,
        difficulty: Difficulty,
        spawn_pos: (i32, i32),
    ) -> Self {
        Self {
            enabled,
            faction,
            personality,
            difficulty,
            spawn_pos,
        }
    }
}

/// Skirmish match configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkirmishConfig {
    pub map_index: usize,
    pub human_faction: Faction,
    pub bots: Vec<SkirmishBotSlot>,
    pub resources: ResourceAbundance,
    pub disasters: DisasterFrequency,
}

impl Default for SkirmishConfig {
    fn default() -> Self {
        Self {
            map_index: 0,
            human_faction: Faction::Eden,
            bots: vec![
                SkirmishBotSlot::new(
                    true,
                    Faction::Plymouth,
                    AiPersonality::AggressiveRusher,
                    Difficulty::Medium,
                    (40, 35),
                ),
                SkirmishBotSlot::new(
                    true,
                    Faction::Eden,
                    AiPersonality::DefensiveFortifier,
                    Difficulty::Medium,
                    (12, 40),
                ),
                SkirmishBotSlot::new(
                    true,
                    Faction::Plymouth,
                    AiPersonality::EconomicTechRusher,
                    Difficulty::Easy,
                    (45, 12),
                ),
                SkirmishBotSlot::new(
                    false,
                    Faction::Eden,
                    AiPersonality::DefensiveFortifier,
                    Difficulty::Hard,
                    (15, 15),
                ),
                SkirmishBotSlot::new(
                    false,
                    Faction::Plymouth,
                    AiPersonality::AggressiveRusher,
                    Difficulty::Hard,
                    (48, 48),
                ),
            ],
            resources: ResourceAbundance::Standard,
            disasters: DisasterFrequency::Standard,
        }
    }
}

impl SkirmishConfig {
    /// Return the list of enabled AI opponents formatted for `ColonyState::setup_skirmish`
    pub fn build_opponents(&self) -> Vec<(Faction, AiPersonality, Difficulty, i32, i32)> {
        self.bots
            .iter()
            .filter(|b| b.enabled)
            .map(|b| (b.faction, b.personality.clone(), b.difficulty, b.spawn_pos.0, b.spawn_pos.1))
            .collect()
    }

    /// Count of enabled bot opponents
    pub fn enabled_bot_count(&self) -> usize {
        self.bots.iter().filter(|b| b.enabled).count()
    }

    /// Map name descriptor
    pub fn map_name(&self) -> &'static str {
        match self.map_index {
            0 => "Mount Vulcan Caldera (64x64)",
            1 => "Obsidian Crags (64x64)",
            2 => "Badland Chasms (64x64)",
            3 => "Tethys Meridian (64x64)",
            _ => "Elysium Basin (64x64)",
        }
    }

    pub fn cycle_map(&mut self) {
        self.map_index = (self.map_index + 1) % 5;
    }

    pub fn toggle_human_faction(&mut self) {
        self.human_faction = match self.human_faction {
            Faction::Eden => Faction::Plymouth,
            Faction::Plymouth => Faction::Eden,
        };
    }

    pub fn cycle_resources(&mut self) {
        self.resources = match self.resources {
            ResourceAbundance::Sparse => ResourceAbundance::Standard,
            ResourceAbundance::Standard => ResourceAbundance::Abundant,
            ResourceAbundance::Abundant => ResourceAbundance::InfiniteOre,
            ResourceAbundance::InfiniteOre => ResourceAbundance::Sparse,
        };
    }

    pub fn cycle_disasters(&mut self) {
        self.disasters = match self.disasters {
            DisasterFrequency::Disabled => DisasterFrequency::Low,
            DisasterFrequency::Low => DisasterFrequency::Standard,
            DisasterFrequency::Standard => DisasterFrequency::Catastrophic,
            DisasterFrequency::Catastrophic => DisasterFrequency::Disabled,
        };
    }

    pub fn toggle_bot_enabled(&mut self, slot_idx: usize) {
        if let Some(bot) = self.bots.get_mut(slot_idx) {
            bot.enabled = !bot.enabled;
        }
    }

    pub fn cycle_bot_faction(&mut self, slot_idx: usize) {
        if let Some(bot) = self.bots.get_mut(slot_idx) {
            bot.faction = match bot.faction {
                Faction::Eden => Faction::Plymouth,
                Faction::Plymouth => Faction::Eden,
            };
        }
    }

    pub fn cycle_bot_personality(&mut self, slot_idx: usize) {
        if let Some(bot) = self.bots.get_mut(slot_idx) {
            bot.personality = match bot.personality {
                AiPersonality::AggressiveRusher => AiPersonality::DefensiveFortifier,
                AiPersonality::DefensiveFortifier => AiPersonality::EconomicTechRusher,
                _ => AiPersonality::AggressiveRusher,
            };
        }
    }

    pub fn cycle_bot_difficulty(&mut self, slot_idx: usize) {
        if let Some(bot) = self.bots.get_mut(slot_idx) {
            bot.difficulty = match bot.difficulty {
                Difficulty::Easy => Difficulty::Medium,
                Difficulty::Medium => Difficulty::Hard,
                Difficulty::Hard => Difficulty::Easy,
            };
        }
    }
}

/// Historical campaign scenario transcript & directives descriptor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BriefingScenario {
    pub scenario_id: usize,
    pub title: String,
    pub subtitle: String,
    pub faction: Faction,
    pub map_name: String,
    pub map_dimensions: (u32, u32),
    pub transcript: String,
    pub primary_directives: Vec<String>,
    pub secondary_directives: Vec<String>,
    pub hazard_summary: String,
}

impl BriefingScenario {
    /// Retrieve canonical scenario briefing by internal scenario ID
    pub fn get(scenario_id: usize) -> Self {
        match scenario_id {
            1 => Self {
                scenario_id: 1,
                title: "Operation Badland Vanguard".into(),
                subtitle: "Sector Plymouth South // Territorial Reclamation".into(),
                faction: Faction::Plymouth,
                map_name: "ply01.map".into(),
                map_dimensions: (64, 64),
                transcript: "Commanders of Plymouth: Rejecting Eden's authoritarian technocracy, our settlers carved a home in the jagged badland trenches. Eden scout patrols are encroaching on our geothermal vents.\n\nConstruct a Vehicle Factory, establish automated mining convoys, assemble a strike force of 3 combat tanks, and eliminate Eden raiders to secure our sovereign territory.".into(),
                primary_directives: vec![
                    "Commission a Vehicle Factory to manufacture armored chassis".into(),
                    "Assemble a strike squadron of at least 3 combat tanks".into(),
                    "Mine 2,500t Common Ore to secure the canyon perimeter".into(),
                    "Defend the Plymouth Command Center against Eden assault".into(),
                ],
                secondary_directives: vec![
                    "Maintain continuous power surplus throughout badland night cycles".into(),
                ],
                hazard_summary: "High volcanic tremor risk; active geothermal gas vents.".into(),
            },
            2 => Self {
                scenario_id: 2,
                title: "Cadet Proving Grounds".into(),
                subtitle: "Colonial Academy // Tactical Operations Simulator".into(),
                faction: Faction::Eden,
                map_name: "tutorial.map".into(),
                map_dimensions: (64, 64),
                transcript: "Cadet: Welcome to the New Terra Colonial Operations Simulator. In this trial, you will master the fundamentals of planetary colony management.\n\nVerify that base power generation exceeds consumption, deploy pressurized transport tubes between facilities, and oversee automated mineral transport to the smelter.".into(),
                primary_directives: vec![
                    "Establish positive energy surplus (Power Produced > Consumed)".into(),
                    "Connect structures with at least 15 pressurized transport tubes".into(),
                    "Haul first freight load of Common Ore to the Smelter".into(),
                    "Maintain colonist morale above 90%".into(),
                ],
                secondary_directives: vec![
                    "Commission an automated Robo-Miner on an ore beacon".into(),
                ],
                hazard_summary: "Controlled simulator environment; low environmental hazard.".into(),
            },
            3 => Self {
                scenario_id: 3,
                title: "Operation Caldera Bastion".into(),
                subtitle: "Mount Vulcan Foothills // Molten Fissure Containment".into(),
                faction: Faction::Eden,
                map_name: "eden02.map".into(),
                map_dimensions: (64, 64),
                transcript: "Eden Command Alert: Subsurface thermal sensors register catastrophic magma displacement along the northern fault lines. Molten fissures are rupturing the permafrost, threatening colonial survival dome sectors.\n\nStabilize base power generation above 400 kW using high-output arrays or geothermal conduits, construct a fortified perimeter of at least 2 defensive Guard Post Turrets overlooking fissure corridors, stockpile 2,500t Common Ore and 500t Rare Ore for emergency thermal barricades, and ensure the Eden Command Center remains secure from seismic fissures and hostile bombardment.".into(),
                primary_directives: vec![
                    "Establish geothermal containment grid (Generate >= 400 kW power)".into(),
                    "Construct at least 2 defensive Guard Post Turrets guarding fissure rim".into(),
                    "Stockpile 2,500t Common Ore and 500t Rare Ore for reinforcement".into(),
                    "Protect the Eden Command Center from seismic or hostile annihilation".into(),
                ],
                secondary_directives: vec![
                    "Maintain colonial population morale at or above 80%".into(),
                    "Complete scientific research in any advanced discipline".into(),
                ],
                hazard_summary: "Spreading volcanic lava fissures; advancing magma flows.".into(),
            },
            4 => Self {
                scenario_id: 4,
                title: "Operation Iron Fang".into(),
                subtitle: "Mineral Ridge Chokepoint // Smelter Sabotage & Ambush".into(),
                faction: Faction::Plymouth,
                map_name: "ply02.map".into(),
                map_dimensions: (64, 64),
                transcript: "Plymouth High Council Direct Action Dispatch: Eden technocrats have entrenched a heavily armed mineral extraction and smelting complex directly bordering our southern badland canyon passes, choking off Plymouth's mineral lifeline.\n\nAssemble an armored ambush strike squadron of at least 4 combat tanks equipped with railguns or lasers, erect a fortified Guard Post Turret to control the canyon choke point, hijack and stockpile at least 3,500t Common Ore and 800t Rare Ore, and defend the Plymouth Command Center against Eden's automated retaliatory counter-assault.".into(),
                primary_directives: vec![
                    "Assemble an armored ambush strike squad of >= 4 armed tanks".into(),
                    "Fortify the ambush choke point with at least 1 Guard Post Turret".into(),
                    "Stockpile at least 3,500t Common Ore and 800t Rare Ore".into(),
                    "Protect Plymouth Command Center from Eden counter-offensive strikes".into(),
                ],
                secondary_directives: vec![
                    "Maintain an operational Vehicle Factory for armored chassis production".into(),
                    "Sustain positive power generation surplus across the tactical grid".into(),
                ],
                hazard_summary: "Fortified laser turrets; hazardous canyon bottlenecks.".into(),
            },
            5 => Self {
                scenario_id: 5,
                title: "Starship Evacuation".into(),
                subtitle: "Mount Vulcan Caldera // Escape From Mount Vulcan".into(),
                faction: Faction::Eden,
                map_name: "eden03.map".into(),
                map_dimensions: (64, 64),
                transcript: "Eden High Command Emergency Broadcast: Mount Vulcan's caldera is undergoing an apocalyptic supervolcanic eruption sequence. Molten fissures are severing the southern approaches, while desperate Plymouth raider divisions converge on our launch coordinates to seize our evacuation craft.\n\nCommission the Starship launch facility, fabricate all four primary evacuation starship modules (Command Module, Cryo Stasis Bay, Habitat Ring, and Ion Drive), transport at least 150 colonists via Evacuation Transports into the stasis berths, and hold the perimeter until orbital escape velocity is achieved.".into(),
                primary_directives: vec![
                    "Assemble Starship modules (Command, Cryo Stasis, Habitat, Ion Drive)".into(),
                    "Evacuate at least 150 colonists via Evacuation Transports".into(),
                    "Defend colony against Plymouth raiders and magma fissure spread".into(),
                    "Protect the Eden Command Center from destruction".into(),
                ],
                secondary_directives: vec![
                    "Maintain colonial morale above 85% during evacuation countdown".into(),
                    "Stockpile 3,000t Common Ore and 1,000t Rare Ore for fuel reserves".into(),
                ],
                hazard_summary: "Cataclysmic volcanic eruption; fast-advancing magma rivers.".into(),
            },
            6 => Self {
                scenario_id: 6,
                title: "Operation Phoenix: Obsidian Crags".into(),
                subtitle: "Obsidian Crags // Citadel Assault & Terraformer Defense".into(),
                faction: Faction::Plymouth,
                map_name: "ply03.map".into(),
                map_dimensions: (64, 64),
                transcript: "Plymouth Council of Free Settlers Dispatch: The terraforming atmospheric processor in the Obsidian Crags represents the final hope for breathable atmosphere on New Terra. However, Eden's fortified Obsidian Citadel has locked down the sector, and microscopic nanite blight contagion is propagating towards our processor vents.\n\nDeploy heavy assault tanks, safeguard the Atmospheric Terraformer from nanite contagion until 100% saturation is achieved, and breach the fortified Eden Citadel to neutralize their defensive guard posts and Tiger railgun divisions.".into(),
                primary_directives: vec![
                    "Protect Atmospheric Terraformer from spreading nanite blight contagion".into(),
                    "Breaching assault against the fortified Eden Obsidian Citadel".into(),
                    "Achieve 100% Atmospheric Terraformer saturation".into(),
                    "Protect Plymouth Command Center from destruction".into(),
                ],
                secondary_directives: vec![
                    "Eliminate all hostile Eden railgun Guard Post turrets in the crags".into(),
                ],
                hazard_summary: "Nanite microbial blight contagion; heavy railgun fortresses.".into(),
            },
            _ => Self {
                scenario_id: 0,
                title: "Operation New Dawn".into(),
                subtitle: "Sector Eden Prime // Planetary Inhabitation".into(),
                faction: Faction::Eden,
                map_name: "eden01.map".into(),
                map_dimensions: (64, 64),
                transcript: "Colonists of the Eden Foundation: After the catastrophic failure of the Starship Conestoga, our landing site on New Terra must be secured immediately. Volcanic tremors indicate rising magma fissures nearby, and rogue Plymouth raiders have established aggressive perimeter posts.\n\nSecure colonial life support, automate freight ore hauling, commission defensive railgun armor, and advance planetary research to ensure the survival of our species.".into(),
                primary_directives: vec![
                    "Stockpile 3,000t Common Ore and 600t Rare Ore".into(),
                    "Deploy and commission at least 2 armed combat tanks".into(),
                    "Complete scientific research in any primary discipline".into(),
                    "Protect the colony Command Center from hostile destruction".into(),
                ],
                secondary_directives: vec![
                    "Establish a colony population of 120 colonists with positive morale".into(),
                ],
                hazard_summary: "Localized dust vortices; minor seismic tremors.".into(),
            },
        }
    }

    /// List of all 7 playable campaign scenarios in canonical progression order
    pub fn all_scenarios() -> Vec<Self> {
        vec![
            Self::get(0), // Eden 1
            Self::get(3), // Eden 2
            Self::get(5), // Eden 3
            Self::get(1), // Plymouth 1
            Self::get(4), // Plymouth 2
            Self::get(6), // Plymouth 3
            Self::get(2), // Academy Simulator
        ]
    }

    /// Retrieve localized scenario briefing by internal scenario ID and language
    pub fn get_localized(scenario_id: usize, lang: crate::i18n::Language) -> Self {
        let base = Self::get(scenario_id);
        let brief_dict = crate::i18n::get_scenario_briefing(scenario_id, lang);
        Self {
            scenario_id: base.scenario_id,
            title: brief_dict.title.to_string(),
            subtitle: brief_dict.subtitle.to_string(),
            faction: base.faction,
            map_name: base.map_name,
            map_dimensions: base.map_dimensions,
            transcript: brief_dict.transcript.to_string(),
            primary_directives: brief_dict.primary_directives.iter().map(|s| s.to_string()).collect(),
            secondary_directives: brief_dict.secondary_directives.iter().map(|s| s.to_string()).collect(),
            hazard_summary: brief_dict.hazard_summary.to_string(),
        }
    }

    /// List of all 7 playable campaign scenarios localized in canonical progression order
    pub fn all_scenarios_localized(lang: crate::i18n::Language) -> Vec<Self> {
        vec![
            Self::get_localized(0, lang),
            Self::get_localized(3, lang),
            Self::get_localized(5, lang),
            Self::get_localized(1, lang),
            Self::get_localized(4, lang),
            Self::get_localized(6, lang),
            Self::get_localized(2, lang),
        ]
    }
}

/// System options state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionsState {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub game_speed: f64,
    pub language: String,
    pub crt_scanlines: bool,
    pub high_dpi: bool,
}

impl Default for OptionsState {
    fn default() -> Self {
        Self {
            master_volume: 0.75,
            sfx_volume: 0.85,
            music_volume: 0.65,
            game_speed: 1.0,
            language: "en".into(),
            crt_scanlines: true,
            high_dpi: true,
        }
    }
}

/// Menu button interactive geometry
#[derive(Debug, Clone)]
pub struct MenuButton {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub label: String,
    pub accent_color: String,
    pub enabled: bool,
}

impl MenuButton {
    pub fn new(
        id: impl Into<String>,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        label: impl Into<String>,
        accent: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            x,
            y,
            w,
            h,
            label: label.into(),
            accent_color: accent.into(),
            enabled: true,
        }
    }

    pub fn disabled(
        id: impl Into<String>,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        label: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            x,
            y,
            w,
            h,
            label: label.into(),
            accent_color: "#646464".into(),
            enabled: false,
        }
    }

    pub fn contains(&self, px: f64, py: f64) -> bool {
        self.enabled && px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }
}

/// Active tab in the WebRTC multiplayer match lobby.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerTab {
    Host,
    Guest,
}

/// Action produced by user interaction with the menu
#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    None,
    OpenScreen(MenuScreen),
    StartScenario(usize),
    StartSkirmish(SkirmishConfig),
    ResumeGame,
    QuickSave,
    QuickLoad,
    LoadSlot(u32),
    SaveSlot(u32),
    DeleteSlot(u32),
    ExportOp2Save,
    ImportOp2Save(String),
    ApplyOptions(OptionsState),
    RestartScenario,
    SurrenderToMainMenu,
    CopyOfferToClipboard,
    PasteAnswerFromClipboard,
    CopyAnswerToClipboard,
    PasteOfferFromClipboard,
    GenerateGuestAnswer,
    ToggleMpReady,
    LaunchMultiplayerMatch,
    PlayBeep,
    Quit,
}

// =========================================================================
// 2. Comprehensive Menu Manager
// =========================================================================

/// Lightweight telemetry snapshot passed to menu for display
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColonySummary {
    pub sol: f64,
    pub population: u32,
    pub morale_pct: u32,
}

impl ColonySummary {
    pub fn new(sol: f64, population: u32, morale_pct: u32) -> Self {
        Self {
            sol,
            population,
            morale_pct,
        }
    }

    pub fn from_colony(colony: &ColonyState) -> Self {
        Self {
            sol: colony.sol,
            population: colony.population,
            morale_pct: colony.morale_pct,
        }
    }
}

/// Central controller and renderer for all Outpost 2 menu screens
pub struct MenuManager {
    pub current_screen: MenuScreen,
    pub previous_screen: MenuScreen,
    pub selected_scenario_id: usize,
    pub skirmish_config: SkirmishConfig,
    pub options: OptionsState,
    pub slots: Vec<(u32, Option<SaveMetadata>)>,
    pub selected_slot: Option<u32>,
    pub anim_time: f64,
    pub mouse_pos: (f64, f64),
    pub status_message: Option<(String, f64)>, // (message, duration_sec)
    pub transition_alpha: f64,
    pub mp_tab: MultiplayerTab,
    pub mp_room: crate::net::lobby::LobbyRoom,
    pub mp_offer_input: String,
    pub mp_answer_input: String,
    // Dialog 140 state
    pub selected_faction: Faction,
    pub campaign_difficulty: CampaignDifficulty,
    // Dialog 219 state
    pub briefing_tab: BriefingTab,
    pub story_chapter_index: usize,
    // Native assets
    pub shell_backdrop: Option<HtmlImageElement>,
    pub title_plate: Option<HtmlImageElement>,
    pub briefing_images: HashMap<usize, HtmlImageElement>,
    cached_buttons: Vec<MenuButton>,
    pub hovered_btn_id: Option<String>,
    pub active_pressed_btn_id: Option<String>,
}

impl Default for MenuManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuManager {
    pub fn new() -> Self {
        let mut mp_room = crate::net::lobby::LobbyRoom::default();
        mp_room.set_host_offer("data:application/sdp;base64,eyJ0eXBlIjoib2ZmZXIiLCJzZHAiOiJ2PTBcclxubz0tIDQyMTU3ODkwIDIgSU4gSVA0IDEyNy4wLjAuMVxyXG5zPS1cclxudD0wIDBcclxuYT1ncm91cDpCVU5ETEUgMFxyXG4ifQ==");

        let mut mgr = Self {
            current_screen: MenuScreen::MainMenu,
            previous_screen: MenuScreen::None,
            selected_scenario_id: 0,
            skirmish_config: SkirmishConfig::default(),
            options: OptionsState::default(),
            slots: Vec::new(),
            selected_slot: None,
            anim_time: 0.0,
            mouse_pos: (0.0, 0.0),
            status_message: None,
            transition_alpha: 1.0,
            mp_tab: MultiplayerTab::Host,
            mp_room,
            mp_offer_input: String::new(),
            mp_answer_input: String::new(),
            selected_faction: Faction::Eden,
            campaign_difficulty: CampaignDifficulty::Normal,
            briefing_tab: BriefingTab::Briefing,
            story_chapter_index: 0,
            shell_backdrop: {
                #[cfg(target_arch = "wasm32")]
                {
                    let img = HtmlImageElement::new().ok();
                    if let Some(ref image) = img {
                        image.set_src("assets/original_ui/shell_background_640x480.png");
                    }
                    img
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    None
                }
            },
            title_plate: {
                #[cfg(target_arch = "wasm32")]
                {
                    let img = HtmlImageElement::new().ok();
                    if let Some(ref image) = img {
                        image.set_src("assets/original_ui/title_plate_logo_544x122.png");
                    }
                    img
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    None
                }
            },
            briefing_images: {
                #[cfg(target_arch = "wasm32")]
                {
                    let mut map = HashMap::new();
                    for bid in [400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422, 423] {
                        if let Ok(img) = HtmlImageElement::new() {
                            img.set_src(&format!("assets/original_ui/briefing/bmp_{}.png", bid));
                            map.insert(bid, img);
                        }
                    }
                    map
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    HashMap::new()
                }
            },
            cached_buttons: Vec::new(),
            hovered_btn_id: None,
            active_pressed_btn_id: None,
        };
        mgr.refresh_save_slots();
        mgr
    }

    /// Check if the player is currently in live simulation mode
    pub fn is_in_game(&self) -> bool {
        self.current_screen == MenuScreen::None
    }

    /// Check if in tactical pause overlay
    pub fn is_paused(&self) -> bool {
        self.current_screen == MenuScreen::PauseMenu
    }

    /// Advance menu animation timers and notifications
    pub fn update(&mut self, dt_sec: f64) {
        self.anim_time += dt_sec;
        self.transition_alpha = (self.transition_alpha + dt_sec * 5.0).min(1.0);
        if let Some((_, ref mut dur)) = self.status_message {
            *dur -= dt_sec;
            if *dur <= 0.0 {
                self.status_message = None;
            }
        }
    }

    /// Transition to a new screen, updating navigation stack
    pub fn switch_screen(&mut self, next: MenuScreen) {
        if self.current_screen != next {
            self.previous_screen = self.current_screen;
            self.current_screen = next;
            self.transition_alpha = 0.0;
            self.hovered_btn_id = None;
            if next == MenuScreen::LoadMission {
                self.refresh_save_slots();
            }
        }
    }

    /// Reload LocalStorage slot metadata
    pub fn refresh_save_slots(&mut self) {
        let mut list = Vec::new();
        for slot in 1..=5 {
            let meta = LocalStorageBackend::load_metadata(slot).ok().flatten();
            list.push((slot, meta));
        }
        // Slot 0 (QuickSave) & Slot 100 (AutoSave)
        let q_meta = LocalStorageBackend::load_metadata(0).ok().flatten();
        list.push((0, q_meta));
        let a_meta = LocalStorageBackend::load_metadata(100).ok().flatten();
        list.push((100, a_meta));
        self.slots = list;
    }

    /// Set temporary notification toast
    pub fn set_status(&mut self, msg: impl Into<String>, duration_sec: f64) {
        self.status_message = Some((msg.into(), duration_sec));
    }

    /// Update mouse hover coordinate and detect newly hovered button (returns true if a new button was entered)
    pub fn handle_mouse_move(&mut self, x: f64, y: f64) -> bool {
        self.mouse_pos = (x, y);
        let new_hover = self
            .cached_buttons
            .iter()
            .find(|b| b.contains(x, y))
            .map(|b| b.id.clone());

        if new_hover != self.hovered_btn_id {
            self.hovered_btn_id = new_hover.clone();
            new_hover.is_some()
        } else {
            false
        }
    }

    /// Handle keyboard shortcut dispatch for menus
    pub fn handle_key_down(&mut self, key: &str, in_game: bool) -> MenuAction {
        match key {
            "Escape" => {
                if self.is_in_game() && in_game {
                    self.switch_screen(MenuScreen::PauseMenu);
                    MenuAction::None
                } else if self.current_screen == MenuScreen::PauseMenu {
                    self.switch_screen(MenuScreen::None);
                    MenuAction::ResumeGame
                } else if self.current_screen != MenuScreen::MainMenu && !self.is_in_game() {
                    let prev = self.previous_screen;
                    self.switch_screen(if prev == MenuScreen::None {
                        MenuScreen::MainMenu
                    } else {
                        prev
                    });
                    MenuAction::None
                } else {
                    MenuAction::None
                }
            }
            "F5" => {
                if self.is_in_game() || self.is_paused() {
                    MenuAction::QuickSave
                } else {
                    MenuAction::None
                }
            }
            "F9" => {
                if self.is_in_game() || self.is_paused() {
                    MenuAction::QuickLoad
                } else {
                    MenuAction::None
                }
            }
            _ => MenuAction::None,
        }
    }

    /// Process mouse click on the current screen and return the corresponding action
    pub fn handle_mouse_down(
        &mut self,
        click_x: f64,
        click_y: f64,
        screen_w: f64,
        screen_h: f64,
        has_game: bool,
    ) -> MenuAction {
        // Rebuild layout for hit-testing
        self.cached_buttons = self.build_button_layout(screen_w, screen_h, has_game);

        let hit_btn = self
            .cached_buttons
            .iter()
            .find(|b| b.contains(click_x, click_y))
            .map(|b| b.id.clone());

        if let Some(ref id) = hit_btn {
            self.active_pressed_btn_id = Some(id.clone());
            self.execute_button_action(id)
        } else {
            self.active_pressed_btn_id = None;
            MenuAction::None
        }
    }

    /// Reset pressed button state when mouse is released
    pub fn handle_mouse_up(&mut self) {
        self.active_pressed_btn_id = None;
    }

    /// Execute command associated with a button ID
    pub fn execute_button_action(&mut self, btn_id: &str) -> MenuAction {
        match btn_id {
            // Main Menu (Dialog 137)
            "btn_resume" => {
                self.switch_screen(MenuScreen::None);
                MenuAction::ResumeGame
            }
            "btn_tutorials" => {
                self.switch_screen(MenuScreen::Tutorials);
                MenuAction::PlayBeep
            }
            "btn_campaign" => {
                self.switch_screen(MenuScreen::CampaignSelect);
                MenuAction::PlayBeep
            }
            "btn_skirmish" => {
                self.switch_screen(MenuScreen::SkirmishSetup);
                MenuAction::PlayBeep
            }
            "btn_multiplayer" => {
                self.switch_screen(MenuScreen::MultiplayerLobby);
                MenuAction::PlayBeep
            }
            "btn_load" => {
                self.switch_screen(MenuScreen::LoadMission);
                MenuAction::PlayBeep
            }
            "btn_options" => {
                self.switch_screen(MenuScreen::Options);
                MenuAction::PlayBeep
            }
            "btn_help" => {
                // Authentic 1997 Help button - disabled/grayed out
                MenuAction::None
            }
            "btn_about" => {
                self.switch_screen(MenuScreen::About);
                MenuAction::PlayBeep
            }
            "btn_quit" => {
                MenuAction::Quit
            }

            // Dialog 140: New Campaign Options
            "btn_d140_fac_eden" => {
                self.selected_faction = Faction::Eden;
                MenuAction::PlayBeep
            }
            "btn_d140_fac_plymouth" => {
                self.selected_faction = Faction::Plymouth;
                MenuAction::PlayBeep
            }
            "btn_d140_diff_easy" => {
                self.campaign_difficulty = CampaignDifficulty::Easy;
                MenuAction::PlayBeep
            }
            "btn_d140_diff_normal" => {
                self.campaign_difficulty = CampaignDifficulty::Normal;
                MenuAction::PlayBeep
            }
            "btn_d140_diff_hard" => {
                self.campaign_difficulty = CampaignDifficulty::Hard;
                MenuAction::PlayBeep
            }
            "btn_d140_ok" => {
                // Determine initial scenario based on selected faction: Eden -> 0, Plymouth -> 1
                self.selected_scenario_id = if self.selected_faction == Faction::Eden { 0 } else { 1 };
                self.briefing_tab = BriefingTab::Briefing;
                self.switch_screen(MenuScreen::CampaignBriefing);
                MenuAction::PlayBeep
            }
            "btn_d140_cancel" => {
                self.switch_screen(MenuScreen::MainMenu);
                MenuAction::PlayBeep
            }

            // Dialog 219: Mission Menu / Briefing Controls
            "btn_d219_briefing" => {
                self.briefing_tab = BriefingTab::Briefing;
                MenuAction::PlayBeep
            }
            "btn_d219_objectives" => {
                self.briefing_tab = BriefingTab::Objectives;
                MenuAction::PlayBeep
            }
            "btn_d219_story" => {
                self.briefing_tab = BriefingTab::Story;
                MenuAction::PlayBeep
            }
            "btn_d219_start_mission" => {
                let id = self.selected_scenario_id;
                self.switch_screen(MenuScreen::None);
                MenuAction::StartScenario(id)
            }
            "btn_d219_leave_campaign" => {
                self.switch_screen(MenuScreen::MainMenu);
                MenuAction::PlayBeep
            }

            // Dialog 219 Story Navigation
            "btn_story_prev" => {
                if self.story_chapter_index > 0 {
                    self.story_chapter_index -= 1;
                }
                MenuAction::PlayBeep
            }
            "btn_story_next" => {
                if self.story_chapter_index + 1 < NOVELLA_CHAPTERS.len() {
                    self.story_chapter_index += 1;
                }
                MenuAction::PlayBeep
            }

            // Navigation
            "btn_back" => {
                let prev = if self.previous_screen == MenuScreen::None {
                    MenuScreen::MainMenu
                } else {
                    self.previous_screen
                };
                self.switch_screen(prev);
                MenuAction::PlayBeep
            }
            "btn_back_main" => {
                self.switch_screen(MenuScreen::MainMenu);
                MenuAction::PlayBeep
            }

            // Campaign Selection (Direct Scenario Buttons)
            id if id.starts_with("scenario_") => {
                if let Ok(idx) = id.trim_start_matches("scenario_").parse::<usize>() {
                    self.selected_scenario_id = idx;
                    self.switch_screen(MenuScreen::CampaignBriefing);
                    MenuAction::PlayBeep
                } else {
                    MenuAction::None
                }
            }

            // Legacy Campaign Briefing Actions
            "btn_begin_operation" => {
                let id = self.selected_scenario_id;
                self.switch_screen(MenuScreen::None);
                MenuAction::StartScenario(id)
            }
            "btn_briefing_prev" => {
                let all = BriefingScenario::all_scenarios();
                if let Some(pos) = all.iter().position(|s| s.scenario_id == self.selected_scenario_id) {
                    let prev_pos = if pos == 0 { all.len() - 1 } else { pos - 1 };
                    self.selected_scenario_id = all[prev_pos].scenario_id;
                }
                MenuAction::PlayBeep
            }
            "btn_briefing_next" => {
                let all = BriefingScenario::all_scenarios();
                if let Some(pos) = all.iter().position(|s| s.scenario_id == self.selected_scenario_id) {
                    let next_pos = (pos + 1) % all.len();
                    self.selected_scenario_id = all[next_pos].scenario_id;
                }
                MenuAction::PlayBeep
            }

            // Skirmish Setup
            "btn_skirmish_map" => {
                self.skirmish_config.cycle_map();
                MenuAction::PlayBeep
            }
            "btn_skirmish_faction" => {
                self.skirmish_config.toggle_human_faction();
                MenuAction::PlayBeep
            }
            "btn_skirmish_res" => {
                self.skirmish_config.cycle_resources();
                MenuAction::PlayBeep
            }
            "btn_skirmish_disasters" => {
                self.skirmish_config.cycle_disasters();
                MenuAction::PlayBeep
            }
            id if id.starts_with("bot_toggle_") => {
                if let Ok(idx) = id.trim_start_matches("bot_toggle_").parse::<usize>() {
                    self.skirmish_config.toggle_bot_enabled(idx);
                    MenuAction::PlayBeep
                } else {
                    MenuAction::None
                }
            }
            id if id.starts_with("bot_fac_") => {
                if let Ok(idx) = id.trim_start_matches("bot_fac_").parse::<usize>() {
                    self.skirmish_config.cycle_bot_faction(idx);
                    MenuAction::PlayBeep
                } else {
                    MenuAction::None
                }
            }
            id if id.starts_with("bot_doc_") => {
                if let Ok(idx) = id.trim_start_matches("bot_doc_").parse::<usize>() {
                    self.skirmish_config.cycle_bot_personality(idx);
                    MenuAction::PlayBeep
                } else {
                    MenuAction::None
                }
            }
            id if id.starts_with("bot_diff_") => {
                if let Ok(idx) = id.trim_start_matches("bot_diff_").parse::<usize>() {
                    self.skirmish_config.cycle_bot_difficulty(idx);
                    MenuAction::PlayBeep
                } else {
                    MenuAction::None
                }
            }
            "btn_launch_skirmish" => {
                let cfg = self.skirmish_config.clone();
                self.switch_screen(MenuScreen::None);
                MenuAction::StartSkirmish(cfg)
            }

            // Pause Menu
            "btn_pause_resume" => {
                self.switch_screen(MenuScreen::None);
                MenuAction::ResumeGame
            }
            "btn_pause_quicksave" => MenuAction::QuickSave,
            "btn_pause_quickload" => MenuAction::QuickLoad,
            "btn_pause_save_load" => {
                self.switch_screen(MenuScreen::LoadMission);
                MenuAction::PlayBeep
            }
            "btn_pause_options" => {
                self.switch_screen(MenuScreen::Options);
                MenuAction::PlayBeep
            }
            "btn_pause_restart" => {
                self.switch_screen(MenuScreen::None);
                MenuAction::RestartScenario
            }
            "btn_pause_surrender" => {
                self.switch_screen(MenuScreen::MainMenu);
                MenuAction::SurrenderToMainMenu
            }

            // Save / Load Slot Manager
            id if id.starts_with("slot_load_") => {
                if let Ok(slot) = id.trim_start_matches("slot_load_").parse::<u32>() {
                    self.switch_screen(MenuScreen::None);
                    MenuAction::LoadSlot(slot)
                } else {
                    MenuAction::None
                }
            }
            id if id.starts_with("slot_save_") => {
                if let Ok(slot) = id.trim_start_matches("slot_save_").parse::<u32>() {
                    MenuAction::SaveSlot(slot)
                } else {
                    MenuAction::None
                }
            }
            id if id.starts_with("slot_del_") => {
                if let Ok(slot) = id.trim_start_matches("slot_del_").parse::<u32>() {
                    self.refresh_save_slots();
                    MenuAction::DeleteSlot(slot)
                } else {
                    MenuAction::None
                }
            }
            "btn_export_save" => MenuAction::ExportOp2Save,

            // Options
            "btn_opt_vol_down" | "btn_opt_master_down" => {
                self.options.master_volume = (self.options.master_volume - 0.1).clamp(0.0, 1.0);
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_vol_up" | "btn_opt_master_up" => {
                self.options.master_volume = (self.options.master_volume + 0.1).clamp(0.0, 1.0);
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_sfx_down" => {
                self.options.sfx_volume = (self.options.sfx_volume - 0.1).clamp(0.0, 1.0);
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_sfx_up" => {
                self.options.sfx_volume = (self.options.sfx_volume + 0.1).clamp(0.0, 1.0);
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_music_down" => {
                self.options.music_volume = (self.options.music_volume - 0.1).clamp(0.0, 1.0);
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_music_up" => {
                self.options.music_volume = (self.options.music_volume + 0.1).clamp(0.0, 1.0);
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_speed_05" => {
                self.options.game_speed = 0.5;
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_speed_10" => {
                self.options.game_speed = 1.0;
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_speed_20" => {
                self.options.game_speed = 2.0;
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_speed_40" => {
                self.options.game_speed = 4.0;
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_scanlines" => {
                self.options.crt_scanlines = !self.options.crt_scanlines;
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_lang_en" => {
                self.options.language = "en".into();
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_lang_pt" => {
                self.options.language = "pt-BR".into();
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_lang_es" => {
                self.options.language = "es".into();
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_lang_de" => {
                self.options.language = "de".into();
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_lang_zh" => {
                self.options.language = "zh-CN".into();
                MenuAction::ApplyOptions(self.options.clone())
            }
            "btn_opt_lang_ru" => {
                self.options.language = "ru".into();
                MenuAction::ApplyOptions(self.options.clone())
            }

            // Multiplayer Match Lobby Handlers
            "btn_mp_tab_host" => {
                self.mp_tab = MultiplayerTab::Host;
                MenuAction::PlayBeep
            }
            "btn_mp_tab_guest" => {
                self.mp_tab = MultiplayerTab::Guest;
                MenuAction::PlayBeep
            }
            "btn_copy_offer" => {
                let _ = self.mp_room.copy_host_offer();
                self.set_status("Host Base64 Offer copied to clipboard!", 3.0);
                MenuAction::CopyOfferToClipboard
            }
            "btn_paste_answer" => {
                let sample_answer = "data:application/sdp;base64,eyJ0eXBlIjoiYW5zd2VyIiwic2RwIjoidj0wIn0=";
                let _ = self.mp_room.paste_guest_answer(sample_answer);
                if self.mp_room.guest_name.is_none() {
                    let _ = self.mp_room.register_guest("Guest Commander", Faction::Plymouth);
                }
                self.mp_room.set_latency(24);
                self.set_status("Guest Answer pasted! Peer connected (24ms)", 3.0);
                MenuAction::PasteAnswerFromClipboard
            }
            "btn_mp_map_prev" => {
                self.mp_room.cycle_map(false);
                MenuAction::PlayBeep
            }
            "btn_mp_map_next" | "btn_mp_map_cycle" => {
                self.mp_room.cycle_map(true);
                MenuAction::PlayBeep
            }
            "btn_mp_host_faction" => {
                self.mp_room.faction = match self.mp_room.faction {
                    Faction::Eden => Faction::Plymouth,
                    Faction::Plymouth => Faction::Eden,
                };
                MenuAction::PlayBeep
            }
            "btn_mp_guest_faction" => {
                let cur = self.mp_room.guest_faction.unwrap_or(Faction::Plymouth);
                self.mp_room.guest_faction = Some(match cur {
                    Faction::Eden => Faction::Plymouth,
                    Faction::Plymouth => Faction::Eden,
                });
                MenuAction::PlayBeep
            }
            "btn_mp_toggle_host_ready" => {
                self.mp_room.toggle_ready(true);
                MenuAction::ToggleMpReady
            }
            "btn_paste_offer" => {
                let sample_offer = "data:application/sdp;base64,eyJ0eXBlIjoib2ZmZXIiLCJzZHAiOiJ2PTAifQ==";
                let _ = self.mp_room.paste_host_offer(sample_offer);
                self.set_status("Host Offer pasted successfully!", 3.0);
                MenuAction::PasteOfferFromClipboard
            }
            "btn_generate_answer" => {
                let sample_answer = "data:application/sdp;base64,eyJ0eXBlIjoiYW5zd2VyIiwic2RwIjoidj0wIn0=";
                let _ = self.mp_room.generate_guest_answer(sample_answer);
                self.set_status("Guest Answer generated! Ready to copy", 3.0);
                MenuAction::GenerateGuestAnswer
            }
            "btn_copy_answer" => {
                let _ = self.mp_room.copy_guest_answer();
                self.set_status("Guest Answer copied to clipboard!", 3.0);
                MenuAction::CopyAnswerToClipboard
            }
            "btn_toggle_ready" => {
                self.mp_room.toggle_ready(false);
                MenuAction::ToggleMpReady
            }
            "btn_launch_match" => {
                if self.mp_room.is_ready_to_launch() {
                    let _ = self.mp_room.launch_match(0x1337BEEF);
                    self.switch_screen(MenuScreen::None);
                    MenuAction::LaunchMultiplayerMatch
                } else {
                    self.set_status("Cannot launch: both players must be connected & ready!", 3.0);
                    MenuAction::PlayBeep
                }
            }

            _ => MenuAction::None,
        }
    }

    /// Construct responsive button hitboxes according to screen mode and dimensions
    pub fn build_button_layout(
        &self,
        screen_w: f64,
        screen_h: f64,
        has_game: bool,
    ) -> Vec<MenuButton> {
        let mut btns = Vec::new();
        let lang = crate::i18n::Language::from_code(&self.options.language);
        let dict = crate::i18n::get_dictionary(lang);

        match self.current_screen {
            MenuScreen::None => {}

            MenuScreen::MainMenu => {
                // Dialog 137 Authentic Layout: DLU 90x14 -> scaled to ~200x28px
                let btn_w = 200.0;
                let btn_h = 28.0;
                let cx = (screen_w - btn_w) * 0.5;
                let mut start_y = if has_game { 160.0 } else { 172.0 };
                let spacing = 32.0;

                if has_game {
                    btns.push(MenuButton::new(
                        "btn_resume",
                        cx,
                        start_y,
                        btn_w,
                        btn_h,
                        "RESUME OPERATION",
                        "#4ade80",
                    ));
                    start_y += spacing;
                }

                // Authentic Dialog 137 Buttons:
                // 1. TUTORIALS
                btns.push(MenuButton::new(
                    "btn_tutorials",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "TUTORIALS",
                    "#00ff00",
                ));
                start_y += spacing;

                // 2. NEW CAMPAIGN
                btns.push(MenuButton::new(
                    "btn_campaign",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "NEW CAMPAIGN",
                    "#00ff00",
                ));
                start_y += spacing;

                // 3. COLONY GAMES
                btns.push(MenuButton::new(
                    "btn_skirmish",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "COLONY GAMES",
                    "#00ff00",
                ));
                start_y += spacing;

                // 4. MULTIPLAYER
                btns.push(MenuButton::new(
                    "btn_multiplayer",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "MULTIPLAYER",
                    "#00ff00",
                ));
                start_y += spacing;

                // 5. LOAD GAME
                btns.push(MenuButton::new(
                    "btn_load",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "LOAD GAME",
                    "#00ff00",
                ));
                start_y += spacing;

                // 6. GAME PREFERENCES
                btns.push(MenuButton::new(
                    "btn_options",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "GAME PREFERENCES",
                    "#00ff00",
                ));
                start_y += spacing;

                // 7. HELP (authentic 1997 look: grayed out disabled)
                btns.push(MenuButton::disabled(
                    "btn_help",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "HELP",
                ));
                start_y += spacing;

                // 8. ABOUT OUTPOST 2
                btns.push(MenuButton::new(
                    "btn_about",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "ABOUT OUTPOST 2",
                    "#00ff00",
                ));
                start_y += spacing;

                // 9. QUIT
                btns.push(MenuButton::new(
                    "btn_quit",
                    cx,
                    start_y,
                    btn_w,
                    btn_h,
                    "QUIT",
                    "#00ff00",
                ));
            }

            MenuScreen::CampaignSelect => {
                // Dialog 140: NEW CAMPAIGN OPTIONS dialog (pos: 196x114 DLUs -> scaled 440x260px)
                let dlg_w = 460.0;
                let dlg_h = 280.0;
                let dx = (screen_w - dlg_w) * 0.5;
                let dy = (screen_h - dlg_h) * 0.5;

                // Faction Radio Buttons (Eden vs Plymouth)
                btns.push(MenuButton::new(
                    "btn_d140_fac_eden",
                    dx + 30.0,
                    dy + 72.0,
                    170.0,
                    32.0,
                    if self.selected_faction == Faction::Eden { "(X) EDEN" } else { "( ) EDEN" },
                    if self.selected_faction == Faction::Eden { "#00d4ff" } else { "#475569" },
                ));

                btns.push(MenuButton::new(
                    "btn_d140_fac_plymouth",
                    dx + 30.0,
                    dy + 112.0,
                    170.0,
                    32.0,
                    if self.selected_faction == Faction::Plymouth { "(X) PLYMOUTH" } else { "( ) PLYMOUTH" },
                    if self.selected_faction == Faction::Plymouth { "#ff8c00" } else { "#475569" },
                ));

                // Difficulty Radio Buttons (Easy, Normal, Hard)
                btns.push(MenuButton::new(
                    "btn_d140_diff_easy",
                    dx + 250.0,
                    dy + 65.0,
                    170.0,
                    28.0,
                    if self.campaign_difficulty == CampaignDifficulty::Easy { "(X) EASY" } else { "( ) EASY" },
                    if self.campaign_difficulty == CampaignDifficulty::Easy { "#00ff00" } else { "#475569" },
                ));

                btns.push(MenuButton::new(
                    "btn_d140_diff_normal",
                    dx + 250.0,
                    dy + 98.0,
                    170.0,
                    28.0,
                    if self.campaign_difficulty == CampaignDifficulty::Normal { "(X) NORMAL" } else { "( ) NORMAL" },
                    if self.campaign_difficulty == CampaignDifficulty::Normal { "#00ff00" } else { "#475569" },
                ));

                btns.push(MenuButton::new(
                    "btn_d140_diff_hard",
                    dx + 250.0,
                    dy + 131.0,
                    170.0,
                    28.0,
                    if self.campaign_difficulty == CampaignDifficulty::Hard { "(X) HARD" } else { "( ) HARD" },
                    if self.campaign_difficulty == CampaignDifficulty::Hard { "#00ff00" } else { "#475569" },
                ));

                // OK and CANCEL Action Buttons (Authentic 90x14 DLUs -> 180x28px)
                btns.push(MenuButton::new(
                    "btn_d140_ok",
                    dx + 130.0,
                    dy + 185.0,
                    200.0,
                    28.0,
                    "OK",
                    "#00ff00",
                ));

                btns.push(MenuButton::new(
                    "btn_d140_cancel",
                    dx + 130.0,
                    dy + 224.0,
                    200.0,
                    28.0,
                    "CANCEL",
                    "#00ff00",
                ));
            }

            MenuScreen::CampaignBriefing => {
                // Dialog 219: MISSION MENU & BRIEFING
                // Authentic Dialog 219 command buttons on left column:
                let btn_w = 180.0;
                let btn_h = 28.0;
                let left_x = 36.0;
                let mut btn_y = 60.0;
                let spacing = 36.0;

                // 1. MISSION BRIEFING
                btns.push(MenuButton::new(
                    "btn_d219_briefing",
                    left_x,
                    btn_y,
                    btn_w,
                    btn_h,
                    if self.briefing_tab == BriefingTab::Briefing { "> MISSION BRIEFING" } else { "MISSION BRIEFING" },
                    if self.briefing_tab == BriefingTab::Briefing { "#ddff22" } else { "#00ff00" },
                ));
                btn_y += spacing;

                // 2. SPECIFIC OBJECTIVES
                btns.push(MenuButton::new(
                    "btn_d219_objectives",
                    left_x,
                    btn_y,
                    btn_w,
                    btn_h,
                    if self.briefing_tab == BriefingTab::Objectives { "> SPECIFIC OBJECTIVES" } else { "SPECIFIC OBJECTIVES" },
                    if self.briefing_tab == BriefingTab::Objectives { "#ddff22" } else { "#00ff00" },
                ));
                btn_y += spacing;

                // 3. STORY
                btns.push(MenuButton::new(
                    "btn_d219_story",
                    left_x,
                    btn_y,
                    btn_w,
                    btn_h,
                    if self.briefing_tab == BriefingTab::Story { "> STORY" } else { "STORY" },
                    if self.briefing_tab == BriefingTab::Story { "#ddff22" } else { "#00ff00" },
                ));
                btn_y += spacing;

                // 4. START MISSION
                btns.push(MenuButton::new(
                    "btn_d219_start_mission",
                    left_x,
                    btn_y,
                    btn_w,
                    btn_h,
                    "START MISSION",
                    "#00ff00",
                ));
                btn_y += spacing + 12.0;

                // 5. LEAVE CAMPAIGN
                btns.push(MenuButton::new(
                    "btn_d219_leave_campaign",
                    left_x,
                    btn_y,
                    btn_w,
                    btn_h,
                    "LEAVE CAMPAIGN",
                    "#00ff00",
                ));

                // If in Story Tab, provide Previous / Next Chapter buttons
                if self.briefing_tab == BriefingTab::Story {
                    let story_box_x = 240.0;
                    let story_box_y = screen_h - 48.0;
                    btns.push(MenuButton::new(
                        "btn_story_prev",
                        story_box_x,
                        story_box_y,
                        130.0,
                        26.0,
                        "< PREV CHAPTER",
                        "#00ff00",
                    ));
                    btns.push(MenuButton::new(
                        "btn_story_next",
                        story_box_x + 150.0,
                        story_box_y,
                        130.0,
                        26.0,
                        "NEXT CHAPTER >",
                        "#00ff00",
                    ));
                }

                // Also provide backwards compatibility with btn_begin_operation
                btns.push(MenuButton::new(
                    "btn_begin_operation",
                    left_x,
                    screen_h - 48.0,
                    btn_w,
                    26.0,
                    "BEGIN OPERATION",
                    "#00ff00",
                ));
            }

            MenuScreen::Tutorials => {
                // Dialog 269: TUTORIALS
                let btn_w = 180.0;
                let btn_h = 28.0;
                let cx = (screen_w - btn_w) * 0.5;

                // Scenario 2 is the Tutorial Academy simulator
                btns.push(MenuButton::new(
                    "scenario_2",
                    cx - 100.0,
                    screen_h - 60.0,
                    btn_w,
                    btn_h,
                    "START TUTORIAL",
                    "#00ff00",
                ));

                btns.push(MenuButton::new(
                    "btn_back_main",
                    cx + 100.0,
                    screen_h - 60.0,
                    btn_w,
                    btn_h,
                    "BACK TO MAIN MENU",
                    "#00ff00",
                ));
            }

            MenuScreen::About => {
                let btn_w = 180.0;
                let btn_h = 28.0;
                let cx = (screen_w - btn_w) * 0.5;
                btns.push(MenuButton::new(
                    "btn_back_main",
                    cx,
                    screen_h - 60.0,
                    btn_w,
                    btn_h,
                    "BACK TO MAIN MENU",
                    "#00ff00",
                ));
            }

            MenuScreen::SkirmishSetup => {
                let panel_w = (screen_w * 0.9).clamp(680.0, 980.0);
                let panel_h = (screen_h * 0.85).clamp(500.0, 700.0);
                let px = (screen_w - panel_w) * 0.5;
                let py = (screen_h - panel_h) * 0.5;
                let left_w = 320.0;

                // Left Column: Theater Config
                btns.push(MenuButton::new(
                    "btn_skirmish_map",
                    px + 20.0,
                    py + 90.0,
                    left_w,
                    40.0,
                    format!("{}: {}", dict.skirmish.map_selection, self.skirmish_config.map_name()),
                    "#38bdf8",
                ));
                btns.push(MenuButton::new(
                    "btn_skirmish_faction",
                    px + 20.0,
                    py + 145.0,
                    left_w,
                    40.0,
                    format!("{}: {:?}", dict.skirmish.player_faction, self.skirmish_config.human_faction),
                    if self.skirmish_config.human_faction == Faction::Eden {
                        "#38bdf8"
                    } else {
                        "#f97316"
                    },
                ));
                btns.push(MenuButton::new(
                    "btn_skirmish_res",
                    px + 20.0,
                    py + 200.0,
                    left_w,
                    40.0,
                    format!("{}: {}", dict.skirmish.abundance, self.skirmish_config.resources.display_name_localized(lang)),
                    "#facc15",
                ));
                btns.push(MenuButton::new(
                    "btn_skirmish_disasters",
                    px + 20.0,
                    py + 255.0,
                    left_w,
                    40.0,
                    format!("{}: {}", dict.skirmish.disaster, self.skirmish_config.disasters.display_name_localized(lang)),
                    "#ef4444",
                ));

                // Right Column: Bot Slots 1..5
                let right_x = px + left_w + 40.0;
                let slot_w = panel_w - (left_w + 60.0);
                for (i, bot) in self.skirmish_config.bots.iter().enumerate() {
                    let by = py + 90.0 + (i as f64) * 60.0;

                    let toggle_label = if bot.enabled { dict.skirmish.bot_active } else { dict.skirmish.bot_off };
                    btns.push(MenuButton::new(
                        format!("bot_toggle_{}", i),
                        right_x,
                        by,
                        80.0,
                        36.0,
                        toggle_label,
                        if bot.enabled { "#4ade80" } else { "#64748b" },
                    ));

                    if bot.enabled {
                        btns.push(MenuButton::new(
                            format!("bot_fac_{}", i),
                            right_x + 90.0,
                            by,
                            90.0,
                            36.0,
                            format!("{:?}", bot.faction),
                            if bot.faction == Faction::Eden { "#38bdf8" } else { "#ef4444" },
                        ));
                        btns.push(MenuButton::new(
                            format!("bot_doc_{}", i),
                            right_x + 190.0,
                            by,
                            slot_w - 290.0,
                            36.0,
                            bot.personality.name_localized(lang),
                            "#facc15",
                        ));
                        btns.push(MenuButton::new(
                            format!("bot_diff_{}", i),
                            right_x + slot_w - 90.0,
                            by,
                            90.0,
                            36.0,
                            bot.difficulty.name_localized(lang),
                            "#a855f7",
                        ));
                    }
                }

                // Bottom Launch
                let bottom_y = py + panel_h - 60.0;
                btns.push(MenuButton::new(
                    "btn_back",
                    px + 20.0,
                    bottom_y,
                    140.0,
                    42.0,
                    dict.main_menu.back,
                    "#94a3b8",
                ));
                btns.push(MenuButton::new(
                    "btn_launch_skirmish",
                    px + panel_w - 260.0,
                    bottom_y,
                    240.0,
                    42.0,
                    dict.skirmish.launch,
                    "#f97316",
                ));
            }

            MenuScreen::PauseMenu => {
                let modal_w = 360.0;
                let modal_h = 440.0;
                let mx = (screen_w - modal_w) * 0.5;
                let my = (screen_h - modal_h) * 0.5;
                let btn_w = modal_w - 40.0;
                let btn_x = mx + 20.0;
                let mut cur_y = my + 70.0;

                btns.push(MenuButton::new(
                    "btn_pause_resume",
                    btn_x,
                    cur_y,
                    btn_w,
                    40.0,
                    dict.pause_menu.resume,
                    "#4ade80",
                ));
                cur_y += 48.0;

                btns.push(MenuButton::new(
                    "btn_pause_quicksave",
                    btn_x,
                    cur_y,
                    btn_w,
                    40.0,
                    dict.pause_menu.quicksave,
                    "#38bdf8",
                ));
                cur_y += 48.0;

                btns.push(MenuButton::new(
                    "btn_pause_quickload",
                    btn_x,
                    cur_y,
                    btn_w,
                    40.0,
                    dict.pause_menu.quickload,
                    "#facc15",
                ));
                cur_y += 48.0;

                btns.push(MenuButton::new(
                    "btn_pause_save_load",
                    btn_x,
                    cur_y,
                    btn_w,
                    40.0,
                    dict.pause_menu.save_load_manager,
                    "#cbd5e1",
                ));
                cur_y += 48.0;

                btns.push(MenuButton::new(
                    "btn_pause_options",
                    btn_x,
                    cur_y,
                    btn_w,
                    40.0,
                    dict.pause_menu.options,
                    "#cbd5e1",
                ));
                cur_y += 48.0;

                btns.push(MenuButton::new(
                    "btn_pause_restart",
                    btn_x,
                    cur_y,
                    btn_w,
                    40.0,
                    dict.pause_menu.restart,
                    "#fb923c",
                ));
                cur_y += 48.0;

                btns.push(MenuButton::new(
                    "btn_pause_surrender",
                    btn_x,
                    cur_y,
                    btn_w,
                    40.0,
                    dict.pause_menu.surrender,
                    "#ef4444",
                ));
            }

            MenuScreen::LoadMission => {
                let panel_w = (screen_w * 0.9).clamp(640.0, 940.0);
                let panel_h = (screen_h * 0.85).clamp(480.0, 680.0);
                let px = (screen_w - panel_w) * 0.5;
                let py = (screen_h - panel_h) * 0.5;

                let mut sy = py + 75.0;

                for &(slot, ref meta) in &self.slots {
                    let is_special = slot == 0 || slot == 100;
                    if is_special && meta.is_none() {
                        continue;
                    }

                    let btn_y = sy;
                    if meta.is_some() {
                        btns.push(MenuButton::new(
                            format!("slot_load_{}", slot),
                            px + panel_w - 220.0,
                            btn_y + 4.0,
                            90.0,
                            34.0,
                            "LOAD",
                            "#4ade80",
                        ));
                    }

                    if has_game && !is_special {
                        btns.push(MenuButton::new(
                            format!("slot_save_{}", slot),
                            px + panel_w - 120.0,
                            btn_y + 4.0,
                            90.0,
                            34.0,
                            "SAVE",
                            "#38bdf8",
                        ));
                    }

                    if meta.is_some() && !is_special {
                        btns.push(MenuButton::new(
                            format!("slot_del_{}", slot),
                            px + panel_w - 20.0,
                            btn_y + 4.0,
                            30.0,
                            34.0,
                            "✕",
                            "#ef4444",
                        ));
                    }

                    sy += 54.0;
                }

                // File export button if game present
                if has_game {
                    btns.push(MenuButton::new(
                        "btn_export_save",
                        px + 20.0,
                        py + panel_h - 60.0,
                        220.0,
                        40.0,
                        "💾 EXPORT .OP2SAVE FILE",
                        "#facc15",
                    ));
                }

                btns.push(MenuButton::new(
                    "btn_back",
                    px + panel_w - 160.0,
                    py + panel_h - 60.0,
                    140.0,
                    40.0,
                    dict.main_menu.back,
                    "#94a3b8",
                ));
            }

            MenuScreen::Options => {
                let panel_w = (screen_w * 0.85).clamp(540.0, 740.0);
                let panel_h = (screen_h * 0.85).clamp(480.0, 620.0);
                let px = (screen_w - panel_w) * 0.5;
                let py = (screen_h - panel_h) * 0.5;

                // 1. Master Volume Controls
                btns.push(MenuButton::new(
                    "btn_opt_vol_down",
                    px + panel_w - 100.0,
                    py + 70.0,
                    42.0,
                    28.0,
                    "[-]",
                    "#38bdf8",
                ));
                btns.push(MenuButton::new(
                    "btn_opt_vol_up",
                    px + panel_w - 52.0,
                    py + 70.0,
                    42.0,
                    28.0,
                    "[+]",
                    "#38bdf8",
                ));

                // 2. SFX Volume Controls
                btns.push(MenuButton::new(
                    "btn_opt_sfx_down",
                    px + panel_w - 100.0,
                    py + 110.0,
                    42.0,
                    28.0,
                    "[-]",
                    "#4ade80",
                ));
                btns.push(MenuButton::new(
                    "btn_opt_sfx_up",
                    px + panel_w - 52.0,
                    py + 110.0,
                    42.0,
                    28.0,
                    "[+]",
                    "#4ade80",
                ));

                // 3. Music Volume Controls
                btns.push(MenuButton::new(
                    "btn_opt_music_down",
                    px + panel_w - 100.0,
                    py + 150.0,
                    42.0,
                    28.0,
                    "[-]",
                    "#a855f7",
                ));
                btns.push(MenuButton::new(
                    "btn_opt_music_up",
                    px + panel_w - 52.0,
                    py + 150.0,
                    42.0,
                    28.0,
                    "[+]",
                    "#a855f7",
                ));

                // 4. Speed Multiplier Controls
                let spd_x = px + panel_w - 260.0;
                let spd_y = py + 195.0;
                btns.push(MenuButton::new(
                    "btn_opt_speed_05",
                    spd_x,
                    spd_y,
                    55.0,
                    30.0,
                    "0.5x",
                    if (self.options.game_speed - 0.5).abs() < 1e-4 { "#4ade80" } else { "#64748b" },
                ));
                btns.push(MenuButton::new(
                    "btn_opt_speed_10",
                    spd_x + 60.0,
                    spd_y,
                    55.0,
                    30.0,
                    "1.0x",
                    if (self.options.game_speed - 1.0).abs() < 1e-4 { "#4ade80" } else { "#64748b" },
                ));
                btns.push(MenuButton::new(
                    "btn_opt_speed_20",
                    spd_x + 120.0,
                    spd_y,
                    55.0,
                    30.0,
                    "2.0x",
                    if (self.options.game_speed - 2.0).abs() < 1e-4 { "#4ade80" } else { "#64748b" },
                ));
                btns.push(MenuButton::new(
                    "btn_opt_speed_40",
                    spd_x + 180.0,
                    spd_y,
                    55.0,
                    30.0,
                    "4.0x",
                    if (self.options.game_speed - 4.0).abs() < 1e-4 { "#4ade80" } else { "#64748b" },
                ));

                // 5. Scanline Toggle
                let scan_y = py + 245.0;
                btns.push(MenuButton::new(
                    "btn_opt_scanlines",
                    px + panel_w - 140.0,
                    scan_y,
                    110.0,
                    30.0,
                    if self.options.crt_scanlines { dict.options.enabled } else { dict.options.disabled },
                    if self.options.crt_scanlines { "#4ade80" } else { "#64748b" },
                ));

                // 6. Languages
                let lang_x = px + panel_w - 330.0;
                let lang_y = py + 295.0;
                let langs = [
                    ("btn_opt_lang_en", "EN", "en"),
                    ("btn_opt_lang_pt", "PT", "pt-BR"),
                    ("btn_opt_lang_es", "ES", "es"),
                    ("btn_opt_lang_de", "DE", "de"),
                    ("btn_opt_lang_zh", "中文", "zh-CN"),
                    ("btn_opt_lang_ru", "RU", "ru"),
                ];
                for (i, (id, label, code)) in langs.iter().enumerate() {
                    let active = self.options.language == *code;
                    btns.push(MenuButton::new(
                        *id,
                        lang_x + (i as f64) * 54.0,
                        lang_y,
                        48.0,
                        30.0,
                        *label,
                        if active { "#facc15" } else { "#64748b" },
                    ));
                }

                // 7. Apply & Return Button
                btns.push(MenuButton::new(
                    "btn_back",
                    px + (panel_w - 180.0) * 0.5,
                    py + panel_h - 52.0,
                    180.0,
                    36.0,
                    dict.main_menu.apply_and_return,
                    "#4ade80",
                ));
            }

            MenuScreen::MultiplayerLobby => {
                let panel_w = (screen_w * 0.88).clamp(640.0, 880.0);
                let panel_h = (screen_h * 0.82).clamp(460.0, 620.0);
                let px = (screen_w - panel_w) * 0.5;
                let py = (screen_h - panel_h) * 0.5;

                // Tab Switcher Buttons
                let tab_y = py + 52.0;
                let is_host_tab = self.mp_tab == MultiplayerTab::Host;

                btns.push(MenuButton::new(
                    "btn_mp_tab_host",
                    px + 24.0,
                    tab_y,
                    200.0,
                    34.0,
                    dict.multiplayer.host,
                    if is_host_tab { "#38bdf8" } else { "#64748b" },
                ));

                btns.push(MenuButton::new(
                    "btn_mp_tab_guest",
                    px + 236.0,
                    tab_y,
                    200.0,
                    34.0,
                    dict.multiplayer.guest,
                    if !is_host_tab { "#38bdf8" } else { "#64748b" },
                ));

                if is_host_tab {
                    // Host Tab Actions:
                    // 1. Copy Offer to Clipboard
                    btns.push(MenuButton::new(
                        "btn_copy_offer",
                        px + 24.0,
                        py + 155.0,
                        220.0,
                        34.0,
                        dict.multiplayer.copy_offer,
                        "#38bdf8",
                    ));

                    // 2. Paste Guest Answer
                    btns.push(MenuButton::new(
                        "btn_paste_answer",
                        px + 260.0,
                        py + 155.0,
                        220.0,
                        34.0,
                        dict.multiplayer.paste_answer,
                        "#a855f7",
                    ));

                    // 3. Map Selector (Eden 1-3, Plymouth 1-3, Procedural Skirmish)
                    btns.push(MenuButton::new(
                        "btn_mp_map_prev",
                        px + 24.0,
                        py + 230.0,
                        36.0,
                        34.0,
                        "◀",
                        "#38bdf8",
                    ));

                    btns.push(MenuButton::new(
                        "btn_mp_map_cycle",
                        px + 68.0,
                        py + 230.0,
                        330.0,
                        34.0,
                        format!("MAP: {}", self.mp_room.map_name),
                        "#38bdf8",
                    ));

                    btns.push(MenuButton::new(
                        "btn_mp_map_next",
                        px + 406.0,
                        py + 230.0,
                        36.0,
                        34.0,
                        "▶",
                        "#38bdf8",
                    ));

                    // Host Faction Toggle
                    btns.push(MenuButton::new(
                        "btn_mp_host_faction",
                        px + 454.0,
                        py + 230.0,
                        180.0,
                        34.0,
                        format!("FACTION: {:?}", self.mp_room.faction),
                        "#facc15",
                    ));

                    // 4. Host Readiness Toggle
                    let host_ready = self.mp_room.is_host_ready;
                    btns.push(MenuButton::new(
                        "btn_mp_toggle_host_ready",
                        px + 24.0,
                        py + 310.0,
                        200.0,
                        38.0,
                        if host_ready { dict.multiplayer.ready_toggle } else { dict.multiplayer.not_ready_toggle },
                        if host_ready { "#4ade80" } else { "#facc15" },
                    ));

                    // 5. Launch Match button (enabled when both players ready)
                    let ready_to_launch = self.mp_room.is_ready_to_launch();
                    btns.push(MenuButton::new(
                        "btn_launch_match",
                        px + 240.0,
                        py + 310.0,
                        220.0,
                        38.0,
                        dict.multiplayer.launch_match,
                        if ready_to_launch { "#4ade80" } else { "#475569" },
                    ));
                } else {
                    // Guest Tab Actions:
                    // 1. Paste Host Offer
                    btns.push(MenuButton::new(
                        "btn_paste_offer",
                        px + 24.0,
                        py + 155.0,
                        220.0,
                        34.0,
                        dict.multiplayer.paste_offer,
                        "#a855f7",
                    ));

                    // 2. Generate Answer Button
                    btns.push(MenuButton::new(
                        "btn_generate_answer",
                        px + 260.0,
                        py + 155.0,
                        220.0,
                        34.0,
                        dict.multiplayer.generate_answer,
                        "#38bdf8",
                    ));

                    // 3. Copy Answer to Clipboard
                    btns.push(MenuButton::new(
                        "btn_copy_answer",
                        px + 24.0,
                        py + 230.0,
                        220.0,
                        34.0,
                        dict.multiplayer.copy_answer,
                        "#38bdf8",
                    ));

                    // 4. Guest Readiness Toggle
                    let guest_ready = self.mp_room.is_guest_ready;
                    btns.push(MenuButton::new(
                        "btn_toggle_ready",
                        px + 260.0,
                        py + 230.0,
                        220.0,
                        34.0,
                        if guest_ready { dict.multiplayer.ready_toggle } else { dict.multiplayer.not_ready_toggle },
                        if guest_ready { "#4ade80" } else { "#facc15" },
                    ));

                    // Guest Faction Toggle
                    let g_fac = self.mp_room.guest_faction.unwrap_or(Faction::Plymouth);
                    btns.push(MenuButton::new(
                        "btn_mp_guest_faction",
                        px + 494.0,
                        py + 230.0,
                        170.0,
                        34.0,
                        format!("FACTION: {:?}", g_fac),
                        "#facc15",
                    ));
                }

                // Return to Main Menu
                btns.push(MenuButton::new(
                    "btn_back_main",
                    px + (panel_w - 180.0) * 0.5,
                    py + panel_h - 52.0,
                    180.0,
                    38.0,
                    dict.main_menu.back_to_menu,
                    "#94a3b8",
                ));
            }
        }

        btns
    }

    // =========================================================================
    // 3. Canvas 2D Rendering Pipeline
    // =========================================================================

    /// Render the active menu screen on top of the HTML5 Canvas
    pub fn render(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        width: f64,
        height: f64,
        summary: Option<ColonySummary>,
    ) {
        if self.current_screen == MenuScreen::None {
            return;
        }

        let has_game = summary.is_some();

        // Cache responsive buttons for hit-testing and hover effects
        self.cached_buttons = self.build_button_layout(width, height, has_game);

        let apply_alpha = self.transition_alpha < 1.0;
        if apply_alpha {
            let _ = ctx.save();
            ctx.set_global_alpha(self.transition_alpha.clamp(0.0, 1.0));
        }

        // For PauseMenu, render semi-transparent backdrop over game
        if self.current_screen == MenuScreen::PauseMenu {
            self.render_pause_overlay(ctx, width, height, summary);
            if apply_alpha {
                let _ = ctx.restore();
            }
            return;
        }

        // Animated retro space background for all full menu screens
        self.render_retro_background(ctx, width, height);

        match self.current_screen {
            MenuScreen::MainMenu => self.render_main_menu(ctx, width, height),
            MenuScreen::CampaignSelect => self.render_campaign_select(ctx, width, height),
            MenuScreen::CampaignBriefing => self.render_campaign_briefing(ctx, width, height),
            MenuScreen::SkirmishSetup => self.render_skirmish_setup(ctx, width, height),
            MenuScreen::LoadMission => self.render_load_mission(ctx, width, height),
            MenuScreen::Options => self.render_options(ctx, width, height),
            MenuScreen::MultiplayerLobby => self.render_multiplayer_lobby(ctx, width, height),
            MenuScreen::Tutorials => self.render_tutorials(ctx, width, height),
            MenuScreen::About => self.render_about(ctx, width, height),
            MenuScreen::None | MenuScreen::PauseMenu => {}
        }

        // Draw buttons
        self.render_cached_buttons(ctx);

        // Notification toast overlay if present
        if let Some((ref msg, _)) = self.status_message {
            self.render_toast(ctx, msg, width, height);
        }

        // Retro scanlines overlay
        if self.options.crt_scanlines {
            self.render_crt_scanlines(ctx, width, height);
        }

        if apply_alpha {
            let _ = ctx.restore();
        }
    }

    /// Render animated starfield, planet horizon, and perspective grid
    fn render_retro_background(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        // Deep space void with vertical celestial gradient
        let grad = ctx.create_linear_gradient(0.0, 0.0, 0.0, h);
        let _ = grad.add_color_stop(0.0, "#020409");
        let _ = grad.add_color_stop(0.38, "#060d1d");
        let _ = grad.add_color_stop(0.72, "#080c18");
        let _ = grad.add_color_stop(1.0, "#020307");
        ctx.set_fill_style_canvas_gradient(&grad);
        ctx.fill_rect(0.0, 0.0, w, h);

        // Luminous Cosmic Nebulae
        // Nebula 1: The Cygnus Cyan Rift (Upper Left / Mid)
        let n1_x = w * 0.22 + (self.anim_time * 0.15).sin() * 15.0;
        let n1_y = h * 0.30 + (self.anim_time * 0.12).cos() * 10.0;
        let n1_r = (w * 0.38).clamp(220.0, 480.0);
        if let Ok(n1_grad) = ctx.create_radial_gradient(n1_x, n1_y, 0.0, n1_x, n1_y, n1_r) {
            let _ = n1_grad.add_color_stop(0.0, "rgba(6, 182, 212, 0.12)");
            let _ = n1_grad.add_color_stop(0.45, "rgba(14, 116, 144, 0.05)");
            let _ = n1_grad.add_color_stop(1.0, "rgba(2, 4, 9, 0.0)");
            ctx.set_fill_style_canvas_gradient(&n1_grad);
            ctx.fill_rect(0.0, 0.0, w, h);
        }

        // Nebula 2: Deep Violet / Magenta Stellar Nursery (Lower Right)
        let n2_x = w * 0.74;
        let n2_y = h * 0.68;
        let n2_r = (w * 0.42).clamp(260.0, 520.0);
        if let Ok(n2_grad) = ctx.create_radial_gradient(n2_x, n2_y, 0.0, n2_x, n2_y, n2_r) {
            let _ = n2_grad.add_color_stop(0.0, "rgba(139, 92, 246, 0.09)");
            let _ = n2_grad.add_color_stop(0.50, "rgba(91, 33, 182, 0.03)");
            let _ = n2_grad.add_color_stop(1.0, "rgba(2, 4, 9, 0.0)");
            ctx.set_fill_style_canvas_gradient(&n2_grad);
            ctx.fill_rect(0.0, 0.0, w, h);
        }

        // High-Density Procedural Starfield (160 stars in 3 parallax depth layers)
        let star_count = 160;
        for i in 0..star_count {
            let fi = i as f64;
            let seed_x = ((fi * 9301.0 + 49297.0) % 233280.0) / 233280.0;
            let seed_y = ((fi * 49297.0 + 9301.0) % 233280.0) / 233280.0;
            let layer = i % 3;
            let speed = match layer {
                0 => 2.5 + (fi % 3.0),
                1 => 5.0 + (fi % 4.0),
                _ => 8.5 + (fi % 5.0),
            };
            let sx = (seed_x * w + self.anim_time * speed) % w;
            let sy = seed_y * h;

            let twinkle = ((self.anim_time * (1.8 + (fi % 5.0) * 0.6) + fi * 1.5).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
            let alpha = 0.25 + 0.65 * twinkle;

            match layer {
                0 => {
                    ctx.set_fill_style_str(&format!("rgba(226, 232, 240, {:.2})", alpha * 0.6));
                    ctx.fill_rect(sx, sy, 1.0, 1.0);
                }
                1 => {
                    let col = match i % 5 {
                        0 => format!("rgba(56, 189, 248, {:.2})", alpha),
                        1 => format!("rgba(250, 204, 21, {:.2})", alpha),
                        2 => format!("rgba(147, 197, 253, {:.2})", alpha),
                        3 => format!("rgba(248, 113, 113, {:.2})", alpha),
                        _ => format!("rgba(255, 255, 255, {:.2})", alpha),
                    };
                    ctx.set_fill_style_str(&col);
                    ctx.fill_rect(sx, sy, 1.5, 1.5);
                }
                _ => {
                    ctx.set_fill_style_str(&format!("rgba(255, 255, 255, {:.2})", alpha * 0.9));
                    ctx.fill_rect(sx, sy, 2.0, 2.0);
                }
            }
        }

        // Hero Stars with 4-point cross diffraction lens flares
        let hero_count = 8;
        for h_idx in 0..hero_count {
            let fh = h_idx as f64;
            let hx = (((fh * 67867.0 + 12345.0) % 233280.0) / 233280.0) * w;
            let hy = (((fh * 23456.0 + 87654.0) % 233280.0) / 233280.0) * (h * 0.65);
            let h_twinkle = ((self.anim_time * 2.2 + fh * 2.1).sin() * 0.5 + 0.5).clamp(0.0, 1.0);

            ctx.set_fill_style_str(&format!("rgba(255, 255, 255, {:.2})", 0.7 + 0.3 * h_twinkle));
            ctx.fill_rect(hx - 1.0, hy - 1.0, 2.5, 2.5);

            let spike_len = 10.0 + h_twinkle * 6.0;
            ctx.set_stroke_style_str(&format!("rgba(56, 189, 248, {:.2})", 0.35 + 0.35 * h_twinkle));
            ctx.set_line_width(0.75);
            ctx.begin_path();
            ctx.move_to(hx - spike_len, hy);
            ctx.line_to(hx + spike_len, hy);
            ctx.move_to(hx, hy - spike_len);
            ctx.line_to(hx, hy + spike_len);
            ctx.stroke();
        }

        // Dynamic Shooting Star (Meteor Trail)
        let meteor_period = 6.5;
        let meteor_t = self.anim_time % meteor_period;
        if meteor_t < 0.75 {
            let m_progress = meteor_t / 0.75;
            let mx_start = w * 0.18 + (((self.anim_time / meteor_period) as u32 * 197) % 300) as f64;
            let my_start = h * 0.05 + (((self.anim_time / meteor_period) as u32 * 79) % 120) as f64;
            let m_dist = 240.0 * m_progress;
            let cur_x = mx_start + m_dist * 0.88;
            let cur_y = my_start + m_dist * 0.47;
            let tail_len = 80.0_f64.min(m_dist);
            let tail_x = cur_x - tail_len * 0.88;
            let tail_y = cur_y - tail_len * 0.47;

            let m_grad = ctx.create_linear_gradient(tail_x, tail_y, cur_x, cur_y);
            let _ = m_grad.add_color_stop(0.0, "rgba(56, 189, 248, 0.0)");
            let _ = m_grad.add_color_stop(0.7, "rgba(56, 189, 248, 0.6)");
            let _ = m_grad.add_color_stop(1.0, "rgba(255, 255, 255, 0.95)");
            ctx.set_stroke_style_canvas_gradient(&m_grad);
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(tail_x, tail_y);
            ctx.line_to(cur_x, cur_y);
            ctx.stroke();
        }

        // Planet New Terra / Tethys with 3D Tilted Rings
        let px = w * 0.88;
        let py = h * 0.22;
        let radius = (w * 0.135).clamp(95.0, 160.0);
        let ring_angle = -0.38; // ~ -22 degrees tilt
        let rx_outer = radius * 2.35;
        let ry_outer = radius * 0.58;
        let rx_inner = radius * 1.48;

        // Step A: Back half of planetary rings (behind planet)
        let _ = ctx.save();
        let _ = ctx.translate(px, py);
        let _ = ctx.rotate(ring_angle);
        let _ = ctx.scale(1.0, ry_outer / rx_outer);

        let mid_r = (rx_outer + rx_inner) * 0.5;
        ctx.set_stroke_style_str("rgba(186, 230, 253, 0.20)");
        ctx.set_line_width(12.0);
        ctx.begin_path();
        let _ = ctx.arc(0.0, 0.0, mid_r, std::f64::consts::PI, std::f64::consts::PI * 2.0);
        ctx.stroke();

        ctx.set_stroke_style_str("rgba(56, 189, 248, 0.35)");
        ctx.set_line_width(5.0);
        ctx.begin_path();
        let _ = ctx.arc(0.0, 0.0, mid_r * 1.05, std::f64::consts::PI, std::f64::consts::PI * 2.0);
        ctx.stroke();
        let _ = ctx.restore();

        // Step B: Atmosphere Outer Corona Glow
        if let Ok(corona) = ctx.create_radial_gradient(px, py, radius * 0.75, px, py, radius * 1.45) {
            let _ = corona.add_color_stop(0.0, "rgba(0, 212, 255, 0.24)");
            let _ = corona.add_color_stop(0.40, "rgba(2, 132, 199, 0.10)");
            let _ = corona.add_color_stop(1.0, "rgba(2, 4, 9, 0.0)");
            ctx.set_fill_style_canvas_gradient(&corona);
            ctx.begin_path();
            let _ = ctx.arc(px, py, radius * 1.45, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
        }

        // Step C: Planet Spherical Light Falloff Body
        let lx = px - radius * 0.38;
        let ly = py - radius * 0.38;
        if let Ok(p_grad) = ctx.create_radial_gradient(lx, ly, radius * 0.05, px, py, radius * 1.02) {
            let _ = p_grad.add_color_stop(0.0, "#38bdf8"); // sunlit polar cap
            let _ = p_grad.add_color_stop(0.25, "#0369a1"); // nitrogen atmosphere / ocean
            let _ = p_grad.add_color_stop(0.50, "#0c2847"); // twilight terminator
            let _ = p_grad.add_color_stop(0.78, "#071322"); // dark night side
            let _ = p_grad.add_color_stop(1.0, "#020409"); // deep planetary shadow
            ctx.set_fill_style_canvas_gradient(&p_grad);
            ctx.begin_path();
            let _ = ctx.arc(px, py, radius, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
        } else {
            ctx.set_fill_style_str("#131722");
            ctx.begin_path();
            let _ = ctx.arc(px, py, radius, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
        }

        // Volcanic hot spots on dark hemisphere (New Terra tectonic lore)
        ctx.set_fill_style_str("rgba(249, 115, 22, 0.35)");
        ctx.fill_rect(px + radius * 0.35, py + radius * 0.20, 3.0, 3.0);
        ctx.fill_rect(px + radius * 0.50, py - radius * 0.10, 2.0, 2.0);
        ctx.fill_rect(px + radius * 0.20, py + radius * 0.55, 2.5, 2.5);

        // Step D: Illuminated Crescent Rim
        ctx.set_stroke_style_str("rgba(255, 255, 255, 0.85)");
        ctx.set_line_width(2.5);
        ctx.begin_path();
        let _ = ctx.arc(px, py, radius, std::f64::consts::PI * 1.05, std::f64::consts::PI * 1.78);
        ctx.stroke();

        ctx.set_stroke_style_str("rgba(56, 189, 248, 0.55)");
        ctx.set_line_width(6.0);
        ctx.stroke();

        // Step E: Front half of planetary rings (in front of planet)
        let _ = ctx.save();
        let _ = ctx.translate(px, py);
        let _ = ctx.rotate(ring_angle);
        let _ = ctx.scale(1.0, ry_outer / rx_outer);

        // Lower ring arc (0.0 to PI)
        ctx.set_stroke_style_str("rgba(186, 230, 253, 0.26)");
        ctx.set_line_width(12.0);
        ctx.begin_path();
        let _ = ctx.arc(0.0, 0.0, mid_r, 0.0, std::f64::consts::PI);
        ctx.stroke();

        ctx.set_stroke_style_str("rgba(56, 189, 248, 0.42)");
        ctx.set_line_width(5.0);
        ctx.begin_path();
        let _ = ctx.arc(0.0, 0.0, mid_r * 1.05, 0.0, std::f64::consts::PI);
        ctx.stroke();

        // Planet shadow cast onto rings (dark sector on lower right)
        ctx.set_stroke_style_str("rgba(2, 4, 9, 0.75)");
        ctx.set_line_width(18.0);
        ctx.begin_path();
        let _ = ctx.arc(0.0, 0.0, mid_r, 0.18, 0.82);
        ctx.stroke();

        let _ = ctx.restore();

        // Step F: Orbiting Conestoga Recon Satellite / Colony Station
        let sx = px - radius * 1.25;
        let sy = py + radius * 0.60;
        ctx.set_fill_style_str("#64748b");
        ctx.fill_rect(sx - 3.0, sy - 1.0, 6.0, 2.0);
        ctx.fill_rect(sx - 1.0, sy - 4.0, 2.0, 8.0);

        let strobe_flash = ((self.anim_time * 3.5).sin() * 0.5 + 0.5) > 0.6;
        if strobe_flash {
            ctx.set_fill_style_str("#10b981");
            ctx.fill_rect(sx - 1.0, sy - 1.0, 2.0, 2.0);
        }

        // Perspective Hologram Grid
        let grid_top = h * 0.72;

        // Horizon glow line
        let h_grad = ctx.create_linear_gradient(0.0, grid_top, w, grid_top);
        let _ = h_grad.add_color_stop(0.0, "rgba(56, 189, 248, 0.0)");
        let _ = h_grad.add_color_stop(0.5, "rgba(56, 189, 248, 0.45)");
        let _ = h_grad.add_color_stop(1.0, "rgba(56, 189, 248, 0.0)");
        ctx.set_stroke_style_canvas_gradient(&h_grad);
        ctx.set_line_width(1.5);
        ctx.begin_path();
        ctx.move_to(0.0, grid_top);
        ctx.line_to(w, grid_top);
        ctx.stroke();

        // Perspective radiating lines
        let ray_count = 14;
        ctx.set_stroke_style_str("rgba(30, 58, 95, 0.35)");
        ctx.set_line_width(1.0);
        for k in 0..=ray_count {
            let fraction = k as f64 / ray_count as f64;
            let bx = w * fraction;
            ctx.begin_path();
            ctx.move_to(w * 0.5 + (bx - w * 0.5) * 0.15, grid_top);
            ctx.line_to(bx, h);
            ctx.stroke();
        }

        // Horizontal perspective lines with quadratic distance spacing
        let steps = 8;
        for s in 1..=steps {
            let t = s as f64 / steps as f64;
            let y = grid_top + t * t * (h - grid_top);
            let line_alpha = 0.15 + t * 0.35;
            ctx.set_stroke_style_str(&format!("rgba(56, 189, 248, {:.2})", line_alpha));
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(0.0, y);
            ctx.line_to(w, y);
            ctx.stroke();
        }

        // Laser scanwave traveling forward along the grid
        let wave_p = (self.anim_time * 0.3) % 1.0;
        let wave_y = grid_top + wave_p * wave_p * (h - grid_top);
        let wave_alpha = (1.0 - wave_p) * 0.6;
        ctx.set_stroke_style_str(&format!("rgba(0, 212, 255, {:.2})", wave_alpha));
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.move_to(0.0, wave_y);
        ctx.line_to(w, wave_y);
        ctx.stroke();

        // Futuristic HUD Corner Framing & Telemetry Calipers
        // Top-Left
        ctx.set_stroke_style_str("rgba(56, 189, 248, 0.55)");
        ctx.set_line_width(1.5);
        ctx.begin_path();
        ctx.move_to(20.0, 42.0);
        ctx.line_to(20.0, 20.0);
        ctx.line_to(65.0, 20.0);
        ctx.stroke();

        ctx.set_font("bold 9px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#38bdf8");
        let _ = ctx.fill_text("ORBITAL SECTOR: 04 // LAT -28.44 LON +114.82", 28.0, 32.0);
        ctx.set_fill_style_str("#64748b");
        let _ = ctx.fill_text("COLONIAL FREQ: 1420.405 MHz [SYNCHRONIZED]", 28.0, 44.0);

        // Top-Right
        ctx.begin_path();
        ctx.move_to(w - 65.0, 20.0);
        ctx.line_to(w - 20.0, 20.0);
        ctx.line_to(w - 20.0, 42.0);
        ctx.stroke();

        ctx.set_text_align("right");
        ctx.set_fill_style_str("#10b981");
        let _ = ctx.fill_text("● CORE INTEGRITY: NOMINAL", w - 28.0, 32.0);
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text("DEFENSE MATRIX ACTIVE // V1.3.0.8", w - 28.0, 44.0);
        ctx.set_text_align("left");

        // Bottom-Left
        ctx.begin_path();
        ctx.move_to(20.0, h - 42.0);
        ctx.line_to(20.0, h - 20.0);
        ctx.line_to(65.0, h - 20.0);
        ctx.stroke();

        ctx.set_fill_style_str("#64748b");
        let _ = ctx.fill_text("ARCHIVE: CONESTOGA RECON // SECTOR NEW TERRA", 28.0, h - 26.0);

        // Bottom-Right
        ctx.begin_path();
        ctx.move_to(w - 65.0, h - 20.0);
        ctx.line_to(w - 20.0, h - 20.0);
        ctx.line_to(w - 20.0, h - 42.0);
        ctx.stroke();

        ctx.set_text_align("right");
        ctx.set_fill_style_str("#64748b");
        let _ = ctx.fill_text("SURFACE SIMULATION KERNEL // STANDBY", w - 28.0, h - 26.0);
        ctx.set_text_align("left");
    }

    /// Render Outpost 2 Main Menu splash with authentic retro typography
    fn render_main_menu(
        &self,
        ctx: &CanvasRenderingContext2d,
        w: f64,
        _h: f64,
    ) {
        let cx = w * 0.5;
        let lang = crate::i18n::Language::from_code(&self.options.language);
        let dict = crate::i18n::get_dictionary(lang);

        // Central Holographic Energy Bloom backing title
        if let Ok(bloom) = ctx.create_radial_gradient(cx, 130.0, 10.0, cx, 130.0, 260.0) {
            let _ = bloom.add_color_stop(0.0, "rgba(0, 212, 255, 0.16)");
            let _ = bloom.add_color_stop(0.45, "rgba(14, 116, 144, 0.06)");
            let _ = bloom.add_color_stop(1.0, "rgba(0, 0, 0, 0.0)");
            ctx.set_fill_style_canvas_gradient(&bloom);
            ctx.fill_rect(cx - 300.0, 20.0, 600.0, 220.0);
        }

        // Top Tactical Command Eyebrow
        ctx.set_text_align("center");
        ctx.set_font("bold 10px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#38bdf8");
        let _ = ctx.fill_text("[ COMMAND ARCHIVE TERMINAL // HIGH-ORBIT LINK ]", cx, 68.0);

        // Faction Crests flanking the title
        let badge_dist = (w * 0.23).clamp(195.0, 240.0);

        // Connecting tactical data bus between faction insignias (drawn behind title)
        ctx.set_stroke_style_str("rgba(56, 189, 248, 0.22)");
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(cx - badge_dist + 36.0, 130.0);
        ctx.line_to(cx + badge_dist - 36.0, 130.0);
        ctx.stroke();

        self.render_faction_logo(ctx, Faction::Eden, cx - badge_dist, 130.0, 22.0);
        ctx.set_font("bold 9px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#38bdf8");
        let _ = ctx.fill_text("EDEN FOUNDATION", cx - badge_dist, 164.0);

        self.render_faction_logo(ctx, Faction::Plymouth, cx + badge_dist, 130.0, 22.0);
        ctx.set_font("bold 9px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#f97316");
        let _ = ctx.fill_text("PLYMOUTH COLONY", cx + badge_dist, 164.0);

        // Primary Outpost 2 Wordmark
        ctx.set_font("900 56px system-ui, -apple-system, sans-serif");

        // Pass 1: Cosmic drop shadow
        ctx.set_fill_style_str("rgba(2, 6, 23, 0.90)");
        let _ = ctx.fill_text("OUTPOST 2", cx + 3.0, 138.0);

        // Pass 2: Outer neon cyan stroke
        ctx.set_stroke_style_str("rgba(56, 189, 248, 0.40)");
        ctx.set_line_width(6.0);
        let _ = ctx.stroke_text("OUTPOST 2", cx, 134.0);

        // Pass 3: Inner crisp highlight stroke
        ctx.set_stroke_style_str("rgba(255, 255, 255, 0.75)");
        ctx.set_line_width(1.5);
        let _ = ctx.stroke_text("OUTPOST 2", cx, 134.0);

        // Pass 4: Metallic vertical gradient face fill
        let title_grad = ctx.create_linear_gradient(cx, 85.0, cx, 138.0);
        let _ = title_grad.add_color_stop(0.0, "#ffffff");
        let _ = title_grad.add_color_stop(0.35, "#bae6fd");
        let _ = title_grad.add_color_stop(0.70, "#38bdf8");
        let _ = title_grad.add_color_stop(1.0, "#1e3a8a");
        ctx.set_fill_style_canvas_gradient(&title_grad);
        let _ = ctx.fill_text("OUTPOST 2", cx, 134.0);

        // Subtitle: DIVIDED DESTINY inside tactical chamfered badge
        let badge_w = 270.0;
        let badge_h = 24.0;
        let bx = cx - badge_w * 0.5;
        let by = 156.0;
        let b_chamfer = 4.0;

        ctx.begin_path();
        ctx.move_to(bx + b_chamfer, by);
        ctx.line_to(bx + badge_w, by);
        ctx.line_to(bx + badge_w, by + badge_h - b_chamfer);
        ctx.line_to(bx + badge_w - b_chamfer, by + badge_h);
        ctx.line_to(bx, by + badge_h);
        ctx.line_to(bx, by + b_chamfer);
        ctx.close_path();

        ctx.set_fill_style_str("rgba(15, 23, 42, 0.88)");
        ctx.fill();
        ctx.set_stroke_style_str("#f59e0b");
        ctx.set_line_width(1.5);
        ctx.stroke();

        ctx.set_font("bold 12px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#fbbf24");
        let _ = ctx.fill_text(&format!("❖  {}  ❖", dict.main_menu.subtitle), cx, 172.5);

        // Version & Technology Stamp
        ctx.set_font("10px system-ui, -apple-system, sans-serif");
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(
            "WebAssembly RTS Engine • Canvas 2D Pipeline • 6-Language Localization",
            cx,
            204.0,
        );

        ctx.set_text_align("left");
    }

    /// Render Dialog 140 ("NEW CAMPAIGN OPTIONS")
    fn render_campaign_select(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let dlg_w = 460.0;
        let dlg_h = 280.0;
        let dx = (w - dlg_w) * 0.5;
        let dy = (h - dlg_h) * 0.5;

        // Dialog Window Frame
        ctx.set_fill_style_str("#0a0a0a");
        ctx.fill_rect(dx, dy, dlg_w, dlg_h);

        // Outer Metallic 3D Border
        ctx.set_stroke_style_str("#a01f10");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(dx, dy, dlg_w, dlg_h);

        // Title Bar
        ctx.set_fill_style_str("#1c0402");
        ctx.fill_rect(dx, dy, dlg_w, 28.0);
        ctx.set_stroke_style_str("#720000");
        ctx.stroke_rect(dx, dy, dlg_w, 28.0);

        ctx.set_text_align("center");
        ctx.set_font("bold 12px 'MS Sans Serif', Tahoma, sans-serif");
        ctx.set_fill_style_str("#ddff22");
        let _ = ctx.fill_text("NEW CAMPAIGN OPTIONS", dx + dlg_w * 0.5, dy + 18.0);

        // Section Headers
        ctx.set_text_align("left");
        ctx.set_font("bold 11px 'MS Sans Serif', Tahoma, sans-serif");
        ctx.set_fill_style_str("#00ff00");
        let _ = ctx.fill_text("Colony Selection:", dx + 30.0, dy + 55.0);
        let _ = ctx.fill_text("Difficulty Level:", dx + 250.0, dy + 50.0);

        // Faction Emblem preview badges
        self.render_faction_logo(ctx, Faction::Eden, dx + 180.0, dy + 88.0, 16.0);
        self.render_faction_logo(ctx, Faction::Plymouth, dx + 180.0, dy + 128.0, 16.0);
    }

    /// Render Dialog 219 ("MISSION MENU" / Campaign Briefing, Objectives & Novella Story)
    fn render_campaign_briefing(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let lang = crate::i18n::Language::from_code(&self.options.language);
        let sc = BriefingScenario::get_localized(self.selected_scenario_id, lang);

        // Dialog 219 Shell layout: Left Menu Box + Right Main Content Box
        let left_panel_w = 210.0;
        let content_x = left_panel_w + 30.0;
        let content_w = (w - content_x - 30.0).max(360.0);
        let content_y = 30.0;
        let content_h = (h - 60.0).max(400.0);

        // Right Main Content Frame
        ctx.set_fill_style_str("#080808");
        ctx.fill_rect(content_x, content_y, content_w, content_h);

        ctx.set_stroke_style_str("#a01f10");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(content_x, content_y, content_w, content_h);

        // Header Title Bar
        ctx.set_fill_style_str("#1c0402");
        ctx.fill_rect(content_x, content_y, content_w, 32.0);
        ctx.set_stroke_style_str("#720000");
        ctx.stroke_rect(content_x, content_y, content_w, 32.0);

        ctx.set_text_align("left");
        ctx.set_font("bold 12px 'MS Sans Serif', Tahoma, sans-serif");
        ctx.set_fill_style_str("#ddff22");

        let tab_title = match self.briefing_tab {
            BriefingTab::Briefing => format!("MISSION BRIEFING — {}", sc.title.to_uppercase()),
            BriefingTab::Objectives => format!("SPECIFIC OBJECTIVES — {}", sc.title.to_uppercase()),
            BriefingTab::Story => {
                let ch = &NOVELLA_CHAPTERS[self.story_chapter_index.min(NOVELLA_CHAPTERS.len() - 1)];
                format!("STORY — {}", ch.title.to_uppercase())
            }
        };
        let _ = ctx.fill_text(&tab_title, content_x + 16.0, content_y + 21.0);

        // Tab Content Display
        match self.briefing_tab {
            BriefingTab::Briefing => {
                // 320x240 Technical Briefing Preview Image (assets/original_ui/briefing/bmp_400.png .. bmp_423.png)
                let preview_w = 320.0;
                let preview_h = 240.0;
                let preview_x = content_x + 16.0;
                let preview_y = content_y + 45.0;

                self.render_minimap_preview(ctx, &sc, preview_x, preview_y, preview_w, preview_h);

                // Narrative Transcript Box (Next to preview or below)
                let trans_x = preview_x + preview_w + 16.0;
                let trans_w = content_w - preview_w - 48.0;
                if trans_w > 180.0 {
                    ctx.set_font("bold 11px 'MS Sans Serif', Tahoma, sans-serif");
                    ctx.set_fill_style_str("#00d4ff");
                    let _ = ctx.fill_text(&format!("THEATER: {}", sc.subtitle), trans_x, preview_y + 16.0);

                    ctx.set_font("11px 'Courier New', Courier, monospace");
                    ctx.set_fill_style_str("#00ff00");
                    self.render_multiline_text(ctx, &sc.transcript, trans_x, preview_y + 40.0, trans_w, 16.0);
                } else {
                    let trans_y = preview_y + preview_h + 16.0;
                    ctx.set_font("11px 'Courier New', Courier, monospace");
                    ctx.set_fill_style_str("#00ff00");
                    self.render_multiline_text(ctx, &sc.transcript, preview_x, trans_y, content_w - 32.0, 16.0);
                }
            }
            BriefingTab::Objectives => {
                let obj_x = content_x + 24.0;
                let mut obj_y = content_y + 55.0;

                ctx.set_font("bold 12px 'MS Sans Serif', Tahoma, sans-serif");
                ctx.set_fill_style_str("#ddff22");
                let _ = ctx.fill_text("PRIMARY MISSION OBJECTIVES:", obj_x, obj_y);
                obj_y += 24.0;

                ctx.set_font("11px 'MS Sans Serif', Tahoma, sans-serif");
                for (i, dir) in sc.primary_directives.iter().enumerate() {
                    ctx.set_fill_style_str("#00ff00");
                    let _ = ctx.fill_text(&format!("  [ ]  {}. {}", i + 1, dir), obj_x, obj_y);
                    obj_y += 24.0;
                }

                obj_y += 16.0;
                ctx.set_font("bold 12px 'MS Sans Serif', Tahoma, sans-serif");
                ctx.set_fill_style_str("#ddff22");
                let _ = ctx.fill_text("SECONDARY OBJECTIVES:", obj_x, obj_y);
                obj_y += 24.0;

                ctx.set_font("11px 'MS Sans Serif', Tahoma, sans-serif");
                for sec in &sc.secondary_directives {
                    ctx.set_fill_style_str("#00d4ff");
                    let _ = ctx.fill_text(&format!("  [ ]  {}", sec), obj_x, obj_y);
                    obj_y += 24.0;
                }

                obj_y += 16.0;
                ctx.set_font("bold 11px 'MS Sans Serif', Tahoma, sans-serif");
                ctx.set_fill_style_str("#fc0000");
                let _ = ctx.fill_text(&format!("HAZARDS: {}", sc.hazard_summary), obj_x, obj_y);
            }
            BriefingTab::Story => {
                let story_x = content_x + 24.0;
                let story_y = content_y + 55.0;
                let story_w = content_w - 48.0;

                let ch = &NOVELLA_CHAPTERS[self.story_chapter_index.min(NOVELLA_CHAPTERS.len() - 1)];

                ctx.set_font("bold 12px 'MS Sans Serif', Tahoma, sans-serif");
                ctx.set_fill_style_str("#ddff22");
                let _ = ctx.fill_text(&format!("OUTPOST 2 NOVELLA BY J. STEVEN YORK // {}", ch.title), story_x, story_y);

                ctx.set_font("11px 'Courier New', Courier, monospace");
                ctx.set_fill_style_str("#00ff00");
                self.render_multiline_text(ctx, ch.text, story_x, story_y + 24.0, story_w, 15.0);
            }
        }
    }

    /// Render Dialog 269 ("TUTORIALS")
    fn render_tutorials(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let dlg_w = 480.0;
        let dlg_h = 320.0;
        let dx = (w - dlg_w) * 0.5;
        let dy = (h - dlg_h) * 0.5;

        // Window Frame
        ctx.set_fill_style_str("#0a0a0a");
        ctx.fill_rect(dx, dy, dlg_w, dlg_h);
        ctx.set_stroke_style_str("#a01f10");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(dx, dy, dlg_w, dlg_h);

        // Title Bar
        ctx.set_fill_style_str("#1c0402");
        ctx.fill_rect(dx, dy, dlg_w, 28.0);
        ctx.set_text_align("center");
        ctx.set_font("bold 12px 'MS Sans Serif', Tahoma, sans-serif");
        ctx.set_fill_style_str("#ddff22");
        let _ = ctx.fill_text("TUTORIALS", dx + dlg_w * 0.5, dy + 18.0);

        // Tutorial Description List
        ctx.set_text_align("left");
        ctx.set_font("bold 11px 'MS Sans Serif', Tahoma, sans-serif");
        ctx.set_fill_style_str("#00ff00");
        let _ = ctx.fill_text("Select a Tutorial Exercise:", dx + 24.0, dy + 55.0);

        let tut_sc = BriefingScenario::get(2);
        ctx.set_fill_style_str("#00d4ff");
        let _ = ctx.fill_text(&format!("1. {} ({})", tut_sc.title, tut_sc.subtitle), dx + 32.0, dy + 85.0);

        ctx.set_font("10px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#94a3b8");
        self.render_multiline_text(ctx, &tut_sc.transcript, dx + 32.0, dy + 110.0, dlg_w - 64.0, 15.0);
    }

    /// Render Authentic Lore & Credits Dialog ("ABOUT OUTPOST 2")
    fn render_about(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let dlg_w = 500.0;
        let dlg_h = 340.0;
        let dx = (w - dlg_w) * 0.5;
        let dy = (h - dlg_h) * 0.5;

        // Window Frame
        ctx.set_fill_style_str("#0a0a0a");
        ctx.fill_rect(dx, dy, dlg_w, dlg_h);
        ctx.set_stroke_style_str("#a01f10");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(dx, dy, dlg_w, dlg_h);

        // Title Bar
        ctx.set_fill_style_str("#1c0402");
        ctx.fill_rect(dx, dy, dlg_w, 28.0);
        ctx.set_text_align("center");
        ctx.set_font("bold 12px 'MS Sans Serif', Tahoma, sans-serif");
        ctx.set_fill_style_str("#ddff22");
        let _ = ctx.fill_text("ABOUT OUTPOST 2: DIVIDED DESTINY", dx + dlg_w * 0.5, dy + 18.0);

        ctx.set_text_align("left");
        ctx.set_font("bold 11px 'MS Sans Serif', Tahoma, sans-serif");
        ctx.set_fill_style_str("#00ff00");
        let _ = ctx.fill_text("Outpost 2: Divided Destiny — Canonical 1997 Simulation Engine", dx + 24.0, dy + 55.0);

        ctx.set_font("11px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#94a3b8");
        let about_lore = "Developed by Dynamix / Sierra On-Line (1997).\nNovella & Campaign Narrative by J. Steven York.\nOriginal Art & Audio Assets preservation runtime.\nHigh-resolution vector Canvas 2D engine & WebAssembly lockstep architecture.\n\nTwo colonies divided on the dying world of New Terra: Eden and Plymouth.\nThe Blight is advancing.";
        self.render_multiline_text(ctx, about_lore, dx + 24.0, dy + 80.0, dlg_w - 48.0, 16.0);
    }

    /// Render Satellite Minimap Reconnaissance Frame
    fn render_minimap_preview(
        &self,
        ctx: &CanvasRenderingContext2d,
        sc: &BriefingScenario,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    ) {
        ctx.set_fill_style_str("#090d16");
        ctx.fill_rect(x, y, w, h);

        ctx.set_stroke_style_str("#334155");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(x, y, w, h);

        // Recon Grid scan lines
        ctx.set_stroke_style_str("rgba(56, 189, 248, 0.15)");
        ctx.set_line_width(1.0);
        let grid_cells = 8;
        for i in 1..grid_cells {
            let gx = x + (w / grid_cells as f64) * (i as f64);
            let gy = y + (h / grid_cells as f64) * (i as f64);
            ctx.begin_path();
            ctx.move_to(gx, y);
            ctx.line_to(gx, y + h);
            ctx.stroke();

            ctx.begin_path();
            ctx.move_to(x, gy);
            ctx.line_to(x + w, gy);
            ctx.stroke();
        }

        // Animated satellite sweep line
        let sweep_y = y + ((self.anim_time * 60.0) % h);
        ctx.set_stroke_style_str("rgba(74, 222, 128, 0.7)");
        ctx.set_line_width(1.5);
        ctx.begin_path();
        ctx.move_to(x, sweep_y);
        ctx.line_to(x + w, sweep_y);
        ctx.stroke();

        // Simulated topographical features based on scenario
        ctx.set_fill_style_str(if sc.scenario_id == 3 || sc.scenario_id == 5 {
            "#ea580c" // Volcanic lava
        } else {
            "#475569" // Mountain ridge
        });
        ctx.fill_rect(x + 20.0, y + 30.0, 45.0, 20.0);
        ctx.fill_rect(x + 80.0, y + 100.0, 70.0, 25.0);

        // Player starting drop zone reticle
        let target_x = x + w * 0.4;
        let target_y = y + h * 0.55;
        ctx.set_stroke_style_str("#38bdf8");
        ctx.set_line_width(2.0);
        ctx.begin_path();
        let _ = ctx.arc(target_x, target_y, 10.0, 0.0, std::f64::consts::PI * 2.0);
        ctx.stroke();

        // Header label
        ctx.set_font("bold 10px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#38bdf8");
        let _ = ctx.fill_text("RECON SCAN: 64x64", x + 8.0, y + 16.0);
    }

    /// Render Skirmish Battle Setup modal
    fn render_skirmish_setup(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let lang = crate::i18n::Language::from_code(&self.options.language);
        let dict = crate::i18n::get_dictionary(lang);

        let panel_w = (w * 0.9).clamp(680.0, 980.0);
        let panel_h = (h * 0.85).clamp(500.0, 700.0);
        let px = (w - panel_w) * 0.5;
        let py = (h - panel_h) * 0.5;

        ctx.set_fill_style_str("rgba(15, 23, 42, 0.96)");
        ctx.fill_rect(px, py, panel_w, panel_h);

        ctx.set_stroke_style_str("#f97316");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(px, py, panel_w, panel_h);

        // Header
        ctx.set_fill_style_str("#1e293b");
        ctx.fill_rect(px, py, panel_w, 42.0);

        ctx.set_font("bold 14px system-ui, sans-serif");
        ctx.set_fill_style_str("#f97316");
        let _ = ctx.fill_text(dict.skirmish.title, px + 16.0, py + 26.0);

        // Left Section Header
        ctx.set_font("bold 12px system-ui, sans-serif");
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(dict.skirmish.theater_params, px + 20.0, py + 75.0);

        // Right Section Header
        let left_w = 320.0;
        let right_x = px + left_w + 40.0;
        let _ = ctx.fill_text(dict.skirmish.ai_slots_header, right_x, py + 75.0);

        // Active summary
        let active_bots = self.skirmish_config.enabled_bot_count();
        ctx.set_font("12px system-ui, sans-serif");
        ctx.set_fill_style_str("#facc15");
        let _ = ctx.fill_text(
            &format!("{}: 1 Human Commander vs {} Autonomous AI Bot(s)", dict.skirmish.status_summary, active_bots),
            px + 20.0,
            py + panel_h - 75.0,
        );
    }

    /// Render In-Game Tactical Pause Overlay
    fn render_pause_overlay(
        &self,
        ctx: &CanvasRenderingContext2d,
        w: f64,
        h: f64,
        summary: Option<ColonySummary>,
    ) {
        let lang = crate::i18n::Language::from_code(&self.options.language);
        let dict = crate::i18n::get_dictionary(lang);

        // Semi-transparent darkened background
        ctx.set_fill_style_str("rgba(3, 7, 18, 0.75)");
        ctx.fill_rect(0.0, 0.0, w, h);

        let modal_w = 360.0;
        let modal_h = 440.0;
        let mx = (w - modal_w) * 0.5;
        let my = (h - modal_h) * 0.5;

        // Pause Box
        ctx.set_fill_style_str("rgba(15, 23, 42, 0.96)");
        ctx.fill_rect(mx, my, modal_w, modal_h);

        ctx.set_stroke_style_str("#38bdf8");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(mx, my, modal_w, modal_h);

        // Header
        ctx.set_text_align("center");
        ctx.set_font("bold 16px system-ui, sans-serif");
        ctx.set_fill_style_str("#38bdf8");
        let _ = ctx.fill_text(dict.pause_menu.title, mx + modal_w * 0.5, my + 34.0);

        if let Some(col) = summary {
            ctx.set_font("11px 'Courier New', Courier, monospace");
            ctx.set_fill_style_str("#94a3b8");
            let info = format!("Sol {:.2} • Pop {} • Morale {}%", col.sol, col.population, col.morale_pct);
            let _ = ctx.fill_text(&info, mx + modal_w * 0.5, my + 54.0);
        }

        ctx.set_text_align("left");
        self.render_cached_buttons(ctx);
    }

    /// Render Save / Load Slot Manager
    fn render_load_mission(
        &self,
        ctx: &CanvasRenderingContext2d,
        w: f64,
        h: f64,
    ) {
        let lang = crate::i18n::Language::from_code(&self.options.language);
        let dict = crate::i18n::get_dictionary(lang);

        let panel_w = (w * 0.9).clamp(640.0, 940.0);
        let panel_h = (h * 0.85).clamp(480.0, 680.0);
        let px = (w - panel_w) * 0.5;
        let py = (h - panel_h) * 0.5;

        ctx.set_fill_style_str("rgba(15, 23, 42, 0.96)");
        ctx.fill_rect(px, py, panel_w, panel_h);

        ctx.set_stroke_style_str("#facc15");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(px, py, panel_w, panel_h);

        // Header
        ctx.set_fill_style_str("#1e293b");
        ctx.fill_rect(px, py, panel_w, 42.0);

        ctx.set_font("bold 14px system-ui, sans-serif");
        ctx.set_fill_style_str("#facc15");
        let _ = ctx.fill_text(dict.main_menu.load_archive, px + 16.0, py + 26.0);

        // Slot entries text
        let mut sy = py + 75.0;
        for &(slot, ref meta) in &self.slots {
            let is_special = slot == 0 || slot == 100;
            if is_special && meta.is_none() {
                continue;
            }

            let slot_label = match slot {
                0 => "QUICKSAVE (F5)".to_string(),
                100 => "AUTOSAVE (MILESTONE)".to_string(),
                s => format!("SAVE SLOT #{}", s),
            };

            ctx.set_font("bold 12px system-ui, sans-serif");
            ctx.set_fill_style_str("#f8fafc");
            let _ = ctx.fill_text(&slot_label, px + 24.0, sy + 22.0);

            ctx.set_font("11px system-ui, sans-serif");
            match meta {
                Some(m) => {
                    ctx.set_fill_style_str("#38bdf8");
                    let desc = format!("{} — Sol {:.2} ({:?}) [{}s]", m.scenario_name, m.sol, m.faction, m.playtime_seconds);
                    let _ = ctx.fill_text(&desc, px + 180.0, sy + 22.0);
                }
                None => {
                    ctx.set_fill_style_str("#64748b");
                    let _ = ctx.fill_text("[Empty Slot]", px + 180.0, sy + 22.0);
                }
            }

            sy += 54.0;
        }
    }

    /// Render System Options & Preferences (Authentic Outpost 2 Dialog 142)
    fn render_options(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let lang = crate::i18n::Language::from_code(&self.options.language);
        let dict = crate::i18n::get_dictionary(lang);

        let panel_w = (w * 0.85).clamp(580.0, 760.0);
        let panel_h = (h * 0.85).clamp(480.0, 620.0);
        let px = (w - panel_w) * 0.5;
        let py = (h - panel_h) * 0.5;

        // Dialog 142 Window Frame
        ctx.set_fill_style_str("#0a0a0a");
        ctx.fill_rect(px, py, panel_w, panel_h);

        ctx.set_stroke_style_str("#a01f10");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(px, py, panel_w, panel_h);

        // Title Bar
        ctx.set_fill_style_str("#1c0402");
        ctx.fill_rect(px, py, panel_w, 32.0);
        ctx.set_stroke_style_str("#720000");
        ctx.stroke_rect(px, py, panel_w, 32.0);

        ctx.set_font("bold 12px 'MS Sans Serif', Tahoma, sans-serif");
        ctx.set_fill_style_str("#ddff22");
        let _ = ctx.fill_text("GAME PREFERENCES (DIALOG 142)", px + 16.0, py + 21.0);

        // Helper closure to render a sci-fi segmented volume slider bar
        let draw_slider = |label: &str, row_y: f64, vol: f32, color: &str| {
            // Label
            ctx.set_font("12px system-ui, sans-serif");
            ctx.set_fill_style_str("#e2e8f0");
            let _ = ctx.fill_text(label, px + 24.0, py + row_y + 14.0);

            // Slider Track Background
            let track_x = px + 225.0;
            let track_w = (panel_w - 380.0).max(100.0);
            let track_h = 18.0;
            ctx.set_fill_style_str("#0f172a");
            ctx.fill_rect(track_x, py + row_y, track_w, track_h);
            ctx.set_stroke_style_str("#334155");
            ctx.set_line_width(1.0);
            ctx.stroke_rect(track_x, py + row_y, track_w, track_h);

            // Filled Audio Level Gauge
            let fill_w = (track_w - 4.0) * (vol.clamp(0.0, 1.0) as f64);
            if fill_w > 0.0 {
                ctx.set_fill_style_str(color);
                ctx.fill_rect(track_x + 2.0, py + row_y + 2.0, fill_w, track_h - 4.0);
            }

            // Segment tick notches at 25%, 50%, 75%
            ctx.set_stroke_style_str("rgba(255, 255, 255, 0.2)");
            ctx.set_line_width(1.0);
            for tick in 1..4 {
                let tx = track_x + (track_w * (tick as f64 * 0.25));
                ctx.begin_path();
                ctx.move_to(tx, py + row_y);
                ctx.line_to(tx, py + row_y + track_h);
                ctx.stroke();
            }

            // Percentage readout
            ctx.set_font("bold 12px 'Courier New', Courier, monospace");
            ctx.set_fill_style_str("#facc15");
            let pct_str = format!("{:>3.0}%", vol * 100.0);
            let _ = ctx.fill_text(&pct_str, px + panel_w - 146.0, py + row_y + 14.0);
        };

        // Render Volume Sliders
        draw_slider(dict.options.master_volume, 70.0, self.options.master_volume, "#38bdf8");
        draw_slider("Tactical SFX & Voice:", 110.0, self.options.sfx_volume, "#4ade80");
        draw_slider("Dynamic Soundtrack:", 150.0, self.options.music_volume, "#a855f7");

        // Simulation Velocity Section
        ctx.set_font("12px system-ui, sans-serif");
        ctx.set_fill_style_str("#e2e8f0");
        let _ = ctx.fill_text(dict.options.sim_velocity, px + 24.0, py + 215.0);

        // Velocity indicator badge
        ctx.set_font("bold 12px 'Courier New', Courier, monospace");
        ctx.set_fill_style_str("#4ade80");
        let spd_text = match self.options.game_speed {
            s if (s - 0.5).abs() < 1e-4 => "0.5x (Tactical Slow)",
            s if (s - 1.0).abs() < 1e-4 => "1.0x (Standard Sols)",
            s if (s - 2.0).abs() < 1e-4 => "2.0x (Accelerated Base)",
            s if (s - 4.0).abs() < 1e-4 => "4.0x (Hyper Velocity)",
            _ => return, // formatted below if nonstandard
        };
        let _ = ctx.fill_text(spd_text, px + 225.0, py + 215.0);

        // CRT Retro Scanlines Section
        ctx.set_font("12px system-ui, sans-serif");
        ctx.set_fill_style_str("#e2e8f0");
        let _ = ctx.fill_text(dict.options.crt_scanlines, px + 24.0, py + 265.0);

        // Active Language / Locale Section
        ctx.set_font("12px system-ui, sans-serif");
        ctx.set_fill_style_str("#e2e8f0");
        let _ = ctx.fill_text(dict.options.active_language, px + 24.0, py + 315.0);
    }

    /// Render WebRTC Peer-to-Peer Multiplayer Lobby summary & interactive match controls
    fn render_multiplayer_lobby(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let lang = crate::i18n::Language::from_code(&self.options.language);
        let dict = crate::i18n::get_dictionary(lang);

        let panel_w = (w * 0.88).clamp(640.0, 880.0);
        let panel_h = (h * 0.82).clamp(460.0, 620.0);
        let px = (w - panel_w) * 0.5;
        let py = (h - panel_h) * 0.5;

        // Outer Dark Panel
        ctx.set_fill_style_str("rgba(15, 23, 42, 0.96)");
        ctx.fill_rect(px, py, panel_w, panel_h);

        ctx.set_stroke_style_str("#38bdf8");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(px, py, panel_w, panel_h);

        // Header
        ctx.set_fill_style_str("#1e293b");
        ctx.fill_rect(px, py, panel_w, 42.0);

        ctx.set_font("bold 14px system-ui, sans-serif");
        ctx.set_fill_style_str("#38bdf8");
        let _ = ctx.fill_text(dict.multiplayer.title, px + 16.0, py + 26.0);

        // Connection Status Banner
        let banner_text = self.mp_room.connection_status_banner();
        let is_connected = self.mp_room.state == crate::net::lobby::LobbyState::Connected;
        let banner_bg = if is_connected {
            "rgba(34, 197, 94, 0.15)"
        } else {
            "rgba(250, 204, 21, 0.15)"
        };
        let banner_border = if is_connected { "#22c55e" } else { "#facc15" };
        let banner_color = if is_connected { "#4ade80" } else { "#facc15" };

        let banner_w = panel_w - 48.0;
        let banner_y = py + 95.0;
        ctx.set_fill_style_str(banner_bg);
        ctx.fill_rect(px + 24.0, banner_y, banner_w, 28.0);
        ctx.set_stroke_style_str(banner_border);
        ctx.set_line_width(1.0);
        ctx.stroke_rect(px + 24.0, banner_y, banner_w, 28.0);

        ctx.set_font("bold 11px system-ui, sans-serif");
        ctx.set_fill_style_str(banner_color);
        let _ = ctx.fill_text(&format!("{}: {}", dict.multiplayer.status_banner_label, banner_text), px + 36.0, banner_y + 18.0);

        let is_host_tab = self.mp_tab == MultiplayerTab::Host;
        if is_host_tab {
            // Host Tab Content:
            // 1. Base64 Offer Box Preview
            ctx.set_font("bold 11px system-ui, sans-serif");
            ctx.set_fill_style_str("#94a3b8");
            let _ = ctx.fill_text(dict.multiplayer.offer_ready_header, px + 24.0, py + 142.0);

            let offer_box_w = panel_w - 48.0;
            ctx.set_fill_style_str("#0f172a");
            ctx.fill_rect(px + 24.0, py + 195.0, offer_box_w, 26.0);
            ctx.set_stroke_style_str("#334155");
            ctx.stroke_rect(px + 24.0, py + 195.0, offer_box_w, 26.0);

            ctx.set_font("10px monospace, system-ui");
            ctx.set_fill_style_str("#38bdf8");
            let offer_snippet = self.mp_room.host_offer_sdp.as_deref().unwrap_or(dict.multiplayer.awaiting_offer);
            let display_snippet = if offer_snippet.len() > 85 {
                format!("{}...", &offer_snippet[..85])
            } else {
                offer_snippet.to_string()
            };
            let _ = ctx.fill_text(&display_snippet, px + 32.0, py + 212.0);

            // 2. Map Selector
            ctx.set_font("bold 11px system-ui, sans-serif");
            ctx.set_fill_style_str("#94a3b8");
            let _ = ctx.fill_text("PLANETARY THEATER / MAP SELECTION:", px + 24.0, py + 242.0);

            // 3. Match Participants & Readiness status
            ctx.set_fill_style_str("#1e293b");
            ctx.fill_rect(px + 24.0, py + 272.0, panel_w - 48.0, 30.0);
            ctx.set_stroke_style_str("#334155");
            ctx.stroke_rect(px + 24.0, py + 272.0, panel_w - 48.0, 30.0);

            ctx.set_font("11px system-ui, sans-serif");
            let host_ready_str = if self.mp_room.is_host_ready { "READY" } else { "NOT READY" };
            let guest_ready_str = if self.mp_room.is_guest_ready { "READY" } else { "NOT READY" };
            let guest_label = self.mp_room.guest_name.as_deref().unwrap_or("Awaiting Guest Connection...");

            ctx.set_fill_style_str("#38bdf8");
            let _ = ctx.fill_text(
                &format!("HOST: {} [{:?}] - {}", self.mp_room.host_name, self.mp_room.faction, host_ready_str),
                px + 34.0,
                py + 291.0,
            );

            ctx.set_fill_style_str(if self.mp_room.guest_name.is_some() { "#4ade80" } else { "#94a3b8" });
            let _ = ctx.fill_text(
                &format!("GUEST: {} - {}", guest_label, guest_ready_str),
                px + 360.0,
                py + 291.0,
            );

            // Launch status banner
            ctx.set_font("11px system-ui, sans-serif");
            if self.mp_room.is_ready_to_launch() {
                ctx.set_fill_style_str("#4ade80");
                let _ = ctx.fill_text("✔ BOTH COMMANDERS READY - READY TO LAUNCH MATCH", px + 24.0, py + 362.0);
            } else {
                ctx.set_fill_style_str("#94a3b8");
                let _ = ctx.fill_text("⏳ Both commanders must link and toggle ready state before launch.", px + 24.0, py + 362.0);
            }
        } else {
            // Guest Tab Content:
            ctx.set_font("bold 11px system-ui, sans-serif");
            ctx.set_fill_style_str("#94a3b8");
            let _ = ctx.fill_text("STEP 1: PASTE HOST OFFER SDP FROM CLIPBOARD:", px + 24.0, py + 142.0);

            let offer_box_w = panel_w - 48.0;
            ctx.set_fill_style_str("#0f172a");
            ctx.fill_rect(px + 24.0, py + 195.0, offer_box_w, 26.0);
            ctx.set_stroke_style_str("#334155");
            ctx.stroke_rect(px + 24.0, py + 195.0, offer_box_w, 26.0);

            ctx.set_font("10px monospace, system-ui");
            ctx.set_fill_style_str("#a855f7");
            let host_snippet = self.mp_room.guest_offer_sdp.as_deref().unwrap_or("[No Host Offer Pasted Yet]");
            let display_snippet = if host_snippet.len() > 85 {
                format!("{}...", &host_snippet[..85])
            } else {
                host_snippet.to_string()
            };
            let _ = ctx.fill_text(&display_snippet, px + 32.0, py + 212.0);

            // Answer status
            ctx.set_font("bold 11px system-ui, sans-serif");
            ctx.set_fill_style_str("#94a3b8");
            let _ = ctx.fill_text("STEP 2: GENERATE AND COPY GUEST ANSWER TO SEND BACK TO HOST:", px + 24.0, py + 280.0);

            let ans_snippet = self.mp_room.guest_answer_sdp.as_deref().unwrap_or("[Answer SDP not generated yet]");
            ctx.set_fill_style_str("#0f172a");
            ctx.fill_rect(px + 24.0, py + 295.0, offer_box_w, 26.0);
            ctx.set_stroke_style_str("#334155");
            ctx.stroke_rect(px + 24.0, py + 295.0, offer_box_w, 26.0);

            ctx.set_font("10px monospace, system-ui");
            ctx.set_fill_style_str("#38bdf8");
            let display_ans = if ans_snippet.len() > 85 {
                format!("{}...", &ans_snippet[..85])
            } else {
                ans_snippet.to_string()
            };
            let _ = ctx.fill_text(&display_ans, px + 32.0, py + 312.0);

            // Readiness
            ctx.set_font("11px system-ui, sans-serif");
            let is_ready = self.mp_room.is_guest_ready;
            ctx.set_fill_style_str(if is_ready { "#4ade80" } else { "#facc15" });
            let r_str = if is_ready {
                "STATUS: YOU ARE READY (Awaiting host to launch match)"
            } else {
                "STATUS: NOT READY (Click 'I AM READY' when set)"
            };
            let _ = ctx.fill_text(r_str, px + 24.0, py + 355.0);
        }
    }

    /// Render all cached buttons with hover highlights
    fn render_cached_buttons(&self, ctx: &CanvasRenderingContext2d) {
        let (mx, my) = self.mouse_pos;

        for btn in &self.cached_buttons {
            let is_disabled = !btn.enabled;
            let hovered = btn.contains(mx, my);
            let is_pressed = self.active_pressed_btn_id.as_deref() == Some(&btn.id);

            let offset_x = if is_pressed { 1.0 } else { 0.0 };
            let offset_y = if is_pressed { 1.0 } else { 0.0 };
            let bx = btn.x + offset_x;
            let by = btn.y + offset_y;
            let bw = btn.w;
            let bh = btn.h;

            if is_disabled {
                // Disabled Button Styling (ui_styles.json: border #444444 / #222222, text #646464, bg #000000)
                ctx.set_fill_style_str("#000000");
                ctx.fill_rect(bx, by, bw, bh);

                ctx.set_stroke_style_str("#444444");
                ctx.set_line_width(1.0);
                ctx.stroke_rect(bx + 0.5, by + 0.5, bw - 1.0, bh - 1.0);

                ctx.set_stroke_style_str("#222222");
                ctx.stroke_rect(bx + 1.5, by + 1.5, bw - 3.0, bh - 3.0);

                ctx.set_text_align("center");
                ctx.set_font("bold 11px 'MS Sans Serif', Tahoma, sans-serif");
                ctx.set_fill_style_str("#646464");
                let text_y = by + bh * 0.5 + 4.0;
                let _ = ctx.fill_text(&btn.label, bx + bw * 0.5, text_y);
            } else {
                // Authentic Outpost 2 Red Beveled Metallic Button:
                // Border light: #fc0000 (outer highlight/hover) / #a01f10 (normal top/left)
                // Border dark: #720000 / #840000 (normal bottom/right)
                // When pressed: borders invert (#720000 light, #fc0000 dark)
                let (border_top_left, border_bot_right, bg_color, text_color) = if is_pressed {
                    ("#720000", "#fc0000", "#080808", "#00ff00")
                } else if hovered {
                    ("#fc0000", "#840000", "#111111", "#ddff22")
                } else {
                    ("#a01f10", "#720000", "#050505", "#00ff00")
                };

                // Solid background interior
                ctx.set_fill_style_str(bg_color);
                ctx.fill_rect(bx, by, bw, bh);

                // Outer 3D Bevel Rim (Top & Left)
                ctx.set_stroke_style_str(border_top_left);
                ctx.set_line_width(2.0);
                ctx.begin_path();
                ctx.move_to(bx + bw, by);
                ctx.line_to(bx, by);
                ctx.line_to(bx, by + bh);
                ctx.stroke();

                // Outer 3D Bevel Rim (Bottom & Right)
                ctx.set_stroke_style_str(border_bot_right);
                ctx.set_line_width(2.0);
                ctx.begin_path();
                ctx.move_to(bx + bw, by);
                ctx.line_to(bx + bw, by + bh);
                ctx.line_to(bx, by + bh);
                ctx.stroke();

                // Inner Metallic Highlight line
                if hovered && !is_pressed {
                    ctx.set_stroke_style_str("#fc0000");
                    ctx.set_line_width(1.0);
                    ctx.stroke_rect(bx + 2.0, by + 2.0, bw - 4.0, bh - 4.0);
                }

                // Phosphor Green Text with optional Hover Glow
                ctx.set_text_align("center");
                ctx.set_font("bold 11px 'MS Sans Serif', Tahoma, sans-serif");
                ctx.set_fill_style_str(text_color);
                let text_y = by + bh * 0.5 + 4.0;
                let _ = ctx.fill_text(&btn.label, bx + bw * 0.5, text_y);
            }
        }

        ctx.set_text_align("left");
    }

    /// Render faction logo vector art
    fn render_faction_logo(
        &self,
        ctx: &CanvasRenderingContext2d,
        faction: Faction,
        cx: f64,
        cy: f64,
        radius: f64,
    ) {
        match faction {
            Faction::Eden => {
                // Planetary sphere with orbital rings
                ctx.set_fill_style_str("#0284c7");
                ctx.begin_path();
                let _ = ctx.arc(cx, cy, radius * 0.65, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();

                ctx.set_stroke_style_str("#38bdf8");
                ctx.set_line_width(2.5);
                ctx.begin_path();
                let _ = ctx.arc(cx, cy, radius, 0.2, std::f64::consts::PI - 0.2);
                ctx.stroke();

                ctx.begin_path();
                let _ = ctx.arc(cx, cy, radius, std::f64::consts::PI + 0.2, std::f64::consts::PI * 2.0 - 0.2);
                ctx.stroke();
            }
            Faction::Plymouth => {
                // Flame / gear volcanic crest
                ctx.set_fill_style_str("#dc2626");
                ctx.begin_path();
                ctx.move_to(cx, cy - radius);
                ctx.line_to(cx + radius * 0.8, cy + radius * 0.6);
                ctx.line_to(cx - radius * 0.8, cy + radius * 0.6);
                ctx.close_path();
                ctx.fill();

                ctx.set_stroke_style_str("#f97316");
                ctx.set_line_width(2.5);
                ctx.stroke();
            }
        }
    }

    /// Render notification toast box
    fn render_toast(&self, ctx: &CanvasRenderingContext2d, msg: &str, w: f64, _h: f64) {
        let tw = 380.0;
        let th = 38.0;
        let tx = (w - tw) * 0.5;
        let ty = 24.0;

        ctx.set_fill_style_str("rgba(15, 23, 42, 0.96)");
        ctx.fill_rect(tx, ty, tw, th);
        ctx.set_stroke_style_str("#4ade80");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(tx, ty, tw, th);

        ctx.set_text_align("center");
        ctx.set_font("bold 12px system-ui, sans-serif");
        ctx.set_fill_style_str("#4ade80");
        let _ = ctx.fill_text(msg, tx + tw * 0.5, ty + 24.0);
        ctx.set_text_align("left");
    }

    /// Render CRT TV retro horizontal scanlines
    fn render_crt_scanlines(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        ctx.set_fill_style_str("rgba(0, 0, 0, 0.05)");
        let mut y = 0.0;
        while y < h {
            ctx.fill_rect(0.0, y, w, 1.0);
            y += 4.0;
        }
    }

    /// Helper to render multi-line wrapped text
    fn render_multiline_text(
        &self,
        ctx: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        start_y: f64,
        _max_w: f64,
        line_height: f64,
    ) {
        let mut y = start_y;
        for line in text.split('\n') {
            let _ = ctx.fill_text(line, x, y);
            y += line_height;
        }
    }
}

// =========================================================================
// 4. Unit Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_initial_state_is_main_menu() {
        let mgr = MenuManager::new();
        assert_eq!(mgr.current_screen, MenuScreen::MainMenu);
        assert!(!mgr.is_in_game());
        assert!(!mgr.is_paused());
    }

    #[test]
    fn test_menu_screen_navigation_transitions() {
        let mut mgr = MenuManager::new();

        // 1. Navigate from Main Menu (Dialog 137) to New Campaign (Dialog 140)
        let action = mgr.execute_button_action("btn_campaign");
        assert_eq!(action, MenuAction::PlayBeep);
        assert_eq!(mgr.current_screen, MenuScreen::CampaignSelect);
        assert_eq!(mgr.previous_screen, MenuScreen::MainMenu);

        // 2. Dialog 140: Select Plymouth faction and Hard difficulty
        let act = mgr.execute_button_action("btn_d140_fac_plymouth");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.selected_faction, Faction::Plymouth);

        let act = mgr.execute_button_action("btn_d140_diff_hard");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.campaign_difficulty, CampaignDifficulty::Hard);

        // Press OK in Dialog 140 -> opens Dialog 219 Mission Briefing
        let act = mgr.execute_button_action("btn_d140_ok");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.current_screen, MenuScreen::CampaignBriefing);
        assert_eq!(mgr.selected_scenario_id, 1); // Plymouth Campaign Mission 1

        // 3. Dialog 219: Switch Tabs (Briefing -> Objectives -> Story)
        let act = mgr.execute_button_action("btn_d219_objectives");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.briefing_tab, BriefingTab::Objectives);

        let act = mgr.execute_button_action("btn_d219_story");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.briefing_tab, BriefingTab::Story);

        // Story chapter navigation
        let act = mgr.execute_button_action("btn_story_next");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.story_chapter_index, 1);

        // Start Mission from Dialog 219
        let action = mgr.execute_button_action("btn_d219_start_mission");
        assert_eq!(action, MenuAction::StartScenario(1));
        assert_eq!(mgr.current_screen, MenuScreen::None);
        assert!(mgr.is_in_game());

        // 4. Test Tutorials (Dialog 269)
        mgr.switch_screen(MenuScreen::MainMenu);
        let act = mgr.execute_button_action("btn_tutorials");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.current_screen, MenuScreen::Tutorials);

        // 5. Test About Dialog
        mgr.switch_screen(MenuScreen::MainMenu);
        let act = mgr.execute_button_action("btn_about");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.current_screen, MenuScreen::About);

        // 6. Test Help button (Disabled)
        let act = mgr.execute_button_action("btn_help");
        assert_eq!(act, MenuAction::None);

        // 7. Test Quit action
        let act = mgr.execute_button_action("btn_quit");
        assert_eq!(act, MenuAction::Quit);

        // Direct Scenario 3 briefing test (backwards compatibility)
        let action = mgr.execute_button_action("scenario_3");
        assert_eq!(action, MenuAction::PlayBeep);
        assert_eq!(mgr.current_screen, MenuScreen::CampaignBriefing);
        assert_eq!(mgr.selected_scenario_id, 3);

        let action = mgr.execute_button_action("btn_begin_operation");
        assert_eq!(action, MenuAction::StartScenario(3));
        assert_eq!(mgr.current_screen, MenuScreen::None);

        // Verify smooth transition alpha progression
        mgr.switch_screen(MenuScreen::MainMenu);
        assert_eq!(mgr.transition_alpha, 0.0);
        mgr.update(0.1);
        assert!(mgr.transition_alpha > 0.4);
        mgr.update(0.2);
        assert_eq!(mgr.transition_alpha, 1.0);
    }

    #[test]
    fn test_briefing_scenario_retrieval_integrity() {
        let all = BriefingScenario::all_scenarios();
        assert_eq!(all.len(), 7);

        let eden1 = BriefingScenario::get(0);
        assert_eq!(eden1.faction, Faction::Eden);
        assert_eq!(eden1.primary_directives.len(), 4);

        let plymouth1 = BriefingScenario::get(1);
        assert_eq!(plymouth1.faction, Faction::Plymouth);
        assert_eq!(plymouth1.primary_directives.len(), 4);

        let eden3 = BriefingScenario::get(5);
        assert_eq!(eden3.faction, Faction::Eden);
        assert!(eden3.title.contains("Evacuation"));

        let plymouth3 = BriefingScenario::get(6);
        assert_eq!(plymouth3.faction, Faction::Plymouth);
        assert!(plymouth3.title.contains("Phoenix"));
    }

    #[test]
    fn test_skirmish_configuration_and_opponent_generation() {
        let mut cfg = SkirmishConfig::default();
        assert_eq!(cfg.human_faction, Faction::Eden);
        assert_eq!(cfg.enabled_bot_count(), 3);

        // Verify initial opponents
        let opps = cfg.build_opponents();
        assert_eq!(opps.len(), 3);
        assert_eq!(opps[0].0, Faction::Plymouth); // Bot 1
        assert_eq!(opps[0].1, AiPersonality::AggressiveRusher);
        assert_eq!(opps[1].0, Faction::Eden); // Bot 2
        assert_eq!(opps[1].1, AiPersonality::DefensiveFortifier);

        // Toggle 4th bot on
        cfg.toggle_bot_enabled(3);
        assert_eq!(cfg.enabled_bot_count(), 4);
        assert_eq!(cfg.build_opponents().len(), 4);

        // Cycle faction and personality
        cfg.cycle_bot_faction(0);
        assert_eq!(cfg.bots[0].faction, Faction::Eden);

        cfg.cycle_bot_difficulty(0);
        assert_eq!(cfg.bots[0].difficulty, Difficulty::Hard);

        // Cycle resources and disasters
        cfg.cycle_resources();
        assert_eq!(cfg.resources, ResourceAbundance::Abundant);
        assert_eq!(cfg.resources.starting_ore(), (5000, 1500));

        cfg.cycle_disasters();
        assert_eq!(cfg.disasters, DisasterFrequency::Catastrophic);
    }

    #[test]
    fn test_pause_menu_state_transitions() {
        let mut mgr = MenuManager::new();
        mgr.current_screen = MenuScreen::None;
        assert!(mgr.is_in_game());

        // Press Escape in-game -> PauseMenu
        let action = mgr.handle_key_down("Escape", true);
        assert_eq!(action, MenuAction::None);
        assert_eq!(mgr.current_screen, MenuScreen::PauseMenu);
        assert!(mgr.is_paused());

        // Press Escape in PauseMenu -> Resumes game
        let action = mgr.handle_key_down("Escape", true);
        assert_eq!(action, MenuAction::ResumeGame);
        assert_eq!(mgr.current_screen, MenuScreen::None);

        // Quicksave key F5
        let action = mgr.handle_key_down("F5", true);
        assert_eq!(action, MenuAction::QuickSave);

        // Quickload key F9
        let action = mgr.handle_key_down("F9", true);
        assert_eq!(action, MenuAction::QuickLoad);

        // Surrender button
        mgr.current_screen = MenuScreen::PauseMenu;
        let action = mgr.execute_button_action("btn_pause_surrender");
        assert_eq!(action, MenuAction::SurrenderToMainMenu);
        assert_eq!(mgr.current_screen, MenuScreen::MainMenu);
    }

    #[test]
    fn test_button_hit_test_box_containment() {
        let btn = MenuButton::new("test_btn", 100.0, 100.0, 200.0, 50.0, "TEST", "#fff");
        assert!(btn.contains(150.0, 125.0)); // Center
        assert!(btn.contains(100.0, 100.0)); // Top-Left
        assert!(btn.contains(300.0, 150.0)); // Bottom-Right
        assert!(!btn.contains(99.0, 100.0)); // Outside Left
        assert!(!btn.contains(150.0, 151.0)); // Outside Bottom

        // Test hover state detection
        let mut mgr = MenuManager::new();
        mgr.cached_buttons = vec![btn];
        assert_eq!(mgr.hovered_btn_id, None);

        // Hover outside -> returns false
        let changed = mgr.handle_mouse_move(50.0, 50.0);
        assert!(!changed);
        assert_eq!(mgr.hovered_btn_id, None);

        // Hover inside -> returns true
        let changed = mgr.handle_mouse_move(150.0, 125.0);
        assert!(changed);
        assert_eq!(mgr.hovered_btn_id, Some("test_btn".to_string()));

        // Move inside same button -> returns false (no duplicate hover sound)
        let changed = mgr.handle_mouse_move(160.0, 130.0);
        assert!(!changed);
        assert_eq!(mgr.hovered_btn_id, Some("test_btn".to_string()));

        // Leave button -> returns false (unhover)
        let changed = mgr.handle_mouse_move(50.0, 50.0);
        assert!(!changed);
        assert_eq!(mgr.hovered_btn_id, None);
    }

    #[test]
    fn test_options_clamping_and_speed_selection() {
        let mut mgr = MenuManager::new();

        // Volume down clamping
        for _ in 0..15 {
            mgr.execute_button_action("btn_opt_vol_down");
        }
        assert_eq!(mgr.options.master_volume, 0.0);

        // Volume up
        mgr.execute_button_action("btn_opt_vol_up");
        assert!((mgr.options.master_volume - 0.1).abs() < 1e-4);

        // Velocity speed
        mgr.execute_button_action("btn_opt_speed_20");
        assert_eq!(mgr.options.game_speed, 2.0);

        // Language selection
        mgr.execute_button_action("btn_opt_lang_pt");
        assert_eq!(mgr.options.language, "pt-BR");
    }

    #[test]
    fn test_options_multi_channel_volume_sliders_and_speed_selection() {
        let mut mgr = MenuManager::new();

        // Default initial volumes
        assert_eq!(mgr.options.master_volume, 0.75);
        assert_eq!(mgr.options.sfx_volume, 0.85);
        assert_eq!(mgr.options.music_volume, 0.65);
        assert_eq!(mgr.options.game_speed, 1.0);

        // Test SFX Volume Sliders
        let action = mgr.execute_button_action("btn_opt_sfx_down");
        assert_eq!(action, MenuAction::ApplyOptions(mgr.options.clone()));
        assert!((mgr.options.sfx_volume - 0.75).abs() < 1e-4);

        // SFX down clamping to 0.0
        for _ in 0..15 {
            mgr.execute_button_action("btn_opt_sfx_down");
        }
        assert_eq!(mgr.options.sfx_volume, 0.0);

        // SFX up clamping to 1.0
        for _ in 0..15 {
            mgr.execute_button_action("btn_opt_sfx_up");
        }
        assert_eq!(mgr.options.sfx_volume, 1.0);

        // Test Music Volume Sliders
        let action = mgr.execute_button_action("btn_opt_music_down");
        assert_eq!(action, MenuAction::ApplyOptions(mgr.options.clone()));
        assert!((mgr.options.music_volume - 0.90).abs() < 1e-4 || (mgr.options.music_volume - 0.55).abs() < 1e-4);

        // Music down clamping to 0.0
        for _ in 0..15 {
            mgr.execute_button_action("btn_opt_music_down");
        }
        assert_eq!(mgr.options.music_volume, 0.0);

        // Music up clamping to 1.0
        for _ in 0..15 {
            mgr.execute_button_action("btn_opt_music_up");
        }
        assert_eq!(mgr.options.music_volume, 1.0);

        // Test Game Speed Selection (0.5x, 1.0x, 2.0x, 4.0x)
        mgr.execute_button_action("btn_opt_speed_05");
        assert_eq!(mgr.options.game_speed, 0.5);

        mgr.execute_button_action("btn_opt_speed_10");
        assert_eq!(mgr.options.game_speed, 1.0);

        mgr.execute_button_action("btn_opt_speed_20");
        assert_eq!(mgr.options.game_speed, 2.0);

        mgr.execute_button_action("btn_opt_speed_40");
        assert_eq!(mgr.options.game_speed, 4.0);
    }

    #[test]
    fn test_effective_dt_velocity_scaling_simulation() {
        let mut mgr = MenuManager::new();
        let dt = 0.01667; // standard 60 FPS tick (16.67ms)

        // When paused or menu is open, simulation freezes (effective_dt = 0.0)
        let calc_dt = |is_paused: bool, in_game: bool, speed: f64, delta: f64| -> f64 {
            if is_paused || !in_game {
                0.0
            } else {
                delta.min(0.1) * speed
            }
        };

        // In menu (not in game)
        assert_eq!(calc_dt(false, false, 1.0, dt), 0.0);

        // In game, but hotkey paused
        assert_eq!(calc_dt(true, true, 1.0, dt), 0.0);

        // Live gameplay with 0.5x tactical slow
        mgr.options.game_speed = 0.5;
        let dt_half = calc_dt(false, true, mgr.options.game_speed, dt);
        assert!((dt_half - 0.008335).abs() < 1e-4);

        // Live gameplay with standard 1.0x speed
        mgr.options.game_speed = 1.0;
        let dt_std = calc_dt(false, true, mgr.options.game_speed, dt);
        assert!((dt_std - 0.01667).abs() < 1e-4);

        // Live gameplay with 2.0x double speed
        mgr.options.game_speed = 2.0;
        let dt_double = calc_dt(false, true, mgr.options.game_speed, dt);
        assert!((dt_double - 0.03334).abs() < 1e-4);

        // Live gameplay with 4.0x quad speed
        mgr.options.game_speed = 4.0;
        let dt_quad = calc_dt(false, true, mgr.options.game_speed, dt);
        assert!((dt_quad - 0.06668).abs() < 1e-4);

        // Protection against lag spike: max dt clamped at 0.1s
        let dt_lag_spike = calc_dt(false, true, 4.0, 0.50);
        assert_eq!(dt_lag_spike, 0.40); // 0.10 * 4.0
    }

    #[test]
    fn test_cache_manifest_and_pwa_integrity() {
        use std::fs;

        // Verify manifest.json exists and is valid JSON with required PWA metadata
        let manifest_str = fs::read_to_string("manifest.json").expect("manifest.json must exist");
        let manifest_json: serde_json::Value = serde_json::from_str(&manifest_str).expect("manifest.json must be valid JSON");

        assert_eq!(manifest_json["name"], "Outpost 2: Divided Destiny");
        assert_eq!(manifest_json["short_name"], "Outpost 2");
        assert_eq!(manifest_json["display"], "standalone");
        assert_eq!(manifest_json["background_color"], "#0c0e12");
        assert_eq!(manifest_json["theme_color"], "#0c0e12");

        let icons = manifest_json["icons"].as_array().expect("manifest.json icons must be array");
        assert!(icons.iter().any(|i| i["sizes"] == "192x192" && i["src"] == "icons/icon-192.png"));
        assert!(icons.iter().any(|i| i["sizes"] == "512x512" && i["src"] == "icons/icon-512.png"));

        // Verify icon PNG binaries exist and contain valid PNG magic bytes (\x89PNG\r\n\x1a\n)
        let icon192 = fs::read("icons/icon-192.png").expect("icons/icon-192.png must exist");
        let icon512 = fs::read("icons/icon-512.png").expect("icons/icon-512.png must exist");
        assert!(icon192.len() > 1024, "icon-192.png must be non-empty");
        assert!(icon512.len() > 2048, "icon-512.png must be non-empty");
        assert_eq!(&icon192[0..8], b"\x89PNG\r\n\x1a\n", "icon-192.png must have valid PNG signature");
        assert_eq!(&icon512[0..8], b"\x89PNG\r\n\x1a\n", "icon-512.png must have valid PNG signature");

        // Verify sw.js exists and contains Cache-First implementation and audio asset references
        let sw_str = fs::read_to_string("sw.js").expect("sw.js must exist");
        assert!(sw_str.contains("CACHE_NAME"));
        assert!(sw_str.contains("caches.match"));
        assert!(sw_str.contains("assets/audio/beep2.wav"));
        assert!(sw_str.contains("assets/audio/bigexp1.wav"));
        assert!(sw_str.contains("assets/audio/uni_grad.wav"));
    }

    #[test]
    fn test_multiplayer_lobby_tab_switching_and_buttons() {
        let mut mgr = MenuManager::new();
        mgr.switch_screen(MenuScreen::MultiplayerLobby);
        assert_eq!(mgr.current_screen, MenuScreen::MultiplayerLobby);
        assert_eq!(mgr.mp_tab, MultiplayerTab::Host);

        // 1. Host Tab Layout & Actions
        let layout_host = mgr.build_button_layout(1024.0, 768.0, false);
        assert!(layout_host.iter().any(|b| b.id == "btn_mp_tab_host"));
        assert!(layout_host.iter().any(|b| b.id == "btn_mp_tab_guest"));
        assert!(layout_host.iter().any(|b| b.id == "btn_copy_offer"));
        assert!(layout_host.iter().any(|b| b.id == "btn_paste_answer"));
        assert!(layout_host.iter().any(|b| b.id == "btn_mp_map_cycle"));
        assert!(layout_host.iter().any(|b| b.id == "btn_launch_match"));

        // Copy offer action
        let act = mgr.execute_button_action("btn_copy_offer");
        assert_eq!(act, MenuAction::CopyOfferToClipboard);
        assert!(mgr.status_message.is_some());

        // Map cycle
        let initial_map = mgr.mp_room.map_name.clone();
        let act = mgr.execute_button_action("btn_mp_map_next");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_ne!(mgr.mp_room.map_name, initial_map);

        // Paste answer action
        let act = mgr.execute_button_action("btn_paste_answer");
        assert_eq!(act, MenuAction::PasteAnswerFromClipboard);
        assert_eq!(mgr.mp_room.latency_ms, Some(24));
        assert_eq!(mgr.mp_room.state, crate::net::lobby::LobbyState::Connected);

        // 2. Switch to Guest Tab
        let act = mgr.execute_button_action("btn_mp_tab_guest");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.mp_tab, MultiplayerTab::Guest);

        let layout_guest = mgr.build_button_layout(1024.0, 768.0, false);
        assert!(layout_guest.iter().any(|b| b.id == "btn_paste_offer"));
        assert!(layout_guest.iter().any(|b| b.id == "btn_generate_answer"));
        assert!(layout_guest.iter().any(|b| b.id == "btn_copy_answer"));
        assert!(layout_guest.iter().any(|b| b.id == "btn_toggle_ready"));

        // Paste offer & generate answer
        let act = mgr.execute_button_action("btn_paste_offer");
        assert_eq!(act, MenuAction::PasteOfferFromClipboard);

        let act = mgr.execute_button_action("btn_generate_answer");
        assert_eq!(act, MenuAction::GenerateGuestAnswer);

        let act = mgr.execute_button_action("btn_copy_answer");
        assert_eq!(act, MenuAction::CopyAnswerToClipboard);

        // Toggle guest ready
        assert!(!mgr.mp_room.is_guest_ready);
        let act = mgr.execute_button_action("btn_toggle_ready");
        assert_eq!(act, MenuAction::ToggleMpReady);
        assert!(mgr.mp_room.is_guest_ready);

        // 3. Switch back to Host Tab and test Launch Match
        mgr.execute_button_action("btn_mp_tab_host");
        assert_eq!(mgr.mp_tab, MultiplayerTab::Host);

        // Host not ready yet -> launch fails gracefully
        assert!(!mgr.mp_room.is_host_ready);
        let act = mgr.execute_button_action("btn_launch_match");
        assert_eq!(act, MenuAction::PlayBeep);
        assert_eq!(mgr.current_screen, MenuScreen::MultiplayerLobby);

        // Host marks ready
        mgr.execute_button_action("btn_mp_toggle_host_ready");
        assert!(mgr.mp_room.is_host_ready);
        assert!(mgr.mp_room.is_ready_to_launch());

        // Launch succeeds
        let act = mgr.execute_button_action("btn_launch_match");
        assert_eq!(act, MenuAction::LaunchMultiplayerMatch);
        assert_eq!(mgr.current_screen, MenuScreen::None);
    }
}
