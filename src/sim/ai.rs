//! Tactical AI Personalities & Skirmish Autonomy Engine for Outpost 2
//!
//! Models autonomous computer colony commanders with distinctive strategic doctrines,
//! multi-tiered military coordination, base expansion site selection, and dynamic tech progression.
//!
//! Subsystems:
//! - `AiPersonality`: Strategic doctrines (AggressiveRusher, DefensiveFortifier, EconomicTechRusher, Custom).
//! - `BaseBuildingPlanner`: Clustered colony layout, energy/stockpile balancing, and site clearance validation.
//! - `ProductionManager`: Doctrine-driven unit compositions (miners, haulers, combat hulls, weapon attachments).
//! - `TacticalSquadManager`: Coordinated assault squads, reconnaissance patrols, base escorts, and retreat-to-repair.
//! - `ResearchPlanner`: Autonomous tech tree progression aligned with strategic doctrine.
//! - `AiPlayer`: Complete autonomous agent integrated into `GameSession` with staggered execution ticks.

#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use crate::sim::{
    combat::WeaponType,
    map::GameMap,
    mining::OreDeposit,
    pathfinding::{PathFinder, UnitOrder},
    structures::{connect_with_tubes, StructureType},
    Building, Faction, GameSession, Unit,
};

/// Difficulty settings scaling AI resource efficiency, reaction speed, and tactical aggression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// Decision update frequency in simulation ticks (staggered across AI instances)
    pub fn decision_interval_ticks(&self) -> u32 {
        match self {
            Self::Easy => 60,   // ~3.0s interval
            Self::Medium => 45, // ~2.25s interval
            Self::Hard => 30,   // ~1.5s interval
        }
    }

    /// Starting resource grant for initial colony bootstrapping (common_ore, rare_ore)
    pub fn starting_resources(&self) -> (u32, u32) {
        match self {
            Self::Easy => (1200, 300),
            Self::Medium => (2200, 600),
            Self::Hard => (3500, 1200),
        }
    }

    /// Localized presentation name for skirmish lobby and telemetry
    pub fn name_localized(&self, lang: crate::i18n::Language) -> &'static str {
        let dict = crate::i18n::get_dictionary(lang);
        match self {
            Self::Easy => dict.skirmish.difficulty_easy,
            Self::Medium => dict.skirmish.difficulty_medium,
            Self::Hard => dict.skirmish.difficulty_hard,
        }
    }

    /// Ore production and mining throughput multiplier
    pub fn resource_multiplier(&self) -> f64 {
        match self {
            Self::Easy => 0.85,
            Self::Medium => 1.0,
            Self::Hard => 1.35,
        }
    }

    /// Health ratio at which damaged combat units break off and retreat to repair depots
    pub fn repair_retreat_threshold(&self) -> f64 {
        match self {
            Self::Easy => 0.25,
            Self::Medium => 0.40,
            Self::Hard => 0.55,
        }
    }

    /// Aggressiveness multiplier scaling strike squad launch criteria
    pub fn aggression_multiplier(&self) -> f64 {
        match self {
            Self::Easy => 0.70,
            Self::Medium => 1.00,
            Self::Hard => 1.40,
        }
    }
}

/// Custom configuration parameters for user-defined AI doctrines
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiConfig {
    pub name: String,
    pub rush_preference: f64,
    pub defense_bias: f64,
    pub tech_focus: f64,
    pub economic_expansion_weight: f64,
    pub min_strike_squad_size: usize,
    pub preferred_weapons: Vec<WeaponType>,
    pub target_guard_posts: usize,
    pub target_labs: usize,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            name: "Balanced Commander".to_string(),
            rush_preference: 0.5,
            defense_bias: 0.5,
            tech_focus: 0.5,
            economic_expansion_weight: 0.5,
            min_strike_squad_size: 4,
            preferred_weapons: vec![WeaponType::Laser, WeaponType::Railgun],
            target_guard_posts: 2,
            target_labs: 1,
        }
    }
}

/// Distinct tactical personalities dictating strategic doctrine, build orders, and combat tactics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AiPersonality {
    /// Plymouth doctrine: Rapid Lynx swarms, early smelter exploitation, microwave & acid cloud raids,
    /// relentless harassment of player ore trucks, ConVecs, and unshielded structures.
    AggressiveRusher,
    /// Eden doctrine: Heavy perimeter guard posts, wall enclosures, geothermal plants, shielded hubs,
    /// and Tiger/Panther heavy armored counter-strikes.
    DefensiveFortifier,
    /// Economic doctrine: Rapid Robo-Miner expansion, multiple universities/labs, tech tree beeline
    /// to railguns/superconductors, culminating in mass late-game armored divisions.
    EconomicTechRusher,
    /// Custom user-parameterized AI profile.
    Custom(AiConfig),
}

impl AiPersonality {
    pub fn name(&self) -> &'static str {
        match self {
            Self::AggressiveRusher => "Aggressive Rusher (Plymouth Doctrine)",
            Self::DefensiveFortifier => "Defensive Fortifier (Eden Doctrine)",
            Self::EconomicTechRusher => "Economic Tech Rusher",
            Self::Custom(_) => "Custom AI Doctrine",
        }
    }

    /// Localized personality name
    pub fn name_localized(&self, lang: crate::i18n::Language) -> &'static str {
        let dict = crate::i18n::get_dictionary(lang);
        match self {
            Self::AggressiveRusher => dict.skirmish.personality_aggressive_rusher,
            Self::DefensiveFortifier => dict.skirmish.personality_defensive_fortifier,
            Self::EconomicTechRusher => dict.skirmish.personality_economic_tech_rusher,
            Self::Custom(_) => "Custom AI Doctrine",
        }
    }

    pub fn default_faction(&self) -> Faction {
        match self {
            Self::AggressiveRusher => Faction::Plymouth,
            Self::DefensiveFortifier => Faction::Eden,
            Self::EconomicTechRusher => Faction::Eden,
            Self::Custom(_) => Faction::Plymouth,
        }
    }

    pub fn min_strike_squad_size(&self) -> usize {
        match self {
            Self::AggressiveRusher => 3,
            Self::DefensiveFortifier => 6,
            Self::EconomicTechRusher => 5,
            Self::Custom(cfg) => cfg.min_strike_squad_size.max(2),
        }
    }

    pub fn preferred_weapons(&self) -> Vec<WeaponType> {
        match self {
            Self::AggressiveRusher => vec![WeaponType::Microwave, WeaponType::AcidCloud, WeaponType::Laser],
            Self::DefensiveFortifier => vec![WeaponType::Railgun, WeaponType::Laser, WeaponType::RPG],
            Self::EconomicTechRusher => vec![WeaponType::Railgun, WeaponType::Laser, WeaponType::EMP],
            Self::Custom(cfg) => cfg.preferred_weapons.clone(),
        }
    }

    pub fn tech_priorities(&self) -> Vec<u32> {
        match self {
            Self::AggressiveRusher => vec![
                5104, // Lynx Chassis Reinforced Armor
                5202, // Pulse Laser Optics
                5301, // Magnetic Acceleration & Railguns
            ],
            Self::DefensiveFortifier => vec![
                5401, // Automated Point Defense Turrets (Guard Posts)
                4301, // Vulcanology & Hydrothermal Extraction (Geothermal)
                4201, // Lava Containment Walls
                5301, // Railguns
            ],
            Self::EconomicTechRusher => vec![
                2106, // Basic Laboratory
                3402, // High-Temperature Smelting
                3301, // Hydroponics Automation
                4301, // Hydrothermal Extraction
                5301, // Railguns
                6101, // Cryogenic Evacuation Modules
            ],
            Self::Custom(_) => vec![2106, 5104, 5401, 5301],
        }
    }

    pub fn target_guard_posts(&self) -> usize {
        match self {
            Self::AggressiveRusher => 1,
            Self::DefensiveFortifier => 4,
            Self::EconomicTechRusher => 2,
            Self::Custom(cfg) => cfg.target_guard_posts,
        }
    }

    pub fn target_labs(&self) -> usize {
        match self {
            Self::AggressiveRusher => 1,
            Self::DefensiveFortifier => 1,
            Self::EconomicTechRusher => 3,
            Self::Custom(cfg) => cfg.target_labs,
        }
    }

    /// Desired production ratios for (Lynx, Panther, Tiger) combat hulls
    pub fn combat_hull_ratios(&self) -> (f64, f64, f64) {
        match self {
            Self::AggressiveRusher => (0.65, 0.25, 0.10),
            Self::DefensiveFortifier => (0.15, 0.50, 0.35),
            Self::EconomicTechRusher => (0.20, 0.45, 0.35),
            Self::Custom(cfg) => {
                let rush = cfg.rush_preference.clamp(0.0, 1.0);
                let def = cfg.defense_bias.clamp(0.0, 1.0);
                let lynx = (rush * 0.7 + 0.1).clamp(0.1, 0.8);
                let tiger = (def * 0.5 + 0.1).clamp(0.1, 0.5);
                let panther = (1.0 - lynx - tiger).max(0.1);
                (lynx, panther, tiger)
            }
        }
    }
}

/// Tactical stance reflecting overarching operational posture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticalStance {
    Expanding,
    Defending,
    GatheringForces,
    Assaulting,
    HarassingLogistics,
    Regrouping,
}

/// Dynamic battlefield situation metrics for an AI commander
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTacticalState {
    pub stance: TacticalStance,
    pub last_attack_tick: u32,
    pub threat_level: u32,
    pub enemy_expansion_detected: bool,
    pub units_lost: u32,
    pub enemies_destroyed: u32,
    pub alert_message: Option<String>,
}

impl Default for AiTacticalState {
    fn default() -> Self {
        Self {
            stance: TacticalStance::Expanding,
            last_attack_tick: 0,
            threat_level: 0,
            enemy_expansion_detected: false,
            units_lost: 0,
            enemies_destroyed: 0,
            alert_message: None,
        }
    }
}

/// Squad operational classifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SquadRole {
    Strike,
    Patrol,
    BaseDefense,
    Harassment,
}

/// Tactical formation of combat units coordinated under a unified objective
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TacticalSquad {
    pub id: u32,
    pub role: SquadRole,
    pub unit_ids: Vec<u32>,
    pub target_coords: Option<(i32, i32)>,
    pub target_unit_id: Option<u32>,
    pub rally_point: (i32, i32),
    pub patrol_index: usize,
    pub waypoints: Vec<(i32, i32)>,
}

/// Subsystem 1: Base Building Planner
///
/// Analyzes mineral stockpiles, common/rare ore income, energy deficit,
/// and plans structure placements in clustered colony patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseBuildingPlanner {
    pub base_center: (i32, i32),
    pub cluster_radius: i32,
    pub last_build_tick: u32,
}

impl BaseBuildingPlanner {
    pub fn new(base_center: (i32, i32)) -> Self {
        Self {
            base_center,
            cluster_radius: 18,
            last_build_tick: 0,
        }
    }

    /// Evaluates colony requirements and selects the next structure type to construct
    pub fn plan_next_structure(
        &self,
        personality: &AiPersonality,
        power_produced: u32,
        power_consumed: u32,
        common_ore: u32,
        owned_buildings: &[Building],
        researched_techs: &[u32],
    ) -> Option<StructureType> {
        let has_cc = owned_buildings.iter().any(|b| b.name.contains("Command Center"));
        if !has_cc {
            return Some(StructureType::CommandCenter);
        }

        // Energy deficit check: if margin is less than 50 kW, prioritize power generation
        let net_power = power_produced as i32 - power_consumed as i32;
        if net_power < 50 {
            if researched_techs.contains(&4301) && common_ore >= 350 {
                return Some(StructureType::GeothermalPlant);
            }
            if common_ore >= 200 {
                return Some(StructureType::SolarArray);
            }
        }

        let smelter_count = owned_buildings.iter().filter(|b| b.name.contains("Smelter")).count();
        if smelter_count == 0 && common_ore >= 350 {
            return Some(StructureType::CommonOreSmelter);
        }

        let factory_count = owned_buildings.iter().filter(|b| b.name.contains("Vehicle Factory")).count();
        if factory_count == 0 && common_ore >= 500 {
            return Some(StructureType::VehicleFactory);
        }

        let guard_post_count = owned_buildings.iter().filter(|b| b.name.contains("Guard")).count();
        let target_guards = personality.target_guard_posts();
        let can_build_guards = researched_techs.contains(&5401)
            || *personality == AiPersonality::AggressiveRusher
            || *personality == AiPersonality::DefensiveFortifier;
        if guard_post_count < target_guards && can_build_guards && common_ore >= 250 {
            return Some(StructureType::GuardPost);
        }

        let lab_count = owned_buildings.iter().filter(|b| b.name.contains("Lab")).count();
        let target_labs = personality.target_labs();
        if lab_count < target_labs && common_ore >= 300 {
            return Some(StructureType::StandardLab);
        }

        let agridome_count = owned_buildings.iter().filter(|b| b.name.contains("Bio-Dome")).count();
        if agridome_count == 0 && common_ore >= 200 {
            return Some(StructureType::Agrodome);
        }

        let repair_count = owned_buildings.iter().filter(|b| b.name.contains("Repair")).count();
        if repair_count == 0 && factory_count > 0 && common_ore >= 250 {
            return Some(StructureType::RepairFacility);
        }

        // Secondary smelter expansion if rich in ore
        if smelter_count < 2 && common_ore >= 700 {
            return Some(StructureType::RareOreSmelter);
        }

        None
    }

    /// Finds a valid isometric placement tile footprint clustered around the colony hub
    pub fn find_placement_site(
        &self,
        game_map: &GameMap,
        all_buildings: &[Building],
        st: StructureType,
        search_origin: (i32, i32),
    ) -> Option<(i32, i32)> {
        let (w, h) = st.dimensions();
        let max_r = self.cluster_radius;

        // Spiral outward in Manhattan rings from the search origin
        for r in 2..=max_r {
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx.abs().max(dy.abs()) != r {
                        continue;
                    }
                    let cx = search_origin.0 + dx;
                    let cy = search_origin.1 + dy;

                    if self.is_site_clear(game_map, all_buildings, cx, cy, w, h) {
                        return Some((cx, cy));
                    }
                }
            }
        }
        None
    }

    /// Validates footprint clearance: within bounds, passable, non-lava, and no building overlaps
    pub fn is_site_clear(
        &self,
        game_map: &GameMap,
        all_buildings: &[Building],
        tx: i32,
        ty: i32,
        w: i32,
        h: i32,
    ) -> bool {
        let mw = game_map.width as i32;
        let mh = game_map.height as i32;

        if tx < 2 || ty < 2 || tx + w >= mw - 2 || ty + h >= mh - 2 {
            return false;
        }

        for dy in 0..h {
            for dx in 0..w {
                let x = tx + dx;
                let y = ty + dy;
                if let Some(tile) = game_map.get_tile(x as u32, y as u32) {
                    if tile.is_lava || tile.is_wall_or_building {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        // Clearance buffer from all existing colony buildings
        for b in all_buildings {
            let overlap_x = tx < b.tile_x + b.width + 1 && tx + w + 1 > b.tile_x;
            let overlap_y = ty < b.tile_y + b.height + 1 && ty + h + 1 > b.tile_y;
            if overlap_x && overlap_y {
                return false;
            }
        }

        true
    }

    /// Executes the deployment of a planned structure, deducting ore and connecting tubes
    pub fn execute_build(
        &mut self,
        session: &mut GameSession,
        ai_faction: Faction,
        st: StructureType,
        site: (i32, i32),
        common_ore: &mut u32,
        rare_ore: &mut u32,
        current_tick: u32,
    ) -> Result<u32, String> {
        let (cost_c, cost_r) = st.ore_cost();
        if *common_ore < cost_c || *rare_ore < cost_r {
            return Err("Insufficient ore reserves".to_string());
        }

        *common_ore -= cost_c;
        *rare_ore -= cost_r;
        self.last_build_tick = current_tick;

        let next_id = session.buildings.iter().map(|b| b.id).max().unwrap_or(0) + 1;
        let (w, h) = st.dimensions();
        let power = st.power_cost();

        let new_b = Building {
            id: next_id,
            name: format!("{} #{}", st.name(), next_id),
            tile_x: site.0,
            tile_y: site.1,
            width: w,
            height: h,
            power_cost: power,
            is_active: true,
            is_selected: false,
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
            faction: Some(ai_faction),
        };

        // Mark map tiles as occupied
        for dy in 0..h {
            for dx in 0..w {
                let tx = (site.0 + dx) as u32;
                let ty = (site.1 + dy) as u32;
                let idx = (ty * session.game_map.width + tx) as usize;
                if idx < session.game_map.tiles.len() {
                    session.game_map.tiles[idx].is_wall_or_building = true;
                }
            }
        }

        // Connect with transport tube to the nearest friendly building
        let nearest = session.buildings.iter()
            .filter(|b| b.faction == Some(ai_faction) && b.id != next_id)
            .min_by_key(|b| (b.tile_x - site.0).abs() + (b.tile_y - site.1).abs());

        if let Some(target) = nearest {
            let start = (site.0 + w / 2, site.1 + h / 2);
            let end = (target.tile_x + target.width / 2, target.tile_y + target.height / 2);
            connect_with_tubes(&mut session.tubes, start, end);
        }

        session.buildings.push(new_b);
        session.fog.update_visibility(&session.units, &session.buildings);

        Ok(next_id)
    }
}

/// Subsystem 2: Production Manager
///
/// Determines vehicle composition (Robo-Miners, Cargo Trucks, Lynx/Panther/Tiger combat hulls,
/// weapon attachments) based on personality doctrine and queues factory production.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionManager {
    pub vehicle_factory_id: Option<u32>,
    pub trucks_produced: u32,
    pub miners_produced: u32,
    pub combat_units_produced: u32,
    pub queue_cooldown: u32,
}

impl Default for ProductionManager {
    fn default() -> Self {
        Self {
            vehicle_factory_id: None,
            trucks_produced: 0,
            miners_produced: 0,
            combat_units_produced: 0,
            queue_cooldown: 0,
        }
    }
}

impl ProductionManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Selects the next vehicle specification according to economic needs and combat doctrine
    pub fn select_next_vehicle(
        &self,
        personality: &AiPersonality,
        _difficulty: Difficulty,
        owned_units: &[Unit],
        researched_techs: &[u32],
        ore_deposits: &[OreDeposit],
    ) -> String {
        let miner_count = owned_units.iter().filter(|u| u.name.contains("Miner")).count();
        let truck_count = owned_units.iter().filter(|u| u.name.contains("Cargo")).count();
        let deposits_available = ore_deposits.iter().any(|d| d.remaining_amount > 0);

        // Economic logistics quota: ensure at least 1 miner and 2 cargo trucks if deposits exist
        if deposits_available && miner_count == 0 {
            return "Robo-Miner".to_string();
        }
        if deposits_available && truck_count < 2 {
            return "Cargo Truck".to_string();
        }

        let repair_count = owned_units.iter().filter(|u| u.name.contains("Repair")).count();
        let combat_count = owned_units.iter().filter(|u| u.weapon.is_some()).count();
        if (*personality == AiPersonality::DefensiveFortifier || *personality == AiPersonality::EconomicTechRusher)
            && combat_count >= 3
            && repair_count == 0
        {
            return "Repair Vehicle".to_string();
        }

        // Military combat hull quotas
        let (lynx_ratio, _panther_ratio, _tiger_ratio) = personality.combat_hull_ratios();
        let lynx_count = owned_units.iter().filter(|u| u.name.contains("Lynx") || u.name.contains("Raider")).count();
        let tiger_count = owned_units.iter().filter(|u| u.name.contains("Tiger")).count();

        let can_build_tiger = researched_techs.contains(&5301)
            || *personality == AiPersonality::DefensiveFortifier
            || *personality == AiPersonality::AggressiveRusher;

        let total_combat = combat_count.max(1) as f64;
        let current_lynx_ratio = lynx_count as f64 / total_combat;

        if current_lynx_ratio < lynx_ratio {
            match personality {
                AiPersonality::AggressiveRusher => "Lynx (Microwave)".to_string(),
                _ => "Lynx Scout".to_string(),
            }
        } else if can_build_tiger && (tiger_count == 0 || combat_count >= 5) {
            "Tiger Heavy Tank".to_string()
        } else {
            "Panther Battle Tank".to_string()
        }
    }

    /// Advances the production cycle, enqueuing units into active factories
    pub fn update(
        &mut self,
        ai_faction: Faction,
        personality: &AiPersonality,
        difficulty: Difficulty,
        common_ore: &mut u32,
        _rare_ore: &mut u32,
        session: &mut GameSession,
        _current_tick: u32,
        researched_techs: &[u32],
    ) {
        if self.queue_cooldown > 0 {
            self.queue_cooldown -= 1;
            return;
        }

        let owned_units: Vec<Unit> = session.units.iter().filter(|u| u.faction == ai_faction).cloned().collect();

        // Locate active friendly Vehicle Factory
        let factory = session.buildings.iter_mut().find(|b| {
            b.faction == Some(ai_faction) && b.name.contains("Vehicle Factory") && b.is_active
        });

        if let Some(f) = factory {
            self.vehicle_factory_id = Some(f.id);
            if f.production_queue.is_empty() {
                let next_veh = self.select_next_vehicle(
                    personality,
                    difficulty,
                    &owned_units,
                    researched_techs,
                    &session.ore_deposits,
                );

                let cost = match next_veh.as_str() {
                    "Robo-Miner" => 180,
                    "Cargo Truck" => 150,
                    "Repair Vehicle" => 200,
                    "Tiger Heavy Tank" => 400,
                    "Panther Battle Tank" => 280,
                    _ => 160, // Lynx Scout / Lynx Raider
                };

                if *common_ore >= cost {
                    *common_ore -= cost;
                    f.production_queue.push(next_veh.clone());
                    self.queue_cooldown = match difficulty {
                        Difficulty::Hard => 20,
                        Difficulty::Medium => 35,
                        Difficulty::Easy => 50,
                    };

                    if next_veh.contains("Cargo") {
                        self.trucks_produced += 1;
                    } else if next_veh.contains("Miner") {
                        self.miners_produced += 1;
                    } else {
                        self.combat_units_produced += 1;
                    }
                }
            }
        }
    }
}

/// Subsystem 3: Tactical Squad Manager
///
/// Groups idle military units into strike squads, patrol wings, and base defense escorts.
/// Directs coordinated attacks toward player colonies, targeting high-value targets
/// (ConVecs, ore trucks, power plants, command centers). Damaged units disengage to seek repair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalSquadManager {
    pub squads: Vec<TacticalSquad>,
    pub next_squad_id: u32,
    pub patrol_waypoints: Vec<(i32, i32)>,
    pub patrol_index: usize,
    pub base_center: (i32, i32),
}

impl TacticalSquadManager {
    pub fn new(base_center: (i32, i32)) -> Self {
        let waypoints = vec![
            (base_center.0 - 6, base_center.1 - 4),
            (base_center.0 + 6, base_center.1 - 4),
            (base_center.0 + 6, base_center.1 + 6),
            (base_center.0 - 6, base_center.1 + 6),
        ];

        Self {
            squads: Vec::new(),
            next_squad_id: 1,
            patrol_waypoints: waypoints,
            patrol_index: 0,
            base_center,
        }
    }

    /// Evaluates battlefield state, manages squad formations, orders assaults, and triggers unit disengagement
    pub fn update(
        &mut self,
        ai_faction: Faction,
        personality: &AiPersonality,
        difficulty: Difficulty,
        tactical_state: &mut AiTacticalState,
        session: &mut GameSession,
        current_tick: u32,
    ) -> Option<String> {
        let mut alert = None;

        // 1. Remove dead/destroyed units from squads
        let alive_ids: HashSet<u32> = session.units.iter().map(|u| u.id).collect();
        for squad in &mut self.squads {
            squad.unit_ids.retain(|id| alive_ids.contains(id));
        }
        self.squads.retain(|squad| !squad.unit_ids.is_empty());

        // 2. Disengagement and retreat to repair depots for heavily damaged units
        self.process_damaged_unit_retreats(ai_faction, difficulty, session);

        // 3. Collect idle, non-retreating combat units
        let in_squad_ids: HashSet<u32> = self.squads.iter().flat_map(|s| s.unit_ids.clone()).collect();
        let idle_combat_ids: Vec<u32> = session.units.iter()
            .filter(|u| {
                u.faction == ai_faction
                    && u.weapon.is_some()
                    && !in_squad_ids.contains(&u.id)
                    && (u.current_order == UnitOrder::Idle || matches!(u.current_order, UnitOrder::Guard { .. }))
            })
            .map(|u| u.id)
            .collect();

        // 4. Form squads based on personality doctrine
        let min_strike_size = personality.min_strike_squad_size();
        if idle_combat_ids.len() >= min_strike_size {
            let squad_units = idle_combat_ids[..min_strike_size].to_vec();
            let squad_id = self.next_squad_id;
            self.next_squad_id += 1;

            let role = match personality {
                AiPersonality::AggressiveRusher => {
                    if squad_id % 3 == 0 {
                        SquadRole::Harassment
                    } else {
                        SquadRole::Strike
                    }
                }
                AiPersonality::DefensiveFortifier => SquadRole::Strike,
                _ => SquadRole::Strike,
            };

            self.squads.push(TacticalSquad {
                id: squad_id,
                role,
                unit_ids: squad_units,
                target_coords: None,
                target_unit_id: None,
                rally_point: self.base_center,
                patrol_index: 0,
                waypoints: self.patrol_waypoints.clone(),
            });
        }

        // 5. Update orders for active squads
        let base_center = self.base_center;
        for squad in &mut self.squads {
            match squad.role {
                SquadRole::Strike | SquadRole::Harassment => {
                    // Acquire high-value enemy targets
                    let target = Self::find_high_value_target(base_center, ai_faction, squad.role, session);
                    if let Some((target_coord, enemy_unit_id)) = target {
                        squad.target_coords = Some(target_coord);
                        squad.target_unit_id = enemy_unit_id;

                        for &uid in &squad.unit_ids {
                            if let Some(u) = session.units.iter_mut().find(|u| u.id == uid) {
                                if let Some(euid) = enemy_unit_id {
                                    u.current_order = UnitOrder::Attack { target_unit_id: euid };
                                } else {
                                    u.current_order = UnitOrder::MoveTo {
                                        target_x: target_coord.0,
                                        target_y: target_coord.1,
                                    };
                                }
                            }
                        }

                        if tactical_state.stance != TacticalStance::Assaulting {
                            tactical_state.stance = TacticalStance::Assaulting;
                            tactical_state.last_attack_tick = current_tick;
                            alert = Some(format!(
                                "Tactical Incursion: {:?} strike squad mobilizing toward colony assets!",
                                ai_faction
                            ));
                        }
                    }
                }
                SquadRole::Patrol => {
                    if !self.patrol_waypoints.is_empty() {
                        let wp = self.patrol_waypoints[self.patrol_index % self.patrol_waypoints.len()];
                        for &uid in &squad.unit_ids {
                            if let Some(u) = session.units.iter_mut().find(|u| u.id == uid) {
                                if u.current_order == UnitOrder::Idle {
                                    u.current_order = UnitOrder::MoveTo { target_x: wp.0, target_y: wp.1 };
                                }
                            }
                        }
                        self.patrol_index += 1;
                    }
                }
                SquadRole::BaseDefense => {
                    for &uid in &squad.unit_ids {
                        if let Some(u) = session.units.iter_mut().find(|u| u.id == uid) {
                            if u.current_order == UnitOrder::Idle {
                                u.current_order = UnitOrder::Guard {
                                    anchor_x: self.base_center.0,
                                    anchor_y: self.base_center.1,
                                };
                            }
                        }
                    }
                }
            }
        }

        tactical_state.alert_message = alert.clone();
        alert
    }

    /// Identifies and prioritizes enemy high-value assets:
    /// ConVecs (95) > Cargo Trucks (90) > Command Center (80) > Power/Smelters (70) > Guard Posts (60)
    pub fn select_high_value_target(
        &self,
        ai_faction: Faction,
        role: SquadRole,
        session: &GameSession,
    ) -> Option<((i32, i32), Option<u32>)> {
        Self::find_high_value_target(self.base_center, ai_faction, role, session)
    }

    pub fn find_high_value_target(
        base_center: (i32, i32),
        ai_faction: Faction,
        role: SquadRole,
        session: &GameSession,
    ) -> Option<((i32, i32), Option<u32>)> {
        let mut best_target: Option<((i32, i32), Option<u32>, i32)> = None;

        // 1. Evaluate enemy units
        for u in &session.units {
            if u.faction != ai_faction && u.hp > 0 {
                let base_priority = if u.name.contains("ConVec") {
                    95
                } else if u.name.contains("Cargo") {
                    90
                } else if u.name.contains("Miner") {
                    85
                } else if role == SquadRole::Harassment {
                    30 // Harassment raiders strictly favor economic logistics
                } else {
                    50
                };

                let dist = (u.tile_x - base_center.0).abs() + (u.tile_y - base_center.1).abs();
                let score = base_priority * 10 - dist;

                if best_target.as_ref().map_or(true, |b| score > b.2) {
                    best_target = Some(((u.tile_x, u.tile_y), Some(u.id), score));
                }
            }
        }

        // 2. Evaluate enemy structures if no vital unit was found or if executing general assault
        if best_target.is_none() || role == SquadRole::Strike {
            for b in &session.buildings {
                if b.faction != Some(ai_faction) {
                    let base_priority = if b.name.contains("Command") {
                        80
                    } else if b.name.contains("Solar") || b.name.contains("Geothermal") || b.name.contains("Tokamak") {
                        75
                    } else if b.name.contains("Smelter") {
                        70
                    } else if b.name.contains("Vehicle Factory") {
                        65
                    } else if b.name.contains("Guard") {
                        60
                    } else {
                        40
                    };

                    let center_x = b.tile_x + b.width / 2;
                    let center_y = b.tile_y + b.height / 2;
                    let dist = (center_x - base_center.0).abs() + (center_y - base_center.1).abs();
                    let score = base_priority * 10 - dist;

                    if best_target.as_ref().map_or(true, |b| score > b.2) {
                        best_target = Some(((center_x, center_y), None, score));
                    }
                }
            }
        }

        best_target.map(|(coords, uid, _)| (coords, uid))
    }

    /// Disengages damaged units below the difficulty health threshold, ordering retreat to repair depots
    pub fn process_damaged_unit_retreats(
        &mut self,
        ai_faction: Faction,
        difficulty: Difficulty,
        session: &mut GameSession,
    ) {
        let threshold = difficulty.repair_retreat_threshold();

        let repair_facility = session.buildings.iter()
            .find(|b| b.faction == Some(ai_faction) && b.name.contains("Repair") && b.is_active)
            .map(|b| (b.tile_x + b.width / 2, b.tile_y + b.height / 2));

        if let Some((rx, ry)) = repair_facility {
            for u in &mut session.units {
                if u.faction == ai_faction && u.weapon.is_some() {
                    let hp_ratio = u.hp as f64 / u.max_hp as f64;
                    if hp_ratio <= threshold && hp_ratio > 0.0 {
                        // Disengage and path to repair facility
                        u.current_order = UnitOrder::MoveTo { target_x: rx, target_y: ry };
                        if let Some(path) = PathFinder::find_path(&session.game_map, (u.tile_x, u.tile_y), (rx, ry)) {
                            u.path = path;
                        }
                    }
                }
            }
        }
    }
}

/// Subsystem 4: Research Planner
///
/// Autonomously selects next tech tree advancements matching personality priority
/// and manages technology unlocking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPlanner {
    pub tech_queue: Vec<u32>,
    pub researched_techs: Vec<u32>,
    pub current_research_progress: f64,
}

impl ResearchPlanner {
    pub fn new_for_personality(personality: &AiPersonality) -> Self {
        Self {
            tech_queue: personality.tech_priorities(),
            researched_techs: Vec::new(),
            current_research_progress: 0.0,
        }
    }

    /// Advances research progression when labs are active and common ore budget is available
    pub fn update(&mut self, common_ore: &mut u32, has_active_lab: bool) -> Option<u32> {
        if !has_active_lab {
            return None;
        }

        let next_tech_id = self.tech_queue.iter().find(|id| !self.researched_techs.contains(id)).copied()?;

        let cost = match next_tech_id {
            2106 => 0,    // Basic Lab
            2701 => 400,  // Astronomy
            3301 => 650,  // Hydroponics Automation
            3402 => 800,  // High-Temperature Smelting
            4201 => 1100, // Lava Containment Walls
            4301 => 750,  // Vulcanology & Hydrothermal Extraction
            5104 => 950,  // Lynx Reinforced Armor
            5202 => 1200, // Pulse Laser Optics
            5301 => 1400, // Railguns & Superconductors
            5401 => 600,  // Point Defense Turrets (Guard Posts)
            6101 => 1500, // Cryogenic Evacuation
            _ => 500,
        };

        if *common_ore >= cost {
            *common_ore -= cost;
            self.researched_techs.push(next_tech_id);
            Some(next_tech_id)
        } else {
            None
        }
    }
}

/// Complete Autonomous AI Player Controller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPlayer {
    pub id: u32,
    pub name: String,
    pub faction: Faction,
    pub personality: AiPersonality,
    pub difficulty: Difficulty,
    pub tactical_state: AiTacticalState,
    pub building_planner: BaseBuildingPlanner,
    pub production_manager: ProductionManager,
    pub squad_manager: TacticalSquadManager,
    pub research_planner: ResearchPlanner,
    pub base_x: i32,
    pub base_y: i32,
    pub common_ore: u32,
    pub rare_ore: u32,
    pub power_produced: u32,
    pub power_consumed: u32,
    pub update_interval: u32,
    pub tick_offset: u32,
}

impl AiPlayer {
    pub fn new(
        id: u32,
        name: String,
        faction: Faction,
        personality: AiPersonality,
        difficulty: Difficulty,
        base_x: i32,
        base_y: i32,
    ) -> Self {
        let (init_common, init_rare) = difficulty.starting_resources();
        let update_interval = difficulty.decision_interval_ticks();
        let tick_offset = (id * 11) % update_interval;

        Self {
            id,
            name,
            faction,
            personality: personality.clone(),
            difficulty,
            tactical_state: AiTacticalState::default(),
            building_planner: BaseBuildingPlanner::new((base_x, base_y)),
            production_manager: ProductionManager::new(),
            squad_manager: TacticalSquadManager::new((base_x, base_y)),
            research_planner: ResearchPlanner::new_for_personality(&personality),
            base_x,
            base_y,
            common_ore: init_common,
            rare_ore: init_rare,
            power_produced: 0,
            power_consumed: 0,
            update_interval,
            tick_offset,
        }
    }

    /// Autonomous strategic update loop executed at staggered tick intervals
    pub fn update(&mut self, session: &mut GameSession, current_tick: u32) -> Option<String> {
        // Execute decision logic on staggered tick interval to eliminate frame hitching
        if (current_tick + self.tick_offset) % self.update_interval != 0 {
            return None;
        }

        self.refresh_colony_power(session);
        self.accrue_passive_income();

        // 1. Advance Research Progression
        let has_active_lab = session.buildings.iter().any(|b| {
            b.faction == Some(self.faction) && b.name.contains("Lab") && b.is_active
        });
        self.research_planner.update(&mut self.common_ore, has_active_lab);

        // 2. Base Expansion & Construction Planning
        let owned_buildings: Vec<Building> = session.buildings.iter()
            .filter(|b| b.faction == Some(self.faction))
            .cloned()
            .collect();

        if let Some(next_st) = self.building_planner.plan_next_structure(
            &self.personality,
            self.power_produced,
            self.power_consumed,
            self.common_ore,
            &owned_buildings,
            &self.research_planner.researched_techs,
        ) {
            let origin = (self.base_x, self.base_y);
            if let Some(site) = self.building_planner.find_placement_site(
                &session.game_map,
                &session.buildings,
                next_st,
                origin,
            ) {
                let _ = self.building_planner.execute_build(
                    session,
                    self.faction,
                    next_st,
                    site,
                    &mut self.common_ore,
                    &mut self.rare_ore,
                    current_tick,
                );
            }
        }

        // 3. Factory Vehicle Production Management
        self.production_manager.update(
            self.faction,
            &self.personality,
            self.difficulty,
            &mut self.common_ore,
            &mut self.rare_ore,
            session,
            current_tick,
            &self.research_planner.researched_techs,
        );

        // 4. Tactical Squad Operations & Military Command
        self.squad_manager.update(
            self.faction,
            &self.personality,
            self.difficulty,
            &mut self.tactical_state,
            session,
            current_tick,
        )
    }

    /// Re-evaluates power generation and consumption for all owned structures
    pub fn refresh_colony_power(&mut self, session: &GameSession) {
        let mut produced = 0;
        let mut consumed = 0;

        for b in &session.buildings {
            if b.faction == Some(self.faction) && b.is_active {
                if b.power_cost < 0 {
                    produced += (-b.power_cost) as u32;
                } else {
                    consumed += b.power_cost as u32;
                }
            }
        }

        self.power_produced = produced;
        self.power_consumed = consumed;
    }

    /// Passive economic mineral yield scaling with difficulty and active smelter capacity
    fn accrue_passive_income(&mut self) {
        let base_income = match self.difficulty {
            Difficulty::Easy => 10,
            Difficulty::Medium => 25,
            Difficulty::Hard => 50,
        };
        let bonus = (base_income as f64 * self.difficulty.resource_multiplier()) as u32;
        self.common_ore += bonus;
    }
}

/// Backwards-compatible Faction AI Director for legacy scenario campaign support
#[derive(Debug, Clone)]
pub struct FactionAiDirector {
    pub faction: Faction,
    pub spawn_timer: f64,
    pub patrol_timer: f64,
    pub patrol_index: usize,
    pub base_x: i32,
    pub base_y: i32,
    pub wave_number: u32,
}

impl FactionAiDirector {
    pub fn new(faction: Faction, base_x: i32, base_y: i32) -> Self {
        Self {
            faction,
            spawn_timer: 25.0,
            patrol_timer: 10.0,
            patrol_index: 0,
            base_x,
            base_y,
            wave_number: 0,
        }
    }

    pub fn update(
        &mut self,
        units: &mut Vec<Unit>,
        player_targets: &[(i32, i32)],
        dt_secs: f64,
    ) -> Option<&'static str> {
        let mut alert = None;
        self.spawn_timer -= dt_secs;
        self.patrol_timer -= dt_secs;

        if self.spawn_timer <= 0.0 {
            self.spawn_timer = 45.0;
            self.wave_number += 1;
            let is_assault_wave = self.wave_number % 2 == 0;

            let next_id = units.iter().map(|u| u.id).max().unwrap_or(100) + 1;
            let (name, hp, weapon) = if is_assault_wave {
                ("Plymouth Tiger Assault", 320, Some(WeaponType::Railgun))
            } else if next_id % 2 == 0 {
                ("Plymouth Panther Battle Tank", 260, Some(WeaponType::Laser))
            } else {
                ("Plymouth Raider Lynx", 150, Some(WeaponType::Microwave))
            };

            let spawn_x = self.base_x + (next_id as i32 % 3);
            let spawn_y = self.base_y + (next_id as i32 % 3);

            units.push(Unit {
                id: next_id,
                name: format!("{} #{}", name, next_id),
                faction: self.faction,
                tile_x: spawn_x,
                tile_y: spawn_y,
                hp,
                max_hp: hp,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            if is_assault_wave {
                let escort_id = next_id + 1;
                units.push(Unit {
                    id: escort_id,
                    name: format!("Plymouth Raider Lynx #{}", escort_id),
                    faction: self.faction,
                    tile_x: spawn_x + 1,
                    tile_y: spawn_y + 1,
                    hp: 160,
                    max_hp: 160,
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

                let target_coord = if let Some(&target) = player_targets.first() {
                    let mut closest = target;
                    let mut min_dist = f64::MAX;
                    for &(tx, ty) in player_targets {
                        let d = ((tx - self.base_x).pow(2) + (ty - self.base_y).pow(2)) as f64;
                        if d < min_dist {
                            min_dist = d;
                            closest = (tx, ty);
                        }
                    }
                    Some(closest)
                } else {
                    None
                };

                if let Some((tx, ty)) = target_coord {
                    for u in units.iter_mut() {
                        if u.faction == self.faction
                            && (u.current_order == UnitOrder::Idle || matches!(u.current_order, UnitOrder::MoveTo { .. }))
                        {
                            u.current_order = UnitOrder::MoveTo { target_x: tx, target_y: ty };
                        }
                    }
                }

                alert = Some("Tactical Red Alert: Hostile Plymouth assault squad advancing on colony perimeter!");
            } else {
                alert = Some("Tactical Alert: Hostile Plymouth reinforcements deployed!");
            }
        }

        let waypoints = [
            (self.base_x - 4, self.base_y - 2),
            (self.base_x - 8, self.base_y + 2),
            (self.base_x - 3, self.base_y + 4),
            (self.base_x, self.base_y),
        ];

        if self.patrol_timer <= 0.0 {
            self.patrol_timer = 12.0;
            self.patrol_index = (self.patrol_index + 1) % waypoints.len();
            let (tx, ty) = waypoints[self.patrol_index];

            for u in units.iter_mut() {
                if u.faction == self.faction && u.current_order == UnitOrder::Idle {
                    u.current_order = UnitOrder::MoveTo { target_x: tx, target_y: ty };
                }
            }
        }

        alert
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::mining::OreType;
    use crate::sim::ColonyState;

    #[test]
    fn test_personality_initialization() {
        let rusher = AiPersonality::AggressiveRusher;
        assert_eq!(rusher.default_faction(), Faction::Plymouth);
        assert_eq!(rusher.min_strike_squad_size(), 3);
        assert!(rusher.preferred_weapons().contains(&WeaponType::Microwave));
        assert!(rusher.preferred_weapons().contains(&WeaponType::AcidCloud));
        let (lynx, panther, tiger) = rusher.combat_hull_ratios();
        assert!(lynx > panther && panther > tiger);

        let fortifier = AiPersonality::DefensiveFortifier;
        assert_eq!(fortifier.default_faction(), Faction::Eden);
        assert_eq!(fortifier.min_strike_squad_size(), 6);
        assert_eq!(fortifier.target_guard_posts(), 4);
        assert!(fortifier.tech_priorities().contains(&5401)); // Guard Posts

        let tech_rusher = AiPersonality::EconomicTechRusher;
        assert_eq!(tech_rusher.target_labs(), 3);
        assert!(tech_rusher.tech_priorities().contains(&3402)); // Smelting
        assert!(tech_rusher.tech_priorities().contains(&5301)); // Railguns

        let custom = AiPersonality::Custom(AiConfig {
            name: "Custom Doctrine".into(),
            rush_preference: 0.9,
            defense_bias: 0.1,
            tech_focus: 0.2,
            economic_expansion_weight: 0.3,
            min_strike_squad_size: 2,
            preferred_weapons: vec![WeaponType::Railgun],
            target_guard_posts: 1,
            target_labs: 1,
        });
        assert_eq!(custom.min_strike_squad_size(), 2);
    }

    #[test]
    fn test_production_quotas() {
        let prod = ProductionManager::new();
        let personality = AiPersonality::AggressiveRusher;
        let units = Vec::new();
        let deposits = vec![OreDeposit::new(1, OreType::Common, 10, 10, 5000)];

        // With no miners, logistics must come first
        let next_v = prod.select_next_vehicle(&personality, Difficulty::Medium, &units, &[], &deposits);
        assert_eq!(next_v, "Robo-Miner");

        // With a miner present but insufficient trucks, cargo truck is next
        let units_with_miner = vec![Unit {
            id: 1,
            name: "Robo-Miner #1".into(),
            faction: Faction::Plymouth,
            tile_x: 10,
            tile_y: 10,
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
        }];
        let next_v = prod.select_next_vehicle(&personality, Difficulty::Medium, &units_with_miner, &[], &deposits);
        assert_eq!(next_v, "Cargo Truck");

        // When logistics are established, AggressiveRusher produces Lynx raiders
        let mut full_logistics = units_with_miner.clone();
        full_logistics.push(Unit {
            id: 2,
            name: "Cargo Truck #1".into(),
            faction: Faction::Plymouth,
            tile_x: 10,
            tile_y: 11,
            hp: 150,
            max_hp: 150,
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
        full_logistics.push(Unit {
            id: 3,
            name: "Cargo Truck #2".into(),
            faction: Faction::Plymouth,
            tile_x: 10,
            tile_y: 12,
            hp: 150,
            max_hp: 150,
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

        let next_v = prod.select_next_vehicle(&personality, Difficulty::Medium, &full_logistics, &[], &deposits);
        assert_eq!(next_v, "Lynx (Microwave)");
    }

    #[test]
    fn test_building_placement_site_selection() {
        let session = ColonyState::new_scenario(0);
        let planner = BaseBuildingPlanner::new((40, 30));

        let site = planner.find_placement_site(
            &session.game_map,
            &session.buildings,
            StructureType::SolarArray,
            (40, 30),
        );
        assert!(site.is_some());
        let (sx, sy) = site.unwrap();

        // Site must be within cluster radius and not overlap existing colony buildings
        assert!((sx - 40).abs() <= planner.cluster_radius);
        assert!((sy - 30).abs() <= planner.cluster_radius);
        assert!(planner.is_site_clear(&session.game_map, &session.buildings, sx, sy, 2, 2));
    }

    #[test]
    fn test_tactical_squad_assault_logic() {
        let mut session = ColonyState::new_scenario(0);
        let mut squad_mgr = TacticalSquadManager::new((40, 30));
        let mut tactical_state = AiTacticalState::default();

        // Add 3 hostile Plymouth Lynx units to form a strike squad
        for i in 1..=3 {
            session.units.push(Unit {
                id: 500 + i,
                name: format!("Plymouth Lynx #{}", i),
                faction: Faction::Plymouth,
                tile_x: 40 + i as i32,
                tile_y: 30,
                hp: 150,
                max_hp: 150,
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
        }

        let alert = squad_mgr.update(
            Faction::Plymouth,
            &AiPersonality::AggressiveRusher,
            Difficulty::Hard,
            &mut tactical_state,
            &mut session,
            100,
        );

        assert!(alert.is_some());
        assert_eq!(squad_mgr.squads.len(), 1);
        let squad = &squad_mgr.squads[0];
        assert_eq!(squad.unit_ids.len(), 3);
        assert!(squad.target_coords.is_some());

        // Verify squad members were assigned attack or move orders towards target
        let squad_unit = session.units.iter().find(|u| u.id == 501).unwrap();
        assert!(matches!(squad_unit.current_order, UnitOrder::Attack { .. } | UnitOrder::MoveTo { .. }));
    }

    #[test]
    fn test_damaged_unit_retreat_to_repair() {
        let mut session = ColonyState::new_scenario(0);
        let mut squad_mgr = TacticalSquadManager::new((40, 30));

        // Add a friendly repair facility for Plymouth
        session.buildings.push(Building {
            id: 99,
            name: "Vehicle Repair Depot #99".into(),
            tile_x: 42,
            tile_y: 32,
            width: 2,
            height: 2,
            power_cost: 60,
            is_active: true,
            is_selected: false,
            production_queue: Vec::new(),
            production_progress: 0.0,
            attack_cooldown: 0.0,
            faction: Some(Faction::Plymouth),
        });

        // Add a critically damaged Plymouth unit
        session.units.push(Unit {
            id: 601,
            name: "Plymouth Panther #601".into(),
            faction: Faction::Plymouth,
            tile_x: 25,
            tile_y: 25,
            hp: 40, // 40 / 250 = 16% HP (< repair threshold of 40%)
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

        squad_mgr.process_damaged_unit_retreats(Faction::Plymouth, Difficulty::Medium, &mut session);

        let damaged_unit = session.units.iter().find(|u| u.id == 601).unwrap();
        match damaged_unit.current_order {
            UnitOrder::MoveTo { target_x, target_y } => {
                // Must be directed to repair facility coordinates (42+1, 32+1) = (43, 33)
                assert_eq!(target_x, 43);
                assert_eq!(target_y, 33);
            }
            _ => panic!("Damaged unit failed to disengage and retreat to repair depot"),
        }
    }

    #[test]
    fn test_research_planner_progression() {
        let personality = AiPersonality::EconomicTechRusher;
        let mut planner = ResearchPlanner::new_for_personality(&personality);
        let mut common_ore = 1000;

        // Basic Lab (id 2106, cost 0)
        let researched1 = planner.update(&mut common_ore, true);
        assert_eq!(researched1, Some(2106));
        assert_eq!(common_ore, 1000);

        // Next is High-Temp Smelting (id 3402, cost 800)
        let researched2 = planner.update(&mut common_ore, true);
        assert_eq!(researched2, Some(3402));
        assert_eq!(common_ore, 200);

        // Third requires 650 ore, but we only have 200 remaining -> should return None
        let researched3 = planner.update(&mut common_ore, true);
        assert_eq!(researched3, None);
        assert_eq!(common_ore, 200);
    }

    #[test]
    fn test_multi_ai_staggered_ticks() {
        let ai1 = AiPlayer::new(1, "Bot Alpha".into(), Faction::Plymouth, AiPersonality::AggressiveRusher, Difficulty::Medium, 40, 40);
        let ai2 = AiPlayer::new(2, "Bot Beta".into(), Faction::Eden, AiPersonality::DefensiveFortifier, Difficulty::Medium, 10, 40);
        let ai3 = AiPlayer::new(3, "Bot Gamma".into(), Faction::Plymouth, AiPersonality::EconomicTechRusher, Difficulty::Medium, 40, 10);

        assert_ne!(ai1.tick_offset, ai2.tick_offset);
        assert_ne!(ai2.tick_offset, ai3.tick_offset);
    }

    #[test]
    fn test_difficulty_scalers() {
        let easy = Difficulty::Easy;
        let med = Difficulty::Medium;
        let hard = Difficulty::Hard;

        assert!(easy.decision_interval_ticks() > med.decision_interval_ticks());
        assert!(med.decision_interval_ticks() > hard.decision_interval_ticks());

        assert!(hard.starting_resources().0 > med.starting_resources().0);
        assert!(med.starting_resources().0 > easy.starting_resources().0);

        assert!(hard.aggression_multiplier() > med.aggression_multiplier());
        assert!(med.aggression_multiplier() > easy.aggression_multiplier());

        assert!(hard.repair_retreat_threshold() > med.repair_retreat_threshold());
    }

    #[test]
    fn test_skirmish_setup_with_multiple_ai() {
        let mut session = ColonyState::new_scenario(0);
        let opponents = vec![
            (Faction::Plymouth, AiPersonality::AggressiveRusher, Difficulty::Hard, 40, 35),
            (Faction::Eden, AiPersonality::DefensiveFortifier, Difficulty::Medium, 12, 40),
            (Faction::Plymouth, AiPersonality::EconomicTechRusher, Difficulty::Easy, 45, 12),
        ];

        session.setup_skirmish(Faction::Eden, &opponents);

        assert_eq!(session.ai_players.len(), 3);
        assert_eq!(session.ai_players[0].personality, AiPersonality::AggressiveRusher);
        assert_eq!(session.ai_players[1].personality, AiPersonality::DefensiveFortifier);
        assert_eq!(session.ai_players[2].personality, AiPersonality::EconomicTechRusher);

        // Verify each AI was provisioned with base structures
        for (i, &(_, _, _, bx, by)) in opponents.iter().enumerate() {
            let ai_faction = opponents[i].0;
            let cc = session.buildings.iter().find(|b| b.faction == Some(ai_faction) && b.tile_x == bx && b.tile_y == by);
            assert!(cc.is_some(), "Command Center missing for AI #{}", i + 1);

            let cargo = session.units.iter().find(|u| u.faction == ai_faction && u.name.contains("Cargo"));
            assert!(cargo.is_some(), "Cargo truck missing for AI #{}", i + 1);
        }

        // Run simulation update and ensure AI loops execute without panic
        let update_res = session.update(0.1);
        assert!(session.tick_count > 0);
        let _ = update_res;
    }
}
