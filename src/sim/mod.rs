//! Outpost 2 Colony Simulation and Game State

pub mod disasters;
pub mod fog;
pub mod map;
pub mod pathfinding;
pub mod structures;
pub mod tech;

use std::collections::HashSet;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: u32,
    pub name: String,
    pub tile_x: i32,
    pub tile_y: i32,
    pub hp: u32,
    pub max_hp: u32,
    pub is_selected: bool,
    pub current_order: UnitOrder,
    pub path: Vec<(i32, i32)>,
    pub move_cooldown: f64,
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

        // Starting ConVec and Cargo Truck
        units.push(Unit {
            id: 1,
            name: "ConVec (Structure Kit)".into(),
            tile_x: 21,
            tile_y: 24,
            hp: 200,
            max_hp: 200,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
        });

        units.push(Unit {
            id: 2,
            name: "Cargo Truck".into(),
            tile_x: 23,
            tile_y: 24,
            hp: 150,
            max_hp: 150,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
        });

        units.push(Unit {
            id: 3,
            name: "Robo-Miner".into(),
            tile_x: 25,
            tile_y: 23,
            hp: 180,
            max_hp: 180,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
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

        // Refresh dynamic sensor fog of war
        self.fog.update_visibility(&self.units, &self.buildings);

        warning
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
