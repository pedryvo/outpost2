//! Outpost 2 Colony Simulation and Game State

pub mod ai;
pub mod combat;
pub mod disasters;
pub mod fog;
pub mod map;
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
use pathfinding::{PathFinder, UnitOrder};
use structures::{connect_with_tubes, StructureType};
use tech::TechTree;

const DEFAULT_MAP_DATA: &[u8] = include_bytes!("../../assets/eden01.map");

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGameData {
    pub faction: Faction,
    pub sol: f64,
    pub population: u32,
    pub morale_pct: u32,
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
    pub faction: Faction,
    pub sol: f64,
    pub population: u32,
    pub morale_pct: u32,
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
}

impl ColonyState {
    pub fn new(faction: Faction) -> Self {
        let mut units = Vec::new();
        let mut buildings = Vec::new();

        // Initial Command Center base
        buildings.push(Building {
            id: 1,
            name: "Command Center".into(),
            tile_x: 20,
            tile_y: 20,
            width: 3,
            height: 3,
            power_cost: 40,
            is_active: true,
        });

        // Initial Solar Array
        buildings.push(Building {
            id: 2,
            name: "Solar Power Array".into(),
            tile_x: 24,
            tile_y: 20,
            width: 2,
            height: 2,
            power_cost: -250, // Produces power
            is_active: true,
        });

        // Starting ConVec, Cargo Truck, and Combat Escorts
        units.push(Unit {
            id: 1,
            name: "ConVec (Structure Kit)".into(),
            faction: Faction::Eden,
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
        });

        units.push(Unit {
            id: 2,
            name: "Cargo Truck".into(),
            faction: Faction::Eden,
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
        });

        units.push(Unit {
            id: 3,
            name: "Robo-Miner".into(),
            faction: Faction::Eden,
            tile_x: 25,
            tile_y: 23,
            hp: 180,
            max_hp: 180,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon: None,
            attack_cooldown: 0.0,
        });

        units.push(Unit {
            id: 4,
            name: "Lynx Scout (Pulse Laser)".into(),
            faction: Faction::Eden,
            tile_x: 23,
            tile_y: 26,
            hp: 140,
            max_hp: 140,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon: Some(WeaponType::Laser),
            attack_cooldown: 0.0,
        });

        units.push(Unit {
            id: 5,
            name: "Panther Medium Tank (Railgun)".into(),
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
        });

        // Hostile Plymouth scout raiders
        units.push(Unit {
            id: 101,
            name: "Plymouth Raider Lynx".into(),
            faction: Faction::Plymouth,
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
        });

        units.push(Unit {
            id: 102,
            name: "Plymouth Tiger Assault".into(),
            faction: Faction::Plymouth,
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
        });

        let game_map = GameMap::parse(DEFAULT_MAP_DATA)
            .unwrap_or_else(|_| GameMap {
                width: 64,
                height: 64,
                tiles: Vec::new(),
            });

        let mut tubes = HashSet::new();
        // Initial arterial transport tube between Command Center and Solar Power Array
        connect_with_tubes(&mut tubes, (22, 21), (24, 21));

        let map_w = game_map.width as usize;
        let map_h = game_map.height as usize;
        let mut fog = FogOfWar::new(map_w, map_h);
        fog.update_visibility(&units, &buildings);

        Self {
            faction,
            sol: 1.0,
            population: 120,
            morale_pct: 95,
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
            ai_director: FactionAiDirector::new(Faction::Plymouth, 38, 30),
        }
    }

    pub fn update(&mut self, dt_secs: f64) -> Option<&'static str> {
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
                                combat_events.push((
                                    tid,
                                    (u.tile_x as f64, u.tile_y as f64),
                                    (tx, ty),
                                    weapon,
                                    weapon.damage(),
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Apply weapon damage and spawn beams/explosions
        let mut destroyed_units = Vec::new();
        for (target_id, from_pos, to_pos, weapon, dmg) in combat_events {
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

        // Advance Rival Faction AI Director
        let ai_alert = self.ai_director.update(&mut self.units, dt_secs);

        // Refresh dynamic sensor fog of war
        self.fog.update_visibility(&self.units, &self.buildings);

        warning.or(ai_alert)
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

    /// Export colony state to a JSON-serializable snapshot
    pub fn to_save_data(&self) -> SaveGameData {
        let researched_tech_ids = self.tech_tree.techs.iter()
            .filter(|t| t.is_researched)
            .map(|t| t.id)
            .collect();

        SaveGameData {
            faction: self.faction,
            sol: self.sol,
            population: self.population,
            morale_pct: self.morale_pct,
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
        self.faction = data.faction;
        self.sol = data.sol;
        self.population = data.population;
        self.morale_pct = data.morale_pct;
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
