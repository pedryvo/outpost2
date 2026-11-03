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

#[allow(dead_code)]
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

    pub fn new_eden_mission_2() -> Self {
        Self {
            scenario_id: 3,
            codename: "Operation Caldera Bastion — Molten Fissure Containment".into(),
            faction: Faction::Eden,
            briefing: "Eden Command Alert: Subsurface thermal sensors register catastrophic magma displacement along the northern fault lines. Molten fissures are rupturing the permafrost, threatening colonial survival dome sectors.\n\nStabilize base power generation above 400 kW using high-output arrays or geothermal conduits, construct a fortified perimeter of at least 2 defensive Guard Post Turrets overlooking fissure corridors, stockpile 2,500t Common Ore and 500t Rare Ore for emergency thermal barricades, and ensure the Eden Command Center remains secure from seismic fissures and hostile bombardment.".into(),
            objectives: vec![
                Objective {
                    id: 1,
                    description: "Establish geothermal containment power grid (Generate at least 400 kW power)".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 2,
                    description: "Construct at least 2 defensive Guard Post Turrets guarding the fissure rim".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 3,
                    description: "Stockpile 2,500t Common Ore and 500t Rare Ore for emergency reinforcement".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 4,
                    description: "Protect the colony Command Center from seismic or hostile annihilation".into(),
                    is_completed: true,
                    is_primary: true,
                },
                Objective {
                    id: 5,
                    description: "Secondary: Maintain colonial population morale at or above 80%".into(),
                    is_completed: false,
                    is_primary: false,
                },
                Objective {
                    id: 6,
                    description: "Secondary: Complete scientific research in any advanced discipline".into(),
                    is_completed: false,
                    is_primary: false,
                },
            ],
            status: MissionStatus::InProgress,
        }
    }

    pub fn new_plymouth_mission_2() -> Self {
        Self {
            scenario_id: 4,
            codename: "Operation Iron Fang — Smelter Sabotage & Ambush".into(),
            faction: Faction::Plymouth,
            briefing: "Plymouth High Council Direct Action Dispatch: Eden technocrats have entrenched a heavily armed mineral extraction and smelting complex directly bordering our southern badland canyon passes, choking off Plymouth's mineral lifeline.\n\nAssemble an armored ambush strike squadron of at least 4 combat tanks equipped with railguns or lasers, erect a fortified Guard Post Turret to control the canyon choke point, hijack and stockpile at least 3,500t Common Ore and 800t Rare Ore, and defend the Plymouth Command Center against Eden's automated retaliatory counter-assault.".into(),
            objectives: vec![
                Objective {
                    id: 1,
                    description: "Assemble an armored ambush strike squad of at least 4 armed combat tanks".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 2,
                    description: "Fortify the ambush choke point with at least 1 Guard Post Turret".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 3,
                    description: "Stockpile at least 3,500t Common Ore and 800t Rare Ore".into(),
                    is_completed: false,
                    is_primary: true,
                },
                Objective {
                    id: 4,
                    description: "Protect Plymouth Command Center from Eden counter-offensive strikes".into(),
                    is_completed: true,
                    is_primary: true,
                },
                Objective {
                    id: 5,
                    description: "Secondary: Maintain an operational Vehicle Factory for armored chassis production".into(),
                    is_completed: false,
                    is_primary: false,
                },
                Objective {
                    id: 6,
                    description: "Secondary: Sustain positive power generation surplus across the tactical grid".into(),
                    is_completed: false,
                    is_primary: false,
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
            3 => {
                // Eden Mission 2: Molten Fissure Containment
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
                    obj.is_completed = power_produced >= 400;
                }

                let guard_posts = buildings.iter()
                    .filter(|b| b.name.contains("Guard") && b.is_active)
                    .count();
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
                    obj.is_completed = guard_posts >= 2;
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
                    obj.is_completed = common_ore >= 2500 && rare_ore >= 500;
                }

                // Objective 4 is Command Center protection (checked above)

                // Secondary 5: Colony Morale >= 80%
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 5) {
                    obj.is_completed = morale_pct >= 80;
                }

                // Secondary 6: Research completed
                let research_done = tech_tree.techs.iter().any(|t| t.is_researched);
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 6) {
                    obj.is_completed = research_done;
                }
            }
            4 => {
                // Plymouth Mission 2: Smelter Sabotage & Ambush
                let armed_units_count = units.iter()
                    .filter(|u| u.faction == Faction::Plymouth && u.weapon.is_some())
                    .count();
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
                    obj.is_completed = armed_units_count >= 4;
                }

                let guard_posts = buildings.iter()
                    .filter(|b| b.name.contains("Guard") && b.is_active)
                    .count();
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
                    obj.is_completed = guard_posts >= 1;
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
                    obj.is_completed = common_ore >= 3500 && rare_ore >= 800;
                }

                // Objective 4 is Command Center protection (checked above)

                // Secondary 5: Operational Vehicle Factory
                let has_factory = buildings.iter().any(|b| b.name.contains("Vehicle Factory") && b.is_active);
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 5) {
                    obj.is_completed = has_factory;
                }

                // Secondary 6: Positive Power Surplus
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 6) {
                    obj.is_completed = power_produced > power_consumed;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::combat::WeaponType;

    #[test]
    fn test_eden_mission_2_initialization() {
        let m = Mission::new_eden_mission_2();
        assert_eq!(m.scenario_id, 3);
        assert_eq!(m.faction, Faction::Eden);
        assert_eq!(m.status, MissionStatus::InProgress);
        assert_eq!(m.objectives.len(), 6);
        let primaries = m.objectives.iter().filter(|o| o.is_primary).count();
        let secondaries = m.objectives.iter().filter(|o| !o.is_primary).count();
        assert_eq!(primaries, 4);
        assert_eq!(secondaries, 2);
    }

    #[test]
    fn test_plymouth_mission_2_initialization() {
        let m = Mission::new_plymouth_mission_2();
        assert_eq!(m.scenario_id, 4);
        assert_eq!(m.faction, Faction::Plymouth);
        assert_eq!(m.status, MissionStatus::InProgress);
        assert_eq!(m.objectives.len(), 6);
        let primaries = m.objectives.iter().filter(|o| o.is_primary).count();
        let secondaries = m.objectives.iter().filter(|o| !o.is_primary).count();
        assert_eq!(primaries, 4);
        assert_eq!(secondaries, 2);
    }

    #[test]
    fn test_eden_mission_2_victory_evaluation() {
        let mut m = Mission::new_eden_mission_2();
        let buildings = vec![
            Building {
                id: 1,
                name: "Command Center".into(),
                tile_x: 10,
                tile_y: 10,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
            },
            Building {
                id: 2,
                name: "Guard Post Turret #1".into(),
                tile_x: 15,
                tile_y: 10,
                width: 1,
                height: 1,
                power_cost: 15,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
            },
            Building {
                id: 3,
                name: "Guard Post Turret #2".into(),
                tile_x: 20,
                tile_y: 10,
                width: 1,
                height: 1,
                power_cost: 15,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
            },
        ];
        let units = Vec::new();
        let tech_tree = TechTree::new();

        // Not yet enough power or ore
        let res = m.evaluate(1000, 200, 300, 100, 10, 85, &buildings, &units, &tech_tree);
        assert!(res.is_none());
        assert_eq!(m.status, MissionStatus::InProgress);

        // Meet primary objectives
        let res = m.evaluate(2500, 500, 450, 100, 10, 85, &buildings, &units, &tech_tree);
        assert!(res.is_some());
        assert_eq!(m.status, MissionStatus::Victory);
    }

    #[test]
    fn test_plymouth_mission_2_victory_evaluation() {
        let mut m = Mission::new_plymouth_mission_2();
        let buildings = vec![
            Building {
                id: 1,
                name: "Command Center".into(),
                tile_x: 10,
                tile_y: 10,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
            },
            Building {
                id: 2,
                name: "Guard Post Turret".into(),
                tile_x: 15,
                tile_y: 10,
                width: 1,
                height: 1,
                power_cost: 15,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
            },
            Building {
                id: 3,
                name: "Vehicle Factory".into(),
                tile_x: 20,
                tile_y: 10,
                width: 3,
                height: 3,
                power_cost: 60,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
            },
        ];
        let units = (1..=4).map(|i| Unit {
            id: i,
            name: format!("Lynx #{}", i),
            faction: Faction::Plymouth,
            tile_x: i as i32 * 2,
            tile_y: 12,
            hp: 100,
            max_hp: 100,
            is_selected: false,
            current_order: crate::sim::pathfinding::UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon: Some(WeaponType::Railgun),
            attack_cooldown: 0.0,
            cargo: None,
            hauler_state: None,
            cargo_kit: None,
        }).collect::<Vec<_>>();
        let tech_tree = TechTree::new();

        // Not yet enough ore
        let res = m.evaluate(2000, 500, 200, 100, 10, 85, &buildings, &units, &tech_tree);
        assert!(res.is_none());
        assert_eq!(m.status, MissionStatus::InProgress);

        // Meet primary objectives
        let res = m.evaluate(3500, 800, 200, 100, 10, 85, &buildings, &units, &tech_tree);
        assert!(res.is_some());
        assert_eq!(m.status, MissionStatus::Victory);
    }
}
