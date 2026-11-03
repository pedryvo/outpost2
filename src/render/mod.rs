//! Isometric Viewport and HTML5 Canvas Renderer

pub mod palette;

use web_sys::CanvasRenderingContext2d;
use crate::sim::{fog::FogState, ColonyState, Faction};
use palette::MASTER_PALETTE;

pub struct Viewport {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub tile_width: f64,
    pub tile_height: f64,
    pub map_cols: i32,
    pub map_rows: i32,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            camera_x: 600.0,
            camera_y: 200.0,
            zoom: 1.0,
            tile_width: 64.0,
            tile_height: 32.0,
            map_cols: 64,
            map_rows: 64,
        }
    }

    /// Convert tile grid coordinate to screen pixel coordinates
    pub fn tile_to_screen(&self, tx: f64, ty: f64) -> (f64, f64) {
        let sx = (tx - ty) * (self.tile_width * 0.5 * self.zoom) + self.camera_x;
        let sy = (tx + ty) * (self.tile_height * 0.5 * self.zoom) + self.camera_y;
        (sx, sy)
    }

    /// Convert screen pixel coordinates (sx, sy) back to tile grid coordinates (tx, ty)
    pub fn screen_to_tile(&self, sx: f64, sy: f64) -> (i32, i32) {
        let rel_x = (sx - self.camera_x) / (self.tile_width * 0.5 * self.zoom);
        let rel_y = (sy - self.camera_y) / (self.tile_height * 0.5 * self.zoom);
        let tx = ((rel_y + rel_x) * 0.5).round() as i32;
        let ty = ((rel_y - rel_x) * 0.5).round() as i32;
        (tx, ty)
    }

    /// Render the terrain, colony buildings, units, and active tactical overlays
    pub fn render(
        &self,
        ctx: &CanvasRenderingContext2d,
        colony: &ColonyState,
        width: f64,
        height: f64,
        ghost_placement: Option<(crate::sim::structures::StructureType, (i32, i32), bool)>,
        drag_selection: Option<((f64, f64), (f64, f64))>,
    ) {
        let (shake_x, shake_y) = colony.disasters.quake_shake_offset;
        let _ = ctx.save();
        let _ = ctx.translate(shake_x, shake_y);

        // Clear background (Deep space black)
        ctx.set_fill_style_str("#07090e");
        ctx.fill_rect(0.0, 0.0, width, height);

        let half_w = self.tile_width * 0.5 * self.zoom;
        let half_h = self.tile_height * 0.5 * self.zoom;

        // Render visible isometric terrain tiles
        let start_col = 0;
        let end_col = (colony.game_map.width as i32).min(self.map_cols);
        let start_row = 0;
        let end_row = (colony.game_map.height as i32).min(self.map_rows);

        for r in start_row..end_row {
            for c in start_col..end_col {
                let (sx, sy) = self.tile_to_screen(c as f64, r as f64);

                // Viewport culling
                if sx < -100.0 || sx > width + 100.0 || sy < -100.0 || sy > height + 100.0 {
                    continue;
                }

                let fog = colony.fog.get(c as usize, r as usize);
                if fog == FogState::Unexplored {
                    ctx.set_fill_style_str("#030712");
                    ctx.set_stroke_style_str("#020408");
                    ctx.set_line_width(0.5);

                    ctx.begin_path();
                    ctx.move_to(sx, sy - half_h);
                    ctx.line_to(sx + half_w, sy);
                    ctx.line_to(sx, sy + half_h);
                    ctx.line_to(sx - half_w, sy);
                    ctx.close_path();

                    ctx.fill();
                    ctx.stroke();
                    continue;
                }

                // Ambient light multiplier: Explored is shrouded (0.35x), Visible scales with diurnal sunlight (0.4x - 1.0x)
                let light_mult = match fog {
                    FogState::Explored => 0.35,
                    FogState::Visible => colony.sunlight,
                    FogState::Unexplored => 0.0,
                };

                let tile = colony.game_map.get_tile(c as u32, r as u32);
                let terrain_color = if let Some(t) = tile {
                    if t.is_lava {
                        let r = (239.0 * light_mult.max(0.6)) as u8;
                        let g = (68.0 * light_mult.max(0.6)) as u8;
                        let b = (68.0 * light_mult.max(0.6)) as u8;
                        format!("rgb({},{},{})", r, g, b)
                    } else if t.is_blight {
                        let r = (168.0 * light_mult) as u8;
                        let g = (85.0 * light_mult) as u8;
                        let b = (247.0 * light_mult) as u8;
                        format!("rgb({},{},{})", r, g, b)
                    } else {
                        let pal_idx = match t.cell_type {
                            0 => 18, // Basalt rock
                            1 => 45, // Iron-oxide dust
                            2 => 52, // Highland clay
                            3 => 78, // Ore-rich soil
                            _ => 40 + ((t.tile_mapping % 16) as usize),
                        };
                        let [pr, pg, pb] = MASTER_PALETTE[pal_idx.min(255)];
                        let r = (pr as f64 * light_mult) as u8;
                        let g = (pg as f64 * light_mult) as u8;
                        let b = (pb as f64 * light_mult) as u8;
                        format!("rgb({},{},{})", r, g, b)
                    }
                } else {
                    let [pr, pg, pb] = MASTER_PALETTE[45];
                    let r = (pr as f64 * light_mult) as u8;
                    let g = (pg as f64 * light_mult) as u8;
                    let b = (pb as f64 * light_mult) as u8;
                    format!("rgb({},{},{})", r, g, b)
                };

                ctx.set_fill_style_str(&terrain_color);
                ctx.set_stroke_style_str(if fog == FogState::Explored { "#0f172a" } else { "#1c1917" });
                ctx.set_line_width(0.75);

                ctx.begin_path();
                ctx.move_to(sx, sy - half_h);
                ctx.line_to(sx + half_w, sy);
                ctx.line_to(sx, sy + half_h);
                ctx.line_to(sx - half_w, sy);
                ctx.close_path();

                ctx.fill();
                ctx.stroke();

                if let Some(t) = tile {
                    if (t.is_wall_or_building || t.cell_type == 3) && fog != FogState::Unexplored {
                        // 2.5D cliff drop facet and shadow
                        let drop_h = 6.0 * self.zoom;
                        ctx.set_fill_style_str(if fog == FogState::Explored { "#07090e" } else { "#111726" });
                        ctx.begin_path();
                        ctx.move_to(sx - half_w, sy);
                        ctx.line_to(sx, sy + half_h);
                        ctx.line_to(sx + half_w, sy);
                        ctx.line_to(sx + half_w, sy + drop_h);
                        ctx.line_to(sx, sy + half_h + drop_h);
                        ctx.line_to(sx - half_w, sy + drop_h);
                        ctx.close_path();
                        ctx.fill();
                    }

                    // Atmospheric hazard particle drift: volcanic embers and blight spores
                    if t.is_blight && fog != FogState::Unexplored {
                        let spore_t = colony.sol * 2.0 + (c as f64 * 3.7) + (r as f64 * 5.3);
                        let spore_dy = (spore_t % 1.0) * 16.0 * self.zoom;
                        let spore_dx = spore_t.sin() * 6.0 * self.zoom;
                        ctx.set_fill_style_str("rgba(216, 180, 254, 0.75)");
                        ctx.begin_path();
                        let _ = ctx.arc(sx + spore_dx, sy - spore_dy, 2.0 * self.zoom, 0.0, std::f64::consts::TAU);
                        ctx.fill();
                    } else if t.is_lava && fog != FogState::Unexplored {
                        let ember_t = colony.sol * 4.0 + (c as f64 * 2.1) + (r as f64 * 4.7);
                        let ember_dy = (ember_t % 1.0) * 18.0 * self.zoom;
                        let ember_dx = ember_t.cos() * 4.0 * self.zoom;
                        ctx.set_fill_style_str("rgba(254, 215, 170, 0.85)");
                        ctx.begin_path();
                        let _ = ctx.arc(sx + ember_dx, sy - ember_dy, 1.8 * self.zoom, 0.0, std::f64::consts::TAU);
                        ctx.fill();
                    }
                }
            }
        }

        // Render Pressurized Colonial Transport Tubes
        for &(tx, ty) in &colony.tubes {
            let fog = colony.fog.get(tx as usize, ty as usize);
            if fog == FogState::Unexplored {
                continue;
            }

            let (sx, sy) = self.tile_to_screen(tx as f64, ty as f64);
            if sx < -50.0 || sx > width + 50.0 || sy < -50.0 || sy > height + 50.0 {
                continue;
            }

            let tube_w = 10.0 * self.zoom;
            let tube_h = 5.0 * self.zoom;

            // Outer pressurized metallic conduit casing
            ctx.set_fill_style_str(if fog == FogState::Explored { "#1e293b" } else { "#475569" });
            ctx.begin_path();
            ctx.move_to(sx, sy - tube_h);
            ctx.line_to(sx + tube_w, sy);
            ctx.line_to(sx, sy + tube_h);
            ctx.line_to(sx - tube_w, sy);
            ctx.close_path();
            ctx.fill();

            // Inner cyan luminous plasma life-support artery
            if fog == FogState::Visible {
                ctx.set_stroke_style_str("#38bdf8");
                ctx.set_line_width(1.2 * self.zoom);
                ctx.stroke();

                ctx.set_fill_style_str("#0ea5e9");
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, 2.0 * self.zoom, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();
            }
        }

        // Render Subterranean Ore Deposits
        for dep in &colony.ore_deposits {
            let fog = colony.fog.get(dep.tile_x as usize, dep.tile_y as usize);
            if fog == FogState::Unexplored {
                continue;
            }

            let (sx, sy) = self.tile_to_screen(dep.tile_x as f64, dep.tile_y as f64);
            let color = match dep.ore_type {
                crate::sim::mining::OreType::Common => "#cbd5e1", // Hematite ore
                crate::sim::mining::OreType::Rare => "#38bdf8",   // Titanium vein
            };

            ctx.set_fill_style_str(color);
            ctx.set_stroke_style_str("#0f172a");
            ctx.set_line_width(1.0);
            ctx.begin_path();
            let _ = ctx.arc(sx, sy, 5.0 * self.zoom, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
            ctx.stroke();

            // Label
            if fog == FogState::Visible {
                ctx.set_fill_style_str(color);
                ctx.set_font(&format!("bold {}px sans-serif", (9.0 * self.zoom).max(8.0)));
                let desc = match dep.ore_type {
                    crate::sim::mining::OreType::Common => "⛏️ Common Ore",
                    crate::sim::mining::OreType::Rare => "💎 Rare Ore",
                };
                let _ = ctx.fill_text(desc, sx - 22.0 * self.zoom, sy - 8.0 * self.zoom);
            }
        }

        // Render Buildings (if not in completely unexplored terrain)
        for b in &colony.buildings {
            let fog = colony.fog.get(b.tile_x as usize, b.tile_y as usize);
            if fog == FogState::Unexplored {
                continue;
            }

            let (sx, sy) = self.tile_to_screen(b.tile_x as f64, b.tile_y as f64);
            let b_w = half_w * (b.width as f64);
            let b_h = half_h * (b.height as f64);

            let stroke_col = if b.is_selected {
                "#4ade80" // Green selection highlight
            } else if fog == FogState::Explored {
                "#475569"
            } else if colony.faction == Faction::Eden {
                "#00d4ff"
            } else {
                "#ff8c00"
            };

            // Structure footprint
            ctx.set_fill_style_str(if fog == FogState::Explored { "#0f172a" } else { "#1e293b" });
            ctx.set_stroke_style_str(stroke_col);
            ctx.set_line_width(2.0);

            ctx.begin_path();
            ctx.move_to(sx, sy - b_h);
            ctx.line_to(sx + b_w, sy);
            ctx.line_to(sx, sy + b_h);
            ctx.line_to(sx - b_w, sy);
            ctx.close_path();
            ctx.fill();
            ctx.stroke();

            // Structure 2.5D extrusion
            ctx.set_fill_style_str(if fog == FogState::Explored { "#1e293b" } else { "#334155" });
            ctx.fill_rect(sx - b_w * 0.5, sy - b_h - 20.0 * self.zoom, b_w, 20.0 * self.zoom);

            // Label
            ctx.set_fill_style_str(if fog == FogState::Explored { "#94a3b8" } else { "#ffffff" });
            ctx.set_font(&format!("bold {}px sans-serif", (11.0 * self.zoom).max(9.0)));
            let _ = ctx.fill_text(&b.name, sx - b_w * 0.45, sy - b_h - 6.0 * self.zoom);

            // Active Vehicle / Starship Fabrication Visuals
            if !b.production_queue.is_empty() && b.production_progress > 0.0 && fog == FogState::Visible {
                let spark_pulse = (colony.sol * 12.0).sin().abs();
                let spark_color = if colony.faction == Faction::Eden {
                    format!("rgba(56, 189, 248, {:.2})", 0.4 + 0.5 * spark_pulse)
                } else {
                    format!("rgba(249, 115, 22, {:.2})", 0.4 + 0.5 * spark_pulse)
                };
                ctx.set_stroke_style_str(&spark_color);
                ctx.set_line_width(2.0 * self.zoom);
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, (12.0 + 4.0 * spark_pulse) * self.zoom, 0.0, std::f64::consts::TAU);
                ctx.stroke();

                // Progress Bar
                let bar_w = (b_w * 0.85).max(40.0 * self.zoom);
                let bar_h = 5.0 * self.zoom;
                let bar_x = sx - bar_w * 0.5;
                let bar_y = sy - b_h - 26.0 * self.zoom;

                ctx.set_fill_style_str("#090d16");
                ctx.fill_rect(bar_x, bar_y, bar_w, bar_h);

                let fill_pct = (b.production_progress / 100.0).clamp(0.0, 1.0);
                ctx.set_fill_style_str(if colony.faction == Faction::Eden { "#38bdf8" } else { "#f97316" });
                ctx.fill_rect(bar_x, bar_y, bar_w * fill_pct, bar_h);

                if let Some(item) = b.production_queue.first() {
                    ctx.set_fill_style_str("#e2e8f0");
                    ctx.set_font(&format!("bold {}px monospace", (8.0 * self.zoom).max(8.0)));
                    let label = format!("{} {}%", item, b.production_progress as u32);
                    let _ = ctx.fill_text(&label, bar_x, bar_y - 3.0);
                }
            }

            // Vehicle Repair Depot Diagnostic Field
            if b.name.contains("Repair") && b.is_active && fog == FogState::Visible {
                let repair_pulse = (colony.sol * 4.0).sin().abs();
                ctx.set_stroke_style_str(&format!("rgba(16, 185, 129, {:.2})", 0.4 + 0.4 * repair_pulse));
                ctx.set_line_width(1.5 * self.zoom);
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, 22.0 * self.zoom, 0.0, std::f64::consts::TAU);
                ctx.stroke();

                ctx.set_fill_style_str("#10b981");
                ctx.set_font(&format!("bold {}px sans-serif", (10.0 * self.zoom).max(9.0)));
                let _ = ctx.fill_text("➕", sx - 5.0 * self.zoom, sy - b_h - 22.0 * self.zoom);
            }
        }

        // Render Units (Only visible when active in sensor range)
        for u in &colony.units {
            let fog = colony.fog.get(u.tile_x as usize, u.tile_y as usize);
            if fog != FogState::Visible {
                continue;
            }

            let (sx, sy) = self.tile_to_screen(u.tile_x as f64, u.tile_y as f64);

            // Unit chassis: Repair is emerald green, Earthworker is amber, Eden is Cyan, Plymouth is Crimson Red
            let chassis_color = if u.name.contains("Repair") {
                "#10b981"
            } else if u.name.contains("Earthworker") {
                "#f59e0b"
            } else if u.faction == Faction::Eden {
                "#38bdf8"
            } else {
                "#ef4444"
            };
            ctx.set_fill_style_str(chassis_color);
            ctx.begin_path();
            let _ = ctx.arc(sx, sy, 8.0 * self.zoom, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();

            ctx.set_stroke_style_str("#0f172a");
            ctx.set_line_width(1.5);
            ctx.stroke();

            if u.name.contains("Repair") {
                ctx.set_fill_style_str("#ecfdf5");
                ctx.set_font(&format!("bold {}px sans-serif", (9.0 * self.zoom).max(8.0)));
                let _ = ctx.fill_text("🔧", sx - 5.0 * self.zoom, sy - 18.0 * self.zoom);
            } else if u.name.contains("Earthworker") {
                ctx.set_fill_style_str("#fef3c7");
                ctx.set_font(&format!("bold {}px sans-serif", (9.0 * self.zoom).max(8.0)));
                let _ = ctx.fill_text("🏗️", sx - 5.0 * self.zoom, sy - 18.0 * self.zoom);
            }

            // Unit selection ring
            if u.is_selected {
                ctx.set_stroke_style_str("#4ade80");
                ctx.set_line_width(2.0);
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, 12.0 * self.zoom, 0.0, std::f64::consts::PI * 2.0);
                ctx.stroke();
            }

            // Health bar
            let hp_pct = u.hp as f64 / u.max_hp as f64;
            ctx.set_fill_style_str("#dc2626");
            ctx.fill_rect(sx - 12.0, sy - 14.0 * self.zoom, 24.0, 3.0);
            ctx.set_fill_style_str("#16a34a");
            ctx.fill_rect(sx - 12.0, sy - 14.0 * self.zoom, 24.0 * hp_pct, 3.0);

            // Cargo hauler container payload
            if let Some((ore_type, _)) = u.cargo {
                let cargo_col = match ore_type {
                    crate::sim::mining::OreType::Common => "#cbd5e1",
                    crate::sim::mining::OreType::Rare => "#38bdf8",
                };
                ctx.set_fill_style_str(cargo_col);
                ctx.fill_rect(sx - 3.5 * self.zoom, sy - 3.5 * self.zoom, 7.0 * self.zoom, 7.0 * self.zoom);
                ctx.set_stroke_style_str("#0f172a");
                ctx.set_line_width(1.0);
                ctx.stroke_rect(sx - 3.5 * self.zoom, sy - 3.5 * self.zoom, 7.0 * self.zoom, 7.0 * self.zoom);
            }
        }

        // Render Active Combat Laser & Energy Beams
        for beam in &colony.active_beams {
            let (from_sx, from_sy) = self.tile_to_screen(beam.from_x, beam.from_y);
            let (to_sx, to_sy) = self.tile_to_screen(beam.to_x, beam.to_y);

            ctx.set_stroke_style_str(beam.color);
            ctx.set_line_width(2.5 * self.zoom);
            ctx.begin_path();
            ctx.move_to(from_sx, from_sy);
            ctx.line_to(to_sx, to_sy);
            ctx.stroke();

            // White-hot beam core
            ctx.set_stroke_style_str("#ffffff");
            ctx.set_line_width(1.0 * self.zoom);
            ctx.begin_path();
            ctx.move_to(from_sx, from_sy);
            ctx.line_to(to_sx, to_sy);
            ctx.stroke();
        }

        // Render Active Explosions & Concussive Blast Waves
        for exp in &colony.active_explosions {
            let (sx, sy) = self.tile_to_screen(exp.x, exp.y);
            let r = exp.radius * self.zoom;

            // Outer fiery flash
            ctx.set_fill_style_str("rgba(249, 115, 22, 0.45)");
            ctx.begin_path();
            let _ = ctx.arc(sx, sy, r * 1.5, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();

            // Inner white-hot fireball core
            ctx.set_fill_style_str("rgba(254, 240, 138, 0.85)");
            ctx.begin_path();
            let _ = ctx.arc(sx, sy, r, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();

            ctx.set_stroke_style_str("#ef4444");
            ctx.set_line_width(2.0);
            ctx.stroke();
        }

        // Render Atmospheric Vortices (Dust Devils & Tornadoes)
        for v in &colony.disasters.active_vortices {
            let (sx, sy) = self.tile_to_screen(v.x, v.y);
            let alpha = (v.lifetime / v.max_lifetime).clamp(0.0, 1.0);
            let vortex_r = v.radius * 8.0 * self.zoom * (0.5 + 0.5 * alpha);
            let spin = colony.sol * 12.0;

            for i in 0..4 {
                let a = spin + (i as f64) * std::f64::consts::FRAC_PI_2;
                ctx.set_stroke_style_str(&format!("rgba(217, 119, 6, {:.2})", 0.45 * alpha));
                ctx.set_line_width(3.0 * self.zoom);
                ctx.begin_path();
                ctx.move_to(sx, sy);
                let _ = ctx.quadratic_curve_to(
                    sx + a.cos() * vortex_r * 0.6,
                    sy + a.sin() * vortex_r * 0.6,
                    sx + (a + 0.8).cos() * vortex_r,
                    sy + (a + 0.8).sin() * vortex_r,
                );
                ctx.stroke();
            }

            ctx.set_fill_style_str(&format!("rgba(245, 158, 11, {:.2})", 0.3 * alpha));
            ctx.begin_path();
            let _ = ctx.arc(sx, sy, vortex_r * 0.8, 0.0, std::f64::consts::TAU);
            ctx.fill();

            for p in 0..6 {
                let pa = spin * 1.5 + (p as f64 * 1.05);
                let pd = (vortex_r * 0.4) + ((p as f64 * 4.0) % (vortex_r * 0.5));
                ctx.set_fill_style_str("rgba(254, 240, 138, 0.8)");
                ctx.begin_path();
                let _ = ctx.arc(sx + pa.cos() * pd, sy + pa.sin() * pd, 1.5 * self.zoom, 0.0, std::f64::consts::TAU);
                ctx.fill();
            }
        }

        // Render Holographic Ghost Blueprint Placement
        if let Some((st, (gx, gy), is_valid)) = ghost_placement {
            let (sx, sy) = self.tile_to_screen(gx as f64, gy as f64);
            let (w, h) = st.dimensions();
            let b_w = half_w * (w as f64);
            let b_h = half_h * (h as f64);

            let fill_col = if is_valid { "rgba(56, 189, 248, 0.35)" } else { "rgba(239, 68, 68, 0.35)" };
            let stroke_col = if is_valid { "#38bdf8" } else { "#ef4444" };

            // Isometric blueprint footprint
            ctx.set_fill_style_str(fill_col);
            ctx.set_stroke_style_str(stroke_col);
            ctx.set_line_width(2.0);

            ctx.begin_path();
            ctx.move_to(sx, sy - b_h);
            ctx.line_to(sx + b_w, sy);
            ctx.line_to(sx, sy + b_h);
            ctx.line_to(sx - b_w, sy);
            ctx.close_path();
            ctx.fill();
            ctx.stroke();

            // 2.5D Holographic extrusion
            ctx.set_fill_style_str(if is_valid { "rgba(56, 189, 248, 0.20)" } else { "rgba(239, 68, 68, 0.20)" });
            ctx.fill_rect(sx - b_w * 0.5, sy - b_h - 20.0 * self.zoom, b_w, 20.0 * self.zoom);

            // Projected transport tube connection to nearest structure
            if let Some(nearest) = colony.buildings.iter().min_by_key(|b| (b.tile_x - gx).abs() + (b.tile_y - gy).abs()) {
                let (nx, ny) = self.tile_to_screen(nearest.tile_x as f64 + (nearest.width as f64) * 0.5, nearest.tile_y as f64 + (nearest.height as f64) * 0.5);
                ctx.set_stroke_style_str("rgba(56, 189, 248, 0.7)");
                ctx.set_line_width(1.5);
                ctx.begin_path();
                ctx.move_to(sx, sy);
                ctx.line_to(nx, ny);
                ctx.stroke();
            }

            // Blueprint Header Badge
            let status_msg = if is_valid { "READY TO DEPLOY" } else { "OBSTRUCTED / INSUFFICIENT ORE" };
            ctx.set_fill_style_str(if is_valid { "#4ade80" } else { "#ef4444" });
            ctx.set_font(&format!("bold {}px sans-serif", (11.0 * self.zoom).max(10.0)));
            let badge_text = format!("{} [{}]", st.name(), status_msg);
            let _ = ctx.fill_text(&badge_text, sx - b_w * 0.5, sy - b_h - 26.0 * self.zoom);
        }

        let _ = ctx.restore();

        // Render Tactical Drag-Selection Box (Screen space, outside shake transform)
        if let Some(((x1, y1), (x2, y2))) = drag_selection {
            let min_x = x1.min(x2);
            let min_y = y1.min(y2);
            let box_w = (x1 - x2).abs();
            let box_h = (y1 - y2).abs();

            ctx.set_fill_style_str("rgba(56, 189, 248, 0.18)");
            ctx.fill_rect(min_x, min_y, box_w, box_h);

            ctx.set_stroke_style_str("#38bdf8");
            ctx.set_line_width(1.5);
            ctx.stroke_rect(min_x, min_y, box_w, box_h);

            let corner = (8.0_f64).min(box_w * 0.3).min(box_h * 0.3);
            ctx.set_stroke_style_str("#4ade80");
            ctx.set_line_width(2.0);

            // Top-left corner
            ctx.begin_path();
            ctx.move_to(min_x, min_y + corner);
            ctx.line_to(min_x, min_y);
            ctx.line_to(min_x + corner, min_y);
            ctx.stroke();

            // Bottom-right corner
            ctx.begin_path();
            ctx.move_to(min_x + box_w - corner, min_y + box_h);
            ctx.line_to(min_x + box_w, min_y + box_h);
            ctx.line_to(min_x + box_w, min_y + box_h - corner);
            ctx.stroke();
        }
    }

    /// Render mini-radar overview with authentic Fog of War mask
    pub fn render_minimap(&self, ctx: &CanvasRenderingContext2d, colony: &ColonyState, w: f64, h: f64) {
        ctx.set_fill_style_str("#030712");
        ctx.fill_rect(0.0, 0.0, w, h);

        let scale_x = w / (self.map_cols as f64);
        let scale_y = h / (self.map_rows as f64);

        // Render explored and visible terrain footprint
        for y in 0..self.map_rows {
            for x in 0..self.map_cols {
                let fog = colony.fog.get(x as usize, y as usize);
                if fog == FogState::Unexplored {
                    continue;
                }
                let color = if fog == FogState::Visible {
                    "#334155"
                } else {
                    "#1e293b"
                };
                ctx.set_fill_style_str(color);
                ctx.fill_rect((x as f64) * scale_x, (y as f64) * scale_y, scale_x + 0.5, scale_y + 0.5);
            }
        }

        // Blip transport tubes
        ctx.set_fill_style_str("#64748b");
        for &(tx, ty) in &colony.tubes {
            let fog = colony.fog.get(tx as usize, ty as usize);
            if fog != FogState::Unexplored {
                let px = (tx as f64) * scale_x;
                let py = (ty as f64) * scale_y;
                ctx.fill_rect(px, py, 1.5, 1.5);
            }
        }

        // Blip buildings if explored or visible
        ctx.set_fill_style_str("#38bdf8");
        for b in &colony.buildings {
            let fog = colony.fog.get(b.tile_x as usize, b.tile_y as usize);
            if fog != FogState::Unexplored {
                let bx = (b.tile_x as f64) * scale_x;
                let by = (b.tile_y as f64) * scale_y;
                ctx.fill_rect(bx, by, (b.width as f64) * scale_x + 1.0, (b.height as f64) * scale_y + 1.0);
            }
        }

        // Blip units (friendly Eden in green, hostile Plymouth in red)
        for u in &colony.units {
            let fog = colony.fog.get(u.tile_x as usize, u.tile_y as usize);
            if fog == FogState::Visible {
                let color = if u.faction == Faction::Eden { "#4ade80" } else { "#ef4444" };
                ctx.set_fill_style_str(color);
                let ux = (u.tile_x as f64) * scale_x;
                let uy = (u.tile_y as f64) * scale_y;
                ctx.fill_rect(ux - 1.0, uy - 1.0, 3.0, 3.0);
            }
        }

        // Tactical camera viewport frustum box
        let (center_tx, center_ty) = self.screen_to_tile(400.0, 300.0);
        let fx = ((center_tx as f64 - 10.0) * scale_x).clamp(0.0, w);
        let fy = ((center_ty as f64 - 8.0) * scale_y).clamp(0.0, h);
        let fw = (20.0 * scale_x).clamp(14.0, w);
        let fh = (16.0 * scale_y).clamp(10.0, h);

        ctx.set_stroke_style_str("#38bdf8");
        ctx.set_line_width(1.0);
        ctx.stroke_rect(fx, fy, fw, fh);
    }
}
