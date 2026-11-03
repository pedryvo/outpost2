//! Isometric Viewport and HTML5 Canvas Renderer

pub mod palette;

use web_sys::CanvasRenderingContext2d;
use crate::sim::{ColonyState, Faction};
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

    /// Render the terrain, colony buildings, and units
    pub fn render(&self, ctx: &CanvasRenderingContext2d, colony: &ColonyState, width: f64, height: f64) {
        // Clear background (Deep space black)
        ctx.set_fill_style_str("#07090e");
        ctx.fill_rect(0.0, 0.0, width, height);

        let half_w = self.tile_width * 0.5 * self.zoom;
        let half_h = self.tile_height * 0.5 * self.zoom;

        // Render visible isometric terrain tiles
        let start_col = 0.max(10);
        let end_col = 40.min(self.map_cols);
        let start_row = 0.max(10);
        let end_row = 40.min(self.map_rows);

        for r in start_row..end_row {
            for c in start_col..end_col {
                let (sx, sy) = self.tile_to_screen(c as f64, r as f64);

                // Viewport culling
                if sx < -100.0 || sx > width + 100.0 || sy < -100.0 || sy > height + 100.0 {
                    continue;
                }

                // Authentic New Terra terrain coloration mapped through the 256 master palette
                let pal_idx = match (c + r * 3) % 7 {
                    0 | 1 => 45, // Iron-oxide red
                    2 | 3 => 52, // Highland clay
                    4 => 18,     // Basalt crater
                    _ => 78,     // Ore-rich soil
                };
                let [pr, pg, pb] = MASTER_PALETTE[pal_idx];
                let terrain_color = format!("rgb({},{},{})", pr, pg, pb);

                ctx.set_fill_style_str(&terrain_color);
                ctx.set_stroke_style_str("#1c1917");
                ctx.set_line_width(0.75);

                ctx.begin_path();
                ctx.move_to(sx, sy - half_h);
                ctx.line_to(sx + half_w, sy);
                ctx.line_to(sx, sy + half_h);
                ctx.line_to(sx - half_w, sy);
                ctx.close_path();

                ctx.fill();
                ctx.stroke();
            }
        }

        // Render Buildings
        for b in &colony.buildings {
            let (sx, sy) = self.tile_to_screen(b.tile_x as f64, b.tile_y as f64);
            let b_w = half_w * (b.width as f64);
            let b_h = half_h * (b.height as f64);

            // Structure footprint
            ctx.set_fill_style_str("#1e293b");
            ctx.set_stroke_style_str(if colony.faction == Faction::Eden { "#00d4ff" } else { "#ff8c00" });
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
            ctx.set_fill_style_str("#334155");
            ctx.fill_rect(sx - b_w * 0.5, sy - b_h - 20.0 * self.zoom, b_w, 20.0 * self.zoom);

            // Label
            ctx.set_fill_style_str("#ffffff");
            ctx.set_font(&format!("bold {}px sans-serif", (11.0 * self.zoom).max(9.0)));
            let _ = ctx.fill_text(&b.name, sx - b_w * 0.45, sy - b_h - 6.0 * self.zoom);
        }

        // Render Units
        for u in &colony.units {
            let (sx, sy) = self.tile_to_screen(u.tile_x as f64, u.tile_y as f64);

            // Unit chassis
            ctx.set_fill_style_str(if colony.faction == Faction::Eden { "#38bdf8" } else { "#fb923c" });
            ctx.begin_path();
            let _ = ctx.arc(sx, sy, 8.0 * self.zoom, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();

            ctx.set_stroke_style_str("#0f172a");
            ctx.set_line_width(1.5);
            ctx.stroke();

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
        }
    }

    /// Render mini-radar overview
    pub fn render_minimap(&self, ctx: &CanvasRenderingContext2d, colony: &ColonyState, w: f64, h: f64) {
        ctx.set_fill_style_str("#090d16");
        ctx.fill_rect(0.0, 0.0, w, h);

        let scale_x = w / (self.map_cols as f64);
        let scale_y = h / (self.map_rows as f64);

        // Blip buildings
        ctx.set_fill_style_str("#38bdf8");
        for b in &colony.buildings {
            let bx = (b.tile_x as f64) * scale_x;
            let by = (b.tile_y as f64) * scale_y;
            ctx.fill_rect(bx, by, (b.width as f64) * scale_x + 1.0, (b.height as f64) * scale_y + 1.0);
        }

        // Blip units
        ctx.set_fill_style_str("#4ade80");
        for u in &colony.units {
            let ux = (u.tile_x as f64) * scale_x;
            let uy = (u.tile_y as f64) * scale_y;
            ctx.fill_rect(ux - 1.0, uy - 1.0, 3.0, 3.0);
        }
    }
}
