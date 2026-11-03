//! Outpost 2 Colony Simulation and Game State

pub mod map;
pub mod pathfinding;
pub mod tech;

use serde::{Deserialize, Serialize};
use map::GameMap;
use pathfinding::{PathFinder, UnitOrder};
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
    pub game_map: GameMap,
    pub tech_tree: TechTree,
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
            game_map,
            tech_tree: TechTree::new(),
        }
    }

    pub fn update(&mut self, dt_secs: f64) {
        self.sol += dt_secs * 0.05; // 1 Sol every ~20 seconds

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
