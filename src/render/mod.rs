//! Top-down viewport and HTML5 Canvas renderer

pub mod hud;
pub mod menu;
pub mod novella;
pub mod palette;
pub mod sprites;
pub mod touch;

#[allow(unused_imports)]
pub use hud::{HudRenderer, HudState};
#[allow(unused_imports)]
pub use menu::{
    BriefingScenario, ColonySummary, DisasterFrequency, MenuAction, MenuButton, MenuManager,
    MenuScreen, OptionsState, ResourceAbundance, SkirmishBotSlot, SkirmishConfig,
};
#[allow(unused_imports)]
pub use touch::{
    compute_high_dpi_canvas_dimensions, snap_to_pixel_boundary, HighDpiDimensions,
    TouchGestureEvent, TouchGestureTracker,
};

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
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
    pub sprite_mgr: std::cell::RefCell<sprites::UnitSpriteManager>,
    terrain: Option<HtmlImageElement>,
}

fn building_sprite_frame(name: &str, faction: Faction) -> u32 {
    let eden = faction == Faction::Eden;
    if name.contains("Command Center") { if eden { 1845 } else { 1854 } }
    else if name.contains("Solar") { if eden { 941 } else { 942 } }
    else if name.contains("Rare Ore Smelter") { if eden { 1841 } else { 1010 } }
    else if name.contains("Smelter") { if eden { 1855 } else { 1859 } }
    else if name.contains("Tokamak") { if eden { 2035 } else { 652 } }
    else if name.contains("Vehicle Factory") { if eden { 807 } else { 1853 } }
    else if name.contains("Structure Factory") { if eden { 73 } else { 1860 } }
    else if name.contains("Spaceport") || name.contains("Starship") { if eden { 963 } else { 965 } }
    else if name.contains("Geothermal") { 427 }
    else if name.contains("Guard") { if eden { 648 } else { 939 } }
    else if name.contains("Standard Lab") { if eden { 669 } else { 671 } }
    else if name.contains("Basic Lab") { if eden { 642 } else { 2034 } }
    else if name.contains("Lab") { if eden { 600 } else { 643 } }
    else if name.contains("Residence") { if eden { 610 } else { 612 } }
    else if eden { 647 } else { 645 }
}

fn building_animation_frame(sol: f64, frame_count: usize) -> usize {
    if frame_count == 0 { 0 } else { ((sol * 160.0).max(0.0) as usize) % frame_count }
}

fn covered_tile_rect(x: f64, y: f64, size: f64, dpr: f64) -> (f64, f64, f64, f64) {
    let scale = dpr.max(1.0);
    let left = (x * scale).floor() / scale;
    let top = (y * scale).floor() / scale;
    let right = ((x + size) * scale).ceil() / scale;
    let bottom = ((y + size) * scale).ceil() / scale;
    (left, top, right - left, bottom - top)
}

/// Chimney positions relative to the original sprite canvas.
fn building_exhaust_sources(name: &str, faction: Faction) -> &'static [(f64, f64)] {
    if name.contains("Rare Ore Smelter") {
        if faction == Faction::Eden { &[(0.36, 0.08), (0.55, 0.08)] }
        else { &[(0.72, 0.10)] }
    } else if name.contains("Smelter") {
        if faction == Faction::Eden { &[(0.55, 0.07)] }
        else { &[(0.30, 0.12), (0.78, 0.12)] }
    } else if name.contains("Geothermal") {
        &[(0.32, 0.12), (0.70, 0.12)]
    } else {
        &[]
    }
}

fn draw_building_exhaust(
    ctx: &CanvasRenderingContext2d,
    name: &str,
    faction: Faction,
    building_id: u32,
    sol: f64,
    sprite_left: f64,
    sprite_top: f64,
    sprite_width: f64,
    sprite_height: f64,
    zoom: f64,
) {
    let sources = building_exhaust_sources(name, faction);
    for (source_index, &(source_x, source_y)) in sources.iter().enumerate() {
        let origin_x = sprite_left + sprite_width * source_x;
        let origin_y = sprite_top + sprite_height * source_y;
        for puff in 0..4 {
            let phase = (sol * 30.0 + building_id as f64 * 0.13 + source_index as f64 * 0.41 + puff as f64 * 0.25).rem_euclid(1.0);
            let drift = if source_index % 2 == 0 { 1.0 } else { -1.0 };
            let x = origin_x + drift * phase * 9.0 * zoom;
            let y = origin_y - phase * 25.0 * zoom;
            let radius = (2.0 + phase * 3.0) * zoom;
            let alpha = (1.0 - phase) * 0.36;
            let color = if name.contains("Rare") { (175, 194, 221) }
                else if name.contains("Geothermal") { (221, 226, 225) }
                else { (190, 190, 185) };
            ctx.set_fill_style_str(&format!("rgba({}, {}, {}, {:.3})", color.0, color.1, color.2, alpha));
            ctx.begin_path();
            let _ = ctx.arc(x, y, radius, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }
    }
}

#[cfg(test)]
mod building_animation_tests {
    use super::*;

    #[test]
    fn building_animation_advances_and_wraps_original_frames() {
        let art = crate::archive::prt::PrtFile::parse(include_bytes!("../../assets/op2_art.prt")).unwrap();
        let count = art.animations[1845].frames.len();
        assert!(count > 1);
        assert_eq!(building_animation_frame(0.0, count), 0);
        assert_eq!(building_animation_frame(0.00625, count), 1);
        assert_eq!(building_animation_frame(count as f64 / 160.0, count), 0);
    }

    #[test]
    fn industrial_buildings_have_rooftop_exhaust_sources() {
        assert_eq!(building_exhaust_sources("Common Ore Smelter", Faction::Eden).len(), 1);
        assert_eq!(building_exhaust_sources("Common Ore Smelter", Faction::Plymouth).len(), 2);
        assert_eq!(building_exhaust_sources("Rare Ore Smelter", Faction::Eden).len(), 2);
        assert_eq!(building_exhaust_sources("Geothermal Power Plant", Faction::Eden).len(), 2);
        assert!(building_exhaust_sources("Residence", Faction::Eden).is_empty());
    }
}

#[cfg(test)]
mod camera_tests {
    use super::*;

    fn test_viewport() -> Viewport {
        Viewport {
            camera_x: 0.0, camera_y: 0.0, zoom: 1.0,
            tile_width: 32.0, tile_height: 32.0,
            map_cols: 64, map_rows: 64,
            sprite_mgr: std::cell::RefCell::new(sprites::UnitSpriteManager::new(sprites::SpriteAtlas::new())),
            terrain: None,
        }
    }

    #[test]
    fn wheel_zoom_keeps_the_point_under_the_pointer() {
        let mut viewport = test_viewport();
        viewport.camera_x = -400.0;
        viewport.camera_y = -300.0;
        let before_x = (250.0 - viewport.camera_x) / (viewport.tile_width * viewport.zoom);
        let before_y = (180.0 - viewport.camera_y) / (viewport.tile_height * viewport.zoom);
        viewport.zoom_at(250.0, 180.0, 1.25, 800.0, 600.0);
        let after_x = (250.0 - viewport.camera_x) / (viewport.tile_width * viewport.zoom);
        let after_y = (180.0 - viewport.camera_y) / (viewport.tile_height * viewport.zoom);
        assert!((before_x - after_x).abs() < 1e-9);
        assert!((before_y - after_y).abs() < 1e-9);
    }

    #[test]
    fn panning_stays_inside_the_map_and_centers_small_maps() {
        let mut viewport = test_viewport();
        viewport.pan_by(10_000.0, 10_000.0, 800.0, 600.0);
        assert_eq!((viewport.camera_x, viewport.camera_y), (0.0, 0.0));
        viewport.map_cols = 10;
        viewport.map_rows = 10;
        viewport.clamp_camera(800.0, 600.0);
        assert_eq!((viewport.camera_x, viewport.camera_y), (240.0, 140.0));
    }

    #[test]
    fn neighboring_tiles_cover_the_same_hidpi_edge() {
        let a = covered_tile_rect(13.2, 7.4, 36.8, 1.5);
        let b = covered_tile_rect(50.0, 7.4, 36.8, 1.5);
        assert!(a.0 + a.2 >= b.0);
        assert!(a.1 + a.3 >= b.1);
    }

    #[test]
    fn wide_original_map_tiles_remain_reachable() {
        let mut viewport = test_viewport();
        viewport.map_cols = 128;
        viewport.camera_x = -2500.0;
        let (left, right, _, _) = viewport.visible_tile_bounds(800.0, 600.0, 128, 64);
        assert!(left > 64);
        assert!(right <= 128 && right > left);
    }
}

impl Viewport {
    pub fn new() -> Self {
        let terrain = HtmlImageElement::new().ok();
        if let Some(image) = &terrain {
            image.set_src("assets/terrain.png");
        }
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            tile_width: 32.0,
            tile_height: 32.0,
            map_cols: 64,
            map_rows: 64,
            sprite_mgr: std::cell::RefCell::new(sprites::UnitSpriteManager::new(sprites::SpriteAtlas::new())),
            terrain,
        }
    }

    /// Convert tile grid coordinate to screen pixel coordinates
    pub fn tile_to_screen(&self, tx: f64, ty: f64) -> (f64, f64) {
        let sx = tx * self.tile_width * self.zoom + self.camera_x;
        let sy = ty * self.tile_height * self.zoom + self.camera_y;
        (sx, sy)
    }

    /// Convert screen pixel coordinates (sx, sy) back to tile grid coordinates (tx, ty)
    pub fn screen_to_tile(&self, sx: f64, sy: f64) -> (i32, i32) {
        let tx = ((sx - self.camera_x) / (self.tile_width * self.zoom)).floor() as i32;
        let ty = ((sy - self.camera_y) / (self.tile_height * self.zoom)).floor() as i32;
        (tx, ty)
    }

    pub fn clamp_camera(&mut self, view_width: f64, view_height: f64) {
        let map_width = self.map_cols as f64 * self.tile_width * self.zoom;
        let map_height = self.map_rows as f64 * self.tile_height * self.zoom;
        self.camera_x = if map_width <= view_width {
            (view_width - map_width) * 0.5
        } else {
            self.camera_x.clamp(view_width - map_width, 0.0)
        };
        self.camera_y = if map_height <= view_height {
            (view_height - map_height) * 0.5
        } else {
            self.camera_y.clamp(view_height - map_height, 0.0)
        };
    }

    pub fn pan_by(&mut self, dx: f64, dy: f64, view_width: f64, view_height: f64) {
        self.camera_x += dx;
        self.camera_y += dy;
        self.clamp_camera(view_width, view_height);
    }

    pub fn zoom_at(&mut self, x: f64, y: f64, factor: f64, view_width: f64, view_height: f64) {
        let old_zoom = self.zoom;
        self.zoom = (self.zoom * factor).clamp(0.4, 2.5);
        let ratio = self.zoom / old_zoom;
        self.camera_x = x - (x - self.camera_x) * ratio;
        self.camera_y = y - (y - self.camera_y) * ratio;
        self.clamp_camera(view_width, view_height);
    }

    fn visible_tile_bounds(&self, view_width: f64, view_height: f64, map_width: i32, map_height: i32) -> (i32, i32, i32, i32) {
        let tile_w = self.tile_width * self.zoom;
        let tile_h = self.tile_height * self.zoom;
        let left = ((-self.camera_x / tile_w).floor() as i32 - 1).clamp(0, map_width);
        let right = (((view_width - self.camera_x) / tile_w).ceil() as i32 + 1).clamp(0, map_width);
        let top = ((-self.camera_y / tile_h).floor() as i32 - 1).clamp(0, map_height);
        let bottom = (((view_height - self.camera_y) / tile_h).ceil() as i32 + 1).clamp(0, map_height);
        (left, right, top, bottom)
    }

    /// Render the terrain, colony buildings, units, and active tactical overlays
    pub fn render(
        &self,
        ctx: &CanvasRenderingContext2d,
        colony: &ColonyState,
        frame_dt: f64,
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
        let (start_col, end_col, start_row, end_row) = self.visible_tile_bounds(
            width, height, colony.game_map.width as i32, colony.game_map.height as i32,
        );

        if let Some(terrain) = self.terrain.as_ref().filter(|image| image.natural_width() > 0) {
            let tile_size = 32.0 * self.zoom;
            let dpr = web_sys::window().map_or(1.0, |window| window.device_pixel_ratio());
            for r in start_row..end_row {
                for c in start_col..end_col {
                    let (sx, sy) = self.tile_to_screen(c as f64, r as f64);
                    if sx + tile_size < 0.0 || sx > width || sy + tile_size < 0.0 || sy > height {
                        continue;
                    }
                    let (draw_x, draw_y, draw_w, draw_h) = covered_tile_rect(sx, sy, tile_size, dpr);
                    let fog = colony.fog.get(c as usize, r as usize);
                    if fog == FogState::Unexplored {
                        ctx.set_fill_style_str("#050505");
                        ctx.fill_rect(draw_x, draw_y, draw_w, draw_h);
                        continue;
                    }
                    let mapping = colony.game_map.get_tile(c as u32, r as u32)
                        .map(|tile| tile.tile_mapping as u32).unwrap_or(0);
                    let source_x = (mapping % 64 * 32) as f64;
                    let source_y = (mapping / 64 * 32) as f64;
                    let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        terrain, source_x, source_y, 32.0, 32.0, draw_x, draw_y, draw_w, draw_h,
                    );
                    if let Some(tile) = colony.game_map.get_tile(c as u32, r as u32) {
                        if tile.is_lava {
                            ctx.set_fill_style_str("rgba(229, 73, 29, 0.48)");
                            ctx.fill_rect(draw_x, draw_y, draw_w, draw_h);
                        } else if tile.is_blight {
                            ctx.set_fill_style_str("rgba(119, 47, 139, 0.34)");
                            ctx.fill_rect(draw_x, draw_y, draw_w, draw_h);
                        }
                    }
                    if fog == FogState::Explored {
                        ctx.set_fill_style_str("rgba(0, 0, 0, 0.55)");
                        ctx.fill_rect(draw_x, draw_y, draw_w, draw_h);
                    }
                }
            }
        } else {
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

                }
            }
        }
        }

        // The original transport network is a narrow, square conduit.
        for &(tx, ty) in &colony.tubes {
            let fog = colony.fog.get(tx as usize, ty as usize);
            if fog == FogState::Unexplored {
                continue;
            }

            let (tile_x, tile_y) = self.tile_to_screen(tx as f64, ty as f64);
            let (sx, sy) = (tile_x + 16.0 * self.zoom, tile_y + 16.0 * self.zoom);
            if sx < -50.0 || sx > width + 50.0 || sy < -50.0 || sy > height + 50.0 {
                continue;
            }

            ctx.set_fill_style_str(if fog == FogState::Explored { "#4a4947" } else { "#8d8d89" });
            ctx.fill_rect(sx - 2.0 * self.zoom, sy - 2.0 * self.zoom, 4.0 * self.zoom, 4.0 * self.zoom);
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if colony.tubes.contains(&(tx + dx, ty + dy)) {
                    if dx != 0 {
                        ctx.fill_rect(sx.min(sx + dx as f64 * 16.0 * self.zoom), sy - 2.0 * self.zoom,
                            16.0 * self.zoom, 4.0 * self.zoom);
                    } else {
                        ctx.fill_rect(sx - 2.0 * self.zoom, sy.min(sy + dy as f64 * 16.0 * self.zoom),
                            4.0 * self.zoom, 16.0 * self.zoom);
                    }
                }
            }
        }

        // Render Subterranean Ore Deposits & Active Mine Heads
        for dep in &colony.ore_deposits {
            let fog = colony.fog.get(dep.tile_x as usize, dep.tile_y as usize);
            if fog == FogState::Unexplored {
                continue;
            }

            let (tile_x, tile_y) = self.tile_to_screen(dep.tile_x as f64, dep.tile_y as f64);
            let (sx, sy) = (tile_x + 16.0 * self.zoom, tile_y + 16.0 * self.zoom);

            if dep.has_miner {
                let group = match dep.ore_type {
                    crate::sim::mining::OreType::Common => if colony.faction == Faction::Eden { 1001 } else { 1848 },
                    crate::sim::mining::OreType::Rare => if colony.faction == Faction::Eden { 407 } else { 594 },
                };
                if let Ok(mut sprite_mgr) = self.sprite_mgr.try_borrow_mut() {
                    let _ = sprite_mgr.atlas.draw_group(ctx, group, sx, sy + 16.0 * self.zoom,
                        self.zoom, colony.faction == Faction::Eden);
                }
            } else {
                // Ore beacons are small surface markers, without floating labels.
                ctx.set_stroke_style_str("#d3c29a");
                ctx.set_line_width(2.0 * self.zoom);
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, 7.0 * self.zoom, 0.0, std::f64::consts::TAU);
                ctx.stroke();
                ctx.begin_path();
                ctx.move_to(sx - 4.0 * self.zoom, sy);
                ctx.line_to(sx + 4.0 * self.zoom, sy);
                ctx.move_to(sx, sy - 4.0 * self.zoom);
                ctx.line_to(sx, sy + 4.0 * self.zoom);
                ctx.stroke();
            }
        }

        // Render Buildings (if not in completely unexplored terrain)
        for b in &colony.buildings {
            let fog = colony.fog.get(b.tile_x as usize, b.tile_y as usize);
            if fog == FogState::Unexplored {
                continue;
            }

            let (sx, sy) = self.tile_to_screen(b.tile_x as f64, b.tile_y as f64);
            if b.construction_progress < 100.0 {
                let width = b.width as f64 * 32.0 * self.zoom;
                let height = b.height as f64 * 32.0 * self.zoom;
                ctx.set_fill_style_str("rgba(130, 145, 155, 0.45)");
                ctx.fill_rect(sx, sy, width, height);
                ctx.set_stroke_style_str("#b8c7ce");
                ctx.set_line_width(2.0 * self.zoom);
                ctx.stroke_rect(sx, sy, width, height);
                if b.construction_progress > 0.0 {
                    if let Ok(mut sprite_mgr) = self.sprite_mgr.try_borrow_mut() {
                        let faction = b.faction.unwrap_or(colony.faction);
                        let group = building_sprite_frame(&b.name, faction);
                        if let Some((left, top, right, bottom)) = sprite_mgr.atlas.prt.as_ref()
                            .and_then(|prt| prt.animation_bounds(group as usize)) {
                            let sprite_width = (right - left) as f64 * self.zoom;
                            let sprite_height = (bottom - top) as f64 * self.zoom;
                            let center_x = sx + width * 0.5;
                            let bottom_y = sy + height;
                            let revealed_height = sprite_height * (b.construction_progress / 100.0).clamp(0.0, 1.0);
                            let _ = ctx.save();
                            ctx.begin_path();
                            ctx.rect(center_x - sprite_width * 0.5, bottom_y - revealed_height, sprite_width, revealed_height);
                            ctx.clip();
                            let _ = sprite_mgr.atlas.draw_group_frame(ctx, group, 0, center_x, bottom_y, self.zoom, faction == Faction::Eden);
                            let _ = ctx.restore();
                        }
                    }
                }
                ctx.set_fill_style_str("#0f172a");
                ctx.fill_rect(sx, sy - 8.0 * self.zoom, width, 5.0 * self.zoom);
                ctx.set_fill_style_str("#e2d39a");
                ctx.fill_rect(sx, sy - 8.0 * self.zoom, width * b.construction_progress / 100.0, 5.0 * self.zoom);
                continue;
            }
            if let Ok(mut sprite_mgr) = self.sprite_mgr.try_borrow_mut() {
                let frame = building_sprite_frame(&b.name, b.faction.unwrap_or(colony.faction));
                let frame_count = sprite_mgr.atlas.prt.as_ref()
                    .and_then(|prt| prt.animations.get(frame as usize))
                    .map_or(0, |group| group.frames.len());
                let animation_frame = building_animation_frame(colony.sol, frame_count);
                let center_x = sx + b.width as f64 * self.tile_width * self.zoom * 0.5;
                let bottom_y = sy + b.height as f64 * self.tile_height * self.zoom;
                if let Ok(true) = sprite_mgr.atlas.draw_group_frame(
                    ctx, frame, animation_frame, center_x, bottom_y, self.zoom,
                    b.faction.unwrap_or(colony.faction) == Faction::Eden,
                ) {
                        if b.is_active && fog == FogState::Visible {
                            if let Some((left, top, right, bottom)) = sprite_mgr.atlas.group_bounds(frame) {
                                let sprite_width = (right - left) as f64 * self.zoom;
                                let sprite_height = (bottom - top) as f64 * self.zoom;
                                draw_building_exhaust(
                                    ctx, &b.name, b.faction.unwrap_or(colony.faction), b.id,
                                    colony.sol, center_x - sprite_width * 0.5,
                                    bottom_y - sprite_height, sprite_width, sprite_height, self.zoom,
                                );
                            }
                        }
                        if b.is_selected {
                            ctx.set_stroke_style_str("#b8e868");
                            ctx.set_line_width(1.0);
                            ctx.stroke_rect(sx, sy, b.width as f64 * 32.0 * self.zoom, b.height as f64 * 32.0 * self.zoom);
                        }
                        if b.hp < b.max_hp {
                            ctx.set_fill_style_str("#7f1d1d");
                            ctx.fill_rect(sx, sy - 6.0 * self.zoom, b.width as f64 * 32.0 * self.zoom, 4.0 * self.zoom);
                            ctx.set_fill_style_str("#22c55e");
                            ctx.fill_rect(sx, sy - 6.0 * self.zoom, b.width as f64 * 32.0 * self.zoom * b.hp as f64 / b.max_hp.max(1) as f64, 4.0 * self.zoom);
                        }
                        continue;
                }
            }
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

            // Common & Rare Ore Smelter Industrial Processing Visuals
            if b.name.contains("Smelter") && b.is_active && fog == FogState::Visible {
                let is_rare = b.name.contains("Rare");
                let heat_cycle = (colony.sol * 6.0).sin().abs();

                // Molten crucible vat in center
                let vat_color = if is_rare {
                    format!("rgba(192, 132, 252, {:.2})", 0.6 + 0.35 * heat_cycle)
                } else {
                    format!("rgba(249, 115, 22, {:.2})", 0.6 + 0.35 * heat_cycle)
                };
                ctx.set_fill_style_str(&vat_color);
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, 8.0 * self.zoom, 0.0, std::f64::consts::TAU);
                ctx.fill();

                // Molten heat glow ring
                let ring_color = if is_rare {
                    format!("rgba(168, 85, 247, {:.2})", 0.3 + 0.3 * heat_cycle)
                } else {
                    format!("rgba(239, 68, 68, {:.2})", 0.3 + 0.3 * heat_cycle)
                };
                ctx.set_stroke_style_str(&ring_color);
                ctx.set_line_width(2.0 * self.zoom);
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, (12.0 + 3.0 * heat_cycle) * self.zoom, 0.0, std::f64::consts::TAU);
                ctx.stroke();

                // Smokestack exhaust plumes drifting upwards
                for i in 0..3 {
                    let puff_offset = (colony.sol * 4.0 + i as f64 * 1.8) % 6.0;
                    let puff_y = sy - b_h - (12.0 + puff_offset * 7.0) * self.zoom;
                    let puff_x = sx + (puff_offset * 1.5 - 2.0 + (i as f64 * 4.0 - 4.0)) * self.zoom;
                    let puff_alpha = (1.0 - (puff_offset / 6.0)).max(0.0) * 0.45;
                    let puff_rad = (3.0 + puff_offset * 1.2) * self.zoom;

                    ctx.set_fill_style_str(&if is_rare {
                        format!("rgba(147, 197, 253, {:.2})", puff_alpha)
                    } else {
                        format!("rgba(254, 215, 170, {:.2})", puff_alpha)
                    });
                    ctx.begin_path();
                    let _ = ctx.arc(puff_x, puff_y, puff_rad, 0.0, std::f64::consts::TAU);
                    ctx.fill();
                }
            }

            // Guard Post Automated Perimeter Defense Turret
            if b.name.contains("Guard") && fog == FogState::Visible {
                // Fortified hexagonal bunker core
                ctx.set_fill_style_str(if b.is_active { "#1e293b" } else { "#0f172a" });
                ctx.set_stroke_style_str(if b.is_active { "#ef4444" } else { "#475569" });
                ctx.set_line_width(2.0 * self.zoom);
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, 10.0 * self.zoom, 0.0, std::f64::consts::TAU);
                ctx.fill();
                ctx.stroke();

                if b.is_active {
                    // Rotating Turret Head aiming either toward scan angle or animated rotation
                    let turret_angle = colony.sol * 2.5;
                    let barrel_len = 14.0 * self.zoom;
                    let barrel_end_x = sx + turret_angle.cos() * barrel_len;
                    let barrel_end_y = sy + turret_angle.sin() * barrel_len;

                    // Dual twin laser/plasma barrels
                    let norm_x = -turret_angle.sin() * 3.0 * self.zoom;
                    let norm_y = turret_angle.cos() * 3.0 * self.zoom;

                    ctx.set_stroke_style_str("#ef4444");
                    ctx.set_line_width(2.5 * self.zoom);
                    ctx.begin_path();
                    ctx.move_to(sx + norm_x, sy + norm_y);
                    ctx.line_to(barrel_end_x + norm_x, barrel_end_y + norm_y);
                    ctx.move_to(sx - norm_x, sy - norm_y);
                    ctx.line_to(barrel_end_x - norm_x, barrel_end_y - norm_y);
                    ctx.stroke();

                    // Turret cupola dome
                    ctx.set_fill_style_str("#dc2626");
                    ctx.begin_path();
                    let _ = ctx.arc(sx, sy, 5.0 * self.zoom, 0.0, std::f64::consts::TAU);
                    ctx.fill();

                    // Laser diode targeting reticle indicator
                    ctx.set_fill_style_str("#f87171");
                    ctx.begin_path();
                    let _ = ctx.arc(barrel_end_x, barrel_end_y, 2.0 * self.zoom, 0.0, std::f64::consts::TAU);
                    ctx.fill();
                }
            }

            // Geothermal Power Plant Thermal Generation Visuals
            if b.name.contains("Geothermal") && b.is_active && fog == FogState::Visible {
                let steam_cycle = (colony.sol * 5.0).sin().abs();

                // Volcanic core heat vent
                ctx.set_fill_style_str(&format!("rgba(249, 115, 22, {:.2})", 0.5 + 0.4 * steam_cycle));
                ctx.begin_path();
                let _ = ctx.arc(sx, sy, 7.0 * self.zoom, 0.0, std::f64::consts::TAU);
                ctx.fill();

                // Dual geothermal steam stacks
                let stack_offset = 6.0 * self.zoom;
                for side in [-1.0, 1.0] {
                    let stack_x = sx + side * stack_offset;
                    let stack_y = sy - b_h * 0.4;

                    // Steam plume
                    for p in 0..3 {
                        let puff = (colony.sol * 4.5 + p as f64 * 1.5 + side * 0.7) % 5.0;
                        let puff_y = stack_y - (6.0 + puff * 6.0) * self.zoom;
                        let puff_x = stack_x + (puff * 0.8 - 1.0) * self.zoom;
                        let alpha = (1.0 - puff / 5.0).max(0.0) * 0.4;
                        let rad = (2.5 + puff * 1.2) * self.zoom;

                        ctx.set_fill_style_str(&format!("rgba(241, 245, 249, {:.2})", alpha));
                        ctx.begin_path();
                        let _ = ctx.arc(puff_x, puff_y, rad, 0.0, std::f64::consts::TAU);
                        ctx.fill();
                    }
                }
            }
        }

        // Update Unit Sprite & Animation Pipeline
        let unit_ptrs: Vec<&crate::sim::Unit> = colony.units.iter().collect();
        if let Ok(mut sprite_mgr) = self.sprite_mgr.try_borrow_mut() {
            sprite_mgr.update(frame_dt.clamp(0.0, 0.25), &unit_ptrs);
        }

        // Render Units (Only visible when active in sensor range)
        for u in &colony.units {
            let fog = colony.fog.get(u.tile_x as usize, u.tile_y as usize);
            if fog != FogState::Visible {
                continue;
            }

            let (sx, sy) = self.tile_to_screen(u.tile_x as f64, u.tile_y as f64);

            // Render Unit using authentic sprite blitting or directional vector rendering
            if let Ok(mut sprite_mgr) = self.sprite_mgr.try_borrow_mut() {
                sprite_mgr.render_unit(ctx, u, sx + 16.0 * self.zoom, sy + 16.0 * self.zoom, self.zoom);
            }

            if self.sprite_mgr.borrow().atlas.prt.is_none() {
            if u.name.contains("Repair") {
                ctx.set_fill_style_str("#ecfdf5");
                ctx.set_font(&format!("bold {}px sans-serif", (9.0 * self.zoom).max(8.0)));
                let _ = ctx.fill_text("🔧", sx - 5.0 * self.zoom, sy - 18.0 * self.zoom);
            } else if u.name.contains("Earthworker") {
                ctx.set_fill_style_str("#fef3c7");
                ctx.set_font(&format!("bold {}px sans-serif", (9.0 * self.zoom).max(8.0)));
                let _ = ctx.fill_text("🏗️", sx - 5.0 * self.zoom, sy - 18.0 * self.zoom);
            } else if u.name.contains("ConVec") {
                ctx.set_fill_style_str("#fef08a");
                ctx.set_font(&format!("bold {}px sans-serif", (9.0 * self.zoom).max(8.0)));
                let icon = if u.cargo_kit.is_some() { "📦" } else { "🚛" };
                let _ = ctx.fill_text(icon, sx - 5.0 * self.zoom, sy - 18.0 * self.zoom);
            } else if u.name.contains("Tiger") {
                ctx.set_fill_style_str("#e0f2fe");
                ctx.set_font(&format!("bold {}px sans-serif", (9.0 * self.zoom).max(8.0)));
                let _ = ctx.fill_text("🐅", sx - 5.0 * self.zoom, sy - 18.0 * self.zoom);
            } else if u.name.contains("Robo-Miner") {
                ctx.set_fill_style_str("#fef08a");
                ctx.set_font(&format!("bold {}px sans-serif", (9.0 * self.zoom).max(8.0)));
                let _ = ctx.fill_text("⛏️", sx - 5.0 * self.zoom, sy - 18.0 * self.zoom);
            }

            // Stance badge (Guard or Patrol)
            match u.current_order {
                crate::sim::pathfinding::UnitOrder::Guard { .. } => {
                    ctx.set_font(&format!("bold {}px sans-serif", (8.0 * self.zoom).max(7.0)));
                    let _ = ctx.fill_text("🛡️", sx + 6.0 * self.zoom, sy - 10.0 * self.zoom);
                }
                crate::sim::pathfinding::UnitOrder::Patrol { .. } => {
                    ctx.set_font(&format!("bold {}px sans-serif", (8.0 * self.zoom).max(7.0)));
                    let _ = ctx.fill_text("🔄", sx + 6.0 * self.zoom, sy - 10.0 * self.zoom);
                }
                _ => {}
            }
            }

            // Four corner brackets match the original unit selection marker.
            if u.is_selected {
                let cx = sx + 16.0 * self.zoom;
                let cy = sy + 16.0 * self.zoom;
                let extent = 13.0 * self.zoom;
                let corner = 5.0 * self.zoom;
                ctx.set_stroke_style_str("#eeeeea");
                ctx.set_line_width(1.0 * self.zoom);
                ctx.begin_path();
                for (side_x, side_y) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
                    let x = cx + side_x * extent;
                    let y = cy + side_y * extent;
                    ctx.move_to(x - side_x * corner, y);
                    ctx.line_to(x, y);
                    ctx.line_to(x, y - side_y * corner);
                }
                ctx.stroke();
            }

            // Health bar
            if u.is_selected || u.hp < u.max_hp {
                let hp_pct = u.hp as f64 / u.max_hp as f64;
                ctx.set_fill_style_str("#dc2626");
                ctx.fill_rect(sx + 4.0 * self.zoom, sy - 5.0 * self.zoom, 24.0 * self.zoom, 3.0);
                ctx.set_fill_style_str("#16a34a");
                ctx.fill_rect(sx + 4.0 * self.zoom, sy - 5.0 * self.zoom, 24.0 * self.zoom * hp_pct, 3.0);
            }

            // Cargo hauler container payload
            if self.sprite_mgr.borrow().atlas.prt.is_none() {
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

        // Render Multi-Stage Sprite Fireball Detonations
        if let Ok(mut sprite_mgr) = self.sprite_mgr.try_borrow_mut() {
            sprite_mgr.render_explosions(ctx, |tx, ty| self.tile_to_screen(tx, ty), self.zoom);
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

        // Render Ascending Evacuation Starship & Ignition Plumes
        if colony.starship_launched {
            let (base_sx, base_sy) = self.tile_to_screen(24.0, 20.0);
            let t = colony.launch_anim_time;
            let ship_y = base_sy - (t * 80.0 + t.powi(2) * 20.0) * self.zoom;
            let ship_x = base_sx;

            // Exhaust flame plume
            let flame_len = (45.0 + (t * 30.0).sin() * 8.0) * self.zoom;
            ctx.set_fill_style_str("#fb923c");
            ctx.begin_path();
            ctx.move_to(ship_x - 8.0 * self.zoom, ship_y + 15.0 * self.zoom);
            ctx.line_to(ship_x + 8.0 * self.zoom, ship_y + 15.0 * self.zoom);
            ctx.line_to(ship_x, ship_y + 15.0 * self.zoom + flame_len);
            ctx.close_path();
            ctx.fill();

            // Inner white-hot flame
            ctx.set_fill_style_str("#ffffff");
            ctx.begin_path();
            ctx.move_to(ship_x - 4.0 * self.zoom, ship_y + 15.0 * self.zoom);
            ctx.line_to(ship_x + 4.0 * self.zoom, ship_y + 15.0 * self.zoom);
            ctx.line_to(ship_x, ship_y + 15.0 * self.zoom + flame_len * 0.55);
            ctx.close_path();
            ctx.fill();

            // Billowing launch smoke at ground pad
            let pad_smoke_r = (20.0 + t * 25.0).min(65.0) * self.zoom;
            ctx.set_fill_style_str("rgba(203, 213, 225, 0.45)");
            ctx.begin_path();
            let _ = ctx.arc(base_sx, base_sy, pad_smoke_r, 0.0, std::f64::consts::TAU);
            ctx.fill();

            // Starship Hull (Streamlined Apollo / OP2 Colony Ark)
            ctx.set_fill_style_str("#e2e8f0");
            ctx.begin_path();
            ctx.move_to(ship_x, ship_y - 30.0 * self.zoom); // Nose cone
            ctx.line_to(ship_x + 10.0 * self.zoom, ship_y + 10.0 * self.zoom);
            ctx.line_to(ship_x + 14.0 * self.zoom, ship_y + 16.0 * self.zoom); // Right booster fin
            ctx.line_to(ship_x - 14.0 * self.zoom, ship_y + 16.0 * self.zoom); // Left booster fin
            ctx.line_to(ship_x - 10.0 * self.zoom, ship_y + 10.0 * self.zoom);
            ctx.close_path();
            ctx.fill();

            ctx.set_stroke_style_str("#0284c7");
            ctx.set_line_width(2.0);
            ctx.stroke();

            // Cockpit visor
            ctx.set_fill_style_str("#0284c7");
            ctx.fill_rect(ship_x - 4.0 * self.zoom, ship_y - 12.0 * self.zoom, 8.0 * self.zoom, 4.0 * self.zoom);
        }

        // Render Holographic Ghost Blueprint Placement
        if let Some((st, (gx, gy), is_valid)) = ghost_placement {
            let (sx, sy) = self.tile_to_screen(gx as f64, gy as f64);
            let (w, h) = st.dimensions();
            let b_w = self.tile_width * self.zoom * w as f64;
            let b_h = self.tile_height * self.zoom * h as f64;

            let fill_col = if is_valid { "rgba(56, 189, 248, 0.35)" } else { "rgba(239, 68, 68, 0.35)" };
            let stroke_col = if is_valid { "#38bdf8" } else { "#ef4444" };

            ctx.set_fill_style_str(fill_col);
            ctx.set_stroke_style_str(stroke_col);
            ctx.set_line_width(2.0);
            ctx.fill_rect(sx, sy, b_w, b_h);
            ctx.stroke_rect(sx, sy, b_w, b_h);

            // Projected transport tube connection to nearest structure
            if let Some(nearest) = colony.buildings.iter().min_by_key(|b| (b.tile_x - gx).abs() + (b.tile_y - gy).abs()) {
                let (nx, ny) = self.tile_to_screen(nearest.tile_x as f64 + (nearest.width as f64) * 0.5, nearest.tile_y as f64 + (nearest.height as f64) * 0.5);
                ctx.set_stroke_style_str("rgba(56, 189, 248, 0.7)");
                ctx.set_line_width(1.5);
                ctx.begin_path();
                ctx.move_to(sx + b_w * 0.5, sy + b_h * 0.5);
                ctx.line_to(nx, ny);
                ctx.stroke();
            }

            // Blueprint Header Badge
            let status_msg = if is_valid { "READY TO DEPLOY" } else { "OBSTRUCTED / INSUFFICIENT ORE" };
            ctx.set_fill_style_str(if is_valid { "#4ade80" } else { "#ef4444" });
            ctx.set_font(&format!("bold {}px sans-serif", (11.0 * self.zoom).max(10.0)));
            let badge_text = format!("{} [{}]", st.name(), status_msg);
            let _ = ctx.fill_text(&badge_text, sx, sy - 7.0 * self.zoom);
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
    #[allow(dead_code)]
    pub fn render_minimap(&self, ctx: &CanvasRenderingContext2d, colony: &ColonyState, w: f64, h: f64) {
        let hud = hud::HudRenderer::new(self.map_cols, self.map_rows, w, h);
        hud.render_radar(
            ctx,
            colony,
            self.camera_x,
            self.camera_y,
            800.0,
            600.0,
            self.tile_width,
            self.tile_height,
            self.zoom,
            w,
            h,
        );
    }
}
