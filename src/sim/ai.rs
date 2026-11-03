//! Hostile Plymouth Faction AI Director
//!
//! Autonomous decision engine for rival colony factions on New Terra:
//! - Patrol routines along defensive perimeters and choke points.
//! - Reinforcement production of combat tanks and raiders.
//! - Threat response and coordinated counter-attacks against Eden incursions.

use super::{Faction, Unit, pathfinding::UnitOrder, combat::WeaponType};

#[derive(Debug, Clone)]
pub struct FactionAiDirector {
    pub faction: Faction,
    pub spawn_timer: f64,
    pub patrol_timer: f64,
    pub patrol_index: usize,
    pub base_x: i32,
    pub base_y: i32,
}

impl FactionAiDirector {
    pub fn new(faction: Faction, base_x: i32, base_y: i32) -> Self {
        Self {
            faction,
            spawn_timer: 25.0, // First reinforcement wave in 25 seconds
            patrol_timer: 10.0,
            patrol_index: 0,
            base_x,
            base_y,
        }
    }

    /// Advance AI director logic
    pub fn update(&mut self, units: &mut Vec<Unit>, dt_secs: f64) -> Option<&'static str> {
        let mut alert = None;
        self.spawn_timer -= dt_secs;
        self.patrol_timer -= dt_secs;

        // Periodic hostile reinforcement production
        if self.spawn_timer <= 0.0 {
            self.spawn_timer = 40.0; // Next reinforcement in 40 seconds
            let next_id = units.iter().map(|u| u.id).max().unwrap_or(100) + 1;
            let (name, hp, weapon) = if next_id % 2 == 0 {
                ("Plymouth Tiger Assault", 280, Some(WeaponType::AcidCloud))
            } else {
                ("Plymouth Raider Lynx", 150, Some(WeaponType::Microwave))
            };

            units.push(Unit {
                id: next_id,
                name: format!("{} #{}", name, next_id),
                faction: self.faction,
                tile_x: self.base_x + (next_id as i32 % 3),
                tile_y: self.base_y + (next_id as i32 % 3),
                hp,
                max_hp: hp,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon,
                attack_cooldown: 0.0,
            });

            alert = Some("Tactical Alert: Hostile Plymouth reinforcements deployed!");
        }

        // Patrol coordinate waypoints
        let waypoints = [
            (self.base_x - 4, self.base_y - 2),
            (self.base_x - 8, self.base_y + 2),
            (self.base_x - 3, self.base_y + 4),
            (self.base_x, self.base_y),
        ];

        if self.patrol_timer <= 0.0 {
            self.patrol_timer = 12.0;
            self.patrol_index = (self.patrol_index + 1) % waypoints.len();
            let (tx, ty) = waypoints[self.patrol_index];

            for u in units.iter_mut() {
                if u.faction == self.faction && u.current_order == UnitOrder::Idle {
                    u.current_order = UnitOrder::MoveTo { target_x: tx, target_y: ty };
                }
            }
        }

        alert
    }
}
