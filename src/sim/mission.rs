//! Authentic Outpost 2 Campaign Scenario & Mission Objectives Engine
//!
//! Defines mission briefings, tracks primary & secondary objectives,
//! and evaluates live victory / evacuation defeat conditions.

use serde::{Deserialize, Serialize};
use super::{Building, Faction, Unit, tech::TechTree};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionStatus {
    InProgress,
    Victory,
    Defeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub id: u32,
    pub description: String,
    pub is_completed: bool,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub codename: String,
    pub faction: Faction,
    pub briefing: String,
    pub objectives: Vec<Objective>,
    pub status: MissionStatus,
}

impl Mission {
    pub fn new_eden_mission_1() -> Self {
        Self {
            codename: "Operation New Dawn — Sector Eden Prime".into(),
            faction: Faction::Eden,
            briefing: "Colonists of the Eden Foundation: After the catastrophic failure of the Starship Conestoga, our landing site on New Terra must be secured immediately. Volcanic tremors indicate rising magma fissures nearby, and rogue Plymouth raiders have established aggressive perimeter posts.\n\nSecure colonial life support, automate freight ore hauling, commission defensive railgun armor, and advance planetary research to ensure the survival of our species.".into(),
            objectives: vec![
                Objective {
                    id: 1,
                    description: "Stockpile 3,000t Common Ore and 600t Rare Ore".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 2,
                    description: "Deploy and commission at least 2 armed combat tanks".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 3,
                    description: "Complete scientific research in any primary discipline".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 4,
                    description: "Protect the colony Command Center from hostile destruction".into(),
                    is_completed: true,
                    is_primary: true,
                },
            ],
            status: MissionStatus::InProgress,
        }
    }

    /// Evaluate live mission objectives against colony telemetry
    pub fn evaluate(
        &mut self,
        common_ore: u32,
        rare_ore: u32,
        buildings: &[Building],
        units: &[Unit],
        tech_tree: &TechTree,
    ) -> Option<&'static str> {
        if self.status != MissionStatus::InProgress {
            return None;
        }

        // Defeat condition: Command Center destroyed
        let has_cc = buildings.iter().any(|b| b.name.contains("Command Center"));
        if !has_cc {
            self.status = MissionStatus::Defeat;
            return Some("MISSION FAILURE: Command Center has been obliterated!");
        }

        // Objective 1: Ore stockpiles
        let ore_done = common_ore >= 3000 && rare_ore >= 600;
        if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
            obj.is_completed = ore_done;
        }

        // Objective 2: Combat tanks
        let armed_units_count = units.iter()
            .filter(|u| u.faction == Faction::Eden && u.weapon.is_some())
            .count();
        let tanks_done = armed_units_count >= 2;
        if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
            obj.is_completed = tanks_done;
        }

        // Objective 3: Research
        let research_done = tech_tree.techs.iter().any(|t| t.is_researched);
        if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
            obj.is_completed = research_done;
        }

        // Check Victory
        let all_primaries_met = self.objectives.iter()
            .filter(|o| o.is_primary)
            .all(|o| o.is_completed);

        if all_primaries_met {
            self.status = MissionStatus::Victory;
            Some("MISSION OBJECTIVES ACCOMPLISHED: Sector Eden Prime secured!")
        } else {
            None
        }
    }
}
