//! Outpost 2 Colony Simulation and Game State

pub mod ai;
pub mod combat;
pub mod disasters;
pub mod fog;
pub mod map;
pub mod mining;
pub mod mission;
pub mod pathfinding;
pub mod structures;
pub mod tech;

use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use ai::FactionAiDirector;
use combat::{CombatBeam, ExplosionEffect, WeaponType};
use disasters::DisasterSimulator;
use fog::FogOfWar;
use map::GameMap;
use mining::{HaulerState, OreDeposit, OreType};
use mission::Mission;
use pathfinding::{PathFinder, UnitOrder};
use structures::{connect_with_tubes, StructureType};
use tech::TechTree;

const EDEN_MAP_DATA: &[u8] = include_bytes!("../../assets/eden01.map");
const PLYMOUTH_MAP_DATA: &[u8] = include_bytes!("../../assets/ply01.map");
const TUTORIAL_MAP_DATA: &[u8] = include_bytes!("../../assets/tutorial.map");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Faction {
    Eden,
    Plymouth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Default, Clone)]
pub struct SimUpdateResult {
    pub notice: Option<&'static str>,
    pub laser_fired: bool,
    pub railgun_fired: bool,
    pub unit_destroyed: bool,
    pub building_deployed: bool,
    pub vortex_active: bool,
    pub meteor_hit: bool,
    pub mission_ended: Option<mission::MissionStatus>,
    pub starship_launched: bool,
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
            _ => (Faction::Eden, EDEN_MAP_DATA, Mission::new_eden_mission_1(), Faction::Plymouth),
        };

        let mut units = Vec::new();
        let mut buildings = Vec::new();

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

        let game_map = GameMap::parse(map_data)
            .unwrap_or_else(|_| GameMap {
                width: 64,
                height: 64,
                tiles: Vec::new(),
            });

        let mut tubes = HashSet::new();
        connect_with_tubes(&mut tubes, (22, 21), (24, 21));
        connect_with_tubes(&mut tubes, (21, 22), (21, 25));
        if scenario_id == 1 {
            connect_with_tubes(&mut tubes, (20, 24), (18, 24));
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

        Self {
            scenario_id,
            faction,
            sol: 1.0,
            population: 120,
            morale_pct: 95,
            starship_progress: 0.0,
            common_ore: 2500,
            rare_ore: 500,
            power_produced: 500,
            power_consumed: 320,
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
        }
    }

    pub fn update(&mut self, dt_secs: f64) -> SimUpdateResult {
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
        let mut destroyed_units = Vec::new();
        for (target_id, from_pos, to_pos, weapon, dmg) in combat_events {
            if weapon == WeaponType::Railgun {
                railgun_fired = true;
            } else {
                laser_fired = true;
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
                target.hp = target.hp.saturating_sub(dmg);
                if target.hp == 0 {
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

                    for &(other_id, other_faction, ox, oy, other_hp) in &unit_positions {
                        if other_faction != player_faction && other_hp > 0 {
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
                        self.active_beams.push(CombatBeam {
                            from_x: bx,
                            from_y: by,
                            to_x: tx,
                            to_y: ty,
                            color: "#ef4444",
                            lifetime: 0.22,
                            max_lifetime: 0.22,
                        });

                        let dmg = 25 + laser_bonus;
                        if let Some(target) = self.units.iter_mut().find(|u| u.id == target_id) {
                            target.hp = target.hp.saturating_sub(dmg);
                            if target.hp == 0 {
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
        let smelter_coords: Vec<(u32, i32, i32)> = self.buildings.iter()
            .filter(|b| b.name.contains("Smelter"))
            .map(|b| (b.id, b.tile_x, b.tile_y))
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
                                if let Some(&(smelter_id, sx, sy)) = smelter_coords.first() {
                                    if let Some(path) = PathFinder::find_path(&self.game_map, (u.tile_x, u.tile_y), (sx, sy)) {
                                        u.path = path;
                                        u.hauler_state = Some(HaulerState::DrivingToSmelter { smelter_id, ore_type: ore_t, amount: 50 });
                                    }
                                }
                            }
                        }
                    }
                    Some(HaulerState::DrivingToSmelter { smelter_id, ore_type, amount }) => {
                        if let Some(&(_, sx, sy)) = smelter_coords.iter().find(|s| s.0 == smelter_id) {
                            let dist = (u.tile_x - sx).abs() + (u.tile_y - sy).abs();
                            if dist <= 2 {
                                u.hauler_state = Some(HaulerState::Unloading { smelter_id, ore_type, amount, progress_pct: 0 });
                            }
                        }
                    }
                    Some(HaulerState::Unloading { smelter_id: _, ore_type, amount, ref mut progress_pct }) => {
                        *progress_pct += (dt_secs * 70.0) as u32;
                        if *progress_pct >= 100 {
                            match ore_type {
                                OreType::Common => ore_delivered_common += amount,
                                OreType::Rare => ore_delivered_rare += amount,
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
        self.common_ore += (ore_delivered_common as f64 * smelt_mult) as u32;
        self.rare_ore += (ore_delivered_rare as f64 * smelt_mult) as u32;

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
        for (_b_id, spawn_x, spawn_y, item) in completed_items {
            let next_id = self.units.len() as u32 + 1;
            match item.as_str() {
                "Panther Battle Tank" => {
                    let hp = 250 + self.tech_tree.armor_bonus();
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Panther Battle Tank #{}", next_id),
                        faction: self.faction,
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
                    unit_notice = Some("FABRICATION COMPLETE: Panther Battle Tank rolled out of production bay.");
                }
                "Lynx Scout" => {
                    let hp = 140 + self.tech_tree.armor_bonus();
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Lynx Scout #{}", next_id),
                        faction: self.faction,
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
                    unit_notice = Some("FABRICATION COMPLETE: Lynx Scout commissioned for reconnaissance.");
                }
                "Cargo Truck" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Cargo Truck #{}", next_id),
                        faction: self.faction,
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
                    unit_notice = Some("FABRICATION COMPLETE: Cargo Truck deployed for ore logistics.");
                }
                "Repair Vehicle" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Repair Vehicle #{}", next_id),
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
                        cargo_kit: None,
                    });
                    unit_notice = Some("FABRICATION COMPLETE: Repair Vehicle deployed for field maintenance.");
                }
                "Earthworker" => {
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Earthworker #{}", next_id),
                        faction: self.faction,
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
                    unit_notice = Some("FABRICATION COMPLETE: Earthworker commissioned for engineering and terrain fortification.");
                }
                "ConVec" => {
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
                        cargo_kit: None,
                    });
                    unit_notice = Some("FABRICATION COMPLETE: ConVec deployed for modular kit transport.");
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
                    let hp = 420 + self.tech_tree.armor_bonus();
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Tiger Heavy Tank #{}", next_id),
                        faction: self.faction,
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
                    unit_notice = Some("FABRICATION COMPLETE: Tiger Heavy Assault Tank with Railgun rolled out.");
                }
                "Robo-Miner" => {
                    let hp = 160 + self.tech_tree.armor_bonus();
                    self.units.push(Unit {
                        id: next_id,
                        name: format!("Robo-Miner #{}", next_id),
                        faction: self.faction,
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
                    unit_notice = Some("FABRICATION COMPLETE: Robo-Miner ready to construct subterranean mine heads.");
                }
                "Starship Component" => {
                    self.starship_progress = (self.starship_progress + 25.0).min(100.0);
                    if self.starship_progress >= 100.0 {
                        unit_notice = Some("STARSHIP COMPLETE: Evacuation Starship is fully assembled and ready for launch!");
                    } else {
                        unit_notice = Some("STARSHIP PROGRESS: Evacuation module installed (+25%).");
                    }
                }
                _ => {}
            }
        }

        // Field Repairs & Vehicle Maintenance Depots
        let repair_depots: Vec<(i32, i32)> = self.buildings.iter()
            .filter(|b| b.is_active && b.name.contains("Repair"))
            .map(|b| (b.tile_x + b.width / 2, b.tile_y + b.height / 2))
            .collect();

        // 1. Repair Depots radiate emerald nano-welding beams to nearby damaged units
        for (dx, dy) in &repair_depots {
            for u in &mut self.units {
                if u.faction == self.faction && u.hp < u.max_hp {
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

        // Advance Rival Faction AI Director
        let ai_alert = self.ai_director.update(&mut self.units, dt_secs);

        // Evaluate Campaign Mission Objectives
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
                    u.hp = u.hp.saturating_sub(blast_dmg.max(25));
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
        }

        // Refresh dynamic sensor fog of war
        self.fog.update_visibility(&self.units, &self.buildings);

        let vortex_active = !self.disasters.active_vortices.is_empty();

        SimUpdateResult {
            notice: warning.or(ai_alert).or(unit_notice).or(mission_alert),
            laser_fired,
            railgun_fired,
            unit_destroyed,
            building_deployed,
            vortex_active,
            meteor_hit,
            mission_ended,
            starship_launched: self.starship_launched,
        }
    }

    /// Deploy a colonial structure kit, deducting ore costs and weaving transport tubes
    pub fn deploy_structure(&mut self, structure_type: StructureType, tile_x: i32, tile_y: i32) -> Result<String, String> {
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
            "Starship Component" => (500, 200),
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
}
