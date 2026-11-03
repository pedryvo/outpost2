//! Outpost 2 Mission Scripting & Dynamic Event Engine
//!
//! Provides a reactive trigger-condition-action system for tactical campaigns,
//! emergency colonial dispatches, hostile ambush script sequencing, natural disaster
//! occurrences, and dynamic objective modifications.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::sim::{
    combat::WeaponType,
    mining::HaulerState,
    pathfinding::UnitOrder,
    ColonyState, Faction, Unit,
};

/// Type alias for the colonial simulation state used by scripting triggers.
pub type GameSession = ColonyState;

/// Identifier for scientific technology nodes.
pub type TechId = u32;

/// Priority classification for tactical dispatches and audio-visual alerts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Lifecycle status for campaign objectives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectiveStatus {
    Pending,
    Active,
    Completed,
    Failed,
}

impl Default for ObjectiveStatus {
    fn default() -> Self {
        ObjectiveStatus::Pending
    }
}

/// Unit chassis / classification types supported in dynamic event scripts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitType {
    CargoTruck,
    ConVec,
    RepairVehicle,
    Earthworker,
    Lynx,
    Panther,
    Tiger,
    RoboMiner,
    EvacuationTransport,
}

impl UnitType {
    /// Check whether a live simulation unit matches this unit classification.
    pub fn matches_unit(&self, unit: &Unit) -> bool {
        match self {
            UnitType::CargoTruck => unit.name.contains("Cargo"),
            UnitType::ConVec => unit.name.contains("ConVec"),
            UnitType::RepairVehicle => unit.name.contains("Repair"),
            UnitType::Earthworker => unit.name.contains("Earthworker"),
            UnitType::Lynx => unit.name.contains("Lynx"),
            UnitType::Panther => unit.name.contains("Panther"),
            UnitType::Tiger => unit.name.contains("Tiger"),
            UnitType::RoboMiner => unit.name.contains("Miner"),
            UnitType::EvacuationTransport => unit.name.contains("Evac") || unit.name.contains("Transport"),
        }
    }

    /// Identify unit classification from a unit name or designation string.
    pub fn from_name(name: &str) -> Option<Self> {
        if name.contains("Cargo") {
            Some(UnitType::CargoTruck)
        } else if name.contains("ConVec") {
            Some(UnitType::ConVec)
        } else if name.contains("Repair") {
            Some(UnitType::RepairVehicle)
        } else if name.contains("Earthworker") {
            Some(UnitType::Earthworker)
        } else if name.contains("Lynx") {
            Some(UnitType::Lynx)
        } else if name.contains("Panther") {
            Some(UnitType::Panther)
        } else if name.contains("Tiger") {
            Some(UnitType::Tiger)
        } else if name.contains("Miner") {
            Some(UnitType::RoboMiner)
        } else if name.contains("Evac") || name.contains("Transport") {
            Some(UnitType::EvacuationTransport)
        } else {
            None
        }
    }

    /// Canonical display designation for this unit type.
    pub fn display_name(&self) -> &'static str {
        match self {
            UnitType::CargoTruck => "Cargo Truck",
            UnitType::ConVec => "ConVec",
            UnitType::RepairVehicle => "Repair Vehicle",
            UnitType::Earthworker => "Earthworker",
            UnitType::Lynx => "Lynx Scout",
            UnitType::Panther => "Panther Battle Tank",
            UnitType::Tiger => "Tiger Assault Tank",
            UnitType::RoboMiner => "RoboMiner",
            UnitType::EvacuationTransport => "Evacuation Transport",
        }
    }

    /// Construct a fully initialized simulation unit instance of this type.
    pub fn spawn_unit(&self, id: u32, faction: Faction, x: i32, y: i32) -> Unit {
        let (name, hp, weapon, hauler_state) = match self {
            UnitType::Lynx => (
                format!("Lynx Scout #{}", id),
                140,
                Some(WeaponType::Laser),
                None,
            ),
            UnitType::Panther => (
                format!("Panther Battle Tank #{}", id),
                250,
                Some(WeaponType::Railgun),
                None,
            ),
            UnitType::Tiger => (
                format!("Tiger Assault Tank #{}", id),
                350,
                Some(WeaponType::Railgun),
                None,
            ),
            UnitType::CargoTruck => (
                format!("Cargo Truck #{}", id),
                150,
                None,
                Some(HaulerState::Idle),
            ),
            UnitType::ConVec => (
                format!("ConVec #{}", id),
                180,
                None,
                None,
            ),
            UnitType::RepairVehicle => (
                format!("Repair Vehicle #{}", id),
                180,
                None,
                None,
            ),
            UnitType::Earthworker => (
                format!("Earthworker #{}", id),
                200,
                None,
                None,
            ),
            UnitType::RoboMiner => (
                format!("RoboMiner #{}", id),
                160,
                None,
                None,
            ),
            UnitType::EvacuationTransport => (
                format!("Evacuation Transport #{}", id),
                220,
                None,
                None,
            ),
        };

        Unit {
            id,
            name,
            faction,
            tile_x: x,
            tile_y: y,
            hp,
            max_hp: hp,
            is_selected: false,
            current_order: UnitOrder::Idle,
            path: Vec::new(),
            move_cooldown: 0.0,
            weapon,
            attack_cooldown: 0.0,
            cargo: None,
            hauler_state,
            cargo_kit: None,
        }
    }
}

/// Planetary hazard and natural disaster types that can be triggered dynamically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisasterType {
    SeismicFissure,
    AtmosphericVortex,
    MeteorImpact,
    MicrobialBlight,
}

/// Evaluatable preconditions that trigger script actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerCondition {
    /// Triggers when colony Sol (or tick count) reaches or exceeds the specified threshold.
    SolTimer(u32),
    /// Triggers when any unit (or specific unit classification) is inside the bounding rectangle.
    UnitInArea {
        unit_type: Option<UnitType>,
        x_min: i32,
        y_min: i32,
        x_max: i32,
        y_max: i32,
    },
    /// Triggers when a structure with the specified building ID has been destroyed.
    StructureDestroyed(u32),
    /// Triggers when common/rare ore and/or food reserves meet or exceed target thresholds.
    ResourceThreshold {
        metal_min: Option<u32>,
        food_min: Option<u32>,
    },
    /// Triggers when cumulative colonial casualties reach or exceed the specified count.
    CasualtiesExceeded(u32),
    /// All nested conditions must evaluate to true.
    CompoundAnd(Vec<TriggerCondition>),
    /// At least one nested condition must evaluate to true.
    CompoundOr(Vec<TriggerCondition>),
}

impl TriggerCondition {
    /// Evaluates this condition against the current game session telemetry.
    pub fn evaluate(&self, session: &GameSession) -> bool {
        match self {
            TriggerCondition::SolTimer(threshold) => {
                session.sol >= *threshold as f64 || session.tick_count >= *threshold as u64
            }
            TriggerCondition::UnitInArea {
                unit_type,
                x_min,
                y_min,
                x_max,
                y_max,
            } => {
                let min_x = (*x_min).min(*x_max);
                let max_x = (*x_min).max(*x_max);
                let min_y = (*y_min).min(*y_max);
                let max_y = (*y_min).max(*y_max);

                session.units.iter().any(|u| {
                    u.hp > 0
                        && u.tile_x >= min_x
                        && u.tile_x <= max_x
                        && u.tile_y >= min_y
                        && u.tile_y <= max_y
                        && unit_type.map_or(true, |ut| ut.matches_unit(u))
                })
            }
            TriggerCondition::StructureDestroyed(building_id) => {
                session.destroyed_building_ids.contains(building_id)
                    || (session.initial_building_ids.contains(building_id)
                        && !session.buildings.iter().any(|b| b.id == *building_id))
            }
            TriggerCondition::ResourceThreshold {
                metal_min,
                food_min,
            } => {
                let metal_ok = match metal_min {
                    Some(min) => {
                        session.common_ore >= *min
                            || (session.common_ore + session.rare_ore) >= *min
                    }
                    None => true,
                };
                let food_ok = match food_min {
                    Some(min) => session.food_units >= *min,
                    None => true,
                };
                metal_ok && food_ok
            }
            TriggerCondition::CasualtiesExceeded(threshold) => {
                session.casualties >= *threshold
            }
            TriggerCondition::CompoundAnd(conditions) => {
                conditions.iter().all(|c| c.evaluate(session))
            }
            TriggerCondition::CompoundOr(conditions) => {
                conditions.iter().any(|c| c.evaluate(session))
            }
        }
    }
}

/// Tactical notification record stored in the event engine log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TacticalNotification {
    pub title: String,
    pub message: String,
    pub priority: NotificationPriority,
}

/// Actions dispatched when a script trigger fires.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventAction {
    /// Spawns a squadron of reinforcements at specified coordinates.
    SpawnReinforcements {
        faction: Faction,
        unit_type: UnitType,
        count: usize,
        x: i32,
        y: i32,
    },
    /// Triggers an immediate planetary hazard (fissure, vortex, meteor, blight).
    TriggerDisaster {
        disaster_type: DisasterType,
        x: i32,
        y: i32,
    },
    /// Immediately marks a technology researched in the tech tree.
    UnlockTechnology(TechId),
    /// Dispatches a tactical audio-visual advisory to the colony HUD.
    TacticalNotification {
        title: String,
        message: String,
        priority: NotificationPriority,
    },
    /// Updates the status and completion state of an active mission objective.
    ModifyObjective {
        objective_id: u32,
        status: ObjectiveStatus,
    },
    /// Queues a radio voice clip for audio playback.
    PlayVoiceClip(String),
}

impl EventAction {
    /// Executes the action against live game session state.
    pub fn execute(
        &self,
        session: &mut GameSession,
        notifications: &mut Vec<TacticalNotification>,
        pending_voice_clips: &mut Vec<String>,
    ) {
        match self {
            EventAction::SpawnReinforcements {
                faction,
                unit_type,
                count,
                x,
                y,
            } => {
                for i in 0..*count {
                    let next_id = session.next_unit_id();
                    let spawn_x = *x + (i as i32 % 3);
                    let spawn_y = *y + (i as i32 / 3);
                    let unit = unit_type.spawn_unit(next_id, *faction, spawn_x, spawn_y);
                    session.units.push(unit);
                }
            }
            EventAction::TriggerDisaster {
                disaster_type,
                x,
                y,
            } => match disaster_type {
                DisasterType::SeismicFissure => {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let tx = (*x + dx).clamp(0, session.game_map.width as i32 - 1) as u32;
                            let ty = (*y + dy).clamp(0, session.game_map.height as i32 - 1) as u32;
                            let idx = (ty * session.game_map.width + tx) as usize;
                            if idx < session.game_map.tiles.len() {
                                session.game_map.tiles[idx].is_lava = true;
                                session.disasters.active_lava_tiles.push((tx, ty));
                            }
                        }
                    }
                    session.disasters.quake_duration = 3.5;
                    session.disasters.eruption_count += 1;
                }
                DisasterType::AtmosphericVortex => {
                    session.disasters.active_vortices.push(
                        crate::sim::disasters::AtmosphericVortex {
                            x: *x as f64,
                            y: *y as f64,
                            vx: 1.5,
                            vy: 1.0,
                            radius: 3.5,
                            lifetime: 25.0,
                            max_lifetime: 25.0,
                        },
                    );
                }
                DisasterType::MeteorImpact => {
                    session.disasters.pending_meteors.push(
                        crate::sim::disasters::MeteorImpact {
                            x: *x as f64,
                            y: *y as f64,
                            delay: 0.5,
                            max_delay: 0.5,
                            radius: 3.5,
                            has_impacted: false,
                        },
                    );
                }
                DisasterType::MicrobialBlight => {
                    let tx = (*x).clamp(0, session.game_map.width as i32 - 1) as u32;
                    let ty = (*y).clamp(0, session.game_map.height as i32 - 1) as u32;
                    let idx = (ty * session.game_map.width + tx) as usize;
                    if idx < session.game_map.tiles.len() {
                        session.game_map.tiles[idx].is_blight = true;
                        session.disasters.active_blight_tiles.push((tx, ty));
                    }
                }
            },
            EventAction::UnlockTechnology(tech_id) => {
                if let Some(tech) = session.tech_tree.techs.iter_mut().find(|t| t.id == *tech_id) {
                    tech.is_researched = true;
                }
            }
            EventAction::TacticalNotification {
                title,
                message,
                priority,
            } => {
                notifications.push(TacticalNotification {
                    title: title.clone(),
                    message: message.clone(),
                    priority: *priority,
                });
            }
            EventAction::ModifyObjective {
                objective_id,
                status,
            } => {
                session.mission.modify_objective(*objective_id, *status);
            }
            EventAction::PlayVoiceClip(clip) => {
                pending_voice_clips.push(clip.clone());
            }
        }
    }
}

/// Script trigger record binding a condition to one or more actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptTrigger {
    pub id: u32,
    pub name: String,
    pub condition: TriggerCondition,
    pub actions: Vec<EventAction>,
    pub fired: bool,
    pub repeatable: bool,
    pub cooldown_ticks: u32,
    pub last_fired_tick: u32,
}

impl ScriptTrigger {
    /// Create a new one-shot script trigger.
    pub fn new(
        id: u32,
        name: impl Into<String>,
        condition: TriggerCondition,
        actions: Vec<EventAction>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            condition,
            actions,
            fired: false,
            repeatable: false,
            cooldown_ticks: 0,
            last_fired_tick: 0,
        }
    }

    /// Configure this trigger to be repeatable with a cooldown interval.
    pub fn with_cooldown(mut self, cooldown_ticks: u32) -> Self {
        self.repeatable = true;
        self.cooldown_ticks = cooldown_ticks;
        self
    }
}

/// Dynamic Event Engine managing active triggers and dispatches.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventEngine {
    pub triggers: Vec<ScriptTrigger>,
    pub notifications: Vec<TacticalNotification>,
    pub pending_voice_clips: Vec<String>,
    pub current_tick: u32,
}

impl EventEngine {
    /// Create a new empty EventEngine.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a script trigger to the engine.
    pub fn add_trigger(&mut self, trigger: ScriptTrigger) {
        self.triggers.push(trigger);
    }

    /// Retrieve references to all tactical notifications generated.
    pub fn notifications(&self) -> &[TacticalNotification] {
        &self.notifications
    }

    /// Drain and retrieve pending audio voice clips.
    pub fn drain_voice_clips(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending_voice_clips)
    }

    /// Advance the event engine tick, evaluate trigger conditions, and execute fired actions.
    pub fn update(&mut self, session: &mut GameSession) {
        self.current_tick = self.current_tick.wrapping_add(1);
        let now = self.current_tick;

        let mut actions_to_execute = Vec::new();

        for trigger in &mut self.triggers {
            let can_fire = if !trigger.fired {
                true
            } else if trigger.repeatable {
                now.saturating_sub(trigger.last_fired_tick) >= trigger.cooldown_ticks
            } else {
                false
            };

            if can_fire && trigger.condition.evaluate(session) {
                trigger.fired = true;
                trigger.last_fired_tick = now;
                actions_to_execute.extend(trigger.actions.clone());
            }
        }

        for action in actions_to_execute {
            action.execute(session, &mut self.notifications, &mut self.pending_voice_clips);
        }
    }

    /// Instantiate canonical campaign script triggers for the designated scenario.
    pub fn for_scenario(scenario_id: usize) -> Self {
        let mut engine = Self::new();

        match scenario_id {
            0 => {
                // Eden Mission 1: Operation New Dawn
                engine.add_trigger(ScriptTrigger::new(
                    101,
                    "Eden Prime Briefing Dispatch",
                    TriggerCondition::SolTimer(1),
                    vec![
                        EventAction::TacticalNotification {
                            title: "Eden High Council Dispatch".into(),
                            message: "Operation New Dawn active: Establish mineral hauling convoys and defend the perimeter against rogue Plymouth raiders.".into(),
                            priority: NotificationPriority::High,
                        },
                        EventAction::PlayVoiceClip("eden_01_briefing.wav".into()),
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    102,
                    "Hydrothermal Mineral Basin Discovery",
                    TriggerCondition::UnitInArea {
                        unit_type: None,
                        x_min: 30,
                        y_min: 15,
                        x_max: 45,
                        y_max: 30,
                    },
                    vec![
                        EventAction::TacticalNotification {
                            title: "Geological Survey Report".into(),
                            message: "Rich hydrothermal mineral vein detected in eastern canyon. Automated freight haulers advised.".into(),
                            priority: NotificationPriority::Medium,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    103,
                    "Plymouth Badland Ambush",
                    TriggerCondition::UnitInArea {
                        unit_type: None,
                        x_min: 42,
                        y_min: 10,
                        x_max: 58,
                        y_max: 25,
                    },
                    vec![
                        EventAction::SpawnReinforcements {
                            faction: Faction::Plymouth,
                            unit_type: UnitType::Lynx,
                            count: 2,
                            x: 52,
                            y: 15,
                        },
                        EventAction::TacticalNotification {
                            title: "Hostile Contact Alert".into(),
                            message: "Plymouth raiders emerging from northern canyon passes! Defensive railguns engage!".into(),
                            priority: NotificationPriority::Critical,
                        },
                        EventAction::PlayVoiceClip("red_alert.wav".into()),
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    104,
                    "Eden Emergency Reinforcements",
                    TriggerCondition::CasualtiesExceeded(2),
                    vec![
                        EventAction::SpawnReinforcements {
                            faction: Faction::Eden,
                            unit_type: UnitType::Panther,
                            count: 1,
                            x: 22,
                            y: 22,
                        },
                        EventAction::TacticalNotification {
                            title: "Eden Command Reinforcements".into(),
                            message: "Casualty threshold reached. Emergency Panther Battle Tank deployed to defend the Command Center.".into(),
                            priority: NotificationPriority::High,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    105,
                    "Ore Stockpile Quota Satisfied",
                    TriggerCondition::ResourceThreshold {
                        metal_min: Some(3000),
                        food_min: None,
                    },
                    vec![
                        EventAction::ModifyObjective {
                            objective_id: 1,
                            status: ObjectiveStatus::Completed,
                        },
                        EventAction::TacticalNotification {
                            title: "Resource Quota Secured".into(),
                            message: "3,000t Common Ore target accomplished for New Dawn survival.".into(),
                            priority: NotificationPriority::Medium,
                        },
                    ],
                ));
            }
            1 => {
                // Plymouth Mission 1: Operation Badland Vanguard
                engine.add_trigger(ScriptTrigger::new(
                    201,
                    "Plymouth Vanguard Dispatch",
                    TriggerCondition::SolTimer(1),
                    vec![
                        EventAction::TacticalNotification {
                            title: "Plymouth High Council".into(),
                            message: "Reject Eden tyranny. Commission a Vehicle Factory and build an armored strike squadron.".into(),
                            priority: NotificationPriority::High,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    202,
                    "Advanced Armor Fabrication Online",
                    TriggerCondition::UnitInArea {
                        unit_type: Some(UnitType::Panther),
                        x_min: 14,
                        y_min: 22,
                        x_max: 20,
                        y_max: 28,
                    },
                    vec![
                        EventAction::UnlockTechnology(4201),
                        EventAction::TacticalNotification {
                            title: "Railgun Upgrade Synchronized".into(),
                            message: "First battle tank commissioned. Kinetic railgun upgrades authorized.".into(),
                            priority: NotificationPriority::Medium,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    203,
                    "Eden Perimeter Patrol Ambush",
                    TriggerCondition::UnitInArea {
                        unit_type: None,
                        x_min: 40,
                        y_min: 30,
                        x_max: 58,
                        y_max: 50,
                    },
                    vec![
                        EventAction::SpawnReinforcements {
                            faction: Faction::Eden,
                            unit_type: UnitType::Lynx,
                            count: 2,
                            x: 50,
                            y: 38,
                        },
                        EventAction::TacticalNotification {
                            title: "Eden Recon Intercepted".into(),
                            message: "Eden patrols spotted scouting Plymouth geothermal energy grids!".into(),
                            priority: NotificationPriority::High,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    204,
                    "Badland Militia Emergency Response",
                    TriggerCondition::CasualtiesExceeded(2),
                    vec![
                        EventAction::SpawnReinforcements {
                            faction: Faction::Plymouth,
                            unit_type: UnitType::Lynx,
                            count: 2,
                            x: 18,
                            y: 24,
                        },
                        EventAction::TacticalNotification {
                            title: "Badland Militia Response".into(),
                            message: "Local guerilla pilots dispatched 2 Lynx scouts to reinforce our lines.".into(),
                            priority: NotificationPriority::High,
                        },
                    ],
                ));
            }
            3 => {
                // Eden Mission 2: Operation Caldera Bastion
                engine.add_trigger(ScriptTrigger::new(
                    301,
                    "Caldera Seismic Rupture",
                    TriggerCondition::SolTimer(2),
                    vec![
                        EventAction::TriggerDisaster {
                            disaster_type: DisasterType::SeismicFissure,
                            x: 22,
                            y: 16,
                        },
                        EventAction::TacticalNotification {
                            title: "Seismic Tremor Warning".into(),
                            message: "Catastrophic magma fissure ruptured across northern caldera corridor!".into(),
                            priority: NotificationPriority::Critical,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    302,
                    "Atmospheric Vortex Flare",
                    TriggerCondition::UnitInArea {
                        unit_type: None,
                        x_min: 20,
                        y_min: 12,
                        x_max: 32,
                        y_max: 22,
                    },
                    vec![
                        EventAction::TriggerDisaster {
                            disaster_type: DisasterType::AtmosphericVortex,
                            x: 26,
                            y: 18,
                        },
                        EventAction::TacticalNotification {
                            title: "Atmospheric Vortex Sighted".into(),
                            message: "Rotary thermal dust storm spawned near fissure vents!".into(),
                            priority: NotificationPriority::High,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    303,
                    "Plymouth Geothermal Raid",
                    TriggerCondition::SolTimer(4),
                    vec![
                        EventAction::SpawnReinforcements {
                            faction: Faction::Plymouth,
                            unit_type: UnitType::Panther,
                            count: 2,
                            x: 48,
                            y: 20,
                        },
                        EventAction::TacticalNotification {
                            title: "Hostile Incursion".into(),
                            message: "Plymouth assault tanks advancing towards geothermal facilities!".into(),
                            priority: NotificationPriority::Critical,
                        },
                    ],
                ));
            }
            4 => {
                // Plymouth Mission 2: Operation Iron Fang
                engine.add_trigger(ScriptTrigger::new(
                    401,
                    "Iron Fang Ambush Preparation",
                    TriggerCondition::SolTimer(1),
                    vec![
                        EventAction::TacticalNotification {
                            title: "Operation Iron Fang".into(),
                            message: "Position strike squadron at the canyon choke point to intercept Eden ore convoys.".into(),
                            priority: NotificationPriority::High,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    402,
                    "Eden Smelter Convoy Entry",
                    TriggerCondition::UnitInArea {
                        unit_type: None,
                        x_min: 28,
                        y_min: 20,
                        x_max: 42,
                        y_max: 35,
                    },
                    vec![
                        EventAction::SpawnReinforcements {
                            faction: Faction::Eden,
                            unit_type: UnitType::CargoTruck,
                            count: 2,
                            x: 36,
                            y: 24,
                        },
                        EventAction::SpawnReinforcements {
                            faction: Faction::Eden,
                            unit_type: UnitType::Panther,
                            count: 1,
                            x: 38,
                            y: 25,
                        },
                        EventAction::TacticalNotification {
                            title: "Target Convoy Entering Killzone".into(),
                            message: "Eden mineral transport convoy sighted! Spring the ambush!".into(),
                            priority: NotificationPriority::Critical,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    403,
                    "Smelter Sabotage Ore Quota Met",
                    TriggerCondition::ResourceThreshold {
                        metal_min: Some(3500),
                        food_min: None,
                    },
                    vec![
                        EventAction::ModifyObjective {
                            objective_id: 3,
                            status: ObjectiveStatus::Completed,
                        },
                        EventAction::TacticalNotification {
                            title: "Mineral Reserves Hijacked".into(),
                            message: "Ore supplies secured. Eden smelter operations crippled.".into(),
                            priority: NotificationPriority::Medium,
                        },
                    ],
                ));
            }
            5 => {
                // Eden Mission 3: Starship Evacuation: Escape From Mount Vulcan
                engine.add_trigger(ScriptTrigger::new(
                    501,
                    "Mount Vulcan Caldera Seismic Tremors",
                    TriggerCondition::SolTimer(1),
                    vec![
                        EventAction::TriggerDisaster {
                            disaster_type: DisasterType::SeismicFissure,
                            x: 25,
                            y: 35,
                        },
                        EventAction::TacticalNotification {
                            title: "Mount Vulcan Volcanic Alert".into(),
                            message: "Caldera fissures erupting across southern sectors! Expedite evacuation protocols!".into(),
                            priority: NotificationPriority::Critical,
                        },
                        EventAction::PlayVoiceClip("red_alert.wav".into()),
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    502,
                    "Plymouth Raider Division Breach",
                    TriggerCondition::SolTimer(3),
                    vec![
                        EventAction::SpawnReinforcements {
                            faction: Faction::Plymouth,
                            unit_type: UnitType::Lynx,
                            count: 2,
                            x: 36,
                            y: 32,
                        },
                        EventAction::SpawnReinforcements {
                            faction: Faction::Plymouth,
                            unit_type: UnitType::Panther,
                            count: 1,
                            x: 38,
                            y: 34,
                        },
                        EventAction::TacticalNotification {
                            title: "Hostile Raider Incursion".into(),
                            message: "Plymouth raider divisions advancing towards the launch facility! Defend the evacuation transports!".into(),
                            priority: NotificationPriority::Critical,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    503,
                    "Southern Magma Fissure Spread",
                    TriggerCondition::SolTimer(5),
                    vec![
                        EventAction::TriggerDisaster {
                            disaster_type: DisasterType::SeismicFissure,
                            x: 28,
                            y: 38,
                        },
                        EventAction::TacticalNotification {
                            title: "Subsurface Fault Collapse".into(),
                            message: "Magma fissure expanding along southern ridge! Structural conduits threatened!".into(),
                            priority: NotificationPriority::High,
                        },
                    ],
                ));
            }
            6 => {
                // Plymouth Mission 3: Operation Phoenix: Obsidian Crags Assault
                engine.add_trigger(ScriptTrigger::new(
                    601,
                    "Operation Phoenix Briefing",
                    TriggerCondition::SolTimer(1),
                    vec![
                        EventAction::TacticalNotification {
                            title: "Operation Phoenix Engaged".into(),
                            message: "Breach the Eden Obsidian Citadel and protect the Atmospheric Terraformer from nanite contagion!".into(),
                            priority: NotificationPriority::High,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    602,
                    "Nanite Contagion Encroachment",
                    TriggerCondition::SolTimer(2),
                    vec![
                        EventAction::TriggerDisaster {
                            disaster_type: DisasterType::MicrobialBlight,
                            x: 30,
                            y: 16,
                        },
                        EventAction::TacticalNotification {
                            title: "Nanite Contagion Warning".into(),
                            message: "Microbial nanite blight contagion encroaching on terraformer vents! Maintain perimeter security!".into(),
                            priority: NotificationPriority::Critical,
                        },
                    ],
                ));

                engine.add_trigger(ScriptTrigger::new(
                    603,
                    "Eden Citadel Counter-Assault",
                    TriggerCondition::SolTimer(4),
                    vec![
                        EventAction::SpawnReinforcements {
                            faction: Faction::Eden,
                            unit_type: UnitType::Panther,
                            count: 1,
                            x: 44,
                            y: 22,
                        },
                        EventAction::SpawnReinforcements {
                            faction: Faction::Eden,
                            unit_type: UnitType::Tiger,
                            count: 1,
                            x: 46,
                            y: 20,
                        },
                        EventAction::TacticalNotification {
                            title: "Citadel Counter-Attack".into(),
                            message: "Fortified Eden Citadel divisions launching heavy railgun counter-strike!".into(),
                            priority: NotificationPriority::Critical,
                        },
                    ],
                ));
            }
            _ => {}
        }

        engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::Faction;

    #[test]
    fn test_sol_timer_trigger() {
        let mut session = ColonyState::new_scenario(0);
        let mut engine = EventEngine::new();

        engine.add_trigger(ScriptTrigger::new(
            1,
            "Sol 5 Dispatch",
            TriggerCondition::SolTimer(5),
            vec![EventAction::TacticalNotification {
                title: "Sol 5 Reached".into(),
                message: "Five sols on New Terra.".into(),
                priority: NotificationPriority::Medium,
            }],
        ));

        // Initial sol is 0.0 -> trigger must not fire
        engine.update(&mut session);
        assert_eq!(engine.notifications().len(), 0);
        assert!(!engine.triggers[0].fired);

        // Advance sol telemetry to 5.5
        session.sol = 5.5;
        engine.update(&mut session);
        assert_eq!(engine.notifications().len(), 1);
        assert_eq!(engine.notifications()[0].title, "Sol 5 Reached");
        assert!(engine.triggers[0].fired);

        // One-shot trigger must not fire again
        engine.update(&mut session);
        assert_eq!(engine.notifications().len(), 1);
    }

    #[test]
    fn test_unit_in_area_trigger_and_reinforcement_spawning() {
        let mut session = ColonyState::new_scenario(0);
        let mut engine = EventEngine::new();

        let initial_units = session.units.len();

        engine.add_trigger(ScriptTrigger::new(
            2,
            "Mineral Canyon Ambush",
            TriggerCondition::UnitInArea {
                unit_type: Some(UnitType::Panther),
                x_min: 30,
                y_min: 30,
                x_max: 40,
                y_max: 40,
            },
            vec![EventAction::SpawnReinforcements {
                faction: Faction::Plymouth,
                unit_type: UnitType::Lynx,
                count: 3,
                x: 35,
                y: 35,
            }],
        ));

        // No Panther in area (30..40, 30..40)
        engine.update(&mut session);
        assert_eq!(session.units.len(), initial_units);
        assert!(!engine.triggers[0].fired);

        // Move or spawn a Panther into the trigger zone
        let next_id = session.next_unit_id();
        let panther = UnitType::Panther.spawn_unit(next_id, Faction::Eden, 32, 33);
        session.units.push(panther);

        engine.update(&mut session);
        assert!(engine.triggers[0].fired);
        assert_eq!(session.units.len(), initial_units + 1 + 3);

        // Verify spawned reinforcements
        let spawned_lynx: Vec<&Unit> = session
            .units
            .iter()
            .filter(|u| u.faction == Faction::Plymouth && u.name.contains("Lynx"))
            .collect();
        assert_eq!(spawned_lynx.len(), 3);
        assert_eq!(spawned_lynx[0].hp, 140);
        assert_eq!(spawned_lynx[0].weapon, Some(WeaponType::Laser));
    }

    #[test]
    fn test_casualties_exceeded_and_objective_modification() {
        let mut session = ColonyState::new_scenario(0);
        let mut engine = EventEngine::new();

        engine.add_trigger(ScriptTrigger::new(
            3,
            "Emergency Reinforcement on Casualties",
            TriggerCondition::CasualtiesExceeded(2),
            vec![
                EventAction::ModifyObjective {
                    objective_id: 1,
                    status: ObjectiveStatus::Failed,
                },
                EventAction::SpawnReinforcements {
                    faction: Faction::Eden,
                    unit_type: UnitType::Tiger,
                    count: 1,
                    x: 20,
                    y: 20,
                },
            ],
        ));

        session.casualties = 1;
        engine.update(&mut session);
        assert!(!engine.triggers[0].fired);

        session.casualties = 2;
        engine.update(&mut session);
        assert!(engine.triggers[0].fired);

        // Check objective status
        let obj1 = session.mission.objectives.iter().find(|o| o.id == 1).unwrap();
        assert_eq!(obj1.status, ObjectiveStatus::Failed);
        assert!(!obj1.is_completed);

        // Check Tiger tank spawned
        assert!(session.units.iter().any(|u| u.name.contains("Tiger")));
    }

    #[test]
    fn test_compound_conditions() {
        let mut session = ColonyState::new_scenario(0);
        let mut engine = EventEngine::new();

        engine.add_trigger(ScriptTrigger::new(
            4,
            "Compound Condition Check",
            TriggerCondition::CompoundAnd(vec![
                TriggerCondition::SolTimer(10),
                TriggerCondition::ResourceThreshold {
                    metal_min: Some(1000),
                    food_min: None,
                },
            ]),
            vec![EventAction::UnlockTechnology(2701)], // Astronomy
        ));

        session.sol = 12.0;
        session.common_ore = 500;
        session.rare_ore = 0;
        engine.update(&mut session);
        assert!(!engine.triggers[0].fired);

        session.common_ore = 1200;
        engine.update(&mut session);
        assert!(engine.triggers[0].fired);

        let astro = session.tech_tree.techs.iter().find(|t| t.id == 2701).unwrap();
        assert!(astro.is_researched);
    }

    #[test]
    fn test_repeatable_trigger_cooldown() {
        let mut session = ColonyState::new_scenario(0);
        let mut engine = EventEngine::new();

        engine.add_trigger(
            ScriptTrigger::new(
                5,
                "Periodic Radar Sweep",
                TriggerCondition::SolTimer(1),
                vec![EventAction::TacticalNotification {
                    title: "Sweep".into(),
                    message: "Radar sweep clear.".into(),
                    priority: NotificationPriority::Low,
                }],
            )
            .with_cooldown(5),
        );

        session.sol = 2.0;

        // Tick 1: fires
        engine.update(&mut session);
        assert_eq!(engine.notifications().len(), 1);

        // Ticks 2..5: cooldown active (ticks elapsed < 5)
        for _ in 0..4 {
            engine.update(&mut session);
            assert_eq!(engine.notifications().len(), 1);
        }

        // Tick 6: cooldown (5 ticks) elapsed -> fires again
        engine.update(&mut session);
        assert_eq!(engine.notifications().len(), 2);
    }
}
