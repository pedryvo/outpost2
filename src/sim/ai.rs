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
    pub wave_number: u32,
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
            wave_number: 0,
        }
    }

    /// Advance AI director logic and coordinate tactical assault waves
    pub fn update(
        &mut self,
        units: &mut Vec<Unit>,
        player_targets: &[(i32, i32)],
        dt_secs: f64,
    ) -> Option<&'static str> {
        let mut alert = None;
        self.spawn_timer -= dt_secs;
        self.patrol_timer -= dt_secs;

        // Periodic hostile reinforcement production
        if self.spawn_timer <= 0.0 {
            self.spawn_timer = 45.0; // Next reinforcement in 45 seconds
            self.wave_number += 1;
            let is_assault_wave = self.wave_number % 2 == 0;

            let next_id = units.iter().map(|u| u.id).max().unwrap_or(100) + 1;
            let (name, hp, weapon) = if is_assault_wave {
                ("Plymouth Tiger Assault", 320, Some(WeaponType::Railgun))
            } else if next_id % 2 == 0 {
                ("Plymouth Panther Battle Tank", 260, Some(WeaponType::Laser))
            } else {
                ("Plymouth Raider Lynx", 150, Some(WeaponType::Microwave))
            };

            let spawn_x = self.base_x + (next_id as i32 % 3);
            let spawn_y = self.base_y + (next_id as i32 % 3);

            units.push(Unit {
                id: next_id,
                name: format!("{} #{}", name, next_id),
                faction: self.faction,
                tile_x: spawn_x,
                tile_y: spawn_y,
                hp,
                max_hp: hp,
                is_selected: false,
                current_order: UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon,
                attack_cooldown: 0.0,
                cargo: None,
                hauler_state: None,
                cargo_kit: None,
            });

            if is_assault_wave {
                // Also spawn an escort raider in assault waves
                let escort_id = next_id + 1;
                units.push(Unit {
                    id: escort_id,
                    name: format!("Plymouth Raider Lynx #{}", escort_id),
                    faction: self.faction,
                    tile_x: spawn_x + 1,
                    tile_y: spawn_y + 1,
                    hp: 160,
                    max_hp: 160,
                    is_selected: false,
                    current_order: UnitOrder::Idle,
                    path: Vec::new(),
                    move_cooldown: 0.0,
                    weapon: Some(WeaponType::AcidCloud),
                    attack_cooldown: 0.0,
                    cargo: None,
                    hauler_state: None,
                    cargo_kit: None,
                });

                // Acquire closest player target to launch coordinated incursion
                let target_coord = if let Some(&target) = player_targets.first() {
                    let mut closest = target;
                    let mut min_dist = f64::MAX;
                    for &(tx, ty) in player_targets {
                        let d = ((tx - self.base_x).pow(2) + (ty - self.base_y).pow(2)) as f64;
                        if d < min_dist {
                            min_dist = d;
                            closest = (tx, ty);
                        }
                    }
                    Some(closest)
                } else {
                    None
                };

                if let Some((tx, ty)) = target_coord {
                    for u in units.iter_mut() {
                        if u.faction == self.faction && (u.current_order == UnitOrder::Idle || matches!(u.current_order, UnitOrder::MoveTo { .. })) {
                            u.current_order = UnitOrder::MoveTo { target_x: tx, target_y: ty };
                        }
                    }
                }

                alert = Some("Tactical Red Alert: Hostile Plymouth assault squad advancing on colony perimeter!");
            } else {
                alert = Some("Tactical Alert: Hostile Plymouth reinforcements deployed!");
            }
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
