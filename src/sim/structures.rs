//! Colonial Structures, Kits, and Pressurized Tube Network
//!
//! In Outpost 2, structures must be physically connected via pressurized transport
//! tubes to receive power, life support, and colony resource transit from the Command Center.

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructureType {
    CommandCenter,
    SolarArray,
    TokamakReactor,
    CommonOreSmelter,
    RareOreSmelter,
    DirtLab,
    StandardLab,
    Residence,
    Agrodome,
    VehicleFactory,
}

impl StructureType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::CommandCenter => "Command Center",
            Self::SolarArray => "Solar Power Array",
            Self::TokamakReactor => "Tokamak Fusion Reactor",
            Self::CommonOreSmelter => "Common Ore Smelter",
            Self::RareOreSmelter => "Rare Ore Smelter",
            Self::DirtLab => "D.I.R.T. Laboratory",
            Self::StandardLab => "Standard Research Lab",
            Self::Residence => "Colonial Residence",
            Self::Agrodome => "Bio-Dome Hydroponics",
            Self::VehicleFactory => "Vehicle Factory",
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        match self {
            Self::CommandCenter => (3, 3),
            Self::SolarArray => (2, 2),
            Self::TokamakReactor => (3, 3),
            Self::VehicleFactory => (3, 3),
            _ => (2, 2),
        }
    }

    pub fn power_cost(&self) -> i32 {
        match self {
            Self::CommandCenter => 40,
            Self::SolarArray => -250,      // Produces solar power
            Self::TokamakReactor => -800,   // Produces massive fusion power
            Self::CommonOreSmelter => 50,
            Self::RareOreSmelter => 75,
            Self::DirtLab => 60,
            Self::StandardLab => 90,
            Self::Residence => 20,
            Self::Agrodome => 30,
            Self::VehicleFactory => 120,
        }
    }

    pub fn ore_cost(&self) -> (u32, u32) {
        // (common_ore, rare_ore)
        match self {
            Self::CommandCenter => (500, 100),
            Self::SolarArray => (200, 0),
            Self::TokamakReactor => (600, 250),
            Self::CommonOreSmelter => (350, 50),
            Self::RareOreSmelter => (400, 150),
            Self::DirtLab => (300, 100),
            Self::StandardLab => (500, 200),
            Self::Residence => (150, 0),
            Self::Agrodome => (200, 50),
            Self::VehicleFactory => (500, 150),
        }
    }
}

/// Computes straight-line / orthogonal tube conduit paths between two points
pub fn connect_with_tubes(tubes: &mut HashSet<(i32, i32)>, start: (i32, i32), end: (i32, i32)) {
    let mut curr_x = start.0;
    let mut curr_y = start.1;

    // First align horizontally (X axis)
    while curr_x != end.0 {
        tubes.insert((curr_x, curr_y));
        if curr_x < end.0 {
            curr_x += 1;
        } else {
            curr_x -= 1;
        }
    }

    // Then align vertically (Y axis)
    while curr_y != end.1 {
        tubes.insert((curr_x, curr_y));
        if curr_y < end.1 {
            curr_y += 1;
        } else {
            curr_y -= 1;
        }
    }

    tubes.insert((end.0, end.1));
}
