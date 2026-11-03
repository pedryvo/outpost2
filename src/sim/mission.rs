//! Authentic Outpost 2 Campaign Scenario & Mission Objectives Engine
//!
//! Defines mission briefings, tracks primary & secondary objectives,
//! and evaluates live victory / evacuation defeat conditions.

use serde::{Deserialize, Serialize};
use super::{Building, Faction, Unit, tech::TechTree};
pub use crate::sim::events::ObjectiveStatus;

/// Alias for campaign scenario mission state.
#[allow(dead_code)]
pub type MissionState = Mission;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionStatus {
    InProgress,
    Victory,
    Defeat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Objective {
    pub id: u32,
    pub description: String,
    pub is_completed: bool,
    pub is_primary: bool,
    #[serde(default)]
    pub status: ObjectiveStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 2,
                    description: "Deploy and commission at least 2 armed combat tanks".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 3,
                    description: "Complete scientific research in any primary discipline".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 4,
                    description: "Protect the colony Command Center from hostile destruction".into(),
                    is_completed: true,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
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
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 2,
                    description: "Assemble a strike squadron of at least 3 combat units".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 3,
                    description: "Stockpile at least 2,000t Common Ore".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 4,
                    description: "Protect Plymouth Command Center from destruction".into(),
                    is_completed: true,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
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
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 2,
                    description: "Connect structures with at least 15 pressurized transport tubes".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 3,
                    description: "Haul first freight load of Common Ore to the Smelter".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 4,
                    description: "Maintain colonist morale above 90%".into(),
                    is_completed: true,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
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
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 2,
                    description: "Construct at least 2 defensive Guard Post Turrets guarding the fissure rim".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 3,
                    description: "Stockpile 2,500t Common Ore and 500t Rare Ore for emergency reinforcement".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 4,
                    description: "Protect the colony Command Center from seismic or hostile annihilation".into(),
                    is_completed: true,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 5,
                    description: "Secondary: Maintain colonial population morale at or above 80%".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 6,
                    description: "Secondary: Complete scientific research in any advanced discipline".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
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
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 2,
                    description: "Fortify the ambush choke point with at least 1 Guard Post Turret".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 3,
                    description: "Stockpile at least 3,500t Common Ore and 800t Rare Ore".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 4,
                    description: "Protect Plymouth Command Center from Eden counter-offensive strikes".into(),
                    is_completed: true,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 5,
                    description: "Secondary: Maintain an operational Vehicle Factory for armored chassis production".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 6,
                    description: "Secondary: Sustain positive power generation surplus across the tactical grid".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
                },
            ],
            status: MissionStatus::InProgress,
        }
    }

    pub fn new_eden_mission_3() -> Self {
        Self {
            scenario_id: 5,
            codename: "Starship Evacuation: Escape From Mount Vulcan".into(),
            faction: Faction::Eden,
            briefing: "Eden High Command Emergency Broadcast: Mount Vulcan's caldera is undergoing an apocalyptic supervolcanic eruption sequence. Molten fissures are severing the southern approaches, while desperate Plymouth raider divisions converge on our launch coordinates to seize our evacuation craft.\n\nCommission the Starship launch facility, fabricate all four primary evacuation starship modules (Command Module, Cryo Stasis Bay, Habitat Ring, and Ion Drive), transport at least 150 colonists via Evacuation Transports into the stasis berths, and hold the perimeter until orbital escape velocity is achieved.".into(),
            objectives: vec![
                Objective {
                    id: 1,
                    description: "Assemble Starship modules (Command Module, Cryo Stasis Bay, Habitat Ring, Ion Drive)".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 2,
                    description: "Evacuate at least 150 colonists via Evacuation Transports".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 3,
                    description: "Defend colony against Plymouth raider divisions and southern magma fissure spread".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 4,
                    description: "Protect the Eden Command Center from destruction".into(),
                    is_completed: true,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 5,
                    description: "Secondary: Maintain colonial morale above 85% during evacuation countdown".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 6,
                    description: "Secondary: Stockpile 3,000t Common Ore and 1,000t Rare Ore for emergency fuel reserves".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
                },
            ],
            status: MissionStatus::InProgress,
        }
    }

    pub fn new_plymouth_mission_3() -> Self {
        Self {
            scenario_id: 6,
            codename: "Operation Phoenix: Obsidian Crags Assault".into(),
            faction: Faction::Plymouth,
            briefing: "Plymouth Council of Free Settlers Dispatch: The terraforming atmospheric processor in the Obsidian Crags represents the final hope for breathable atmosphere on New Terra. However, Eden's fortified Obsidian Citadel has locked down the sector, and microscopic nanite blight contagion is propagating towards our processor vents.\n\nDeploy heavy assault tanks, safeguard the Atmospheric Terraformer from nanite contagion until 100% saturation is achieved, and breach the fortified Eden Citadel to neutralize their defensive guard posts and Tiger railgun divisions.".into(),
            objectives: vec![
                Objective {
                    id: 1,
                    description: "Protect Atmospheric Terraformer from spreading nanite blight contagion".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 2,
                    description: "Breaching assault against the fortified Eden Obsidian Citadel".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 3,
                    description: "Achieve 100% Terraformer saturation".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 4,
                    description: "Protect Plymouth Command Center from destruction".into(),
                    is_completed: true,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 5,
                    description: "Secondary: Commission at least 3 heavy combat tanks (Tiger or Panther chassis)".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 6,
                    description: "Secondary: Sustain continuous geothermal power generation above 450 kW".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
                },
            ],
            status: MissionStatus::InProgress,
        }
    }

    /// Free-form procedural multiplayer or skirmish scenario vs AI commanders
    pub fn new_skirmish(faction: Faction) -> Self {
        Self {
            scenario_id: 100,
            codename: "Planetary Skirmish: Divided Destiny".into(),
            faction,
            briefing: "Colony command alert: Tactical skirmish zone established on New Terra. Scout the canyon passes, secure central mineral caches, harness geothermal energy, and neutralize hostile rival command centers.".into(),
            objectives: vec![
                Objective {
                    id: 1,
                    description: "Maintain Command Center integrity".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 2,
                    description: "Neutralize all hostile rival Command Centers".into(),
                    is_completed: false,
                    is_primary: true,
                    status: ObjectiveStatus::Active,
                },
                Objective {
                    id: 3,
                    description: "Secondary: Establish geothermal baseload energy (>= 450 kW)".into(),
                    is_completed: false,
                    is_primary: false,
                    status: ObjectiveStatus::Active,
                },
            ],
            status: MissionStatus::InProgress,
        }
    }

    /// Modify objective status programmatically from script triggers
    pub fn modify_objective(&mut self, objective_id: u32, status: ObjectiveStatus) -> bool {
        if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == objective_id) {
            obj.status = status;
            obj.is_completed = status == ObjectiveStatus::Completed;
            if status == ObjectiveStatus::Failed && obj.is_primary {
                self.status = MissionStatus::Defeat;
            }
            true
        } else {
            false
        }
    }

    /// Query the current status of an objective by ID
    pub fn get_objective_status(&self, objective_id: u32) -> Option<ObjectiveStatus> {
        self.objectives.iter().find(|o| o.id == objective_id).map(|o| o.status)
    }

    /// Advance dynamic mission scripting events against colony state
    pub fn tick_events(&mut self, engine: &mut crate::sim::events::EventEngine, session: &mut crate::sim::GameSession) {
        engine.update(session);
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
        starship_progress: f64,
        evacuated_colonists: u32,
        terraformer_saturation: f64,
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
            5 => {
                // Eden Mission 3: Starship Evacuation: Escape From Mount Vulcan
                let starship_done = starship_progress >= 100.0;
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
                    obj.is_completed = starship_done;
                }

                let evac_done = evacuated_colonists >= 150;
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
                    obj.is_completed = evac_done;
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
                    obj.is_completed = starship_done && evac_done;
                }

                // Obj 4: Command Center protected (checked above)

                // Secondary 5: Colony Morale >= 85%
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 5) {
                    obj.is_completed = morale_pct >= 85;
                }

                // Secondary 6: Stockpile 3,000t Common Ore and 1,000t Rare Ore
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 6) {
                    obj.is_completed = common_ore >= 3000 && rare_ore >= 1000;
                }
            }
            6 => {
                // Plymouth Mission 3: Operation Phoenix: Obsidian Crags Assault
                let has_terraformer = buildings.iter().any(|b| b.name.contains("Terraformer") && b.is_active);
                if !has_terraformer {
                    self.status = MissionStatus::Defeat;
                    return Some("MISSION FAILURE: Atmospheric Terraformer has been contaminated or destroyed!");
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
                    obj.is_completed = has_terraformer;
                }

                // Neutralize enemy Eden Obsidian Citadel structures
                let eden_citadel_remaining = buildings.iter()
                    .filter(|b| b.faction == Some(Faction::Eden) && (b.name.contains("Citadel") || b.name.contains("Command Center")))
                    .count();
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
                    obj.is_completed = eden_citadel_remaining == 0;
                }

                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
                    obj.is_completed = terraformer_saturation >= 100.0;
                }

                // Obj 4: Command Center protected (checked above)

                // Secondary 5: Deploy at least 3 heavy combat tanks
                let heavy_tanks = units.iter()
                    .filter(|u| u.faction == Faction::Plymouth && (u.name.contains("Tiger") || u.name.contains("Panther")))
                    .count();
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 5) {
                    obj.is_completed = heavy_tanks >= 3;
                }

                // Secondary 6: Sustain continuous geothermal energy generation >= 450 kW
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 6) {
                    obj.is_completed = power_produced >= 450;
                }
            }
            100 => {
                // Procedural Skirmish Map
                // Obj 1: Command Center preserved
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 1) {
                    obj.is_completed = true;
                }
                // Obj 2: All enemy command centers destroyed
                let enemy_ccs = buildings.iter()
                    .filter(|b| b.faction.is_some() && b.faction != Some(self.faction) && b.name.contains("Command Center"))
                    .count();
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 2) {
                    obj.is_completed = enemy_ccs == 0;
                }
                // Obj 3: Geothermal power generation
                if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == 3) {
                    obj.is_completed = power_produced >= 450;
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
    fn test_eden_mission_3_initialization() {
        let m = Mission::new_eden_mission_3();
        assert_eq!(m.scenario_id, 5);
        assert_eq!(m.faction, Faction::Eden);
        assert_eq!(m.status, MissionStatus::InProgress);
        assert_eq!(m.codename, "Starship Evacuation: Escape From Mount Vulcan");
        assert_eq!(m.objectives.len(), 6);
        let primaries = m.objectives.iter().filter(|o| o.is_primary).count();
        let secondaries = m.objectives.iter().filter(|o| !o.is_primary).count();
        assert_eq!(primaries, 4);
        assert_eq!(secondaries, 2);
    }

    #[test]
    fn test_plymouth_mission_3_initialization() {
        let m = Mission::new_plymouth_mission_3();
        assert_eq!(m.scenario_id, 6);
        assert_eq!(m.faction, Faction::Plymouth);
        assert_eq!(m.status, MissionStatus::InProgress);
        assert_eq!(m.codename, "Operation Phoenix: Obsidian Crags Assault");
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
                faction: Some(Faction::Eden),
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
                faction: Some(Faction::Eden),
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
                faction: Some(Faction::Eden),
            },
        ];
        let units = Vec::new();
        let tech_tree = TechTree::new();

        // Not yet enough power or ore
        let res = m.evaluate(1000, 200, 300, 100, 10, 85, &buildings, &units, &tech_tree, 0.0, 0, 0.0);
        assert!(res.is_none());
        assert_eq!(m.status, MissionStatus::InProgress);

        // Meet primary objectives
        let res = m.evaluate(2500, 500, 450, 100, 10, 85, &buildings, &units, &tech_tree, 0.0, 0, 0.0);
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
                faction: Some(Faction::Plymouth),
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
                faction: Some(Faction::Plymouth),
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
                faction: Some(Faction::Plymouth),
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
        let res = m.evaluate(2000, 500, 200, 100, 10, 85, &buildings, &units, &tech_tree, 0.0, 0, 0.0);
        assert!(res.is_none());
        assert_eq!(m.status, MissionStatus::InProgress);

        // Meet primary objectives
        let res = m.evaluate(3500, 800, 200, 100, 10, 85, &buildings, &units, &tech_tree, 0.0, 0, 0.0);
        assert!(res.is_some());
        assert_eq!(m.status, MissionStatus::Victory);
    }

    #[test]
    fn test_eden_mission_3_victory_evaluation() {
        let mut m = Mission::new_eden_mission_3();
        let buildings = vec![
            Building {
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
                faction: Some(Faction::Eden),
            },
            Building {
                id: 2,
                name: "Starship Launch Gantry".into(),
                tile_x: 15,
                tile_y: 20,
                width: 3,
                height: 3,
                power_cost: 160,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(Faction::Eden),
            },
        ];
        let units = Vec::new();
        let tech_tree = TechTree::new();

        // Starship not ready, evacuated 50
        let res = m.evaluate(2000, 400, 600, 200, 20, 90, &buildings, &units, &tech_tree, 75.0, 50, 0.0);
        assert!(res.is_none());
        assert_eq!(m.status, MissionStatus::InProgress);

        // Starship 100%, evacuated 150
        let res = m.evaluate(3200, 1100, 800, 200, 20, 92, &buildings, &units, &tech_tree, 100.0, 150, 0.0);
        assert!(res.is_some());
        assert_eq!(m.status, MissionStatus::Victory);
    }

    #[test]
    fn test_plymouth_mission_3_victory_evaluation() {
        let mut m = Mission::new_plymouth_mission_3();
        let buildings = vec![
            Building {
                id: 1,
                name: "Command Center".into(),
                tile_x: 18,
                tile_y: 22,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(Faction::Plymouth),
            },
            Building {
                id: 2,
                name: "Atmospheric Terraformer".into(),
                tile_x: 22,
                tile_y: 18,
                width: 3,
                height: 3,
                power_cost: 100,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(Faction::Plymouth),
            },
            // Enemy Citadel still standing
            Building {
                id: 10,
                name: "Eden Obsidian Citadel".into(),
                tile_x: 48,
                tile_y: 20,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(Faction::Eden),
            },
        ];
        let units = Vec::new();
        let tech_tree = TechTree::new();

        // Terraformer 80%, Citadel still standing
        let res = m.evaluate(1500, 300, 500, 140, 15, 90, &buildings, &units, &tech_tree, 0.0, 0, 80.0);
        assert!(res.is_none());
        assert_eq!(m.status, MissionStatus::InProgress);

        // Remove Citadel (dismantled) and achieve 100% saturation
        let buildings_citadel_destroyed = vec![
            buildings[0].clone(),
            buildings[1].clone(),
        ];
        let res = m.evaluate(1500, 300, 500, 140, 15, 90, &buildings_citadel_destroyed, &units, &tech_tree, 0.0, 0, 100.0);
        assert!(res.is_some());
        assert_eq!(m.status, MissionStatus::Victory);
    }

    #[test]
    fn test_plymouth_mission_3_terraformer_destroyed_defeat() {
        let mut m = Mission::new_plymouth_mission_3();
        let buildings = vec![
            Building {
                id: 1,
                name: "Command Center".into(),
                tile_x: 18,
                tile_y: 22,
                width: 3,
                height: 3,
                power_cost: 40,
                is_active: true,
                is_selected: false,
                production_queue: Vec::new(),
                production_progress: 0.0,
                attack_cooldown: 0.0,
                faction: Some(Faction::Plymouth),
            },
            // No Terraformer!
        ];
        let units = Vec::new();
        let tech_tree = TechTree::new();

        let res = m.evaluate(1500, 300, 500, 140, 15, 90, &buildings, &units, &tech_tree, 0.0, 0, 50.0);
        assert!(res.is_some());
        assert_eq!(m.status, MissionStatus::Defeat);
    }
}
