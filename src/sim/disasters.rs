//! Outpost 2 Natural Disasters & Dynamic Hazard Simulator
//! Simulates volcanic magma expansion, seismic tremors, and Microbial Blight contagion.

use crate::sim::map::GameMap;

#[derive(Debug, Clone)]
pub struct DisasterSimulator {
    pub sol_timer: f64,
    pub active_lava_tiles: Vec<(u32, u32)>,
    pub active_blight_tiles: Vec<(u32, u32)>,
    pub eruption_count: u32,
    pub quake_shake_offset: (f64, f64),
    pub quake_duration: f64,
}

impl DisasterSimulator {
    pub fn new() -> Self {
        Self {
            sol_timer: 0.0,
            active_lava_tiles: Vec::new(),
            active_blight_tiles: Vec::new(),
            eruption_count: 0,
            quake_shake_offset: (0.0, 0.0),
            quake_duration: 0.0,
        }
    }

    /// Advance planetary natural disasters according to New Terra volcanic cycles
    pub fn update(&mut self, map: &mut GameMap, dt_secs: f64) -> Option<&'static str> {
        self.sol_timer += dt_secs;

        // Quake camera shake dissipation
        if self.quake_duration > 0.0 {
            self.quake_duration -= dt_secs;
            let magnitude = (self.quake_duration * 12.0).min(6.0);
            self.quake_shake_offset = (
                (self.sol_timer * 40.0).sin() * magnitude,
                (self.sol_timer * 35.0).cos() * magnitude,
            );
        } else {
            self.quake_shake_offset = (0.0, 0.0);
        }

        // Trigger volcanic fissure event every ~45 seconds
        if self.sol_timer > 45.0 {
            self.sol_timer = 0.0;
            self.eruption_count += 1;
            self.trigger_fissure(map);
            self.quake_duration = 3.5; // 3.5 seconds of planetary tremor
            return Some("SEISMIC WARNING: Volcanic fissure erupted on New Terra! Magma is advancing.");
        }

        None
    }

    /// Spread magma along valid lava channels
    fn trigger_fissure(&mut self, map: &mut GameMap) {
        // Eruption epicenter near northern volcanic ridge (x: 15..25, y: 12..18)
        let center_x = 18 + (self.eruption_count * 3) % 15;
        let center_y = 14 + (self.eruption_count * 2) % 10;

        for dy in -2..=2 {
            for dx in -2..=2 {
                let tx = (center_x as i32 + dx).clamp(0, map.width as i32 - 1) as u32;
                let ty = (center_y as i32 + dy).clamp(0, map.height as i32 - 1) as u32;
                let idx = (ty * map.width + tx) as usize;

                if idx < map.tiles.len() {
                    // Ignite tile as active molten lava
                    map.tiles[idx].is_lava = true;
                    self.active_lava_tiles.push((tx, ty));
                }
            }
        }
    }
}
