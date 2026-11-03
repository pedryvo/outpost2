//! Outpost 2 Colony Simulation and Game State

pub mod ai;
pub mod combat;
pub mod debrief;
pub mod disasters;
pub mod events;
pub mod fog;
pub mod generator;
pub mod map;
pub mod mining;
pub mod mission;
pub mod pathfinding;
pub mod save;
pub mod structures;
pub mod tech;

#[allow(unused_imports)]
pub use generator::{
    GeneratedMap, MapGenerator, MapGeneratorConfig, MapSymmetry, OreAbundance, TerrainBiome,
    TileMap,
};

#[allow(unused_imports)]
pub use debrief::{ColonialRank, DefeatReason, EndGameSequence, MissionDebrief, VictoryType};

#[allow(unused_imports)]
pub use save::{
    execute_autosave, execute_quickload, execute_quicksave, is_autosave_due, ColonyResourcesSave,
    DisasterSaveState, EventEngineSaveState, FileExportBackend, LocalStorageBackend,
    MissionSaveState, PowerStatus, SaveGameState, SaveMetadata, StructureSaveState, UnitSaveState,
    SLOT_AUTOSAVE, SLOT_MAX_USER, SLOT_MIN_USER, SLOT_QUICKSAVE,
};

use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use ai::FactionAiDirector;
#[allow(unused_imports)]
pub use ai::{
    AiConfig, AiPersonality, AiPlayer, AiTacticalState, BaseBuildingPlanner, Difficulty,
    ProductionManager, ResearchPlanner, TacticalSquadManager, TacticalStance,
};
use combat::{CombatBeam, ExplosionEffect, WeaponType};
use disasters::DisasterSimulator;
#[allow(unused_imports)]
pub use events::{
    DisasterType, EventAction, EventEngine, GameSession, NotificationPriority,
    ObjectiveStatus, ScriptTrigger, TacticalNotification, TechId, TriggerCondition, UnitType,
};
use fog::FogOfWar;
use map::GameMap;
use mining::{HaulerState, OreDeposit, OreType};
use mission::Mission;
use pathfinding::{PathFinder, UnitOrder};
pub use structures::{connect_with_tubes, StructureType};
use tech::TechTree;

const EDEN_MAP_DATA: &[u8] = include_bytes!("../../assets/eden01.map");
const PLYMOUTH_MAP_DATA: &[u8] = include_bytes!("../../assets/ply01.map");
const TUTORIAL_MAP_DATA: &[u8] = include_bytes!("../../assets/tutorial.map");
const EDEN02_MAP_DATA: &[u8] = include_bytes!("../../assets/eden02.map");
const PLY02_MAP_DATA: &[u8] = include_bytes!("../../assets/ply02.map");
const EDEN03_MAP_DATA: &[u8] = include_bytes!("../../assets/eden03.map");
const PLY03_MAP_DATA: &[u8] = include_bytes!("../../assets/ply03.map");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Faction {
    #[default]
    Eden,
    Plymouth,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub name: String,
    pub faction: Faction,
    pub tile_x: i32,
    pub tile_y: i32,
    pub hp: u32,
    pub max_hp: u32,
    pub is_selected: bool,
    pub current_order: UnitOrder,
    pub path: Vec<(i32, i32)>,
    pub move_cooldown: f64,
    pub weapon: Option<WeaponType>,
    pub attack_cooldown: f64,
    pub cargo: Option<(OreType, u32)>,
    pub hauler_state: Option<HaulerState>,
    #[serde(default)]
    pub cargo_kit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Building {
    pub id: u32,
    pub name: String,
    pub tile_x: i32,
    pub tile_y: i32,
    pub width: i32,
    pub height: i32,
    pub power_cost: i32,
    pub is_active: bool,
    pub is_selected: bool,
    pub production_queue: Vec<String>,
    pub production_progress: f64,
    #[serde(default)]
    pub attack_cooldown: f64,
    #[serde(default)]
    pub faction: Option<Faction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGameData {
    pub scenario_id: usize,
    pub faction: Faction,
    pub sol: f64,
    pub population: u32,
    pub morale_pct: u32,
    pub starship_progress: f64,
    pub common_ore: u32,
    pub rare_ore: u32,
    pub power_produced: u32,
    pub power_consumed: u32,
    pub food_units: u32,
    pub units: Vec<Unit>,
    pub buildings: Vec<Building>,
    pub tubes: Vec<(i32, i32)>,
    pub researched_tech_ids: Vec<u32>,
    pub day_cycle: f64,
}

#[derive(Debug, Clone)]
pub struct ColonyState {
    pub scenario_id: usize,
    pub faction: Faction,
    pub sol: f64,
    pub population: u32,
    pub morale_pct: u32,
    pub starship_progress: f64,
    pub common_ore: u32,
    pub rare_ore: u32,
    pub power_produced: u32,
    pub power_consumed: u32,
    pub food_units: u32,
    pub units: Vec<Unit>,
    pub buildings: Vec<Building>,
    pub tubes: HashSet<(i32, i32)>,
    pub game_map: GameMap,
    pub tech_tree: TechTree,
    pub disasters: DisasterSimulator,
    pub fog: FogOfWar,
    pub day_cycle: f64,
    pub sunlight: f64,
    pub active_beams: Vec<CombatBeam>,
    pub active_explosions: Vec<ExplosionEffect>,
    pub ai_director: FactionAiDirector,
    pub ore_deposits: Vec<OreDeposit>,
    pub mission: Mission,
    pub starship_launched: bool,
    pub launch_anim_time: f64,
    pub debriefing_shown: bool,
    pub tick_count: u64,
    pub casualties: u32,
    pub evacuated_colonists: u32,
    pub terraformer_saturation: f64,
    #[allow(dead_code)]
    pub total_common_ore_mined: u32,
    #[allow(dead_code)]
    pub total_rare_ore_mined: u32,
    #[allow(dead_code)]
    pub enemy_units_destroyed: u32,
    #[allow(dead_code)]
    pub friendly_units_lost: u32,
    #[allow(dead_code)]
    pub end_game: Option<EndGameSequence>,
    pub destroyed_building_ids: HashSet<u32>,
    pub initial_building_ids: HashSet<u32>,
    pub event_engine: EventEngine,
    #[allow(dead_code)]
    pub ai_players: Vec<AiPlayer>,
    pub last_under_attack_sol: f64,
}

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct SimUpdateResult {
    pub notice: Option<&'static str>,
    pub laser_fired: bool,
    pub railgun_fired: bool,
    pub microwave_fired: bool,
    pub acid_fired: bool,
    pub unit_destroyed: bool,
    pub friendly_vehicle_destroyed: bool,
    pub structure_destroyed: bool,
    pub volcano_erupted: bool,
    pub vortex_spawned: bool,
    pub building_deployed: bool,
    pub vortex_active: bool,
    pub meteor_hit: bool,
    pub disaster_siren: bool,
    pub mission_ended: Option<mission::MissionStatus>,
    pub starship_launched: bool,
    pub under_attack: bool,
    #[allow(dead_code)]
    pub end_game_sequence: Option<EndGameSequence>,
}

impl ColonyState {
    pub fn new(faction: Faction) -> Self {
        let scenario_id = match faction {
            Faction::Eden => 0,
            Faction::Plymouth => 1,
        };
        Self::new_scenario(scenario_id)
    }

    pub fn new_scenario(scenario_id: usize) -> Self {
        let (faction, map_data, mission, enemy_faction) = match scenario_id {
            1 => (Faction::Plymouth, PLYMOUTH_MAP_DATA, Mission::new_plymouth_mission_1(), Faction::Eden),
            2 => (Faction::Eden, TUTORIAL_MAP_DATA, Mission::new_tutorial_mission(), Faction::Plymouth),
            3 => (Faction::Eden, EDEN02_MAP_DATA, Mission::new_eden_mission_2(), Faction::Plymouth),
            4 => (Faction::Plymouth, PLY02_MAP_DATA, Mission::new_plymouth_mission_2(), Faction::Eden),
            5 => (Faction::Eden, EDEN03_MAP_DATA, Mission::new_eden_mission_3(), Faction::Plymouth),
            6 => (Faction::Plymouth, PLY03_MAP_DATA, Mission::new_plymouth_mission_3(), Faction::Eden),
            _ => (Faction::Eden, EDEN_MAP_DATA, Mission::new_eden_mission_1(), Faction::Plymouth),
        };

        let mut units = Vec::new();
        let mut buildings = Vec::new();

        if scenario_id == 5 {
            // Eden Mission 3 ("Starship Evacuation: Escape From Mount Vulcan")
            // Base initialization: Command Center, 2 Tokamak Power Plants, 2 Advanced Smelters, Spaceport / Launch Facility
            buildings.push(Building {
                id: 1,
                name: "Command Center".into(),
                tile_x: 20,
                tile_y: 20,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 2,
                name: "Tokamak Fusion Reactor #1".into(),
                tile_x: 25,
                tile_y: 20,
                width: 3,
                height: 3,
                power_cost: -800,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 3,
                name: "Tokamak Fusion Reactor #2".into(),
                tile_x: 25,
                tile_y: 24,
                width: 3,
                height: 3,
                power_cost: -800,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 4,
                name: "Rare Ore Smelter".into(),
                tile_x: 20,
                tile_y: 25,
                width: 2,
                height: 2,
                power_cost: 75,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 5,
                name: "Common Ore Smelter".into(),
                tile_x: 23,
                tile_y: 25,
                width: 2,
                height: 2,
                power_cost: 50,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 6,
                name: "Starship Launch Gantry".into(),
                tile_x: 15,
                tile_y: 20,
                width: 3,
                height: 3,
                power_cost: 160,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 7,
                name: "Vehicle Factory".into(),
                tile_x: 10,
                tile_y: 20,
                width: 3,
                height: 3,
                power_cost: 120,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });

            // Standard logistics vehicles
            units.push(Unit {
                id: 1,
                name: "Cargo Truck".into(),
                faction,
                tile_x: 21,
                tile_y: 24,
                hp: 150,
                max_hp: 150,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: Some(HaulerState::Idle),
                cargo_kit: None,
            });
            units.push(Unit {
                id: 2,
                name: "Robo-Miner".into(),
                faction,
                tile_x: 26,
                tile_y: 22,
                hp: 180,
                max_hp: 180,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 3,
                name: "ConVec".into(),
                faction,
                tile_x: 23,
                tile_y: 22,
                hp: 200,
                max_hp: 200,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            // Evacuation Transports
            units.push(Unit {
                id: 4,
                name: "Evacuation Transport #1".into(),
                faction,
                tile_x: 16,
                tile_y: 24,
                hp: 220,
                max_hp: 220,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 5,
                name: "Evacuation Transport #2".into(),
                faction,
                tile_x: 18,
                tile_y: 24,
                hp: 220,
                max_hp: 220,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 6,
                name: "Evacuation Transport #3".into(),
                faction,
                tile_x: 14,
                tile_y: 24,
                hp: 220,
                max_hp: 220,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            // Heavy Panther Combat Tanks
            units.push(Unit {
                id: 7,
                name: "Panther Heavy Tank #1".into(),
                faction,
                tile_x: 19,
                tile_y: 18,
                hp: 250,
                max_hp: 250,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Railgun),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 8,
                name: "Panther Heavy Tank #2".into(),
                faction,
                tile_x: 23,
                tile_y: 18,
                hp: 250,
                max_hp: 250,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Railgun),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            // Encroaching Plymouth raiders
            units.push(Unit {
                id: 101,
                name: "Plymouth Raider Division Scout".into(),
                faction: enemy_faction,
                tile_x: 36,
                tile_y: 30,
                hp: 140,
                max_hp: 140,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Laser),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 102,
                name: "Plymouth Raider Heavy Vanguard".into(),
                faction: enemy_faction,
                tile_x: 38,
                tile_y: 32,
                hp: 280,
                max_hp: 280,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Railgun),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
        } else if scenario_id == 6 {
            // Plymouth Mission 3 ("Operation Phoenix: Obsidian Crags Assault")
            // Base initialization: Command Center, Geothermal Plant, 2 Smelters, Heavy Vehicle Factory, Atmospheric Terraformer
            buildings.push(Building {
                id: 1,
                name: "Command Center".into(),
                tile_x: 18,
                tile_y: 22,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 2,
                name: "Geothermal Power Plant".into(),
                tile_x: 23,
                tile_y: 22,
                width: 2,
                height: 2,
                power_cost: -450,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 3,
                name: "Common Ore Smelter".into(),
                tile_x: 18,
                tile_y: 26,
                width: 2,
                height: 2,
                power_cost: 50,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 4,
                name: "Rare Ore Smelter".into(),
                tile_x: 21,
                tile_y: 26,
                width: 2,
                height: 2,
                power_cost: 75,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 5,
                name: "Vehicle Factory".into(),
                tile_x: 14,
                tile_y: 22,
                width: 3,
                height: 3,
                power_cost: 120,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });
            buildings.push(Building {
                id: 6,
                name: "Atmospheric Terraformer".into(),
                tile_x: 22,
                tile_y: 18,
                width: 3,
                height: 3,
                power_cost: 100,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });

            // Fortified Eden Obsidian Citadel base
            buildings.push(Building {
                id: 101,
                name: "Eden Obsidian Citadel".into(),
                tile_x: 48,
                tile_y: 20,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(Faction::Eden),
            });
            buildings.push(Building {
                id: 102,
                name: "Guard Post Turret #1".into(),
                tile_x: 45,
                tile_y: 18,
                width: 2,
                height: 2,
                power_cost: 50,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(Faction::Eden),
            });
            buildings.push(Building {
                id: 103,
                name: "Guard Post Turret #2".into(),
                tile_x: 45,
                tile_y: 24,
                width: 2,
                height: 2,
                power_cost: 50,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(Faction::Eden),
            });

            // Plymouth assault units
            units.push(Unit {
                id: 1,
                name: "Cargo Truck".into(),
                faction,
                tile_x: 20,
                tile_y: 25,
                hp: 150,
                max_hp: 150,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: Some(HaulerState::Idle),
                cargo_kit: None,
            });
            units.push(Unit {
                id: 2,
                name: "Robo-Miner".into(),
                faction,
                tile_x: 24,
                tile_y: 25,
                hp: 180,
                max_hp: 180,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 3,
                name: "ConVec".into(),
                faction,
                tile_x: 18,
                tile_y: 25,
                hp: 200,
                max_hp: 200,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 4,
                name: "Tiger Heavy Assault Tank #1".into(),
                faction,
                tile_x: 16,
                tile_y: 18,
                hp: 420,
                max_hp: 420,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Railgun),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 5,
                name: "Tiger Heavy Assault Tank #2".into(),
                faction,
                tile_x: 18,
                tile_y: 18,
                hp: 420,
                max_hp: 420,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Railgun),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            // Fortified Eden Citadel defenders
            units.push(Unit {
                id: 201,
                name: "Eden Tiger Railgun Division #1".into(),
                faction: Faction::Eden,
                tile_x: 46,
                tile_y: 19,
                hp: 420,
                max_hp: 420,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Railgun),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 202,
                name: "Eden Tiger Railgun Division #2".into(),
                faction: Faction::Eden,
                tile_x: 46,
                tile_y: 23,
                hp: 420,
                max_hp: 420,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Railgun),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
            units.push(Unit {
                id: 203,
                name: "Eden Panther Interceptor".into(),
                faction: Faction::Eden,
                tile_x: 43,
                tile_y: 21,
                hp: 250,
                max_hp: 250,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Laser),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });
        } else {
            // Base Command Center
            buildings.push(Building {
                id: 1,
                name: "Command Center".into(),
                tile_x: 20,
                tile_y: 20,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });

            // Solar Array
            buildings.push(Building {
                id: 2,
                name: "Solar Power Array".into(),
                tile_x: 24,
                tile_y: 20,
                width: 2,
                height: 2,
                power_cost: -250,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });

            // Common Ore Smelter
            buildings.push(Building {
                id: 3,
                name: "Common Ore Smelter".into(),
                tile_x: 21,
                tile_y: 25,
                width: 2,
                height: 2,
                power_cost: 50,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(faction),
            });

            if scenario_id == 1 {
                // Plymouth starts with an active Vehicle Factory
                buildings.push(Building {
                    id: 4,
                    name: "Vehicle Factory".into(),
                    tile_x: 16,
                    tile_y: 24,
                    width: 3,
                    height: 3,
                    power_cost: 120,
                    is_active: true,
                    is_selected: false,
                    production_queue: Vec::new(),
                    production_progress: 0.0,
                    attack_cooldown: 0.0,
                    faction: Some(faction),
                });
            }

            // Starting Units
            units.push(Unit {
                id: 1,
                name: "ConVec (Structure Kit)".into(),
                faction,
                tile_x: 21,
                tile_y: 24,
                hp: 200,
                max_hp: 200,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: Some("Common Ore Smelter".to_string()),
            });

            units.push(Unit {
                id: 2,
                name: "Cargo Truck".into(),
                faction,
                tile_x: 23,
                tile_y: 24,
                hp: 150,
                max_hp: 150,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: Some(HaulerState::Idle),
                cargo_kit: None,
            });

            units.push(Unit {
                id: 3,
                name: "Robo-Miner".into(),
                faction,
                tile_x: 26,
                tile_y: 22,
                hp: 180,
                max_hp: 180,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            units.push(Unit {
                id: 4,
                name: if faction == Faction::Eden { "Lynx Scout #1".into() } else { "Plymouth Raider #1".into() },
                faction,
                tile_x: 18,
                tile_y: 21,
                hp: 140,
                max_hp: 140,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(WeaponType::Laser),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            if scenario_id == 0 {
                // Eden starts with 1 Panther Tank
                units.push(Unit {
                    id: 5,
                    name: "Panther Heavy Tank #1".into(),
                    faction: Faction::Eden,
                    tile_x: 25,
                    tile_y: 25,
                    hp: 250,
                    max_hp: 250,
                    is_selected: false,
                    current_order: UnitOrder::Idle,
                    path: Vec::new(),
                    move_cooldown: 0.0,
                    weapon: Some(WeaponType::Railgun),
                    attack_cooldown: 0.0,
                    cargo: None,
                    hauler_state: None,
                    cargo_kit: None,
                });
            }

            // Rival enemy incursion scouts
            if scenario_id != 2 {
                units.push(Unit {
                    id: 101,
                    name: format!("{:?} Incursion Scout", enemy_faction),
                    faction: enemy_faction,
                    tile_x: 35,
                    tile_y: 28,
                    hp: 130,
                    max_hp: 130,
                    is_selected: false,
                    current_order: UnitOrder::Idle,
                    path: Vec::new(),
                    move_cooldown: 0.0,
                    weapon: Some(WeaponType::Microwave),
                    attack_cooldown: 0.0,
                    cargo: None,
                    hauler_state: None,
                    cargo_kit: None,
                });

                units.push(Unit {
                    id: 102,
                    name: format!("{:?} Heavy Vanguard", enemy_faction),
                    faction: enemy_faction,
                    tile_x: 37,
                    tile_y: 30,
                    hp: 300,
                    max_hp: 300,
                    is_selected: false,
                    current_order: UnitOrder::Idle,
                    path: Vec::new(),
                    move_cooldown: 0.0,
                    weapon: Some(WeaponType::AcidCloud),
                    attack_cooldown: 0.0,
                    cargo: None,
                    hauler_state: None,
                    cargo_kit: None,
                });
            }
        }

        let game_map = GameMap::parse(map_data)
            .unwrap_or_else(|_| GameMap {
                width: 64,
                height: 64,
                tiles: Vec::new(),
            });

        let mut tubes = HashSet::new();
        if scenario_id == 5 {
            connect_with_tubes(&mut tubes, (12, 21), (15, 21));
            connect_with_tubes(&mut tubes, (17, 21), (20, 21));
            connect_with_tubes(&mut tubes, (22, 21), (25, 21));
            connect_with_tubes(&mut tubes, (25, 22), (25, 24));
            connect_with_tubes(&mut tubes, (21, 22), (21, 25));
            connect_with_tubes(&mut tubes, (21, 25), (23, 25));
        } else if scenario_id == 6 {
            connect_with_tubes(&mut tubes, (18, 22), (23, 22));
            connect_with_tubes(&mut tubes, (18, 23), (18, 26));
            connect_with_tubes(&mut tubes, (18, 26), (21, 26));
            connect_with_tubes(&mut tubes, (18, 22), (15, 22));
            connect_with_tubes(&mut tubes, (22, 22), (22, 19));
        } else {
            connect_with_tubes(&mut tubes, (22, 21), (24, 21));
            connect_with_tubes(&mut tubes, (21, 22), (21, 25));
            if scenario_id == 1 {
                connect_with_tubes(&mut tubes, (20, 24), (18, 24));
            }
        }

        let mut ore_deposits = Vec::new();
        let mut dep1 = OreDeposit::new(1, OreType::Common, 26, 22, 50_000);
        dep1.has_miner = true;
        let dep2 = OreDeposit::new(2, OreType::Rare, 18, 28, 25_000);
        let dep3 = OreDeposit::new(3, OreType::Common, 30, 16, 60_000);
        ore_deposits.push(dep1);
        ore_deposits.push(dep2);
        ore_deposits.push(dep3);

        let map_w = game_map.width as usize;
        let map_h = game_map.height as usize;
        let mut fog = FogOfWar::new(map_w, map_h);
        fog.update_visibility(&units, &buildings);

        let initial_building_ids: HashSet<u32> = buildings.iter().map(|b| b.id).collect();
        let event_engine = EventEngine::for_scenario(scenario_id);

        let (init_pop, init_common, init_rare, init_power_prod, init_power_cons) = match scenario_id {
            5 => (200, 3500, 1200, 1600, 380),
            6 => (150, 3000, 800, 700, 350),
            _ => (120, 2500, 500, 500, 320),
        };

        Self {
            scenario_id,
            faction,
            sol: 1.0,
            population: init_pop,
            morale_pct: 95,
            starship_progress: 0.0,
            common_ore: init_common,
            rare_ore: init_rare,
            power_produced: init_power_prod,
            power_consumed: init_power_cons,
            food_units: 1400,
            units,
            buildings,
            tubes,
            game_map,
            tech_tree: TechTree::new(),
            disasters: DisasterSimulator::new(),
            fog,
            day_cycle: 0.25,
            sunlight: 1.0,
            active_beams: Vec::new(),
            active_explosions: Vec::new(),
            ai_director: FactionAiDirector::new(enemy_faction, 38, 30),
            ore_deposits,
            mission,
            starship_launched: false,
            launch_anim_time: 0.0,
            debriefing_shown: false,
            tick_count: 0,
            casualties: 0,
            evacuated_colonists: 0,
            terraformer_saturation: 0.0,
            total_common_ore_mined: 0,
            total_rare_ore_mined: 0,
            enemy_units_destroyed: 0,
            friendly_units_lost: 0,
            end_game: None,
            destroyed_building_ids: HashSet::new(),
            initial_building_ids,
            event_engine,
            ai_players: Vec::new(),
            last_under_attack_sol: -999.0,
        }
    }

    /// Helper to get the next sequential unit ID
    pub fn next_unit_id(&self) -> u32 {
        self.units.iter().map(|u| u.id).max().unwrap_or(0) + 1
    }

    /// Register an autonomous AI commander into the simulation
    #[allow(dead_code)]
    pub fn add_ai_player(&mut self, player: AiPlayer) {
        self.ai_players.push(player);
    }

    /// Set up a skirmish match running 1 to 5 AI opponents against human players or each other
    #[allow(dead_code)]
    pub fn setup_skirmish(
        &mut self,
        player_faction: Faction,
        ai_opponents: &[(Faction, AiPersonality, Difficulty, i32, i32)],
    ) {
        self.faction = player_faction;
        self.ai_players.clear();

        for (idx, &(ai_faction, ref personality, difficulty, bx, by)) in ai_opponents.iter().enumerate() {
            let ai_id = (idx + 1) as u32;
            let ai_name = format!("{:?} AI #{}", ai_faction, ai_id);
            let mut ai = AiPlayer::new(ai_id, ai_name, ai_faction, personality.clone(), difficulty, bx, by);

            let cc_id = self.buildings.len() as u32 + 1;
            let sol_id = cc_id + 1;
            let sm_id = cc_id + 2;
            let vf_id = cc_id + 3;

            self.buildings.push(Building {
                id: cc_id,
                name: format!("Command Center #{}", cc_id),
                tile_x: bx,
                tile_y: by,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(ai_faction),
            });

            self.buildings.push(Building {
                id: sol_id,
                name: format!("Solar Power Array #{}", sol_id),
                tile_x: bx + 4,
                tile_y: by,
                width: 2,
                height: 2,
                power_cost: -250,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(ai_faction),
            });

            self.buildings.push(Building {
                id: sm_id,
                name: format!("Common Ore Smelter #{}", sm_id),
                tile_x: bx,
                tile_y: by + 4,
                width: 2,
                height: 2,
                power_cost: 50,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(ai_faction),
            });

            self.buildings.push(Building {
                id: vf_id,
                name: format!("Vehicle Factory #{}", vf_id),
                tile_x: bx + 4,
                tile_y: by + 4,
                width: 3,
                height: 3,
                power_cost: 120,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(ai_faction),
            });

            for b_idx in self.buildings.len() - 4..self.buildings.len() {
                let b = &self.buildings[b_idx];
                for dy in 0..b.height {
                    for dx in 0..b.width {
                        let tx = (b.tile_x + dx) as u32;
                        let ty = (b.tile_y + dy) as u32;
                        let idx = (ty * self.game_map.width + tx) as usize;
                        if idx < self.game_map.tiles.len() {
                            self.game_map.tiles[idx].is_wall_or_building = true;
                        }
                    }
                }
            }

            connect_with_tubes(&mut self.tubes, (bx + 1, by + 1), (bx + 5, by + 1));
            connect_with_tubes(&mut self.tubes, (bx + 1, by + 1), (bx + 1, by + 5));
            connect_with_tubes(&mut self.tubes, (bx + 1, by + 5), (bx + 5, by + 5));

            let u1_id = self.next_unit_id();
            let u2_id = u1_id + 1;
            let u3_id = u1_id + 2;

            self.units.push(Unit {
                id: u1_id,
                name: format!("Cargo Truck #{}", u1_id),
                faction: ai_faction,
                tile_x: bx + 2,
                tile_y: by + 3,
                hp: 150,
                max_hp: 150,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: Some(HaulerState::Idle),
                cargo_kit: None,
            });

            self.units.push(Unit {
                id: u2_id,
                name: format!("Robo-Miner #{}", u2_id),
                faction: ai_faction,
                tile_x: bx + 3,
                tile_y: by + 3,
                hp: 180,
                max_hp: 180,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            let (guard_name, guard_weapon) = match ai_faction {
                Faction::Plymouth => ("Plymouth Raider Lynx", WeaponType::Microwave),
                Faction::Eden => ("Eden Lynx Scout", WeaponType::Laser),
            };

            self.units.push(Unit {
                id: u3_id,
                name: format!("{} #{}", guard_name, u3_id),
                faction: ai_faction,
                tile_x: bx - 1,
                tile_y: by,
                hp: 140,
                max_hp: 140,
                is_selected: false,
                current_order: UnitOrder::Guard { anchor_x: bx, anchor_y: by },
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(guard_weapon),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            ai.refresh_colony_power(self);
            self.ai_players.push(ai);
        }

        self.fog.update_visibility(&self.units, &self.buildings);
    }

    /// Construct a complete playable skirmish session from a procedurally generated map
    #[allow(dead_code)]
    pub fn from_generated_map(mut gen_map: GeneratedMap, player_faction: Faction) -> Self {
        let scenario_id = 100;
        let map_w = gen_map.tile_map.width as usize;
        let map_h = gen_map.tile_map.height as usize;

        let mut units = Vec::new();
        let mut buildings = Vec::new();
        let mut tubes = HashSet::new();
        let mut ai_players = Vec::new();

        // 1. Set up Player Base at spawn_points[0]
        let (px, py) = if let Some(&p) = gen_map.spawn_points.first() {
            p
        } else {
            (10, 10)
        };

        // Command Center (3x3)
        buildings.push(Building {
            id: 1,
            name: "Command Center".into(),
            tile_x: px,
            tile_y: py,
            width: 3,
            height: 3,
            power_cost: 40,
            is_active: true,
            is_selected: false,
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
            faction: Some(player_faction),
        });

        // Solar Power Array (2x2)
        buildings.push(Building {
            id: 2,
            name: "Solar Power Array".into(),
            tile_x: px + 4,
            tile_y: py,
            width: 2,
            height: 2,
            power_cost: -250,
            is_active: true,
            is_selected: false,
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
            faction: Some(player_faction),
        });

        // Common Ore Smelter (2x2)
        buildings.push(Building {
            id: 3,
            name: "Common Ore Smelter".into(),
            tile_x: px,
            tile_y: py + 4,
            width: 2,
            height: 2,
            power_cost: 50,
            is_active: true,
            is_selected: false,
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
            faction: Some(player_faction),
        });

        // Vehicle Factory (3x3)
        buildings.push(Building {
            id: 4,
            name: "Vehicle Factory".into(),
            tile_x: px + 3,
            tile_y: py + 3,
            width: 3,
            height: 3,
            power_cost: 120,
            is_active: true,
            is_selected: false,
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
            faction: Some(player_faction),
        });

        // Connect player base with pressurized tubes
        connect_with_tubes(&mut tubes, (px + 1, py + 1), (px + 4, py + 1));
        connect_with_tubes(&mut tubes, (px + 1, py + 1), (px + 1, py + 4));
        connect_with_tubes(&mut tubes, (px + 1, py + 4), (px + 3, py + 4));

        // Mark player buildings as obstacles on tile map
        for b in &buildings {
            for dy in 0..b.height {
                for dx in 0..b.width {
                    let tx = (b.tile_x + dx) as u32;
                    let ty = (b.tile_y + dy) as u32;
                    if let Some(t) = gen_map.tile_map.get_tile_mut(tx, ty) {
                        t.is_wall_or_building = true;
                    }
                }
            }
        }

        // Starting Player Units
        units.push(Unit {
            id: 1,
            name: "ConVec (Structure Kit)".into(),
            faction: player_faction,
            tile_x: px + 1,
            tile_y: py + 3,
            hp: 200,
            max_hp: 200,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon: None,
            attack_cooldown: 0.0,
            cargo: None,
            hauler_state: None,
            cargo_kit: Some("Geothermal Power Plant".to_string()),
        });

        units.push(Unit {
            id: 2,
            name: "Cargo Truck".into(),
            faction: player_faction,
            tile_x: px + 2,
            tile_y: py + 3,
            hp: 150,
            max_hp: 150,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon: None,
            attack_cooldown: 0.0,
            cargo: None,
            hauler_state: Some(HaulerState::Idle),
            cargo_kit: None,
        });

        units.push(Unit {
            id: 3,
            name: "Robo-Miner".into(),
            faction: player_faction,
            tile_x: px + 3,
            tile_y: py + 3,
            hp: 180,
            max_hp: 180,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon: None,
            attack_cooldown: 0.0,
            cargo: None,
            hauler_state: None,
            cargo_kit: None,
        });

        let scout_weapon = match player_faction {
            Faction::Eden => WeaponType::Laser,
            Faction::Plymouth => WeaponType::Microwave,
        };
        let scout_name = if player_faction == Faction::Eden { "Lynx Scout" } else { "Plymouth Raider" };
        units.push(Unit {
            id: 4,
            name: format!("{} #1", scout_name),
            faction: player_faction,
            tile_x: px - 1,
            tile_y: py,
            hp: 140,
            max_hp: 140,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon: Some(scout_weapon),
            attack_cooldown: 0.0,
            cargo: None,
            hauler_state: None,
            cargo_kit: None,
        });

        // 2. Set up AI opponents at remaining spawn points
        let mut next_b_id = 5u32;
        let mut next_u_id = 5u32;
        let personalities = [
            AiPersonality::AggressiveRusher,
            AiPersonality::DefensiveFortifier,
            AiPersonality::EconomicTechRusher,
        ];

        let enemy_faction_default = match player_faction {
            Faction::Eden => Faction::Plymouth,
            Faction::Plymouth => Faction::Eden,
        };

        let mut primary_enemy_spawn = (px + 20, py + 20);

        for (ai_idx, &(ax, ay)) in gen_map.spawn_points.iter().skip(1).enumerate() {
            if ai_idx == 0 {
                primary_enemy_spawn = (ax, ay);
            }
            let ai_id = (ai_idx + 1) as u32;
            let ai_faction = if (ai_idx % 2) == 0 {
                enemy_faction_default
            } else {
                player_faction
            };
            let personality = personalities[ai_idx % personalities.len()].clone();
            let ai = AiPlayer::new(
                ai_id,
                format!("{:?} AI #{}", ai_faction, ai_id),
                ai_faction,
                personality,
                Difficulty::Medium,
                ax,
                ay,
            );

            let cc_id = next_b_id;
            let sol_id = cc_id + 1;
            let sm_id = cc_id + 2;
            let vf_id = cc_id + 3;
            next_b_id += 4;

            buildings.push(Building {
                id: cc_id,
                name: format!("Command Center #{}", cc_id),
                tile_x: ax,
                tile_y: ay,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(ai_faction),
            });

            buildings.push(Building {
                id: sol_id,
                name: format!("Solar Power Array #{}", sol_id),
                tile_x: ax + 4,
                tile_y: ay,
                width: 2,
                height: 2,
                power_cost: -250,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(ai_faction),
            });

            buildings.push(Building {
                id: sm_id,
                name: format!("Common Ore Smelter #{}", sm_id),
                tile_x: ax,
                tile_y: ay + 4,
                width: 2,
                height: 2,
                power_cost: 50,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(ai_faction),
            });

            buildings.push(Building {
                id: vf_id,
                name: format!("Vehicle Factory #{}", vf_id),
                tile_x: ax + 4,
                tile_y: ay + 4,
                width: 3,
                height: 3,
                power_cost: 120,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(ai_faction),
            });

            // Mark AI buildings in tile_map
            let b_len = buildings.len();
            for b in &buildings[b_len - 4..b_len] {
                for dy in 0..b.height {
                    for dx in 0..b.width {
                        let tx = (b.tile_x + dx) as u32;
                        let ty = (b.tile_y + dy) as u32;
                        if let Some(t) = gen_map.tile_map.get_tile_mut(tx, ty) {
                            t.is_wall_or_building = true;
                        }
                    }
                }
            }

            connect_with_tubes(&mut tubes, (ax + 1, ay + 1), (ax + 5, ay + 1));
            connect_with_tubes(&mut tubes, (ax + 1, ay + 1), (ax + 1, ay + 5));
            connect_with_tubes(&mut tubes, (ax + 1, ay + 5), (ax + 5, ay + 5));

            let u1_id = next_u_id;
            let u2_id = u1_id + 1;
            let u3_id = u1_id + 2;
            next_u_id += 3;

            units.push(Unit {
                id: u1_id,
                name: format!("Cargo Truck #{}", u1_id),
                faction: ai_faction,
                tile_x: ax + 2,
                tile_y: ay + 3,
                hp: 150,
                max_hp: 150,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: Some(HaulerState::Idle),
                cargo_kit: None,
            });

            units.push(Unit {
                id: u2_id,
                name: format!("Robo-Miner #{}", u2_id),
                faction: ai_faction,
                tile_x: ax + 3,
                tile_y: ay + 3,
                hp: 180,
                max_hp: 180,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: None,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            let ai_scout_weapon = match ai_faction {
                Faction::Plymouth => WeaponType::Microwave,
                Faction::Eden => WeaponType::Laser,
            };
            units.push(Unit {
                id: u3_id,
                name: format!("Defense Guard #{}", u3_id),
                faction: ai_faction,
                tile_x: ax - 1,
                tile_y: ay,
                hp: 140,
                max_hp: 140,
                is_selected: false,
                current_order: UnitOrder::Guard { anchor_x: ax, anchor_y: ay },
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon: Some(ai_scout_weapon),
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            ai_players.push(ai);
        }

        // Initialize Fog of War
        let mut fog = FogOfWar::new(map_w, map_h);
        fog.update_visibility(&units, &buildings);

        let initial_building_ids: HashSet<u32> = buildings.iter().map(|b| b.id).collect();

        // Skirmish Mission
        let mission = Mission::new_skirmish(player_faction);

        let mut state = Self {
            scenario_id,
            faction: player_faction,
            sol: 1.0,
            population: 150,
            morale_pct: 100,
            starship_progress: 0.0,
            common_ore: 3000,
            rare_ore: 1000,
            power_produced: 500,
            power_consumed: 210,
            food_units: 1500,
            units,
            buildings,
            tubes,
            game_map: gen_map.tile_map,
            tech_tree: TechTree::new(),
            disasters: DisasterSimulator::new(),
            fog,
            day_cycle: 0.25,
            sunlight: 1.0,
            active_beams: Vec::new(),
            active_explosions: Vec::new(),
            ai_director: FactionAiDirector::new(enemy_faction_default, primary_enemy_spawn.0, primary_enemy_spawn.1),
            ore_deposits: gen_map.ore_deposits,
            mission,
            starship_launched: false,
            launch_anim_time: 0.0,
            debriefing_shown: false,
            tick_count: 0,
            casualties: 0,
            evacuated_colonists: 0,
            terraformer_saturation: 0.0,
            total_common_ore_mined: 0,
            total_rare_ore_mined: 0,
            enemy_units_destroyed: 0,
            friendly_units_lost: 0,
            end_game: None,
            destroyed_building_ids: HashSet::new(),
            initial_building_ids,
            event_engine: EventEngine::new(),
            ai_players,
            last_under_attack_sol: -999.0,
        };

        // Refresh power for each AI player
        for i in 0..state.ai_players.len() {
            let mut ai = state.ai_players[i].clone();
            ai.refresh_colony_power(&state);
            state.ai_players[i] = ai;
        }

        state
    }

    /// Construct a complete playable skirmish session directly from generator configuration
    #[allow(dead_code)]
    pub fn new_skirmish_map(config: MapGeneratorConfig, player_faction: Faction) -> Self {
        let gen_map = MapGenerator::generate(&config);
        Self::from_generated_map(gen_map, player_faction)
    }

    pub fn update(&mut self, dt_secs: f64) -> SimUpdateResult {
        self.tick_count = self.tick_count.wrapping_add(1);
        self.sol += dt_secs * 0.05; // 1 Sol every ~20 seconds

        // Dynamic diurnal daylight cycle (~120s per full planetary rotation)
        self.day_cycle = (self.day_cycle + dt_secs * 0.008) % 1.0;
        let raw_sun = (self.day_cycle * std::f64::consts::TAU).sin() * 0.45 + 0.55;
        self.sunlight = raw_sun.clamp(0.20, 1.0);

        // Solar panels vary power with solar intensity
        let solar_output = (250.0 * self.sunlight) as u32;
        self.power_produced = 250 + solar_output;

        // Update planetary disasters
        let warning = self.disasters.update(&mut self.game_map, dt_secs);

        // Update units along their computed path
        for u in &mut self.units {
            if !u.path.is_empty() {
                u.move_cooldown -= dt_secs;
                if u.move_cooldown <= 0.0 {
                    let next_step = u.path.remove(0);
                    u.tile_x = next_step.0;
                    u.tile_y = next_step.1;
                    u.move_cooldown = 0.28; // ~3.5 tiles per second
                }
            } else if let UnitOrder::MoveTo { target_x, target_y } = u.current_order {
                if u.tile_x == target_x && u.tile_y == target_y {
                    u.current_order = UnitOrder::Idle;
                }
            }
        }

        // Update active combat tracer beams and explosion effects
        self.active_beams.retain_mut(|beam| {
            beam.lifetime -= dt_secs;
            beam.lifetime > 0.0
        });

        self.active_explosions.retain_mut(|exp| {
            exp.lifetime -= dt_secs;
            let progress = (1.0 - (exp.lifetime / exp.max_lifetime)).clamp(0.0, 1.0);
            exp.radius = exp.max_radius * progress;
            exp.lifetime > 0.0
        });

        // Unit Combat Engagements
        let mut combat_events = Vec::new();
        let unit_positions: Vec<(u32, Faction, f64, f64, u32)> = self.units.iter()
            .map(|u| (u.id, u.faction, u.tile_x as f64, u.tile_y as f64, u.hp))
            .collect();

        for u in &mut self.units {
            u.attack_cooldown -= dt_secs;
            if let Some(weapon) = u.weapon {
                if u.attack_cooldown <= 0.0 {
                    // Check target
                    let mut target_id = None;
                    if let UnitOrder::Attack { target_unit_id } = u.current_order {
                        target_id = Some(target_unit_id);
                    } else {
                        // Autonomous combat stance: scan for closest enemy faction within range
                        let max_range = weapon.range_tiles();
                        let ux = u.tile_x as f64;
                        let uy = u.tile_y as f64;
                        let mut closest_dist = max_range;
                        for &(other_id, other_faction, ox, oy, other_hp) in &unit_positions {
                            if other_faction != u.faction && other_hp > 0 {
                                let dist = ((ox - ux).powi(2) + (oy - uy).powi(2)).sqrt();
                                if dist <= closest_dist {
                                    closest_dist = dist;
                                    target_id = Some(other_id);
                                }
                            }
                        }
                    }

                    if let Some(tid) = target_id {
                        if let Some(&(_, _, tx, ty, thp)) = unit_positions.iter().find(|p| p.0 == tid) {
                            let dist = ((tx - u.tile_x as f64).powi(2) + (ty - u.tile_y as f64).powi(2)).sqrt();
                            if dist <= weapon.range_tiles() && thp > 0 {
                                u.attack_cooldown = weapon.fire_interval();
                                let mut dmg = weapon.damage();
                                if weapon == WeaponType::Laser && u.faction == self.faction {
                                    dmg += self.tech_tree.laser_damage_bonus();
                                }
                                combat_events.push((
                                    tid,
                                    (u.tile_x as f64, u.tile_y as f64),
                                    (tx, ty),
                                    weapon,
                                    dmg,
                                    u.faction,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Apply weapon damage and spawn beams/explosions
        let mut laser_fired = false;
        let mut railgun_fired = false;
        let mut microwave_fired = false;
        let mut acid_fired = false;
        let mut destroyed_units = Vec::new();
        let mut under_attack = false;
        let mut friendly_vehicle_destroyed = false;
        let mut structure_destroyed = false;

        for (target_id, from_pos, to_pos, weapon, dmg, from_faction) in combat_events {
            match weapon {
                WeaponType::Railgun => railgun_fired = true,
                WeaponType::Microwave => microwave_fired = true,
                WeaponType::AcidCloud => acid_fired = true,
                _ => laser_fired = true,
            }
            self.active_beams.push(CombatBeam {
                from_x: from_pos.0,
                from_y: from_pos.1,
                to_x: to_pos.0,
                to_y: to_pos.1,
                color: weapon.beam_color(),
                lifetime: 0.18,
                max_lifetime: 0.18,
            });

            if let Some(target) = self.units.iter_mut().find(|u| u.id == target_id) {
                if target.faction == self.faction && from_faction != self.faction {
                    if self.sol - self.last_under_attack_sol > 15.0 {
                        self.last_under_attack_sol = self.sol;
                        under_attack = true;
                    }
                }
                target.hp = target.hp.saturating_sub(dmg);
                if target.hp == 0 {
                    if target.faction == self.faction {
                        self.casualties += 1;
                        self.friendly_units_lost += 1;
                        friendly_vehicle_destroyed = true;
                    } else {
                        self.enemy_units_destroyed += 1;
                    }
                    destroyed_units.push((target.tile_x as f64, target.tile_y as f64));
                }
            }
        }

        // Automated Guard Post Perimeter Turret Defense
        let player_faction = self.faction;
        let laser_bonus = self.tech_tree.laser_damage_bonus();
        for b in &mut self.buildings {
            if b.name.contains("Guard") && b.is_active {
                b.attack_cooldown -= dt_secs;
                if b.attack_cooldown <= 0.0 {
                    let bx = b.tile_x as f64 + (b.width as f64 * 0.5);
                    let by = b.tile_y as f64 + (b.height as f64 * 0.5);
                    let max_range = 6.5;
                    let mut closest_target = None;
                    let mut min_d = max_range;
                    let guard_faction = b.faction.unwrap_or(player_faction);

                    for &(other_id, other_faction, ox, oy, other_hp) in &unit_positions {
                        if other_faction != guard_faction && other_hp > 0 {
                            let dist = ((ox - bx).powi(2) + (oy - by).powi(2)).sqrt();
                            if dist <= min_d {
                                min_d = dist;
                                closest_target = Some((other_id, ox, oy));
                            }
                        }
                    }

                    if let Some((target_id, tx, ty)) = closest_target {
                        b.attack_cooldown = 1.1; // Rapid automated defensive cannon
                        laser_fired = true;
                        let beam_color = if guard_faction == Faction::Eden { "#00d4ff" } else { "#ef4444" };
                        self.active_beams.push(CombatBeam {
                            from_x: bx,
                            from_y: by,
                            to_x: tx,
                            to_y: ty,
                            color: beam_color,
                            lifetime: 0.22,
                            max_lifetime: 0.22,
                        });

                        let dmg = 25 + if guard_faction == player_faction { laser_bonus } else { 0 };
                        if let Some(target) = self.units.iter_mut().find(|u| u.id == target_id) {
                            if target.faction == player_faction && guard_faction != player_faction {
                                if self.sol - self.last_under_attack_sol > 15.0 {
                                    self.last_under_attack_sol = self.sol;
                                    under_attack = true;
                                }
                            }
                            target.hp = target.hp.saturating_sub(dmg);
                            if target.hp == 0 {
                                if target.faction == self.faction {
                                    self.casualties += 1;
                                    self.friendly_units_lost += 1;
                                    friendly_vehicle_destroyed = true;
                                } else {
                                    self.enemy_units_destroyed += 1;
                                }
                                destroyed_units.push((target.tile_x as f64, target.tile_y as f64));
                            }
                        }
                    }
                }
            }
        }

        let unit_destroyed = !destroyed_units.is_empty();
        for (dx, dy) in destroyed_units {
            self.active_explosions.push(ExplosionEffect {
                x: dx,
                y: dy,
                radius: 4.0,
                max_radius: 20.0,
                lifetime: 0.45,
                max_lifetime: 0.45,
            });
        }
        self.units.retain(|u| u.hp > 0);

        // Autonomous Mining & Ore Logistics Transport
        let mut ore_delivered_common = 0;
        let mut ore_delivered_rare = 0;
        let deposit_coords: Vec<(u32, i32, i32, OreType, bool)> = self.ore_deposits.iter()
            .map(|d| (d.id, d.tile_x, d.tile_y, d.ore_type, d.has_miner && d.remaining_amount > 0))
            .collect();
        let smelter_coords: Vec<(u32, Option<Faction>, i32, i32)> = self.buildings.iter()
            .filter(|b| b.name.contains("Smelter"))
            .map(|b| (b.id, b.faction, b.tile_x, b.tile_y))
            .collect();

        for u in &mut self.units {
            if u.name.contains("Cargo") {
                match u.hauler_state {
                    Some(HaulerState::Idle) => {
                        if let Some(&(dep_id, dx, dy, _, _)) = deposit_coords.iter().find(|d| d.4) {
                            if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (dx, dy)) {
                                u.path = path;
                                u.hauler_state = Some(HaulerState::DrivingToMine { deposit_id: dep_id });
                            }
                        }
                    }
                    Some(HaulerState::DrivingToMine { deposit_id }) => {
                        if let Some(&(_, dx, dy, _, _)) = deposit_coords.iter().find(|d| d.0 == deposit_id) {
                            let dist = (u.tile_x - dx).abs() + (u.tile_y - dy).abs();
                            if dist <= 1 {
                                u.hauler_state = Some(HaulerState::Loading { deposit_id, progress_pct: 0 });
                            }
                        }
                    }
                    Some(HaulerState::Loading { deposit_id, ref mut progress_pct }) => {
                        *progress_pct += (dt_secs * 60.0) as u32;
                        if *progress_pct >= 100 {
                            if let Some(&(_, _, _, ore_t, _)) = deposit_coords.iter().find(|d| d.0 == deposit_id) {
                                u.cargo = Some((ore_t, 50));
                                let friendly_smelter = smelter_coords.iter()
                                    .filter(|s| s.1 == Some(u.faction) || s.1.is_none())
                                    .min_by_key(|s| (u.tile_x - s.2).abs() + (u.tile_y - s.3).abs())
                                    .or_else(|| smelter_coords.first());
                                if let Some(&(smelter_id, _, sx, sy)) = friendly_smelter {
                                    if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (sx, sy)) {
                                        u.path = path;
                                        u.hauler_state = Some(HaulerState::DrivingToSmelter { smelter_id, ore_type: ore_t, amount: 50 });
                                    }
                                }
                            }
                        }
                    }
                    Some(HaulerState::DrivingToSmelter { smelter_id, ore_type, amount }) => {
                        if let Some(&(_, _, sx, sy)) = smelter_coords.iter().find(|s| s.0 == smelter_id) {
                            let dist = (u.tile_x - sx).abs() + (u.tile_y - sy).abs();
                            if dist <= 2 {
                                u.hauler_state = Some(HaulerState::Unloading { smelter_id, ore_type, amount, progress_pct: 0 });
                            }
                        }
                    }
                    Some(HaulerState::Unloading { smelter_id: _, ore_type, amount, ref mut progress_pct }) => {
                        *progress_pct += (dt_secs * 70.0) as u32;
                        if *progress_pct >= 100 {
                            if u.faction == self.faction {
                                match ore_type {
                                    OreType::Common => ore_delivered_common += amount,
                                    OreType::Rare => ore_delivered_rare += amount,
                                }
                            } else if let Some(ai) = self.ai_players.iter_mut().find(|a| a.faction == u.faction) {
                                match ore_type {
                                    OreType::Common => ai.common_ore += amount,
                                    OreType::Rare => ai.rare_ore += amount,
                                }
                            }
                            u.cargo = None;
                            u.hauler_state = Some(HaulerState::Idle);
                        }
                    }
                    None => {}
                }
            }
        }

        let smelt_mult = self.tech_tree.smelting_multiplier();
        let gained_common = (ore_delivered_common as f64 * smelt_mult) as u32;
        let gained_rare = (ore_delivered_rare as f64 * smelt_mult) as u32;
        self.common_ore += gained_common;
        self.rare_ore += gained_rare;
        self.total_common_ore_mined += gained_common;
        self.total_rare_ore_mined += gained_rare;

        // Building Production Queue & Fabrication Progress
        let mut completed_items = Vec::new();
        for b in &mut self.buildings {
            if b.is_active && !b.production_queue.is_empty() {
                b.production_progress += dt_secs * 20.0;
                if b.production_progress >= 100.0 {
                    b.production_progress = 0.0;
                    let item = b.production_queue.remove(0);
                    completed_items.push((b.id, b.tile_x + b.width, b.tile_y + b.height / 2, item));
                }
            }
        }

        let mut unit_notice = None;
        for (b_id, spawn_x, spawn_y, item) in completed_items {
            let b_faction = self.buildings.iter().find(|b| b.id == b_id).and_then(|b| b.faction).unwrap_or(self.faction);
            let next_id = self.units.len() as u32 + 1;
            match item.as_str() {
                "Panther Battle Tank" | "Panther (Railgun)" => {
                    let hp = 250 + if b_faction == self.faction { self.tech_tree.armor_bonus() } else { 0 };
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Panther Battle Tank #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp,
                        max_hp: hp,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: Some(WeaponType::Railgun),
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Panther Battle Tank rolled out of production bay.");
                    }
                }
                "Panther (Laser)" => {
                    let hp = 250 + if b_faction == self.faction { self.tech_tree.armor_bonus() } else { 0 };
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Panther Laser Tank #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp,
                        max_hp: hp,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: Some(WeaponType::Laser),
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Panther Laser Tank rolled out.");
                    }
                }
                "Lynx Scout" | "Lynx (Laser)" => {
                    let hp = 140 + if b_faction == self.faction { self.tech_tree.armor_bonus() } else { 0 };
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Lynx Scout #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp,
                        max_hp: hp,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: Some(WeaponType::Laser),
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Lynx Scout commissioned for reconnaissance.");
                    }
                }
                "Lynx (Microwave)" => {
                    let hp = 140;
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Lynx Microwave Raider #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp,
                        max_hp: hp,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: Some(WeaponType::Microwave),
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Lynx Microwave Raider deployed.");
                    }
                }
                "Lynx (Acid Cloud)" => {
                    let hp = 140;
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Lynx Acid Raider #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp,
                        max_hp: hp,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: Some(WeaponType::AcidCloud),
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Lynx Acid Raider deployed.");
                    }
                }
                "Cargo Truck" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Cargo Truck #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 150,
                        max_hp: 150,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: Some(HaulerState::Idle),
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Cargo Truck deployed for ore logistics.");
                    }
                }
                "Repair Vehicle" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Repair Vehicle #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 180,
                        max_hp: 180,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Repair Vehicle deployed for field maintenance.");
                    }
                }
                "Earthworker" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Earthworker #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 200,
                        max_hp: 200,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Earthworker commissioned for engineering and terrain fortification.");
                    }
                }
                "ConVec" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("ConVec #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 180,
                        max_hp: 180,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: ConVec deployed for modular kit transport.");
                    }
                }
                "ConVec (Smelter)" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("ConVec #{}", next_id),
                        faction: self.faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 180,
                        max_hp: 180,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: Some("Common Ore Smelter".to_string()),
                    });
                    unit_notice = Some("FABRICATION COMPLETE: ConVec pre-loaded with Common Smelter Kit.");
                }
                "ConVec (Solar)" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("ConVec #{}", next_id),
                        faction: self.faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 180,
                        max_hp: 180,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: Some("Solar Power Array".to_string()),
                    });
                    unit_notice = Some("FABRICATION COMPLETE: ConVec pre-loaded with Solar Array Kit.");
                }
                "ConVec (Guard)" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("ConVec #{}", next_id),
                        faction: self.faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 180,
                        max_hp: 180,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: Some("Guard Post Turret".to_string()),
                    });
                    unit_notice = Some("FABRICATION COMPLETE: ConVec pre-loaded with Guard Post Turret Kit.");
                }
                "ConVec (Agridome)" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("ConVec #{}", next_id),
                        faction: self.faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 180,
                        max_hp: 180,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: Some("Bio-Dome Hydroponics".to_string()),
                    });
                    unit_notice = Some("FABRICATION COMPLETE: ConVec pre-loaded with Hydroponics Kit.");
                }
                "ConVec (Geothermal)" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("ConVec #{}", next_id),
                        faction: self.faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp: 180,
                        max_hp: 180,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: Some("Geothermal Power Plant".to_string()),
                    });
                    unit_notice = Some("FABRICATION COMPLETE: ConVec pre-loaded with Geothermal Plant Kit.");
                }
                "Tiger Heavy Tank" => {
                    let hp = 420 + if b_faction == self.faction { self.tech_tree.armor_bonus() } else { 0 };
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Tiger Heavy Tank #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp,
                        max_hp: hp,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: Some(WeaponType::Railgun),
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Tiger Heavy Assault Tank with Railgun rolled out.");
                    }
                }
                "Robo-Miner" => {
                    let hp = 160 + if b_faction == self.faction { self.tech_tree.armor_bonus() } else { 0 };
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Robo-Miner #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp,
                        max_hp: hp,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Robo-Miner ready to construct subterranean mine heads.");
                    }
                }
                "Command Module" | "Cryo Stasis Bay" | "Habitat Ring" | "Ion Drive" | "Starship Component" => {
                    self.starship_progress = (self.starship_progress + 25.0).min(100.0);
                    if self.starship_progress >= 100.0 {
                        unit_notice = Some("STARSHIP COMPLETE: All evacuation modules installed! Starship ready for orbital launch!");
                    } else {
                        unit_notice = Some("STARSHIP PROGRESS: Evacuation module installed (+25%).");
                    }
                }
                "Evacuation Transport" => {
                    let hp = 220 + if b_faction == self.faction { self.tech_tree.armor_bonus() } else { 0 };
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Evacuation Transport #{}", next_id),
                        faction: b_faction,
                        tile_x: spawn_x,
                        tile_y: spawn_y,
                        hp,
                        max_hp: hp,
                        is_selected: false,
                        current_order: UnitOrder::Idle,
                        path: Vec::new(),
                        move_cooldown: 0.0,
                        weapon: None,
                        attack_cooldown: 0.0,
                        cargo: None,
                        hauler_state: None,
                        cargo_kit: None,
                    });
                    if b_faction == self.faction {
                        unit_notice = Some("FABRICATION COMPLETE: Evacuation Transport rolled out for colonist transit.");
                    }
                }
                _ => {}
            }
        }

        // Field Repairs & Vehicle Maintenance Depots
        let repair_depots: Vec<(i32, i32, Faction)> = self.buildings.iter()
            .filter(|b| b.is_active && b.name.contains("Repair"))
            .map(|b| (b.tile_x + b.width / 2, b.tile_y + b.height / 2, b.faction.unwrap_or(self.faction)))
            .collect();

        // 1. Repair Depots radiate emerald nano-welding beams to nearby damaged units
        for (dx, dy, rep_faction) in &repair_depots {
            for u in &mut self.units {
                if u.faction == *rep_faction && u.hp < u.max_hp {
                    let dist = (u.tile_x - dx).abs() + (u.tile_y - dy).abs();
                    if dist <= 5 {
                        let heal = ((dt_secs * 30.0) as u32).max(1);
                        u.hp = (u.hp + heal).min(u.max_hp);
                        self.active_beams.push(CombatBeam {
                            from_x: *dx as f64,
                            from_y: *dy as f64,
                            to_x: u.tile_x as f64,
                            to_y: u.tile_y as f64,
                            color: "#10b981".into(), // Emerald nano-repair beam
                            lifetime: 0.15,
                            max_lifetime: 0.15,
                        });
                        break;
                    }
                }
            }
        }

        // 2. Mobile Repair Vehicles heal targeted or nearby damaged allies
        let unit_positions: Vec<(u32, Faction, i32, i32, u32, u32)> = self.units.iter()
            .map(|u| (u.id, u.faction, u.tile_x, u.tile_y, u.hp, u.max_hp))
            .collect();

        let mut repair_beams = Vec::new();
        for u in &mut self.units {
            if u.name.contains("Repair") && u.faction == self.faction {
                let mut target_id = None;
                if let UnitOrder::Repair { target_unit_id } = u.current_order {
                    target_id = Some(target_unit_id);
                } else if u.current_order == UnitOrder::Idle {
                    if let Some(target) = unit_positions.iter().find(|p| {
                        p.1 == u.faction && p.0 != u.id && p.4 < p.5 && (p.2 - u.tile_x).abs() + (p.3 - u.tile_y).abs() <= 6
                    }) {
                        target_id = Some(target.0);
                    }
                }

                if let Some(tid) = target_id {
                    if let Some(target_pos) = unit_positions.iter().find(|p| p.0 == tid) {
                        let dist = (target_pos.2 - u.tile_x).abs() + (target_pos.3 - u.tile_y).abs();
                        if dist <= 3 {
                            if target_pos.4 < target_pos.5 {
                                repair_beams.push((u.id, tid, (u.tile_x as f64, u.tile_y as f64), (target_pos.2 as f64, target_pos.3 as f64)));
                            } else if let UnitOrder::Repair { .. } = u.current_order {
                                u.current_order = UnitOrder::Idle;
                            }
                        } else if u.path.is_empty() {
                            if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (target_pos.2, target_pos.3)) {
                                u.path = path;
                            }
                        }
                    }
                }
            }
        }

        for (_rep_id, tid, from_pos, to_pos) in repair_beams {
            self.active_beams.push(CombatBeam {
                from_x: from_pos.0,
                from_y: from_pos.1,
                to_x: to_pos.0,
                to_y: to_pos.1,
                color: "#34d399".into(), // Bright mint repair nanites
                lifetime: 0.15,
                max_lifetime: 0.15,
            });
            if let Some(target) = self.units.iter_mut().find(|u| u.id == tid) {
                let heal = ((dt_secs * 25.0) as u32).max(1);
                target.hp = (target.hp + heal).min(target.max_hp);
            }
        }

        // 3. Microbial Blight Caustic Hazard: units traversing blight tiles take acid damage
        for u in &mut self.units {
            let idx = (u.tile_y.clamp(0, self.game_map.height as i32 - 1) as u32 * self.game_map.width
                + u.tile_x.clamp(0, self.game_map.width as i32 - 1) as u32) as usize;
            if idx < self.game_map.tiles.len() && self.game_map.tiles[idx].is_blight {
                u.hp = u.hp.saturating_sub(((dt_secs * 6.0) as u32).max(1));
            }
        }

        // 4. Earthworker Construction & Hazard Remediation Orders
        for u in &mut self.units {
            if u.name.contains("Earthworker") && u.faction == self.faction {
                match u.current_order {
                    UnitOrder::BuildWall { target_x, target_y } => {
                        let dist = (u.tile_x - target_x).abs() + (u.tile_y - target_y).abs();
                        if dist <= 1 {
                            if self.tech_tree.can_build_walls() && self.common_ore >= 15 {
                                let idx = (target_y.clamp(0, self.game_map.height as i32 - 1) as u32 * self.game_map.width
                                    + target_x.clamp(0, self.game_map.width as i32 - 1) as u32) as usize;
                                if idx < self.game_map.tiles.len() {
                                    self.common_ore -= 15;
                                    self.game_map.tiles[idx].is_wall_or_building = true;
                                    self.active_beams.push(CombatBeam {
                                        from_x: u.tile_x as f64,
                                        from_y: u.tile_y as f64,
                                        to_x: target_x as f64,
                                        to_y: target_y as f64,
                                        color: "#f59e0b".into(),
                                        lifetime: 0.25,
                                        max_lifetime: 0.25,
                                    });
                                }
                            }
                            u.current_order = UnitOrder::Idle;
                        } else if u.path.is_empty() {
                            if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (target_x, target_y)) {
                                u.path = path;
                            }
                        }
                    }
                    UnitOrder::BuildTube { target_x, target_y } => {
                        let dist = (u.tile_x - target_x).abs() + (u.tile_y - target_y).abs();
                        if dist <= 1 {
                            if self.common_ore >= 5 {
                                self.common_ore -= 5;
                                self.tubes.insert((target_x, target_y));
                                self.active_beams.push(CombatBeam {
                                    from_x: u.tile_x as f64,
                                    from_y: u.tile_y as f64,
                                    to_x: target_x as f64,
                                    to_y: target_y as f64,
                                    color: "#38bdf8".into(),
                                    lifetime: 0.25,
                                    max_lifetime: 0.25,
                                });
                            }
                            u.current_order = UnitOrder::Idle;
                        } else if u.path.is_empty() {
                            if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (target_x, target_y)) {
                                u.path = path;
                            }
                        }
                    }
                    UnitOrder::ClearHazard { target_x, target_y } => {
                        let dist = (u.tile_x - target_x).abs() + (u.tile_y - target_y).abs();
                        if dist <= 1 {
                            let idx = (target_y.clamp(0, self.game_map.height as i32 - 1) as u32 * self.game_map.width
                                + target_x.clamp(0, self.game_map.width as i32 - 1) as u32) as usize;
                            if idx < self.game_map.tiles.len() {
                                self.game_map.tiles[idx].is_blight = false;
                                self.active_beams.push(CombatBeam {
                                    from_x: u.tile_x as f64,
                                    from_y: u.tile_y as f64,
                                    to_x: target_x as f64,
                                    to_y: target_y as f64,
                                    color: "#a855f7".into(),
                                    lifetime: 0.25,
                                    max_lifetime: 0.25,
                                  });
                            }
                            u.current_order = UnitOrder::Idle;
                        } else if u.path.is_empty() {
                            if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (target_x, target_y)) {
                                u.path = path;
                            }
                        }
                    }
                    UnitOrder::Bulldoze { target_x, target_y } => {
                        let dist = (u.tile_x - target_x).abs() + (u.tile_y - target_y).abs();
                        if dist <= 1 {
                            let idx = (target_y.clamp(0, self.game_map.height as i32 - 1) as u32 * self.game_map.width
                                + target_x.clamp(0, self.game_map.width as i32 - 1) as u32) as usize;
                            if idx < self.game_map.tiles.len() {
                                self.game_map.tiles[idx].is_wall_or_building = false;
                            }
                            self.tubes.remove(&(target_x, target_y));
                            self.active_beams.push(CombatBeam {
                                from_x: u.tile_x as f64,
                                from_y: u.tile_y as f64,
                                to_x: target_x as f64,
                                to_y: target_y as f64,
                                color: "#ef4444".into(),
                                lifetime: 0.25,
                                max_lifetime: 0.25,
                            });
                            u.current_order = UnitOrder::Idle;
                        } else if u.path.is_empty() {
                            if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (target_x, target_y)) {
                                u.path = path;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // 5. ConVec Structure Deployment Orders
        let mut building_deployed = false;
        let mut convec_deploy_events = Vec::new();
        for u in &mut self.units {
            if u.faction == self.faction {
                if let UnitOrder::DeployStructure { ref kit_name, tile_x, tile_y } = u.current_order {
                    let dist = (u.tile_x - tile_x).abs() + (u.tile_y - tile_y).abs();
                    if dist <= 1 {
                        convec_deploy_events.push((u.id, kit_name.clone(), tile_x, tile_y));
                        u.current_order = UnitOrder::Idle;
                    } else if u.path.is_empty() {
                        if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (tile_x, tile_y)) {
                            u.path = path;
                        }
                    }
                }
            }
        }

        for (uid, _kit_name, tx, ty) in convec_deploy_events {
            if let Ok(_msg) = self.deploy_convec_kit(uid, tx, ty) {
                building_deployed = true;
                unit_notice = Some("STRUCTURE DEPLOYED: ConVec erected installation into operational grid.");
            }
        }

        // 6. Tactical Guard & Patrol Orders
        for u in &mut self.units {
            match u.current_order {
                UnitOrder::Guard { anchor_x, anchor_y } => {
                    let closest_enemy = unit_positions.iter()
                        .filter(|p| p.1 != u.faction && p.4 > 0)
                        .map(|p| {
                            let dist_from_anchor = ((p.2 as f64 - anchor_x as f64).powi(2) + (p.3 as f64 - anchor_y as f64).powi(2)).sqrt();
                            (p.0, p.2, p.3, dist_from_anchor)
                        })
                        .filter(|p| p.3 <= 7.0)
                        .min_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));

                    if let Some((_eid, ex, ey, _)) = closest_enemy {
                        let dist_to_enemy = (u.tile_x - ex).abs() + (u.tile_y - ey).abs();
                        let max_range = u.weapon.map(|w| w.range_tiles() as i32).unwrap_or(1);
                        if dist_to_enemy > max_range && u.path.is_empty() {
                            if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (ex, ey)) {
                                u.path = path;
                            }
                        }
                    } else {
                        let dist_to_anchor = (u.tile_x - anchor_x).abs() + (u.tile_y - anchor_y).abs();
                        if dist_to_anchor > 1 && u.path.is_empty() {
                            if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (anchor_x, anchor_y)) {
                                u.path = path;
                            }
                        }
                    }
                }
                UnitOrder::Patrol { start_x, start_y, end_x, end_y, ref mut heading_to_end } => {
                    let (tx, ty) = if *heading_to_end { (end_x, end_y) } else { (start_x, start_y) };
                    let dist = (u.tile_x - tx).abs() + (u.tile_y - ty).abs();
                    if dist <= 1 {
                        *heading_to_end = !*heading_to_end;
                        let next_target = if *heading_to_end { (end_x, end_y) } else { (start_x, start_y) };
                        if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), next_target) {
                            u.path = path;
                        }
                    } else if u.path.is_empty() {
                        if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (tx, ty)) {
                            u.path = path;
                        }
                    }
                }
                _ => {}
            }
        }

        // 7. Atmospheric Vortices: sever conduits and buffet units
        for v in &self.disasters.active_vortices {
            let vx_tile = v.x.round() as i32;
            let vy_tile = v.y.round() as i32;
            self.tubes.remove(&(vx_tile, vy_tile));

            for u in &mut self.units {
                let dist = ((u.tile_x as f64 - v.x).powi(2) + (u.tile_y as f64 - v.y).powi(2)).sqrt();
                if dist <= v.radius {
                    u.hp = u.hp.saturating_sub(((dt_secs * 8.0) as u32).max(1));
                }
            }
        }

        // 8. Hydroponics Life Support & Demographics
        let food_mult = self.tech_tree.food_multiplier();
        if (self.sol * 10.0) as u32 % 50 == 0 {
            let agro_count = self.buildings.iter().filter(|b| b.name.contains("Bio-Dome") && b.is_active).count() as u32;
            let harvest = (agro_count as f64 * 35.0 * food_mult) as u32;
            self.food_units = self.food_units.saturating_add(harvest).saturating_sub(self.population / 4);
        }

        // Colony Demographics & Life Support Dynamics
        if self.power_consumed > self.power_produced {
            if (self.sol * 10.0) as u32 % 50 == 0 {
                self.morale_pct = self.morale_pct.saturating_sub(1);
                if self.morale_pct < 50 && self.population > 20 {
                    self.population -= 1;
                }
            }
        } else if self.morale_pct < 100 && (self.sol * 10.0) as u32 % 50 == 0 {
            self.morale_pct = (self.morale_pct + 1).min(100);
        }

        // Demographic population growth when thriving with power, food surplus, and high morale
        if self.power_produced >= self.power_consumed && self.food_units > self.population && self.morale_pct >= 90 {
            if self.tick_count % 100 == 0 {
                self.population = self.population.saturating_add(1);
            }
        }

        // Advance Rival Faction AI Director
        let player_targets: Vec<(i32, i32)> = self.buildings
            .iter()
            .map(|b| (b.tile_x + b.width / 2, b.tile_y + b.height / 2))
            .chain(self.units.iter().filter(|u| u.faction == self.faction).map(|u| (u.tile_x, u.tile_y)))
            .collect();
        let ai_alert = self.ai_director.update(&mut self.units, &player_targets, dt_secs);

        // Advance Autonomous Skirmish & Campaign AI Players
        let current_tick = self.tick_count as u32;
        let mut ai_players = std::mem::take(&mut self.ai_players);
        for ai in &mut ai_players {
            let _ = ai.update(self, current_tick);
        }
        self.ai_players = ai_players;

        // Advance Dynamic Mission Scripting Event Engine
        let mut engine = std::mem::take(&mut self.event_engine);
        engine.update(self);
        self.event_engine = engine;

        // Evaluate Campaign Mission Objectives
        if self.scenario_id == 5 {
            let spaceport_pos = self.buildings.iter()
                .find(|b| b.name.contains("Starship Launch Gantry") || b.name.contains("Spaceport"))
                .map(|b| (b.tile_x + b.width / 2, b.tile_y + b.height / 2));

            if let Some((sx, sy)) = spaceport_pos {
                let has_docked_transport = self.units.iter().any(|u| {
                    u.faction == Faction::Eden
                        && (u.name.contains("Evac") || u.name.contains("Transport"))
                        && (u.tile_x - sx).abs() + (u.tile_y - sy).abs() <= 6
                });

                if has_docked_transport && self.evacuated_colonists < 150 && self.population > 0 {
                    let boarding = ((dt_secs * 20.0).ceil() as u32).min(self.population).min(150 - self.evacuated_colonists);
                    self.evacuated_colonists += boarding;
                    self.population = self.population.saturating_sub(boarding);
                }
            }
        }
        if self.starship_launched {
            self.evacuated_colonists = self.evacuated_colonists.max(self.population);
        }
        if self.buildings.iter().any(|b| b.name.contains("Terraformer") && b.is_active) {
            self.terraformer_saturation = (self.terraformer_saturation + dt_secs * 5.0).min(100.0);
        }
        let mission_alert = self.mission.evaluate(
            self.common_ore,
            self.rare_ore,
            self.power_produced,
            self.power_consumed,
            self.tubes.len(),
            self.morale_pct,
            &self.buildings,
            &self.units,
            &self.tech_tree,
            self.starship_progress,
            self.evacuated_colonists,
            self.terraformer_saturation,
        );

        // Process Meteor Impact Detonations
        let mut detonations = Vec::new();
        std::mem::swap(&mut self.disasters.detonation_queue, &mut detonations);
        let meteor_hit = !detonations.is_empty();
        for (mx, my, radius) in detonations {
            self.active_explosions.push(ExplosionEffect {
                x: mx,
                y: my,
                radius: 4.0,
                max_radius: 28.0,
                lifetime: 0.55,
                max_lifetime: 0.55,
            });
            self.disasters.quake_duration = 1.8;

            let m_tile_x = mx.round() as i32;
            let m_tile_y = my.round() as i32;
            self.tubes.remove(&(m_tile_x, m_tile_y));

            // Damage units caught in blast radius
            for u in &mut self.units {
                let dist = ((u.tile_x as f64 - mx).powi(2) + (u.tile_y as f64 - my).powi(2)).sqrt();
                if dist <= radius {
                    let blast_dmg = (80.0 * (1.0 - dist / radius)) as u32;
                    let old_hp = u.hp;
                    u.hp = u.hp.saturating_sub(blast_dmg.max(25));
                    if old_hp > 0 && u.hp == 0 {
                        if u.faction == self.faction {
                            self.casualties += 1;
                            self.friendly_units_lost += 1;
                            friendly_vehicle_destroyed = true;
                        } else {
                            self.enemy_units_destroyed += 1;
                        }
                    }
                }
            }

            // Damage and destroy buildings caught in blast radius
            let mut meteor_destroyed_buildings = Vec::new();
            for b in &self.buildings {
                let bx = b.tile_x as f64 + (b.width as f64) * 0.5;
                let by = b.tile_y as f64 + (b.height as f64) * 0.5;
                let dist = ((bx - mx).powi(2) + (by - my).powi(2)).sqrt();
                if dist <= radius {
                    meteor_destroyed_buildings.push(b.id);
                }
            }
            for b_id in meteor_destroyed_buildings {
                self.destroyed_building_ids.insert(b_id);
                structure_destroyed = true;
                if let Some(idx) = self.buildings.iter().position(|b| b.id == b_id) {
                    let b = self.buildings.remove(idx);
                    if b.power_cost > 0 {
                        self.power_consumed = self.power_consumed.saturating_sub(b.power_cost as u32);
                    }
                }
            }
        }

        // Check if advancing lava flows overrun colony structures
        let mut lava_destroyed_b_ids = Vec::new();
        for b in &self.buildings {
            let mut hit = false;
            for dy in 0..b.height {
                for dx in 0..b.width {
                    if let Some(tile) = self.game_map.get_tile((b.tile_x + dx) as u32, (b.tile_y + dy) as u32) {
                        if tile.is_lava {
                            hit = true;
                            break;
                        }
                    }
                }
                if hit {
                    break;
                }
            }
            if hit {
                lava_destroyed_b_ids.push(b.id);
            }
        }
        for b_id in lava_destroyed_b_ids {
            self.destroyed_building_ids.insert(b_id);
            structure_destroyed = true;
            if let Some(idx) = self.buildings.iter().position(|b| b.id == b_id) {
                let b = self.buildings.remove(idx);
                if b.power_cost > 0 {
                    self.power_consumed = self.power_consumed.saturating_sub(b.power_cost as u32);
                }
            }
        }

        if self.starship_launched {
            self.launch_anim_time += dt_secs;
        }

        let mut mission_ended = None;
        if !self.debriefing_shown {
            if self.mission.status == mission::MissionStatus::Victory || self.mission.status == mission::MissionStatus::Defeat {
                self.debriefing_shown = true;
                mission_ended = Some(self.mission.status);
            } else if self.starship_launched && self.launch_anim_time >= 3.0 {
                self.debriefing_shown = true;
                self.mission.status = mission::MissionStatus::Victory;
                mission_ended = Some(mission::MissionStatus::Victory);
            }

            if let Some(status) = mission_ended {
                let is_vic = status == mission::MissionStatus::Victory;
                let debrief = crate::sim::debrief::MissionDebrief::new(
                    self.sol,
                    self.total_common_ore_mined,
                    self.total_rare_ore_mined,
                    self.tech_tree.researched_count() as u32,
                    self.enemy_units_destroyed,
                    self.friendly_units_lost,
                    self.population,
                    self.casualties,
                    self.starship_progress,
                    is_vic,
                );

                let end_seq = if is_vic {
                    let v_type = match self.scenario_id {
                        5 => crate::sim::debrief::VictoryType::StarshipEvacuation,
                        6 => crate::sim::debrief::VictoryType::TerraformerSaturation,
                        _ => if self.starship_launched {
                            crate::sim::debrief::VictoryType::StarshipEvacuation
                        } else {
                            crate::sim::debrief::VictoryType::ObjectivesCompleted
                        },
                    };
                    crate::sim::debrief::EndGameSequence::Victory {
                        faction: self.faction,
                        victory_type: v_type,
                        score: debrief,
                    }
                } else {
                    let d_reason = match self.scenario_id {
                        6 if !self.buildings.iter().any(|b| b.name.contains("Terraformer") && b.is_active) => {
                            crate::sim::debrief::DefeatReason::TerraformerDestroyed
                        }
                        5 if self.population == 0 && self.evacuated_colonists < 150 => {
                            crate::sim::debrief::DefeatReason::EvacuationFailed
                        }
                        _ if self.population == 0 => crate::sim::debrief::DefeatReason::PopulationExtinct,
                        _ => crate::sim::debrief::DefeatReason::CommandCenterDestroyed,
                    };
                    crate::sim::debrief::EndGameSequence::Defeat {
                        faction: self.faction,
                        defeat_reason: d_reason,
                        score: debrief,
                    }
                };
                self.end_game = Some(end_seq);
            }
        }

        // Refresh dynamic sensor fog of war
        self.fog.update_visibility(&self.units, &self.buildings);

        let vortex_active = !self.disasters.active_vortices.is_empty();
        let volcano_erupted = warning.map(|w| w.contains("Volcanic fissure") || w.contains("eruption")).unwrap_or(false);
        let vortex_spawned = warning.map(|w| w.contains("Rotary dust vortex") || w.contains("Vortex")).unwrap_or(false);
        let disaster_siren = meteor_hit
            || volcano_erupted
            || ai_alert.map(|a| a.contains("Red Alert")).unwrap_or(false)
            || warning.map(|w| w.contains("Disaster") || w.contains("Tremor") || w.contains("Eruption") || w.contains("Blight")).unwrap_or(false);

        SimUpdateResult {
            notice: warning.or(ai_alert).or(unit_notice).or(mission_alert),
            laser_fired,
            railgun_fired,
            microwave_fired,
            acid_fired,
            unit_destroyed,
            friendly_vehicle_destroyed,
            structure_destroyed,
            volcano_erupted,
            vortex_spawned,
            building_deployed,
            vortex_active,
            meteor_hit,
            disaster_siren,
            mission_ended,
            starship_launched: self.starship_launched,
            under_attack,
            end_game_sequence: self.end_game.clone(),
        }
    }

    /// Deploy a colonial structure kit, deducting ore costs and weaving transport tubes
    pub fn deploy_structure(&mut self, structure_type: StructureType, tile_x: i32, tile_y: i32) -> Result<String, String> {
        if structure_type == StructureType::GuardPost && !self.tech_tree.can_build_guard_posts() {
            return Err("Requires Research: Automated Point Defense Turrets (600 C)".into());
        }
        if structure_type == StructureType::GeothermalPlant && !self.tech_tree.can_build_geothermal() {
            return Err("Requires Research: Vulcanology & Hydrothermal Extraction (750 C)".into());
        }

        let (w, h) = structure_type.dimensions();
        let (cost_common, cost_rare) = structure_type.ore_cost();

        if self.common_ore < cost_common || self.rare_ore < cost_rare {
            return Err(format!("Insufficient Ore: requires {} Common, {} Rare", cost_common, cost_rare));
        }

        // Check if footprint collides with existing structures or impassable lava
        for b in &self.buildings {
            let collides_x = tile_x < b.tile_x + b.width && tile_x + w > b.tile_x;
            let collides_y = tile_y < b.tile_y + b.height && tile_y + h > b.tile_y;
            if collides_x && collides_y {
                return Err(format!("Placement conflict: space occupied by {}", b.name));
            }
        }

        for dy in 0..h {
            for dx in 0..w {
                let tx = (tile_x + dx) as u32;
                let ty = (tile_y + dy) as u32;
                if let Some(tile) = self.game_map.get_tile(tx, ty) {
                    if tile.is_lava {
                        return Err("Cannot deploy structure onto active molten lava fissure!".into());
                    }
                }
            }
        }

        // Deduct resources
        self.common_ore -= cost_common;
        self.rare_ore -= cost_rare;

        let next_id = self.buildings.len() as u32 + 1;
        let power = structure_type.power_cost();
        let name = format!("{} #{}", structure_type.name(), next_id);

        let new_b = Building {
            id: next_id,
            name: name.clone(),
            tile_x,
            tile_y,
            width: w,
            height: h,
            power_cost: power,
            is_active: true,
            is_selected: false,
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
            faction: Some(self.faction),
        };

        // Connect new structure to the nearest existing building via pressurized transport tube
        if let Some(nearest) = self.buildings.iter().min_by_key(|b| (b.tile_x - tile_x).abs() + (b.tile_y - tile_y).abs()) {
            let start = (tile_x + w / 2, tile_y + h / 2);
            let end = (nearest.tile_x + nearest.width / 2, nearest.tile_y + nearest.height / 2);
            connect_with_tubes(&mut self.tubes, start, end);
        }

        if power > 0 {
            self.power_consumed += power as u32;
        } else {
            self.power_produced += (-power) as u32;
        }

        self.buildings.push(new_b);
        self.fog.update_visibility(&self.units, &self.buildings);

        Ok(format!("Deployed {} successfully", name))
    }

    /// Deploy a structure from a ConVec's loaded construction kit
    pub fn deploy_convec_kit(&mut self, unit_id: u32, tile_x: i32, tile_y: i32) -> Result<String, String> {
        let (kit_name, ux, uy) = {
            let u = self.units.iter().find(|u| u.id == unit_id)
                .ok_or_else(|| "ConVec not found".to_string())?;
            let kit = u.cargo_kit.as_ref()
                .ok_or_else(|| "ConVec has no construction kit loaded".to_string())?;
            (kit.clone(), u.tile_x, u.tile_y)
        };

        let st = StructureType::from_name(&kit_name)
            .ok_or_else(|| format!("Unknown kit specification: {}", kit_name))?;

        let (w, h) = st.dimensions();

        // Check if footprint collides with existing structures or impassable lava
        for b in &self.buildings {
            let collides_x = tile_x < b.tile_x + b.width && tile_x + w > b.tile_x;
            let collides_y = tile_y < b.tile_y + b.height && tile_y + h > b.tile_y;
            if collides_x && collides_y {
                return Err(format!("Placement conflict: space occupied by {}", b.name));
            }
        }

        for dy in 0..h {
            for dx in 0..w {
                let tx = (tile_x + dx) as u32;
                let ty = (tile_y + dy) as u32;
                if let Some(tile) = self.game_map.get_tile(tx, ty) {
                    if tile.is_lava {
                        return Err("Cannot deploy structure onto active molten lava fissure!".into());
                    }
                }
            }
        }

        let next_id = self.buildings.len() as u32 + 1;
        let power = st.power_cost();
        let name = format!("{} #{}", st.name(), next_id);

        let new_b = Building {
            id: next_id,
            name: name.clone(),
            tile_x,
            tile_y,
            width: w,
            height: h,
            power_cost: power,
            is_active: true,
            is_selected: false,
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
            faction: Some(self.faction),
        };

        // Mark map tiles as wall or building
        for dy in 0..h {
            for dx in 0..w {
                let tx = (tile_x + dx) as u32;
                let ty = (tile_y + dy) as u32;
                let idx = (ty * self.game_map.width + tx) as usize;
                if idx < self.game_map.tiles.len() {
                    self.game_map.tiles[idx].is_wall_or_building = true;
                }
            }
        }

        // Connect new structure to nearest building via transport tubes
        if let Some(nearest) = self.buildings.iter().min_by_key(|b| (b.tile_x - tile_x).abs() + (b.tile_y - tile_y).abs()) {
            let start = (tile_x + w / 2, tile_y + h / 2);
            let end = (nearest.tile_x + nearest.width / 2, nearest.tile_y + nearest.height / 2);
            connect_with_tubes(&mut self.tubes, start, end);
        }

        if power > 0 {
            self.power_consumed += power as u32;
        } else {
            self.power_produced += (-power) as u32;
        }

        self.buildings.push(new_b);
        self.fog.update_visibility(&self.units, &self.buildings);

        // Spawn visual beam effect
        self.active_beams.push(CombatBeam {
            from_x: ux as f64,
            from_y: uy as f64,
            to_x: (tile_x + w / 2) as f64,
            to_y: (tile_y + h / 2) as f64,
            color: "#38bdf8".into(),
            lifetime: 0.35,
            max_lifetime: 0.35,
        });

        // Destroy / consume the ConVec into foundation
        if let Some(u) = self.units.iter_mut().find(|u| u.id == unit_id) {
            u.hp = 0;
        }
        self.units.retain(|u| u.hp > 0);

        Ok(format!("{} deployed successfully!", name))
    }

    /// Load a construction kit onto an idle ConVec
    pub fn load_convec_kit(&mut self, unit_id: u32, kit_name: String) -> Result<String, String> {
        let st = StructureType::from_name(&kit_name)
            .ok_or_else(|| format!("Unknown kit type: {}", kit_name))?;

        if st == StructureType::GuardPost && !self.tech_tree.can_build_guard_posts() {
            return Err("Requires Research: Automated Point Defense Turrets (600 C)".into());
        }
        if st == StructureType::GeothermalPlant && !self.tech_tree.can_build_geothermal() {
            return Err("Requires Research: Vulcanology & Hydrothermal Extraction (750 C)".into());
        }

        let (cost_common, cost_rare) = st.ore_cost();

        if self.common_ore < cost_common || self.rare_ore < cost_rare {
            return Err(format!("Insufficient Ore for {}: requires {} C, {} R", kit_name, cost_common, cost_rare));
        }

        let u = self.units.iter_mut().find(|u| u.id == unit_id)
            .ok_or_else(|| "Unit not found".to_string())?;

        if !u.name.contains("ConVec") {
            return Err("Only ConVec transport vehicles can load construction kits".into());
        }

        if u.cargo_kit.is_some() {
            return Err("ConVec is already carrying a kit".into());
        }

        self.common_ore -= cost_common;
        self.rare_ore -= cost_rare;
        u.cargo_kit = Some(kit_name.clone());
        Ok(format!("Loaded {} Kit onto {}", kit_name, u.name))
    }

    /// Queue a vehicle or starship module in a selected factory
    pub fn queue_production(&mut self, building_id: u32, item: String) -> Result<String, String> {
        // Tech prerequisites
        if item == "Tiger Heavy Tank" && !self.tech_tree.can_build_railgun_tanks() {
            return Err("Requires Research: Magnetic Acceleration & Railgun Armaments (1400 C)".into());
        }
        if (item == "ConVec (Guard)" || item == "Guard Post") && !self.tech_tree.can_build_guard_posts() {
            return Err("Requires Research: Automated Point Defense Turrets (600 C)".into());
        }
        if (item == "ConVec (Geothermal)" || item == "Geothermal Power Plant") && !self.tech_tree.can_build_geothermal() {
            return Err("Requires Research: Vulcanology & Hydrothermal Extraction (750 C)".into());
        }

        let (cost_common, cost_rare) = match item.as_str() {
            "Panther Battle Tank" => (250, 50),
            "Tiger Heavy Tank" => (350, 100),
            "Lynx Scout" => (140, 0),
            "Cargo Truck" => (150, 0),
            "Robo-Miner" => (200, 40),
            "Repair Vehicle" => (180, 20),
            "Earthworker" => (160, 0),
            "ConVec" => (150, 0),
            "ConVec (Smelter)" => (450, 50),
            "ConVec (Solar)" => (320, 0),
            "ConVec (Guard)" => (350, 50),
            "ConVec (Agridome)" => (320, 50),
            "ConVec (Geothermal)" => (450, 75),
            "Command Module" | "Cryo Stasis Bay" | "Habitat Ring" | "Ion Drive" | "Starship Component" => (500, 200),
            "Evacuation Transport" => (200, 50),
            _ => (100, 0),
        };

        if self.common_ore < cost_common || self.rare_ore < cost_rare {
            return Err(format!("Insufficient Ore: requires {} Common, {} Rare", cost_common, cost_rare));
        }

        if let Some(b) = self.buildings.iter_mut().find(|b| b.id == building_id) {
            self.common_ore -= cost_common;
            self.rare_ore -= cost_rare;
            b.production_queue.push(item.clone());
            Ok(format!("Queued {}", item))
        } else {
            Err("Target facility not found".into())
        }
    }

    /// Dismantle and recycle an existing colony structure, reclaiming 50% of original ore investment
    pub fn recycle_structure(&mut self, building_id: u32) -> Result<String, String> {
        let b_idx = self.buildings.iter().position(|b| b.id == building_id)
            .ok_or_else(|| "Structure not found".to_string())?;

        self.destroyed_building_ids.insert(building_id);
        let building = self.buildings.remove(b_idx);
        let base_name = building.name.split(" #").next().unwrap_or(&building.name);
        let st_opt = StructureType::from_name(base_name);
        let (refund_common, refund_rare) = match st_opt {
            Some(st) => {
                let (c, r) = st.ore_cost();
                (c / 2, r / 2)
            }
            None => (100, 25),
        };

        if building.power_cost > 0 {
            self.power_consumed = self.power_consumed.saturating_sub(building.power_cost as u32);
        } else {
            self.power_produced = self.power_produced.saturating_sub((-building.power_cost) as u32);
        }

        self.common_ore += refund_common;
        self.rare_ore += refund_rare;
        self.fog.update_visibility(&self.units, &self.buildings);

        Ok(format!("{} dismantled. Reclaimed {} Common, {} Rare ore.", building.name, refund_common, refund_rare))
    }

    /// Construct an active subterranean mine head onto an unmined ore deposit beacon
    pub fn deploy_mine_head(&mut self, unit_id: u32) -> Result<String, String> {
        let (ux, uy) = {
            let u = self.units.iter().find(|u| u.id == unit_id)
                .ok_or_else(|| "Unit not found".to_string())?;
            if !u.name.contains("Robo-Miner") {
                return Err("Only specialized Robo-Miners can construct subterranean mine heads.".to_string());
            }
            (u.tile_x, u.tile_y)
        };

        // Find closest ore deposit without a miner within 3 tiles
        let mut target_deposit = None;
        for dep in &mut self.ore_deposits {
            let dist = (dep.tile_x - ux).abs() + (dep.tile_y - uy).abs();
            if dist <= 3 && !dep.has_miner {
                dep.has_miner = true;
                target_deposit = Some((dep.ore_type, dep.tile_x, dep.tile_y));
                break;
            }
        }

        if let Some((ore_t, dx, dy)) = target_deposit {
            let desc = match ore_t {
                OreType::Common => "Common Hematite",
                OreType::Rare => "Rare Titanium",
            };
            Ok(format!("Subterranean Mine Head constructed at ({}, {}) on {} vein!", dx, dy, desc))
        } else {
            Err("No unmined ore deposit beacon within range (3 tiles). Drive Robo-Miner closer to beacon.".into())
        }
    }

    /// Export colony state to a JSON-serializable snapshot
    #[allow(dead_code)]
    pub fn to_save_data(&self) -> SaveGameData {
        let researched_tech_ids = self.tech_tree.techs.iter()
            .filter(|t| t.is_researched)
            .map(|t| t.id)
            .collect();

        SaveGameData {
            scenario_id: self.scenario_id,
            faction: self.faction,
            sol: self.sol,
            population: self.population,
            morale_pct: self.morale_pct,
            starship_progress: self.starship_progress,
            common_ore: self.common_ore,
            rare_ore: self.rare_ore,
            power_produced: self.power_produced,
            power_consumed: self.power_consumed,
            food_units: self.food_units,
            units: self.units.clone(),
            buildings: self.buildings.clone(),
            tubes: self.tubes.iter().copied().collect(),
            researched_tech_ids,
            day_cycle: self.day_cycle,
        }
    }

    /// Restore colony state from a save data snapshot
    pub fn apply_save_data(&mut self, data: SaveGameData) {
        self.scenario_id = data.scenario_id;
        self.faction = data.faction;
        self.sol = data.sol;
        self.population = data.population;
        self.morale_pct = data.morale_pct;
        self.starship_progress = data.starship_progress;
        self.common_ore = data.common_ore;
        self.rare_ore = data.rare_ore;
        self.power_produced = data.power_produced;
        self.power_consumed = data.power_consumed;
        self.food_units = data.food_units;
        self.units = data.units;
        self.buildings = data.buildings;
        self.tubes = data.tubes.into_iter().collect();
        self.day_cycle = data.day_cycle;

        for tech in &mut self.tech_tree.techs {
            tech.is_researched = data.researched_tech_ids.contains(&tech.id);
        }

        self.fog.update_visibility(&self.units, &self.buildings);
    }

    /// Initiate final countdown and launch the assembled evacuation starship
    pub fn launch_starship(&mut self) -> Result<String, String> {
        if self.starship_progress < 100.0 {
            return Err("Starship evacuation modules incomplete (requires 100% assembly progress)".into());
        }
        if self.starship_launched {
            return Err("Evacuation Starship has already departed New Terra".into());
        }

        self.starship_launched = true;
        self.launch_anim_time = 0.0;
        self.disasters.quake_duration = 3.0;

        // Giant launch thruster ignition shockwave
        self.active_explosions.push(ExplosionEffect {
            x: 24.0,
            y: 20.0,
            radius: 8.0,
            max_radius: 36.0,
            lifetime: 2.0,
            max_lifetime: 2.0,
        });

        Ok("STARSHIP EVACUATION INITIATED: Main thruster ignition engaged! Evacuating colonists into orbit.".into())
    }

    /// Issue an 8-way movement order to all currently selected units
    pub fn order_selected_move(&mut self, target_x: i32, target_y: i32) {
        for u in &mut self.units {
            if u.is_selected {
                if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (target_x, target_y)) {
                    u.path = path;
                    u.current_order = UnitOrder::MoveTo { target_x, target_y };
                    u.move_cooldown = 0.0;
                }
            }
        }
    }

    /// Issue an 8-way movement order to specific units by id
    #[allow(dead_code)]
    pub fn order_unit_move(&mut self, unit_ids: &[u32], target_x: i32, target_y: i32) {
        for u in &mut self.units {
            if unit_ids.contains(&u.id) {
                if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (target_x, target_y)) {
                    u.path = path;
                    u.current_order = UnitOrder::MoveTo { target_x, target_y };
                    u.move_cooldown = 0.0;
                }
            }
        }
    }

    /// Process and execute an incoming network message deterministically
    #[allow(dead_code)]
    pub fn apply_net_message(&mut self, msg: &crate::net::NetMessage) {
        match msg {
            crate::net::NetMessage::UnitOrder { unit_ids, order } => {
                match order {
                    crate::net::NetOrder::MoveTo(x, y) => {
                        self.order_unit_move(unit_ids, *x, *y);
                    }
                    _ => {
                        let sim_order = order.to_unit_order();
                        for u in &mut self.units {
                            if unit_ids.contains(&u.id) {
                                u.current_order = sim_order.clone();
                            }
                        }
                    }
                }
            }
            crate::net::NetMessage::QueueProduction { building_id, item } => {
                let _ = self.queue_production(*building_id, item.clone());
            }
            crate::net::NetMessage::RecycleStructure { building_id } => {
                let _ = self.recycle_structure(*building_id);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_event_flags_and_audio_triggers() {
        let mut colony = ColonyState::new(Faction::Eden);
        // Initially, no disaster or attacks
        let res = colony.update(0.1);
        assert!(!res.under_attack);
        assert!(!res.friendly_vehicle_destroyed);
        assert!(!res.structure_destroyed);
        assert!(!res.volcano_erupted);
        assert!(!res.vortex_spawned);

        // Force an enemy unit to attack player unit
        let enemy_id = 999;
        colony.units.push(Unit {
            id: enemy_id,
            name: "Enemy Raider".into(),
            faction: Faction::Plymouth,
            tile_x: 21,
            tile_y: 24,
            hp: 100,
            max_hp: 100,
            is_selected: false,
            current_order: UnitOrder::Attack { target_unit_id: 1 }, // Targets player cargo truck
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon: Some(WeaponType::Laser),
            attack_cooldown: 0.0,
            cargo: None,
            hauler_state: None,
            cargo_kit: None,
        });

        // Tick simulation: enemy attacks truck
        let res_attack = colony.update(0.1);
        assert!(res_attack.laser_fired);
        assert!(res_attack.under_attack);

        // Reduce truck hp to 1 so next shot destroys it
        if let Some(truck) = colony.units.iter_mut().find(|u| u.id == 1) {
            truck.hp = 1;
        }
        if let Some(enemy) = colony.units.iter_mut().find(|u| u.id == enemy_id) {
            enemy.attack_cooldown = 0.0;
        }
        let res_kill = colony.update(0.1);
        assert!(res_kill.friendly_vehicle_destroyed);
        assert!(res_kill.unit_destroyed);
    }

    #[test]
    fn test_meteor_structure_destruction_event() {
        let mut colony = ColonyState::new(Faction::Eden);
        let b = colony.buildings.first().unwrap().clone();
        colony.disasters.detonation_queue.push((b.tile_x as f64, b.tile_y as f64, 5.0));

        let res = colony.update(0.1);
        assert!(res.meteor_hit);
        assert!(res.structure_destroyed);
        assert!(colony.destroyed_building_ids.contains(&b.id));
    }
}
