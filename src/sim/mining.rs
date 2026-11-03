//! Authentic Outpost 2 Mining & Resource Logistics Simulation
//!
//! Models mineral deposits, automated drilling extraction by Robo-Miners,
//! and continuous freight shuttle routes by Cargo Trucks between Mines and Smelters.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OreType {
    Common,
    Rare,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OreDeposit {
    pub id: u32,
    pub ore_type: OreType,
    pub tile_x: i32,
    pub tile_y: i32,
    pub remaining_amount: u32,
    pub yield_per_cycle: u32,
    pub has_miner: bool,
}

impl OreDeposit {
    pub fn new(id: u32, ore_type: OreType, tile_x: i32, tile_y: i32, amount: u32) -> Self {
        Self {
            id,
            ore_type,
            tile_x,
            tile_y,
            remaining_amount: amount,
            yield_per_cycle: 50,
            has_miner: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HaulerState {
    Idle,
    DrivingToMine { deposit_id: u32 },
    Loading { deposit_id: u32, progress_pct: u32 },
    DrivingToSmelter { smelter_id: u32, ore_type: OreType, amount: u32 },
    Unloading { smelter_id: u32, ore_type: OreType, amount: u32, progress_pct: u32 },
}
