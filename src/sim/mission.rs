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
    pub scenario_id: usize,
    pub codename: String,
    pub faction: Faction,
    pub briefing: String,
    pub objectives: Vec<Objective>,
    pub status: MissionStatus,
}

impl Mission {
    pub fn new_eden_mission_1() -> Self {
        Self {
            scenario_id: 0,
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

    pub fn new_plymouth_mission_1() -> Self {
        Self {
            scenario_id: 1,
            codename: "Operation Badland Vanguard — Sector Plymouth South".into(),
            faction: Faction::Plymouth,
            briefing: "Commanders of Plymouth: Rejecting Eden's authoritarian technocracy, our settlers carved a home in the jagged badland trenches. Eden scout patrols are encroaching on our geothermal vents.\n\nConstruct a Vehicle Factory, establish automated mining convoys, assemble a strike force of 3 combat tanks, and eliminate Eden raiders to secure our sovereign territory.".into(),
            objectives: vec![
                Objective {
                    id: 1,
                    description: "Commission a Vehicle Factory to manufacture armored combat chassis".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 2,
                    description: "Assemble a strike squadron of at least 3 combat units".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 3,
                    description: "Stockpile at least 2,000t Common Ore".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 4,
                    description: "Protect Plymouth Command Center from destruction".into(),
                    is_completed: true,
                    is_primary: true,
                },
            ],
            status: MissionStatus::InProgress,
        }
    }

    pub fn new_tutorial_mission() -> Self {
        Self {
            scenario_id: 2,
            codename: "Cadet Proving Grounds — Academy Training Simulation".into(),
            faction: Faction::Eden,
            briefing: "Cadet: Welcome to the New Terra Colonial Operations Simulator. In this trial, you will master the fundamentals of planetary colony management.\n\nVerify that base power generation exceeds consumption, deploy pressurized transport tubes between facilities, and oversee automated mineral transport to the smelter.".into(),
            objectives: vec![
                Objective {
                    id: 1,
                    description: "Establish positive energy surplus (Power Produced > Consumed)".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 2,
                    description: "Connect structures with at least 15 pressurized transport tubes".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 3,
                    description: "Haul first freight load of Common Ore to the Smelter".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 4,
                    description: "Maintain colonist morale above 90%".into(),
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
        power_produced: u32,
        power_consumed: u32,
        tubes_count: usize,
        morale_pct: u32,
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

        match self.scenario_id {
            0 => {
                // Eden Mission 1
                let ore_done = common_ore >= 3000 && rare_ore >= 600;
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
                    obj.is_completed = ore_done;
                }

                let armed_units_count = units.iter()
                    .filter(|u| u.faction == Faction::Eden && u.weapon.is_some())
                    .count();
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
                    obj.is_completed = armed_units_count >= 2;
                }

                let research_done = tech_tree.techs.iter().any(|t| t.is_researched);
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
                    obj.is_completed = research_done;
                }
            }
            1 => {
                // Plymouth Mission 1
                let has_factory = buildings.iter().any(|b| b.name.contains("Vehicle Factory"));
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
                    obj.is_completed = has_factory;
                }

                let armed_units_count = units.iter()
                    .filter(|u| u.faction == Faction::Plymouth && u.weapon.is_some())
                    .count();
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
                    obj.is_completed = armed_units_count >= 3;
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
                    obj.is_completed = common_ore >= 2000;
                }
            }
            2 => {
                // Tutorial
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
                    obj.is_completed = power_produced > power_consumed;
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
                    obj.is_completed = tubes_count >= 15;
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
                    obj.is_completed = common_ore >= 2600; // Hauled additional ore beyond start
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 4) {
                    obj.is_completed = morale_pct >= 90;
                }
            }
            _ => {}
        }

        // Check Victory
        let all_primaries_met = self.objectives.iter()
            .filter(|o| o.is_primary)
            .all(|o| o.is_completed);

        if all_primaries_met {
            self.status = MissionStatus::Victory;
            Some("MISSION OBJECTIVES ACCOMPLISHED: Sector objectives secured!")
        } else {
            None
        }
    }
}
