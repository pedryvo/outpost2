//! Authentic Outpost 2 Fog of War and Sensor Grid Engine
//!
//! Features:
//! - 3-tier visibility model: Unexplored (black), Explored (shrouded), Visible (active sensors).
//! - Fast Euclidean sensor radius projection for dynamic units and stationary colony structures.
//! - Day/night ambient light modulation.

use super::{Building, Faction, Unit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FogState {
    Unexplored = 0,
    Explored = 1,
    Visible = 2,
}

#[derive(Debug, Clone)]
pub struct FogOfWar {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<u8>,
}

impl FogOfWar {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            grid: vec![FogState::Unexplored as u8; size],
        }
    }

    /// Reset currently visible tiles to Explored, then project sight circles from units and buildings
    pub fn update_visibility(&mut self, units: &[Unit], buildings: &[Building], player: Faction) {
        // Demote all active Visible (2) tiles back to Explored (1)
        for tile in self.grid.iter_mut() {
            if *tile == FogState::Visible as u8 {
                *tile = FogState::Explored as u8;
            }
        }

        // Project vision from buildings
        for b in buildings {
            if b.faction == Some(player) && (b.is_active || b.construction_progress < 100.0) {
                let center_x = b.tile_x + b.width / 2;
                let center_y = b.tile_y + b.height / 2;
                self.reveal_circle(center_x, center_y, 8);
            }
        }

        // Project vision from mobile units
        for u in units {
            if u.faction != player {
                continue;
            }
            // Units with sensor arrays or scouting capabilities
            let radius = if u.name.contains("Lynx") || u.name.contains("Scout") {
                8
            } else {
                6
            };
            self.reveal_circle(u.tile_x, u.tile_y, radius);
        }
    }

    /// Circle rasterization in tile space
    fn reveal_circle(&mut self, cx: i32, cy: i32, radius: i32) {
        let r2 = radius * radius;
        let w = self.width as i32;
        let h = self.height as i32;

        let min_x = (cx - radius).max(0);
        let max_x = (cx + radius).min(w - 1);
        let min_y = (cy - radius).max(0);
        let max_y = (cy + radius).min(h - 1);

        for y in min_y..=max_y {
            let dy = y - cy;
            for x in min_x..=max_x {
                let dx = x - cx;
                if dx * dx + dy * dy <= r2 {
                    let idx = (y * w + x) as usize;
                    if idx < self.grid.len() {
                        self.grid[idx] = FogState::Visible as u8;
                    }
                }
            }
        }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> FogState {
        if x >= self.width || y >= self.height {
            return FogState::Unexplored;
        }
        match self.grid[y * self.width + x] {
            1 => FogState::Explored,
            2 => FogState::Visible,
            _ => FogState::Unexplored,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::{ColonyState, Faction};

    #[test]
    fn enemy_units_do_not_reveal_the_players_map() {
        let mut colony = ColonyState::new(Faction::Eden);
        let mut enemy = colony.units[0].clone();
        enemy.faction = Faction::Plymouth;
        enemy.tile_x = colony.game_map.width as i32 - 2;
        enemy.tile_y = colony.game_map.height as i32 - 2;
        colony.units.retain(|u| u.faction == Faction::Eden);
        colony.units.push(enemy.clone());
        colony.fog = FogOfWar::new(colony.game_map.width as usize, colony.game_map.height as usize);
        colony.fog.update_visibility(&colony.units, &colony.buildings, colony.faction);
        assert_eq!(colony.fog.get(enemy.tile_x as usize, enemy.tile_y as usize), FogState::Unexplored);
    }

    #[test]
    fn friendly_construction_site_remains_visible() {
        let colony = ColonyState::new(Faction::Eden);
        let mut site = colony.buildings.iter().find(|b| b.faction == Some(colony.faction)).unwrap().clone();
        site.is_active = false;
        site.construction_progress = 0.0;
        let mut fog = FogOfWar::new(colony.game_map.width as usize, colony.game_map.height as usize);
        fog.update_visibility(&[], &[site.clone()], colony.faction);
        assert_eq!(fog.get(site.tile_x as usize, site.tile_y as usize), FogState::Visible);
    }
}
