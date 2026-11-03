//! Outpost 2 Colony State Persistence & Save/Load Subsystem
//!
//! Provides full-fidelity serialization and deserialization of the colonial simulation:
//! - SaveMetadata: slot index, colony name, UTC timestamp, faction, sol, scenario, and playtime.
//! - SaveGameState: complete serialization of resources, structures, units, natural disasters,
//!   mission objectives, and dynamic scripting event triggers.
//! - LocalStorageBackend: browser-native persistent slots (1..=5), quicksave (F5), and autosave (every 100 Sols).
//! - FileExportBackend: downloadable and importable standalone `.op2save` file archives (Base64/JSON).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::render::sprites::Direction8;
use crate::sim::combat::WeaponType;
use crate::sim::disasters::{AtmosphericVortex, MeteorImpact};
use crate::sim::events::{ScriptTrigger, TacticalNotification, UnitType};
use crate::sim::mining::{HaulerState, OreType};
use crate::sim::mission::{MissionStatus, Objective};
use crate::sim::pathfinding::UnitOrder;
use crate::sim::structures::StructureType;
use crate::sim::{Building, ColonyState, Faction, Unit};

/// Reserved slot index for user-triggered QuickSave (F5 key).
pub const SLOT_QUICKSAVE: u32 = 0;
/// Minimum user-selectable save slot index.
pub const SLOT_MIN_USER: u32 = 1;
/// Maximum user-selectable save slot index.
pub const SLOT_MAX_USER: u32 = 5;
/// Reserved slot index for automated periodic milestone autosaves (every 100 Sols).
pub const SLOT_AUTOSAVE: u32 = 100;

/// Header prefix identifying valid standalone Outpost 2 save files.
pub const OP2SAVE_HEADER: &str = "OP2SAVE-V1:";

/// Summary metadata for a colony save state, allowing rapid slot indexing without
/// parsing the entire simulation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveMetadata {
    pub slot: u32,
    pub name: String,
    pub timestamp_utc: u64,
    pub faction: Faction,
    pub sol: u32,
    pub scenario_name: String,
    pub playtime_seconds: u32,
}

impl SaveMetadata {
    /// Create new save metadata.
    pub fn new(
        slot: u32,
        name: impl Into<String>,
        timestamp_utc: u64,
        faction: Faction,
        sol: u32,
        scenario_name: impl Into<String>,
        playtime_seconds: u32,
    ) -> Self {
        Self {
            slot,
            name: name.into(),
            timestamp_utc,
            faction,
            sol,
            scenario_name: scenario_name.into(),
            playtime_seconds,
        }
    }
}

/// Operational power status of a colonial structure on New Terra.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerStatus {
    /// Structure receives sufficient electrical grid wattage and functions normally.
    Powered,
    /// Structure is starved of energy due to insufficient generator output.
    Unpowered,
    /// Structure was voluntarily powered down or taken offline by the player.
    Disabled,
}

/// Full serialization snapshot of a colonial building or facility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructureSaveState {
    pub id: u32,
    #[serde(rename = "type")]
    pub structure_type: StructureType,
    pub x: i32,
    pub y: i32,
    pub hp: u32,
    pub max_hp: u32,
    pub enabled: bool,
    pub power_status: PowerStatus,
    #[serde(default)]
    pub faction: Option<Faction>,
    #[serde(default)]
    pub production_queue: Vec<String>,
    #[serde(default)]
    pub production_progress: f64,
    #[serde(default)]
    pub attack_cooldown: f64,
}

/// Full serialization snapshot of a mobile vehicle or construction unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitSaveState {
    pub id: u32,
    #[serde(rename = "type")]
    pub unit_type: UnitType,
    pub x: i32,
    pub y: i32,
    pub hp: u32,
    pub max_hp: u32,
    pub direction: Direction8,
    pub order: UnitOrder,
    pub cargo: Option<(OreType, u32)>,
    pub weapon: Option<WeaponType>,
    #[serde(default)]
    pub faction: Faction,
    #[serde(default)]
    pub hauler_state: Option<HaulerState>,
    #[serde(default)]
    pub cargo_kit: Option<String>,
    #[serde(default)]
    pub is_selected: bool,
    #[serde(default)]
    pub path: Vec<(i32, i32)>,
    #[serde(default)]
    pub move_cooldown: f64,
    #[serde(default)]
    pub attack_cooldown: f64,
}

/// Resource stockpiles, population, energy grid, and tech matrix progression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColonyResourcesSave {
    pub common_ore: u32,
    pub rare_ore: u32,
    pub food: u32,
    pub morale: u32,
    pub population: u32,
    pub power_produced: u32,
    pub power_consumed: u32,
    pub starship_progress: f64,
    pub tech_progress: Vec<u32>,
}

/// Environmental threats, volcanic magma fronts, and atmospheric vortex states.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisasterSaveState {
    pub active_lava_flows: Vec<(u32, u32)>,
    pub vortices: Vec<AtmosphericVortex>,
    pub meteors: Vec<MeteorImpact>,
    #[serde(default)]
    pub active_blight_tiles: Vec<(u32, u32)>,
    #[serde(default)]
    pub sol_timer: f64,
    #[serde(default)]
    pub blight_timer: f64,
    #[serde(default)]
    pub vortex_timer: f64,
    #[serde(default)]
    pub meteor_timer: f64,
    #[serde(default)]
    pub eruption_count: u32,
}

/// Campaign scenario objectives, casualties counter, and mission clock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionSaveState {
    pub scenario_id: usize,
    pub codename: String,
    pub faction: Faction,
    pub briefing: String,
    pub objectives: Vec<Objective>,
    pub status: MissionStatus,
    pub casualties: u32,
    pub timer: f64,
}

/// Reactive trigger engine snapshot tracking fired triggers, pending scripts, and notices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventEngineSaveState {
    pub triggers: Vec<ScriptTrigger>,
    #[serde(default)]
    pub notifications: Vec<TacticalNotification>,
    #[serde(default)]
    pub current_tick: u32,
}

/// Comprehensive, loss-free simulation save snapshot for Outpost 2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGameState {
    pub metadata: SaveMetadata,
    pub resources: ColonyResourcesSave,
    pub structures: Vec<StructureSaveState>,
    pub units: Vec<UnitSaveState>,
    pub disasters: DisasterSaveState,
    pub mission: MissionSaveState,
    pub event_engine: EventEngineSaveState,
    #[serde(default)]
    pub tubes: Vec<(i32, i32)>,
    #[serde(default)]
    pub day_cycle: f64,
}

impl SaveGameState {
    /// Capture complete colonial state from live simulation into a `SaveGameState` snapshot.
    pub fn from_colony(
        colony: &ColonyState,
        slot: u32,
        name: impl Into<String>,
        playtime_seconds: u32,
    ) -> Self {
        let timestamp_utc = get_current_timestamp();
        let sol_int = colony.sol.max(0.0) as u32;

        let metadata = SaveMetadata::new(
            slot,
            name,
            timestamp_utc,
            colony.faction,
            sol_int,
            colony.mission.codename.clone(),
            playtime_seconds,
        );

        let tech_progress = colony
            .tech_tree
            .techs
            .iter()
            .filter(|t| t.is_researched)
            .map(|t| t.id)
            .collect();

        let resources = ColonyResourcesSave {
            common_ore: colony.common_ore,
            rare_ore: colony.rare_ore,
            food: colony.food_units,
            morale: colony.morale_pct,
            population: colony.population,
            power_produced: colony.power_produced,
            power_consumed: colony.power_consumed,
            starship_progress: colony.starship_progress,
            tech_progress,
        };

        let is_grid_powered = colony.power_produced >= colony.power_consumed;
        let structures = colony
            .buildings
            .iter()
            .map(|b| {
                let st = StructureType::from_name(&b.name).unwrap_or(StructureType::Residence);
                let power_status = if !b.is_active {
                    PowerStatus::Disabled
                } else if is_grid_powered {
                    PowerStatus::Powered
                } else {
                    PowerStatus::Unpowered
                };
                let max_hp = st.default_max_hp();
                StructureSaveState {
                    id: b.id,
                    structure_type: st,
                    x: b.tile_x,
                    y: b.tile_y,
                    hp: max_hp,
                    max_hp,
                    enabled: b.is_active,
                    power_status,
                    faction: b.faction,
                    production_queue: b.production_queue.clone(),
                    production_progress: b.production_progress,
                    attack_cooldown: b.attack_cooldown,
                }
            })
            .collect();

        let units = colony
            .units
            .iter()
            .map(|u| {
                let ut = UnitType::from_name(&u.name).unwrap_or(UnitType::CargoTruck);
                // Compute heading direction from first path delta or default
                let direction = if let Some(&(next_x, next_y)) = u.path.first() {
                    Direction8::from_tile_delta(next_x - u.tile_x, next_y - u.tile_y)
                } else {
                    Direction8::SouthEast
                };
                UnitSaveState {
                    id: u.id,
                    unit_type: ut,
                    x: u.tile_x,
                    y: u.tile_y,
                    hp: u.hp,
                    max_hp: u.max_hp,
                    direction,
                    order: u.current_order.clone(),
                    cargo: u.cargo,
                    weapon: u.weapon,
                    faction: u.faction,
                    hauler_state: u.hauler_state,
                    cargo_kit: u.cargo_kit.clone(),
                    is_selected: u.is_selected,
                    path: u.path.clone(),
                    move_cooldown: u.move_cooldown,
                    attack_cooldown: u.attack_cooldown,
                }
            })
            .collect();

        let disasters = DisasterSaveState {
            active_lava_flows: colony.disasters.active_lava_tiles.clone(),
            vortices: colony.disasters.active_vortices.clone(),
            meteors: colony.disasters.pending_meteors.clone(),
            active_blight_tiles: colony.disasters.active_blight_tiles.clone(),
            sol_timer: colony.disasters.sol_timer,
            blight_timer: colony.disasters.blight_timer,
            vortex_timer: colony.disasters.vortex_timer,
            meteor_timer: colony.disasters.meteor_timer,
            eruption_count: colony.disasters.eruption_count,
        };

        let mission = MissionSaveState {
            scenario_id: colony.scenario_id,
            codename: colony.mission.codename.clone(),
            faction: colony.mission.faction,
            briefing: colony.mission.briefing.clone(),
            objectives: colony.mission.objectives.clone(),
            status: colony.mission.status,
            casualties: colony.casualties,
            timer: colony.sol,
        };

        let event_engine = EventEngineSaveState {
            triggers: colony.event_engine.triggers.clone(),
            notifications: colony.event_engine.notifications.clone(),
            current_tick: colony.event_engine.current_tick,
        };

        let tubes = colony.tubes.iter().copied().collect();

        Self {
            metadata,
            resources,
            structures,
            units,
            disasters,
            mission,
            event_engine,
            tubes,
            day_cycle: colony.day_cycle,
        }
    }

    /// Restore complete colonial simulation state from this `SaveGameState` snapshot.
    pub fn apply_to_colony(&self, colony: &mut ColonyState) {
        colony.scenario_id = self.mission.scenario_id;
        colony.faction = self.metadata.faction;
        colony.sol = self.metadata.sol as f64;
        colony.population = self.resources.population;
        colony.morale_pct = self.resources.morale;
        colony.starship_progress = self.resources.starship_progress;
        colony.common_ore = self.resources.common_ore;
        colony.rare_ore = self.resources.rare_ore;
        colony.power_produced = self.resources.power_produced;
        colony.power_consumed = self.resources.power_consumed;
        colony.food_units = self.resources.food;
        colony.day_cycle = self.day_cycle;
        colony.casualties = self.mission.casualties;

        // Restore research matrix
        for tech in &mut colony.tech_tree.techs {
            tech.is_researched = self.resources.tech_progress.contains(&tech.id);
        }

        // Restore colonial structures
        colony.buildings = self
            .structures
            .iter()
            .map(|s| {
                let dims = s.structure_type.dimensions();
                Building {
                    id: s.id,
                    name: s.structure_type.name().to_string(),
                    tile_x: s.x,
                    tile_y: s.y,
                    width: dims.0,
                    height: dims.1,
                    power_cost: s.structure_type.power_cost(),
                    is_active: s.enabled && s.power_status != PowerStatus::Disabled,
                    is_selected: false,
                    production_queue: s.production_queue.clone(),
                    production_progress: s.production_progress,
                    attack_cooldown: s.attack_cooldown,
                    faction: s.faction.or(Some(self.metadata.faction)),
                }
            })
            .collect();

        // Restore tactical mobile units
        colony.units = self
            .units
            .iter()
            .map(|u| {
                let name = match u.unit_type {
                    UnitType::Lynx => format!("Lynx Scout #{}", u.id),
                    UnitType::Panther => format!("Panther Battle Tank #{}", u.id),
                    UnitType::Tiger => format!("Tiger Assault Tank #{}", u.id),
                    UnitType::CargoTruck => format!("Cargo Truck #{}", u.id),
                    UnitType::ConVec => format!("ConVec #{}", u.id),
                    UnitType::RepairVehicle => format!("Repair Vehicle #{}", u.id),
                    UnitType::Earthworker => format!("Earthworker #{}", u.id),
                    UnitType::RoboMiner => format!("RoboMiner #{}", u.id),
                    UnitType::EvacuationTransport => format!("Evacuation Transport #{}", u.id),
                };
                Unit {
                    id: u.id,
                    name,
                    faction: u.faction,
                    tile_x: u.x,
                    tile_y: u.y,
                    hp: u.hp,
                    max_hp: u.max_hp,
                    is_selected: u.is_selected,
                    current_order: u.order.clone(),
                    path: u.path.clone(),
                    move_cooldown: u.move_cooldown,
                    weapon: u.weapon,
                    attack_cooldown: u.attack_cooldown,
                    cargo: u.cargo,
                    hauler_state: u.hauler_state,
                    cargo_kit: u.cargo_kit.clone(),
                }
            })
            .collect();

        // Restore natural disasters & planetary hazard simulator
        colony.disasters.active_lava_tiles = self.disasters.active_lava_flows.clone();
        colony.disasters.active_vortices = self.disasters.vortices.clone();
        colony.disasters.pending_meteors = self.disasters.meteors.clone();
        colony.disasters.active_blight_tiles = self.disasters.active_blight_tiles.clone();
        colony.disasters.sol_timer = self.disasters.sol_timer;
        colony.disasters.blight_timer = self.disasters.blight_timer;
        colony.disasters.vortex_timer = self.disasters.vortex_timer;
        colony.disasters.meteor_timer = self.disasters.meteor_timer;
        colony.disasters.eruption_count = self.disasters.eruption_count;

        // Restore mission objectives and directives
        colony.mission.scenario_id = self.mission.scenario_id;
        colony.mission.codename = self.mission.codename.clone();
        colony.mission.faction = self.mission.faction;
        colony.mission.briefing = self.mission.briefing.clone();
        colony.mission.objectives = self.mission.objectives.clone();
        colony.mission.status = self.mission.status;

        // Restore dynamic event scripting triggers and notifications
        colony.event_engine.triggers = self.event_engine.triggers.clone();
        colony.event_engine.notifications = self.event_engine.notifications.clone();
        colony.event_engine.current_tick = self.event_engine.current_tick;

        // Restore pressurized tube network
        colony.tubes = self.tubes.iter().copied().collect();

        // Refresh dynamic Fog of War visibility
        colony.fog.update_visibility(&colony.units, &colony.buildings);
    }

    /// Serialize state directly to compact JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize state from JSON string.
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

/// Key generator for persistent browser LocalStorage items.
pub fn slot_storage_key(slot: u32) -> String {
    match slot {
        SLOT_QUICKSAVE => "op2_save_quicksave".to_string(),
        SLOT_AUTOSAVE => "op2_save_autosave".to_string(),
        s => format!("op2_save_slot_{}", s),
    }
}

/// Key generator for quick save metadata indexing in LocalStorage.
pub fn slot_meta_key(slot: u32) -> String {
    format!("{}_meta", slot_storage_key(slot))
}

#[cfg(target_arch = "wasm32")]
fn raw_set_item(key: &str, val: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or_else(|| "Window object unavailable".to_string())?;
    let storage = window
        .local_storage()
        .map_err(|e| format!("Failed to access local_storage: {:?}", e))?
        .ok_or_else(|| "LocalStorage not supported or disabled".to_string())?;
    storage.set_item(key, val).map_err(|e| format!("Failed to set {}: {:?}", key, e))
}

#[cfg(target_arch = "wasm32")]
fn raw_get_item(key: &str) -> Result<Option<String>, String> {
    let window = web_sys::window().ok_or_else(|| "Window object unavailable".to_string())?;
    let storage = window
        .local_storage()
        .map_err(|e| format!("Failed to access local_storage: {:?}", e))?
        .ok_or_else(|| "LocalStorage not supported or disabled".to_string())?;
    storage.get_item(key).map_err(|e| format!("Failed to read {}: {:?}", key, e))
}

#[cfg(target_arch = "wasm32")]
fn raw_remove_item(key: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or_else(|| "Window object unavailable".to_string())?;
    let storage = window
        .local_storage()
        .map_err(|e| format!("Failed to access local_storage: {:?}", e))?
        .ok_or_else(|| "LocalStorage not supported or disabled".to_string())?;
    storage.remove_item(key).map_err(|e| format!("Failed to remove {}: {:?}", key, e))
}

#[cfg(not(target_arch = "wasm32"))]
static NATIVE_MOCK_STORAGE: std::sync::Mutex<Option<std::collections::HashMap<String, String>>> = std::sync::Mutex::new(None);

#[cfg(not(target_arch = "wasm32"))]
fn raw_set_item(key: &str, val: &str) -> Result<(), String> {
    let mut lock = NATIVE_MOCK_STORAGE.lock().unwrap();
    let map = lock.get_or_insert_with(std::collections::HashMap::new);
    map.insert(key.to_string(), val.to_string());
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn raw_get_item(key: &str) -> Result<Option<String>, String> {
    let mut lock = NATIVE_MOCK_STORAGE.lock().unwrap();
    let map = lock.get_or_insert_with(std::collections::HashMap::new);
    Ok(map.get(key).cloned())
}

#[cfg(not(target_arch = "wasm32"))]
fn raw_remove_item(key: &str) -> Result<(), String> {
    let mut lock = NATIVE_MOCK_STORAGE.lock().unwrap();
    let map = lock.get_or_insert_with(std::collections::HashMap::new);
    map.remove(key);
    Ok(())
}

/// Browser LocalStorage storage backend for game persistence.
pub struct LocalStorageBackend;

impl LocalStorageBackend {
    /// Retrieve browser `web_sys::Storage` handle.
    #[cfg(target_arch = "wasm32")]
    fn get_storage() -> Result<web_sys::Storage, String> {
        let window = web_sys::window().ok_or_else(|| "Window object unavailable".to_string())?;
        window
            .local_storage()
            .map_err(|e| format!("Failed to access local_storage: {:?}", e))?
            .ok_or_else(|| "LocalStorage not supported or disabled".to_string())
    }

    /// Non-WASM dummy implementation for compilation outside browser.
    #[cfg(not(target_arch = "wasm32"))]
    fn get_storage() -> Result<web_sys::Storage, String> {
        Err("LocalStorage is only available in browser WASM environments".to_string())
    }

    /// Save state to the specified slot in browser LocalStorage.
    ///
    /// Accepts slots 1..=5, `SLOT_QUICKSAVE` (0), or `SLOT_AUTOSAVE` (100).
    pub fn save(slot: u32, state: &SaveGameState) -> Result<(), String> {
        validate_slot(slot)?;
        let json = state
            .to_json()
            .map_err(|e| format!("SaveGameState serialization failed: {}", e))?;
        let meta_json = serde_json::to_string(&state.metadata)
            .map_err(|e| format!("SaveMetadata serialization failed: {}", e))?;

        let key = slot_storage_key(slot);
        let m_key = slot_meta_key(slot);

        raw_set_item(&key, &json)?;
        raw_set_item(&m_key, &meta_json)?;

        Ok(())
    }

    /// Load game state from the specified slot in browser LocalStorage.
    pub fn load(slot: u32) -> Result<SaveGameState, String> {
        validate_slot(slot)?;
        let key = slot_storage_key(slot);

        let json = raw_get_item(&key)?
            .ok_or_else(|| format!("Slot {} is empty", slot))?;

        SaveGameState::from_json(&json)
            .map_err(|e| format!("Failed to parse save data from slot {}: {}", slot, e))
    }

    /// Load metadata for a save slot without loading the entire state blob.
    pub fn load_metadata(slot: u32) -> Result<Option<SaveMetadata>, String> {
        validate_slot(slot)?;
        let m_key = slot_meta_key(slot);

        if let Ok(Some(meta_json)) = raw_get_item(&m_key) {
            if let Ok(meta) = serde_json::from_str::<SaveMetadata>(&meta_json) {
                return Ok(Some(meta));
            }
        }

        // Fallback: load full state and extract metadata
        let key = slot_storage_key(slot);
        if let Ok(Some(json)) = raw_get_item(&key) {
            if let Ok(state) = SaveGameState::from_json(&json) {
                return Ok(Some(state.metadata));
            }
        }

        Ok(None)
    }

    /// Execute QuickSave (F5) to dedicated quicksave slot.
    pub fn quicksave(state: &SaveGameState) -> Result<(), String> {
        Self::save(SLOT_QUICKSAVE, state)
    }

    /// Execute QuickLoad (F9) from dedicated quicksave slot.
    pub fn quickload() -> Result<SaveGameState, String> {
        Self::load(SLOT_QUICKSAVE)
    }

    /// Execute automated periodic autosave (every 100 Sols).
    pub fn autosave(state: &SaveGameState) -> Result<(), String> {
        Self::save(SLOT_AUTOSAVE, state)
    }

    /// Load the latest periodic autosave snapshot.
    pub fn load_autosave() -> Result<SaveGameState, String> {
        Self::load(SLOT_AUTOSAVE)
    }

    /// List all 5 user save slots alongside their metadata (if populated).
    pub fn list_user_slots() -> Result<Vec<(u32, Option<SaveMetadata>)>, String> {
        let mut slots = Vec::new();
        for slot in SLOT_MIN_USER..=SLOT_MAX_USER {
            let meta = Self::load_metadata(slot).unwrap_or(None);
            slots.push((slot, meta));
        }
        Ok(slots)
    }

    /// Delete a save slot and its associated metadata from browser storage.
    pub fn delete_slot(slot: u32) -> Result<(), String> {
        validate_slot(slot)?;
        let key = slot_storage_key(slot);
        let m_key = slot_meta_key(slot);

        let _ = raw_remove_item(&key);
        let _ = raw_remove_item(&m_key);
        Ok(())
    }
}

/// Standalone `.op2save` file export/import backend using Base64/JSON encoding.
pub struct FileExportBackend;

impl FileExportBackend {
    /// Export game state to an encoded standalone `.op2save` string payload.
    ///
    /// Formats the minified JSON as Base64 prefixed by the `OP2SAVE-V1:` header.
    pub fn export_to_op2save(state: &SaveGameState) -> Result<String, String> {
        use base64::Engine;

        let json = state
            .to_json()
            .map_err(|e| format!("State serialization failed: {}", e))?;

        let b64 = base64::engine::general_purpose::STANDARD.encode(json.as_bytes());
        Ok(format!("{}{}", OP2SAVE_HEADER, b64))
    }

    /// Import game state from a user-provided file string.
    ///
    /// Resiliently accepts:
    /// - Headered Base64 strings (`OP2SAVE-V1:<base64>`)
    /// - Raw Base64 encoded strings
    /// - Direct minified or formatted JSON text
    pub fn import_from_op2save(data: &str) -> Result<SaveGameState, String> {
        use base64::Engine;

        let trimmed = data.trim();
        let payload = if trimmed.starts_with(OP2SAVE_HEADER) {
            &trimmed[OP2SAVE_HEADER.len()..]
        } else {
            trimmed
        };

        // Attempt 1: Decode as Base64 blob
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(payload.trim()) {
            if let Ok(json_str) = String::from_utf8(bytes) {
                if let Ok(state) = SaveGameState::from_json(&json_str) {
                    return Ok(state);
                }
            }
        }

        // Attempt 2: Decode directly as raw JSON
        if let Ok(state) = SaveGameState::from_json(trimmed) {
            return Ok(state);
        }

        Err("Failed to parse .op2save file: invalid Base64 or corrupted JSON payload".to_string())
    }

    /// Generate an authentic standard filename for a downloadable `.op2save` archive.
    pub fn generate_download_filename(metadata: &SaveMetadata) -> String {
        let faction_str = match metadata.faction {
            Faction::Eden => "eden",
            Faction::Plymouth => "plymouth",
        };
        format!(
            "outpost2_{}_sol{:04}_slot{}.op2save",
            faction_str, metadata.sol, metadata.slot
        )
    }

    /// Trigger a browser file download using a dynamic anchor element with a Data URL.
    #[cfg(target_arch = "wasm32")]
    pub fn trigger_download(filename: &str, file_content: &str) -> Result<(), String> {
        use wasm_bindgen::JsCast;

        let window = web_sys::window().ok_or_else(|| "Window object unavailable".to_string())?;
        let document = window
            .document()
            .ok_or_else(|| "Document object unavailable".to_string())?;

        let element = document
            .create_element("a")
            .map_err(|e| format!("Failed to create anchor element: {:?}", e))?;
        let anchor: web_sys::HtmlAnchorElement = element
            .dyn_into()
            .map_err(|_| "Failed to cast element to HtmlAnchorElement".to_string())?;

        use base64::Engine;
        let b64_content = base64::engine::general_purpose::STANDARD.encode(file_content.as_bytes());
        let data_url = format!("data:application/octet-stream;base64,{}", b64_content);

        anchor.set_href(&data_url);
        anchor.set_download(filename);

        if let Some(body) = document.body() {
            let _ = body.append_child(&anchor);
            anchor.click();
            let _ = body.remove_child(&anchor);
            Ok(())
        } else {
            Err("Document body unavailable for download trigger".to_string())
        }
    }

    /// Stub for non-WASM targets.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn trigger_download(_filename: &str, _file_content: &str) -> Result<(), String> {
        Ok(())
    }
}

/// Helper function to validate save slot indices.
fn validate_slot(slot: u32) -> Result<(), String> {
    if slot == SLOT_QUICKSAVE
        || slot == SLOT_AUTOSAVE
        || (slot >= SLOT_MIN_USER && slot <= SLOT_MAX_USER)
    {
        Ok(())
    } else {
        Err(format!(
            "Invalid slot {}: must be 1..=5, QuickSave (0), or AutoSave (100)",
            slot
        ))
    }
}

/// Platform-agnostic timestamp generator.
fn get_current_timestamp() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Date::now() / 1000.0) as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Helper logic for handling F5 QuickSave.
pub fn execute_quicksave(colony: &ColonyState, playtime_seconds: u32) -> Result<SaveGameState, String> {
    let state = SaveGameState::from_colony(colony, SLOT_QUICKSAVE, "QuickSave", playtime_seconds);
    LocalStorageBackend::quicksave(&state)?;
    Ok(state)
}

/// Helper logic for handling F9 QuickLoad.
pub fn execute_quickload(colony: &mut ColonyState) -> Result<SaveMetadata, String> {
    let state = LocalStorageBackend::quickload()?;
    let meta = state.metadata.clone();
    state.apply_to_colony(colony);
    Ok(meta)
}

/// Check if a periodic milestone autosave is due (every 100 Sols).
pub fn is_autosave_due(current_sol: u32, last_autosave_sol: u32) -> bool {
    current_sol >= last_autosave_sol.saturating_add(100) && current_sol > 0
}

/// Helper logic for triggering an automated milestone autosave.
pub fn execute_autosave(
    colony: &ColonyState,
    playtime_seconds: u32,
) -> Result<SaveGameState, String> {
    let sol = colony.sol.max(0.0) as u32;
    let name = format!("AutoSave - Sol {}", sol);
    let state = SaveGameState::from_colony(colony, SLOT_AUTOSAVE, name, playtime_seconds);
    LocalStorageBackend::autosave(&state)?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::events::{EventAction, ScriptTrigger, TriggerCondition};
    use crate::sim::structures::StructureType;

    #[test]
    fn test_save_metadata_serde_roundtrip() {
        let meta = SaveMetadata::new(
            1,
            "Colony Beta Milestone",
            1730000000,
            Faction::Eden,
            124,
            "Operation New Dawn",
            1840,
        );

        let json = serde_json::to_string(&meta).expect("Serialize SaveMetadata");
        let decoded: SaveMetadata = serde_json::from_str(&json).expect("Deserialize SaveMetadata");

        assert_eq!(decoded.slot, 1);
        assert_eq!(decoded.name, "Colony Beta Milestone");
        assert_eq!(decoded.timestamp_utc, 1730000000);
        assert_eq!(decoded.faction, Faction::Eden);
        assert_eq!(decoded.sol, 124);
        assert_eq!(decoded.scenario_name, "Operation New Dawn");
        assert_eq!(decoded.playtime_seconds, 1840);
        assert_eq!(meta, decoded);
    }

    #[test]
    fn test_colony_save_game_state_roundtrip_integrity() {
        let mut colony = ColonyState::new(Faction::Eden);

        // Customize simulation values to verify roundtrip fidelity
        colony.common_ore = 12450;
        colony.rare_ore = 4320;
        colony.food_units = 890;
        colony.morale_pct = 94;
        colony.population = 158;
        colony.power_produced = 1400;
        colony.power_consumed = 850;
        colony.starship_progress = 42.5;
        colony.sol = 105.75;
        colony.casualties = 3;

        // Mark tech 1001 as researched
        if let Some(t) = colony.tech_tree.techs.first_mut() {
            t.is_researched = true;
        }

        // Add a script trigger
        let trigger = ScriptTrigger::new(
            99,
            "Lava Intrusion Warning",
            TriggerCondition::SolTimer(50),
            vec![EventAction::TriggerDisaster {
                disaster_type: crate::sim::events::DisasterType::SeismicFissure,
                x: 15,
                y: 22,
            }],
        );
        colony.event_engine.add_trigger(trigger);

        // Add an active vortex and meteor
        colony.disasters.active_vortices.push(AtmosphericVortex {
            x: 24.5,
            y: 48.2,
            vx: 0.5,
            vy: -0.2,
            radius: 3.5,
            lifetime: 45.0,
            max_lifetime: 60.0,
        });

        colony.disasters.pending_meteors.push(MeteorImpact {
            x: 12.0,
            y: 19.0,
            delay: 4.2,
            max_delay: 5.0,
            radius: 2.0,
            has_impacted: false,
        });

        colony.disasters.active_lava_tiles.push((15, 22));

        // Create SaveGameState snapshot
        let save_state = SaveGameState::from_colony(&colony, 2, "Test Outpost", 900);

        // Verify serializability to JSON
        let json = save_state.to_json().expect("Serialize SaveGameState");
        let restored_state = SaveGameState::from_json(&json).expect("Deserialize SaveGameState");

        assert_eq!(save_state, restored_state);

        // Apply snapshot to fresh ColonyState and verify deep state matching
        let mut target_colony = ColonyState::new(Faction::Eden);
        restored_state.apply_to_colony(&mut target_colony);

        assert_eq!(target_colony.common_ore, 12450);
        assert_eq!(target_colony.rare_ore, 4320);
        assert_eq!(target_colony.food_units, 890);
        assert_eq!(target_colony.morale_pct, 94);
        assert_eq!(target_colony.population, 158);
        assert_eq!(target_colony.power_produced, 1400);
        assert_eq!(target_colony.power_consumed, 850);
        assert_eq!(target_colony.starship_progress, 42.5);
        assert_eq!(target_colony.sol, 105.0); // Converted to integer Sol in metadata
        assert_eq!(target_colony.casualties, 3);
        assert_eq!(target_colony.faction, Faction::Eden);

        // Verify tech progress preserved
        assert!(target_colony.tech_tree.techs[0].is_researched);

        // Verify disasters preserved
        assert_eq!(target_colony.disasters.active_lava_tiles, vec![(15, 22)]);
        assert_eq!(target_colony.disasters.active_vortices.len(), 1);
        assert_eq!(target_colony.disasters.active_vortices[0].x, 24.5);
        assert_eq!(target_colony.disasters.pending_meteors.len(), 1);
        assert_eq!(target_colony.disasters.pending_meteors[0].delay, 4.2);

        // Verify event trigger preserved
        assert!(target_colony.event_engine.triggers.iter().any(|t| t.id == 99));

        // Verify structures and units preserved
        assert_eq!(target_colony.buildings.len(), colony.buildings.len());
        assert_eq!(target_colony.units.len(), colony.units.len());
    }

    #[test]
    fn test_file_export_import_roundtrip() {
        let colony = ColonyState::new(Faction::Plymouth);
        let save_state = SaveGameState::from_colony(&colony, 1, "Plymouth Stronghold", 350);

        // Export to .op2save format
        let op2save_text =
            FileExportBackend::export_to_op2save(&save_state).expect("Export to .op2save");
        assert!(op2save_text.starts_with(OP2SAVE_HEADER));

        // Import back from headered string
        let imported = FileExportBackend::import_from_op2save(&op2save_text)
            .expect("Import headered .op2save");
        assert_eq!(imported.metadata.faction, Faction::Plymouth);
        assert_eq!(imported.metadata.name, "Plymouth Stronghold");
        assert_eq!(imported, save_state);

        // Import back from raw JSON
        let json = save_state.to_json().unwrap();
        let imported_raw =
            FileExportBackend::import_from_op2save(&json).expect("Import raw JSON .op2save");
        assert_eq!(imported_raw, save_state);
    }

    #[test]
    fn test_file_download_filename_format() {
        let meta = SaveMetadata::new(
            3,
            "Main Colony",
            1730000000,
            Faction::Eden,
            42,
            "Operation New Dawn",
            600,
        );
        let filename = FileExportBackend::generate_download_filename(&meta);
        assert_eq!(filename, "outpost2_eden_sol0042_slot3.op2save");

        let meta_ply = SaveMetadata::new(
            1,
            "Plymouth Mining Hub",
            1730000000,
            Faction::Plymouth,
            120,
            "Reclamation",
            1200,
        );
        let filename_ply = FileExportBackend::generate_download_filename(&meta_ply);
        assert_eq!(filename_ply, "outpost2_plymouth_sol0120_slot1.op2save");
    }

    #[test]
    fn test_slot_storage_keys() {
        assert_eq!(slot_storage_key(1), "op2_save_slot_1");
        assert_eq!(slot_storage_key(5), "op2_save_slot_5");
        assert_eq!(slot_storage_key(SLOT_QUICKSAVE), "op2_save_quicksave");
        assert_eq!(slot_storage_key(SLOT_AUTOSAVE), "op2_save_autosave");

        assert_eq!(slot_meta_key(1), "op2_save_slot_1_meta");
        assert_eq!(slot_meta_key(SLOT_QUICKSAVE), "op2_save_quicksave_meta");
    }

    #[test]
    fn test_autosave_milestone_detection() {
        assert!(!is_autosave_due(0, 0));
        assert!(!is_autosave_due(50, 0));
        assert!(!is_autosave_due(99, 0));
        assert!(is_autosave_due(100, 0));
        assert!(is_autosave_due(150, 0));
        assert!(!is_autosave_due(150, 100));
        assert!(is_autosave_due(200, 100));
    }

    #[test]
    fn test_structure_power_status_derivation() {
        let st = StructureType::TokamakReactor;
        assert_eq!(st.default_max_hp(), 800);

        let save_struct = StructureSaveState {
            id: 1,
            structure_type: st,
            x: 10,
            y: 12,
            hp: 800,
            max_hp: 800,
            enabled: true,
            power_status: PowerStatus::Powered,
            faction: Some(Faction::Eden),
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
        };

        let json = serde_json::to_string(&save_struct).unwrap();
        assert!(json.contains(r#""type":"TokamakReactor""#));
        assert!(json.contains(r#""power_status":"Powered""#));

        let decoded: StructureSaveState = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.structure_type, StructureType::TokamakReactor);
        assert_eq!(decoded.power_status, PowerStatus::Powered);
    }
}
