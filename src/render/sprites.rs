//! Outpost 2 Sprite Animation and Texture Atlas Pipeline
//!
//! Provides palette decoding, sprite frame extraction from `OP2_ART.BMP` and `op2_art.prt`,
//! 8-directional unit rotation frames, animated vehicle tracks/treads, independent turret mounts,
//! Robo-Miner subterranean drill rotations, and multi-stage fireball explosion effects.

#![allow(dead_code)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsCast;

use crate::archive::prt::{PrtFile, PrtFrame};
use crate::render::palette::MASTER_PALETTE;
use crate::sim::{Faction, Unit};

/// 8-Directional compass heading for vehicle chassis and weapon turrets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction8 {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

impl Direction8 {
    /// Convert an angle in radians (screen coordinates where +X is East, +Y is South) to Direction8.
    pub fn from_angle_rad(rad: f64) -> Self {
        // Normalize angle to [0, 2*PI)
        let two_pi = std::f64::consts::PI * 2.0;
        let mut angle = rad % two_pi;
        if angle < 0.0 {
            angle += two_pi;
        }

        // Each 45-degree sector is PI / 4 radians.
        // We offset by half-sector (PI / 8) so the principal axes are centered.
        let sector = ((angle + (std::f64::consts::PI / 8.0)) / (std::f64::consts::PI / 4.0)).floor() as usize % 8;

        match sector {
            0 => Direction8::East,
            1 => Direction8::SouthEast,
            2 => Direction8::South,
            3 => Direction8::SouthWest,
            4 => Direction8::West,
            5 => Direction8::NorthWest,
            6 => Direction8::North,
            7 => Direction8::NorthEast,
            _ => Direction8::North,
        }
    }

    /// Calculate Direction8 from a 2D screen movement vector (dx, dy).
    pub fn from_screen_vector(dx: f64, dy: f64) -> Self {
        if dx.abs() < 1e-5 && dy.abs() < 1e-5 {
            return Direction8::North;
        }
        let rad = dy.atan2(dx);
        Self::from_angle_rad(rad)
    }

    /// Calculate Direction8 from an isometric tile delta (dtx, dty).
    /// In Outpost 2's 2:1 isometric projection:
    /// sx = (dtx - dty) * (tile_w / 2)
    /// sy = (dtx + dty) * (tile_h / 2)
    pub fn from_tile_delta(dtx: i32, dty: i32) -> Self {
        if dtx == 0 && dty == 0 {
            return Direction8::North;
        }
        let sx = (dtx - dty) as f64 * 32.0;
        let sy = (dtx + dty) as f64 * 16.0;
        Self::from_screen_vector(sx, sy)
    }

    /// Rotational angle frame offset (0..7) clockwise starting from North.
    pub fn frame_offset(self) -> usize {
        match self {
            Direction8::North => 0,
            Direction8::NorthEast => 1,
            Direction8::East => 2,
            Direction8::SouthEast => 3,
            Direction8::South => 4,
            Direction8::SouthWest => 5,
            Direction8::West => 6,
            Direction8::NorthWest => 7,
        }
    }

    /// Construct Direction8 from frame offset index (0..7).
    pub fn from_frame_offset(offset: usize) -> Self {
        match offset % 8 {
            0 => Direction8::North,
            1 => Direction8::NorthEast,
            2 => Direction8::East,
            3 => Direction8::SouthEast,
            4 => Direction8::South,
            5 => Direction8::SouthWest,
            6 => Direction8::West,
            _ => Direction8::NorthWest,
        }
    }

    /// Unit direction unit vector (dx, dy) in 2D screen coordinates.
    pub fn to_screen_vector(self) -> (f64, f64) {
        let diag = 0.70710678118;
        match self {
            Direction8::North => (0.0, -1.0),
            Direction8::NorthEast => (diag, -diag),
            Direction8::East => (1.0, 0.0),
            Direction8::SouthEast => (diag, diag),
            Direction8::South => (0.0, 1.0),
            Direction8::SouthWest => (-diag, diag),
            Direction8::West => (-1.0, 0.0),
            Direction8::NorthWest => (-diag, -diag),
        }
    }

    /// Opposite 180-degree reversed direction.
    pub fn opposite(self) -> Self {
        Self::from_frame_offset((self.frame_offset() + 4) % 8)
    }

    /// Rotate 45 degrees clockwise.
    pub fn rotate_cw(self) -> Self {
        Self::from_frame_offset((self.frame_offset() + 1) % 8)
    }

    /// Rotate 45 degrees counter-clockwise.
    pub fn rotate_ccw(self) -> Self {
        Self::from_frame_offset((self.frame_offset() + 7) % 8)
    }
}

/// Outpost 2 Mobile Vehicle Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VehicleType {
    Lynx,
    Panther,
    Tiger,
    CargoTruck,
    RoboMiner,
    Earthworker,
    ConVec,
    RoboSurveyor,
    RepairVehicle,
}

impl VehicleType {
    /// Detect vehicle type from unit name string.
    pub fn from_unit_name(name: &str) -> Option<Self> {
        let lower = name.to_lowercase();
        if lower.contains("lynx") || lower.contains("scout") {
            Some(VehicleType::Lynx)
        } else if lower.contains("panther") {
            Some(VehicleType::Panther)
        } else if lower.contains("tiger") {
            Some(VehicleType::Tiger)
        } else if lower.contains("cargo") || lower.contains("truck") || lower.contains("hauler") {
            Some(VehicleType::CargoTruck)
        } else if lower.contains("miner") {
            Some(VehicleType::RoboMiner)
        } else if lower.contains("earthworker") || lower.contains("dozer") {
            Some(VehicleType::Earthworker)
        } else if lower.contains("convec") {
            Some(VehicleType::ConVec)
        } else if lower.contains("surveyor") {
            Some(VehicleType::RoboSurveyor)
        } else if lower.contains("repair") {
            Some(VehicleType::RepairVehicle)
        } else {
            None
        }
    }
}

/// Decoded RGBA 32-bit Sprite Image
#[derive(Debug, Clone)]
pub struct DecodedSprite {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA 4 bytes per pixel
}

impl DecodedSprite {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0u8; (width * height * 4) as usize],
        }
    }

    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x < self.width && y < self.height {
            let idx = ((y * self.width + x) * 4) as usize;
            self.pixels[idx] = r;
            self.pixels[idx + 1] = g;
            self.pixels[idx + 2] = b;
            self.pixels[idx + 3] = a;
        }
    }

    #[inline(always)]
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        if x < self.width && y < self.height {
            let idx = ((y * self.width + x) * 4) as usize;
            [
                self.pixels[idx],
                self.pixels[idx + 1],
                self.pixels[idx + 2],
                self.pixels[idx + 3],
            ]
        } else {
            [0, 0, 0, 0]
        }
    }
}

/// Texture Atlas & Sprite Cache Subsystem
pub struct SpriteAtlas {
    pub prt: Option<PrtFile>,
    pub raw_bitmap: Option<Vec<u8>>,
    pub bitmap_pixel_offset: usize,
    pub decoded_cache: HashMap<u32, DecodedSprite>,
    canvas_cache: HashMap<u32, HtmlCanvasElement>,
    group_cache: HashMap<(u32, usize, bool), HtmlCanvasElement>,
    group_bounds_cache: HashMap<u32, (i32, i32, i32, i32)>,
}

impl SpriteAtlas {
    /// Create an empty SpriteAtlas with fallback master palette support.
    pub fn new() -> Self {
        Self {
            prt: None,
            raw_bitmap: None,
            bitmap_pixel_offset: 1078,
            decoded_cache: HashMap::new(),
            canvas_cache: HashMap::new(),
            group_cache: HashMap::new(),
            group_bounds_cache: HashMap::new(),
        }
    }

    /// Load and initialize the atlas directly from parsed PRT bytes and BMP bytes.
    pub fn load_from_prt_and_bmp(prt_bytes: &[u8], bmp_bytes: &[u8]) -> Result<Self, String> {
        let prt = PrtFile::parse(prt_bytes)?;
        let mut atlas = Self::new();
        atlas.set_raw_bitmap(bmp_bytes.to_vec());
        atlas.prt = Some(prt);
        Ok(atlas)
    }

    /// Set or update the raw OP2_ART.BMP byte data.
    pub fn set_raw_bitmap(&mut self, data: Vec<u8>) {
        if data.len() >= 14 {
            // Read bfOffBits from standard BMP header (offset 0x0A)
            let off = u32::from_le_bytes([data[10], data[11], data[12], data[13]]) as usize;
            self.bitmap_pixel_offset = if off > 0 && off < data.len() { off } else { 1078 };
        }
        self.raw_bitmap = Some(data);
        self.decoded_cache.clear();
        self.canvas_cache.clear();
        self.group_cache.clear();
        self.group_bounds_cache.clear();
    }

    pub fn group_bounds(&mut self, group_id: u32) -> Option<(i32, i32, i32, i32)> {
        if let Some(bounds) = self.group_bounds_cache.get(&group_id) {
            return Some(*bounds);
        }
        let bounds = self.prt.as_ref()?.animation_bounds(group_id as usize)?;
        self.group_bounds_cache.insert(group_id, bounds);
        Some(bounds)
    }

    /// Get sprite metadata by ID.
    pub fn get_sprite_meta(&self, sprite_id: u32) -> Option<&PrtFrame> {
        self.prt.as_ref()?.get_sprite(sprite_id as usize)
    }

    /// Extract or decode an RGBA sprite into memory.
    ///
    /// Applies the 256-color palette index mapping:
    /// - Color index 0 is mapped to transparent (alpha = 0).
    /// - Indices 1..255 are mapped to RGB with full opacity (alpha = 255).
    /// - Shadow masks (imageType & 4 != 0) are mapped to semi-transparent black.
    pub fn extract_rgba(&mut self, sprite_id: u32, palette_override: Option<usize>) -> Option<DecodedSprite> {
        if let Some(cached) = self.decoded_cache.get(&sprite_id) {
            return Some(cached.clone());
        }

        let meta = self.get_sprite_meta(sprite_id)?.clone();
        let bmp_data = self.raw_bitmap.as_ref()?;

        let mut sprite = DecodedSprite::new(meta.width, meta.height);
        let pal_idx = palette_override.unwrap_or(meta.palette_index as usize);

        let pal = if let Some(prt) = &self.prt {
            if pal_idx < prt.palettes.len() {
                &prt.palettes[pal_idx]
            } else {
                &MASTER_PALETTE
            }
        } else {
            &MASTER_PALETTE
        };

        let start_pos = self.bitmap_pixel_offset + meta.pixel_offset as usize;
        let scan_w = meta.scan_line_width as usize;

        if meta.is_shadow {
            // 1-bit monochrome shadow mask (each byte contains 8 binary pixels)
            for y in 0..meta.height {
                let row_offset = start_pos + (y as usize * scan_w);
                for x in 0..meta.width {
                    let byte_idx = row_offset + (x as usize / 8);
                    if byte_idx < bmp_data.len() {
                        let bit = (bmp_data[byte_idx] >> (7 - (x % 8))) & 1;
                        if bit != 0 {
                            // Semi-transparent black shadow
                            sprite.set_pixel(x, y, 0, 0, 0, 110);
                        }
                    }
                }
            }
        } else {
            // 8-bit indexed color pixels
            for y in 0..meta.height {
                let row_offset = start_pos + (y as usize * scan_w);
                for x in 0..meta.width {
                    let px_idx = row_offset + x as usize;
                    if px_idx < bmp_data.len() {
                        let c_idx = bmp_data[px_idx] as usize;
                        if c_idx != 0 {
                            let [r, g, b] = pal[c_idx];
                            sprite.set_pixel(x, y, r, g, b, 255);
                        }
                    }
                }
            }
        }

        self.decoded_cache.insert(sprite_id, sprite.clone());
        Some(sprite)
    }

    /// Blit a sprite directly into an RGBA memory pixel buffer with bounds clipping and alpha blending.
    ///
    /// - `dest`: mutable RGBA memory buffer
    /// - `dest_w`, `dest_h`: dimensions of the target buffer
    /// - `dest_pitch`: byte width of a row (typically dest_w * 4)
    /// - `x`, `y`: top-left destination coordinates
    pub fn draw_to_rgba_buffer(
        &mut self,
        sprite_id: u32,
        dest: &mut [u8],
        dest_w: u32,
        dest_h: u32,
        dest_pitch: u32,
        x: i32,
        y: i32,
        palette_override: Option<usize>,
    ) -> bool {
        let sprite = match self.extract_rgba(sprite_id, palette_override) {
            Some(s) => s,
            None => return false,
        };

        let sw = sprite.width as i32;
        let sh = sprite.height as i32;

        // Viewport bounding box clipping
        let start_x = 0.max(-x);
        let start_y = 0.max(-y);
        let end_x = sw.min(dest_w as i32 - x);
        let end_y = sh.min(dest_h as i32 - y);

        if start_x >= end_x || start_y >= end_y {
            return false;
        }

        for sy in start_y..end_y {
            let dy = (y + sy) as usize;
            let dest_row = dy * dest_pitch as usize;
            for sx in start_x..end_x {
                let dx = (x + sx) as usize;
                let dest_idx = dest_row + (dx * 4);

                let [sr, sg, sb, sa] = sprite.get_pixel(sx as u32, sy as u32);
                if sa == 0 {
                    continue;
                } else if sa == 255 {
                    dest[dest_idx] = sr;
                    dest[dest_idx + 1] = sg;
                    dest[dest_idx + 2] = sb;
                    dest[dest_idx + 3] = 255;
                } else {
                    // Alpha blending
                    let inv_a = 255 - sa as u32;
                    let sa32 = sa as u32;
                    let dr = dest[dest_idx] as u32;
                    let dg = dest[dest_idx + 1] as u32;
                    let db = dest[dest_idx + 2] as u32;

                    dest[dest_idx] = ((sr as u32 * sa32 + dr * inv_a) / 255) as u8;
                    dest[dest_idx + 1] = ((sg as u32 * sa32 + dg * inv_a) / 255) as u8;
                    dest[dest_idx + 2] = ((sb as u32 * sa32 + db * inv_a) / 255) as u8;
                    dest[dest_idx + 3] = 255;
                }
            }
        }

        true
    }

    /// Render a sprite to HTML5 CanvasRenderingContext2d.
    pub fn draw_to_canvas(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        sprite_id: u32,
        x: f64,
        y: f64,
        palette_override: Option<usize>,
        scale: f64,
    ) -> Result<(), JsValue> {
        if !self.canvas_cache.contains_key(&sprite_id) {
            let sprite = match self.extract_rgba(sprite_id, palette_override) {
                Some(s) => s,
                None => return Ok(()),
            };
            let canvas = web_sys::window().unwrap().document().unwrap()
                .create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
            canvas.set_width(sprite.width);
            canvas.set_height(sprite.height);
            let offscreen = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;
            let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(&sprite.pixels[..]), sprite.width, sprite.height,
            )?;
            offscreen.put_image_data(&data, 0.0, 0.0)?;
            self.canvas_cache.insert(sprite_id, canvas);
        }

        let canvas = &self.canvas_cache[&sprite_id];
        ctx.draw_image_with_html_canvas_element_and_dw_and_dh(
            canvas, x, y, canvas.width() as f64 * scale, canvas.height() as f64 * scale,
        )?;

        Ok(())
    }

    /// Draw a complete original animation group, including its layered shadow.
    pub fn draw_group(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        group_id: u32,
        center_x: f64,
        bottom_y: f64,
        scale: f64,
        blue_team: bool,
    ) -> Result<bool, JsValue> {
        self.draw_group_frame(ctx, group_id, 0, center_x, bottom_y, scale, blue_team)
    }

    pub fn draw_group_frame(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        group_id: u32,
        frame_index: usize,
        center_x: f64,
        bottom_y: f64,
        scale: f64,
        blue_team: bool,
    ) -> Result<bool, JsValue> {
        let cache_key = (group_id, frame_index, blue_team);
        if !self.group_cache.contains_key(&cache_key) {
            let layers = match self.prt.as_ref()
                .and_then(|prt| prt.animations.get(group_id as usize))
                .and_then(|group| group.frames.get(frame_index)) {
                Some(layers) if !layers.is_empty() => layers.clone(),
                _ => return Ok(false),
            };
            let bounds = match self.group_bounds(group_id) {
                Some(bounds) => bounds,
                None => return Ok(false),
            };
            let canvas = web_sys::window().unwrap().document().unwrap()
                .create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
            canvas.set_width((bounds.2 - bounds.0) as u32);
            canvas.set_height((bounds.3 - bounds.1) as u32);
            let offscreen = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;
            for layer in layers {
                self.draw_to_canvas(
                    &offscreen,
                    layer.bitmap_index as u32,
                    (layer.x as i32 - bounds.0) as f64,
                    (layer.y as i32 - bounds.1) as f64,
                    None,
                    1.0,
                )?;
            }
            // OP2 stores player-color pixels in palette slots 1..=23. The
            // stock art uses red; Eden's blue team palette is applied at draw time.
            if blue_team {
                if let Some(palette) = self.prt.as_ref().and_then(|prt| prt.palettes.get(1)) {
                    let red_ramp = &palette[1..24];
                    let image = offscreen.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)?;
                    let mut pixels = image.data().to_vec();
                    for pixel in pixels.chunks_exact_mut(4) {
                        if pixel[3] > 0 && red_ramp.iter().any(|color| pixel[..3] == color[..]) {
                            let red = pixel[0];
                            let green = pixel[1];
                            pixel[0] = green / 3;
                            pixel[1] = green / 2 + 15;
                            pixel[2] = red;
                        }
                    }
                    let recolored = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                        wasm_bindgen::Clamped(&pixels[..]), canvas.width(), canvas.height(),
                    )?;
                    offscreen.put_image_data(&recolored, 0.0, 0.0)?;
                }
            }
            self.group_cache.insert(cache_key, canvas);
            if self.group_cache.len() > 256 {
                self.group_cache.retain(|key, _| *key == cache_key);
            }
        }
        let canvas = &self.group_cache[&cache_key];
        let w = canvas.width() as f64 * scale;
        let h = canvas.height() as f64 * scale;
        ctx.draw_image_with_html_canvas_element_and_dw_and_dh(
            canvas, center_x - w / 2.0, bottom_y - h, w, h,
        )?;
        Ok(true)
    }
}

/// Unit Animation State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitAnimationState {
    pub unit_id: u32,
    pub vehicle_type: VehicleType,
    pub chassis_dir: Direction8,
    pub turret_dir: Option<Direction8>,
    pub tread_frame: usize,       // 0..=1 tread movement phase
    pub tread_timer: f64,
    pub is_moving: bool,
    pub is_drilling: bool,        // Active mining cycle for Robo-Miner
    pub drill_frame: usize,       // 0..=7 drill rotation cycle
    pub drill_timer: f64,
    pub last_tile: (i32, i32),
}

impl UnitAnimationState {
    pub fn new(unit_id: u32, vehicle_type: VehicleType, tile: (i32, i32)) -> Self {
        Self {
            unit_id,
            vehicle_type,
            chassis_dir: Direction8::SouthEast,
            turret_dir: None,
            tread_frame: 0,
            tread_timer: 0.0,
            is_moving: false,
            is_drilling: false,
            drill_frame: 0,
            drill_timer: 0.0,
            last_tile: tile,
        }
    }
}

/// Explosion Life-Cycle Stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplosionStage {
    Ignition,    // Frames 0..2: Intense white/yellow spark & detonation flash
    Expansion,   // Frames 3..5: Expanding orange/red fireball
    Peak,        // Frames 6..8: Peak fireball shockwave and black smoke bloom
    Dissipation, // Frames 9..11: Dissipating grey smoke cloud and dying embers
}

/// Active Multi-Stage Fireball Explosion Animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosionAnimation {
    pub id: u64,
    pub tile_x: f64,
    pub tile_y: f64,
    pub current_frame: usize, // 0..11
    pub frame_timer: f64,
    pub frame_duration: f64,  // typically 0.05 seconds per frame
    pub is_finished: bool,
    pub scale: f64,
}

impl ExplosionAnimation {
    pub fn new(id: u64, tile_x: f64, tile_y: f64, scale: f64) -> Self {
        Self {
            id,
            tile_x,
            tile_y,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.05,
            is_finished: false,
            scale,
        }
    }

    /// Advance the explosion animation by dt seconds.
    pub fn update(&mut self, dt: f64) {
        if self.is_finished {
            return;
        }

        self.frame_timer += dt;
        while self.frame_timer >= self.frame_duration - 1e-7 {
            self.frame_timer -= self.frame_duration;
            self.current_frame += 1;
            if self.current_frame >= 12 {
                self.is_finished = true;
                break;
            }
        }
    }

    /// Current visual stage of the explosion.
    pub fn stage(&self) -> ExplosionStage {
        match self.current_frame {
            0..=2 => ExplosionStage::Ignition,
            3..=5 => ExplosionStage::Expansion,
            6..=8 => ExplosionStage::Peak,
            _ => ExplosionStage::Dissipation,
        }
    }

    /// Corresponding sprite ID in OP2_ART.PRT (base frame 4449).
    pub fn sprite_id(&self) -> u32 {
        4449 + (self.current_frame.min(11) as u32)
    }
}

/// Unit & Explosion Sprite Pipeline Manager
pub struct UnitSpriteManager {
    pub atlas: SpriteAtlas,
    pub unit_states: HashMap<u32, UnitAnimationState>,
    pub explosions: Vec<ExplosionAnimation>,
    next_explosion_id: u64,
}

impl UnitSpriteManager {
    /// Create a new UnitSpriteManager wrapping a SpriteAtlas.
    pub fn new(atlas: SpriteAtlas) -> Self {
        Self {
            atlas,
            unit_states: HashMap::new(),
            explosions: Vec::new(),
            next_explosion_id: 1,
        }
    }

    /// Get base sprite frame ID for a given vehicle type and faction.
    pub fn get_base_frame(vehicle_type: VehicleType, faction: Faction) -> u32 {
        match vehicle_type {
            VehicleType::Lynx => if faction == Faction::Eden { 1202 } else { 1227 },
            VehicleType::Panther => if faction == Faction::Eden { 239 } else { 335 },
            VehicleType::Tiger => if faction == Faction::Eden { 231 } else { 279 },
            VehicleType::CargoTruck => if faction == Faction::Eden { 8 } else { 24 },
            VehicleType::RoboMiner => if faction == Faction::Eden { 1170 } else { 1186 },
            VehicleType::Earthworker => if faction == Faction::Eden { 1811 } else { 1819 },
            VehicleType::ConVec => if faction == Faction::Eden { 287 } else { 319 },
            VehicleType::RoboSurveyor => if faction == Faction::Eden { 247 } else { 255 },
            VehicleType::RepairVehicle => 1130,
        }
    }

    /// The PRT groups are arranged in directional runs of 8 or 16 entries.
    /// In 16-entry runs, the animated vehicle groups occupy the even IDs;
    /// movement frames live inside each group, not at the following ID.
    pub fn calculate_sprite_group(
        vehicle_type: VehicleType,
        faction: Faction,
        direction: Direction8,
    ) -> u32 {
        let base = Self::get_base_frame(vehicle_type, faction);
        let direction = direction.frame_offset() as u32;
        let has_sixteen = matches!(vehicle_type, VehicleType::Lynx | VehicleType::CargoTruck
            | VehicleType::ConVec | VehicleType::RoboMiner | VehicleType::RepairVehicle);
        if has_sixteen { base + direction * 2 }
        else { base + direction }
    }

    /// Get turret mount sprite frame ID for weapons (Laser, Railgun, Microwave, Acid).
    pub fn calculate_turret_frame(faction: Faction, direction: Direction8, is_heavy: bool) -> u32 {
        let base: u32 = if is_heavy {
            4068
        } else if faction == Faction::Plymouth {
            4060
        } else {
            4052
        };
        base + (direction.frame_offset() as u32)
    }

    /// Spawn a multi-stage fireball explosion at tile coordinates.
    pub fn trigger_explosion(&mut self, tile_x: f64, tile_y: f64, scale: f64) {
        let id = self.next_explosion_id;
        self.next_explosion_id += 1;
        self.explosions.push(ExplosionAnimation::new(id, tile_x, tile_y, scale));
    }

    /// Advance unit animation timers, track movements, update heading directions, and step explosions.
    pub fn update(&mut self, dt: f64, units: &[&Unit]) {
        // 1. Advance active explosions
        for exp in &mut self.explosions {
            exp.update(dt);
        }
        self.explosions.retain(|e| !e.is_finished);

        // 2. Update unit animation tracks
        for unit in units {
            let v_type = match VehicleType::from_unit_name(&unit.name) {
                Some(v) => v,
                None => continue,
            };

            let state = self.unit_states.entry(unit.id).or_insert_with(|| {
                UnitAnimationState::new(unit.id, v_type, (unit.tile_x, unit.tile_y))
            });

            // Check if unit moved tile or has active path
            let current_tile = (unit.tile_x, unit.tile_y);
            let dtx = current_tile.0 - state.last_tile.0;
            let dty = current_tile.1 - state.last_tile.1;

            if dtx != 0 || dty != 0 {
                state.chassis_dir = Direction8::from_tile_delta(dtx, dty);
                state.is_moving = true;
                state.last_tile = current_tile;
            } else if !unit.path.is_empty() {
                let next_tile = unit.path[0];
                let p_dtx = next_tile.0 - unit.tile_x;
                let p_dty = next_tile.1 - unit.tile_y;
                state.chassis_dir = Direction8::from_tile_delta(p_dtx, p_dty);
                state.is_moving = true;
            } else {
                state.is_moving = false;
            }

            // Tread animation ticking
            if state.is_moving {
                state.tread_timer += dt;
                while state.tread_timer >= 0.12 {
                    state.tread_timer -= 0.12;
                    state.tread_frame = state.tread_frame.wrapping_add(1);
                }
            } else {
                state.tread_frame = 0;
            }

            // Turret orientation
            if unit.weapon.is_some() {
                // If attacking or has target, point towards target; otherwise align with chassis
                state.turret_dir = Some(state.chassis_dir);
            } else {
                state.turret_dir = None;
            }

            // Robo-Miner drill animation
            if v_type == VehicleType::RoboMiner {
                state.is_drilling = unit.hauler_state.is_some() || unit.name.contains("Subterranean");
                if state.is_drilling {
                    state.drill_timer += dt;
                    while state.drill_timer >= 0.08 {
                        state.drill_timer -= 0.08;
                        state.drill_frame = (state.drill_frame + 1) % 8;
                    }
                } else {
                    state.drill_frame = 0;
                }
            }
        }
    }

    /// Render a unit using authentic sprite blitting if OP2_ART.BMP is loaded,
    /// or high-fidelity procedural vector sprites matching Outpost 2 art specifications.
    pub fn render_unit(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        unit: &Unit,
        sx: f64,
        sy: f64,
        zoom: f64,
    ) {
        let v_type = VehicleType::from_unit_name(&unit.name).unwrap_or(VehicleType::Lynx);
        let anim_state = self.unit_states.get(&unit.id).cloned().unwrap_or_else(|| {
            UnitAnimationState::new(unit.id, v_type, (unit.tile_x, unit.tile_y))
        });

        let sprite_group = Self::calculate_sprite_group(
            v_type,
            unit.faction,
            anim_state.chassis_dir,
        );

        // Draw the complete original group (chassis, treads, turret and shadow).
        if self.atlas.raw_bitmap.is_some() && self.atlas.prt.is_some() {
            let frame_count = self.atlas.prt.as_ref()
                .and_then(|prt| prt.animations.get(sprite_group as usize))
                .map_or(0, |group| group.frames.len());
            let frame = if anim_state.is_moving && frame_count > 0 {
                anim_state.tread_frame % frame_count
            } else {
                0
            };
            if let Ok(true) = self.atlas.draw_group_frame(ctx, sprite_group, frame, sx, sy + 16.0 * zoom, zoom, unit.faction == Faction::Eden) {
                return;
            }
        }

        // Procedural Vector Sprite Fallback
        self.render_procedural_unit(ctx, unit, v_type, &anim_state, sx, sy, zoom);
    }

    /// High-detail procedural vector sprite renderer for units with animated tracks,
    /// 8-directional chassis orientation, rotatable turrets, and drill animations.
    fn render_procedural_unit(
        &self,
        ctx: &CanvasRenderingContext2d,
        unit: &Unit,
        v_type: VehicleType,
        anim: &UnitAnimationState,
        sx: f64,
        sy: f64,
        zoom: f64,
    ) {
        let (dir_x, dir_y) = anim.chassis_dir.to_screen_vector();
        let angle = dir_y.atan2(dir_x);

        let _ = ctx.save();
        let _ = ctx.translate(sx, sy);
        let _ = ctx.rotate(angle);

        let faction_primary = if unit.faction == Faction::Eden { "#0284c7" } else { "#b91c1c" };
        let faction_accent = if unit.faction == Faction::Eden { "#38bdf8" } else { "#ef4444" };
        let tread_color = if anim.tread_frame == 0 { "#1e293b" } else { "#334155" };

        match v_type {
            VehicleType::Lynx => {
                // Light fast scout tank (30x18 px bounds)
                let half_l = 12.0 * zoom;
                let half_w = 7.0 * zoom;

                // Treads
                ctx.set_fill_style_str(tread_color);
                ctx.fill_rect(-half_l, -half_w - 2.5 * zoom, half_l * 2.0, 2.5 * zoom);
                ctx.fill_rect(-half_l, half_w, half_l * 2.0, 2.5 * zoom);

                // Chassis body
                ctx.set_fill_style_str(faction_primary);
                ctx.fill_rect(-half_l + 2.0 * zoom, -half_w, half_l * 2.0 - 4.0 * zoom, half_w * 2.0);

                // Cockpit canopy / armor slants
                ctx.set_fill_style_str(faction_accent);
                ctx.begin_path();
                let _ = ctx.move_to(half_l - 2.0 * zoom, 0.0);
                let _ = ctx.line_to(0.0, -half_w * 0.7);
                let _ = ctx.line_to(0.0, half_w * 0.7);
                ctx.close_path();
                ctx.fill();

                // Scout Laser mount
                ctx.set_stroke_style_str("#38bdf8");
                ctx.set_line_width(2.0 * zoom);
                ctx.begin_path();
                let _ = ctx.move_to(0.0, 0.0);
                let _ = ctx.line_to(half_l + 4.0 * zoom, 0.0);
                ctx.stroke();
            }
            VehicleType::Panther => {
                // Medium battle tank (36x24 px bounds)
                let half_l = 15.0 * zoom;
                let half_w = 10.0 * zoom;

                // Heavy dual treads
                ctx.set_fill_style_str(tread_color);
                ctx.fill_rect(-half_l, -half_w - 3.0 * zoom, half_l * 2.0, 3.5 * zoom);
                ctx.fill_rect(-half_l, half_w - 0.5 * zoom, half_l * 2.0, 3.5 * zoom);

                // Armored hull
                ctx.set_fill_style_str(faction_primary);
                ctx.fill_rect(-half_l + 3.0 * zoom, -half_w, half_l * 2.0 - 6.0 * zoom, half_w * 2.0);

                // Turret hub
                ctx.set_fill_style_str("#0f172a");
                ctx.begin_path();
                let _ = ctx.arc(0.0, 0.0, 5.0 * zoom, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();

                // Dual weapon barrels
                ctx.set_stroke_style_str(faction_accent);
                ctx.set_line_width(2.5 * zoom);
                ctx.begin_path();
                let _ = ctx.move_to(2.0 * zoom, -2.5 * zoom);
                let _ = ctx.line_to(half_l + 6.0 * zoom, -2.5 * zoom);
                let _ = ctx.move_to(2.0 * zoom, 2.5 * zoom);
                let _ = ctx.line_to(half_l + 6.0 * zoom, 2.5 * zoom);
                ctx.stroke();
            }
            VehicleType::Tiger => {
                // Heavy fortified siege tank (48x32 px bounds)
                let half_l = 18.0 * zoom;
                let half_w = 13.0 * zoom;

                // Reinforced quad tracks
                ctx.set_fill_style_str(tread_color);
                ctx.fill_rect(-half_l, -half_w - 4.0 * zoom, half_l * 2.0, 4.5 * zoom);
                ctx.fill_rect(-half_l, half_w - 0.5 * zoom, half_l * 2.0, 4.5 * zoom);

                // Heavy armor sloped plates
                ctx.set_fill_style_str(faction_primary);
                ctx.fill_rect(-half_l + 4.0 * zoom, -half_w, half_l * 2.0 - 8.0 * zoom, half_w * 2.0);

                // Turret shield
                ctx.set_fill_style_str("#0f172a");
                ctx.begin_path();
                let _ = ctx.arc(0.0, 0.0, 7.0 * zoom, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();

                // Heavy elongated magnetic Railgun cannon
                ctx.set_stroke_style_str("#67e8f9");
                ctx.set_line_width(3.5 * zoom);
                ctx.begin_path();
                let _ = ctx.move_to(0.0, 0.0);
                let _ = ctx.line_to(half_l + 10.0 * zoom, 0.0);
                ctx.stroke();

                // Muzzle brake
                ctx.set_fill_style_str("#e0f2fe");
                ctx.fill_rect(half_l + 8.0 * zoom, -3.0 * zoom, 3.0 * zoom, 6.0 * zoom);
            }
            VehicleType::CargoTruck => {
                // Industrial hauler with cargo bay (34x20 px)
                let half_l = 14.0 * zoom;
                let half_w = 8.0 * zoom;

                // Wheels/tracks
                ctx.set_fill_style_str("#1e293b");
                ctx.fill_rect(-half_l, -half_w - 2.0 * zoom, half_l * 2.0, 2.0 * zoom);
                ctx.fill_rect(-half_l, half_w, half_l * 2.0, 2.0 * zoom);

                // Cab
                ctx.set_fill_style_str(faction_primary);
                ctx.fill_rect(half_l - 6.0 * zoom, -half_w, 6.0 * zoom, half_w * 2.0);

                // Cargo bed
                ctx.set_fill_style_str("#475569");
                ctx.fill_rect(-half_l + 1.0 * zoom, -half_w + 1.0 * zoom, half_l * 2.0 - 8.0 * zoom, half_w * 2.0 - 2.0 * zoom);

                // Cargo payload visual (Common ore = amber, Rare ore = purple, Food = green)
                if let Some((ore_type, _)) = &unit.cargo {
                    let ore_color = if *ore_type == crate::sim::mining::OreType::Rare {
                        "#c084fc"
                    } else {
                        "#f59e0b"
                    };
                    ctx.set_fill_style_str(ore_color);
                    ctx.fill_rect(-half_l + 3.0 * zoom, -half_w + 3.0 * zoom, half_l * 2.0 - 12.0 * zoom, half_w * 2.0 - 6.0 * zoom);
                }
            }
            VehicleType::RoboMiner => {
                // Automated subterranean drill rig (38x26 px)
                let half_l = 15.0 * zoom;
                let half_w = 10.0 * zoom;

                // Heavy caterpillar treads
                ctx.set_fill_style_str(tread_color);
                ctx.fill_rect(-half_l, -half_w - 3.0 * zoom, half_l * 2.0, 3.0 * zoom);
                ctx.fill_rect(-half_l, half_w, half_l * 2.0, 3.0 * zoom);

                // Machine body
                ctx.set_fill_style_str("#475569");
                ctx.fill_rect(-half_l + 2.0 * zoom, -half_w, half_l * 2.0 - 4.0 * zoom, half_w * 2.0);

                // Animated Rotary Drill Bit
                let drill_rot = (anim.drill_frame as f64) * (std::f64::consts::PI / 4.0);
                ctx.set_stroke_style_str("#38bdf8");
                ctx.set_line_width(3.0 * zoom);
                ctx.begin_path();
                let _ = ctx.move_to(half_l, 0.0);
                let _ = ctx.line_to(half_l + 8.0 * zoom, (drill_rot.sin()) * 4.0 * zoom);
                ctx.stroke();

                // Secondary drill teeth
                ctx.set_stroke_style_str("#facc15");
                ctx.set_line_width(1.5 * zoom);
                ctx.begin_path();
                let _ = ctx.move_to(half_l + 3.0 * zoom, -3.0 * zoom);
                let _ = ctx.line_to(half_l + 7.0 * zoom, 3.0 * zoom);
                ctx.stroke();
            }
            VehicleType::Earthworker => {
                // Heavy terraforming / dozer (36x24 px)
                let half_l = 15.0 * zoom;
                let half_w = 9.0 * zoom;

                // Wide stability treads
                ctx.set_fill_style_str(tread_color);
                ctx.fill_rect(-half_l, -half_w - 3.0 * zoom, half_l * 2.0, 3.0 * zoom);
                ctx.fill_rect(-half_l, half_w, half_l * 2.0, 3.0 * zoom);

                // Orange construction body
                ctx.set_fill_style_str("#f59e0b");
                ctx.fill_rect(-half_l + 2.0 * zoom, -half_w, half_l * 2.0 - 4.0 * zoom, half_w * 2.0);

                // Curved dozer blade
                ctx.set_fill_style_str("#e2e8f0");
                ctx.fill_rect(half_l, -half_w - 2.0 * zoom, 3.5 * zoom, half_w * 2.0 + 4.0 * zoom);
            }
            VehicleType::ConVec => {
                // Heavy construction kit transport (34x22 px)
                let half_l = 14.0 * zoom;
                let half_w = 9.0 * zoom;

                // Treads
                ctx.set_fill_style_str(tread_color);
                ctx.fill_rect(-half_l, -half_w - 2.5 * zoom, half_l * 2.0, 2.5 * zoom);
                ctx.fill_rect(-half_l, half_w, half_l * 2.0, 2.5 * zoom);

                // Yellow chassis
                ctx.set_fill_style_str("#eab308");
                ctx.fill_rect(-half_l + 2.0 * zoom, -half_w, half_l * 2.0 - 4.0 * zoom, half_w * 2.0);

                // Structure kit canister container
                ctx.set_fill_style_str("#3b82f6");
                ctx.begin_path();
                let _ = ctx.arc(-2.0 * zoom, 0.0, 6.0 * zoom, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();
            }
            VehicleType::RoboSurveyor => {
                // Autonomous radar scout
                let half_l = 10.0 * zoom;
                let half_w = 6.0 * zoom;
                ctx.set_fill_style_str(faction_primary);
                ctx.fill_rect(-half_l, -half_w, half_l * 2.0, half_w * 2.0);

                // Radar dish
                ctx.set_stroke_style_str("#38bdf8");
                ctx.set_line_width(2.0 * zoom);
                ctx.begin_path();
                let _ = ctx.arc(0.0, 0.0, 4.0 * zoom, 0.0, std::f64::consts::PI);
                ctx.stroke();
            }
            VehicleType::RepairVehicle => {
                // Emerald maintenance van
                let half_l = 12.0 * zoom;
                let half_w = 8.0 * zoom;
                ctx.set_fill_style_str("#10b981");
                ctx.fill_rect(-half_l, -half_w, half_l * 2.0, half_w * 2.0);

                // Cross insignia
                ctx.set_fill_style_str("#ffffff");
                ctx.fill_rect(-1.0 * zoom, -4.0 * zoom, 2.0 * zoom, 8.0 * zoom);
                ctx.fill_rect(-4.0 * zoom, -1.0 * zoom, 8.0 * zoom, 2.0 * zoom);
            }
        }

        let _ = ctx.restore();
    }

    /// Render all active multi-stage fireball explosions onto the canvas.
    pub fn render_explosions(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        tile_to_screen: impl Fn(f64, f64) -> (f64, f64),
        zoom: f64,
    ) {
        for exp in &self.explosions {
            let (sx, sy) = tile_to_screen(exp.tile_x, exp.tile_y);

            // Attempt bitmap blit
            if self.atlas.raw_bitmap.is_some() && self.atlas.prt.is_some() {
                let sprite_id = exp.sprite_id();
                if let Some(m) = self.atlas.get_sprite_meta(sprite_id) {
                    let ox = sx - (m.width as f64 * 0.5 * zoom * exp.scale);
                    let oy = sy - (m.height as f64 * 0.5 * zoom * exp.scale);
                    if self.atlas.draw_to_canvas(ctx, sprite_id, ox, oy, Some(5), zoom * exp.scale).is_ok() {
                        continue;
                    }
                }
            }

            // Procedural multi-stage explosion fallback
            let stage = exp.stage();
            let frame = exp.current_frame as f64;
            let _ = ctx.save();

            match stage {
                ExplosionStage::Ignition => {
                    // Stage 1: Intense white-hot ignition core & radiation sparks
                    let radius = (6.0 + frame * 3.0) * zoom * exp.scale;
                    ctx.set_fill_style_str("#ffffff");
                    ctx.begin_path();
                    let _ = ctx.arc(sx, sy, radius, 0.0, std::f64::consts::PI * 2.0);
                    ctx.fill();

                    ctx.set_stroke_style_str("#fef08a");
                    ctx.set_line_width(2.0 * zoom);
                    ctx.stroke();
                }
                ExplosionStage::Expansion => {
                    // Stage 2: Blooming fireball (inner bright yellow, outer fiery orange)
                    let outer_r = (14.0 + (frame - 3.0) * 4.0) * zoom * exp.scale;
                    let inner_r = outer_r * 0.6;

                    ctx.set_fill_style_str("#ea580c");
                    ctx.begin_path();
                    let _ = ctx.arc(sx, sy, outer_r, 0.0, std::f64::consts::PI * 2.0);
                    ctx.fill();

                    ctx.set_fill_style_str("#facc15");
                    ctx.begin_path();
                    let _ = ctx.arc(sx, sy, inner_r, 0.0, std::f64::consts::PI * 2.0);
                    ctx.fill();
                }
                ExplosionStage::Peak => {
                    // Stage 3: Detonation peak & shockwave rim
                    let blast_r = (24.0 + (frame - 6.0) * 3.0) * zoom * exp.scale;

                    ctx.set_fill_style_str("rgba(239, 68, 68, 0.7)");
                    ctx.begin_path();
                    let _ = ctx.arc(sx, sy, blast_r, 0.0, std::f64::consts::PI * 2.0);
                    ctx.fill();

                    // Expanding shockwave shock ring
                    ctx.set_stroke_style_str("rgba(254, 240, 138, 0.8)");
                    ctx.set_line_width(3.0 * zoom);
                    ctx.begin_path();
                    let _ = ctx.arc(sx, sy, blast_r * 1.25, 0.0, std::f64::consts::PI * 2.0);
                    ctx.stroke();
                }
                ExplosionStage::Dissipation => {
                    // Stage 4: Billowing soot smoke & cooling embers
                    let smoke_r = (22.0 - (frame - 9.0) * 4.0).max(4.0) * zoom * exp.scale;
                    let alpha = (1.0 - (frame - 9.0) / 3.0).clamp(0.1, 0.8);

                    let smoke_color = format!("rgba(30, 41, 59, {:.2})", alpha);
                    ctx.set_fill_style_str(&smoke_color);
                    ctx.begin_path();
                    let _ = ctx.arc(sx - 3.0 * zoom, sy - 4.0 * zoom, smoke_r, 0.0, std::f64::consts::PI * 2.0);
                    let _ = ctx.arc(sx + 4.0 * zoom, sy - 2.0 * zoom, smoke_r * 0.8, 0.0, std::f64::consts::PI * 2.0);
                    ctx.fill();

                    // Orange dying embers
                    ctx.set_fill_style_str("#f97316");
                    ctx.fill_rect(sx - 2.0 * zoom, sy - 5.0 * zoom, 2.0 * zoom, 2.0 * zoom);
                    ctx.fill_rect(sx + 5.0 * zoom, sy + 3.0 * zoom, 2.0 * zoom, 2.0 * zoom);
                }
            }

            let _ = ctx.restore();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_from_angles() {
        // Screen East: 0 radians
        assert_eq!(Direction8::from_angle_rad(0.0), Direction8::East);
        // Screen South: PI / 2 radians (90 deg)
        assert_eq!(Direction8::from_angle_rad(std::f64::consts::FRAC_PI_2), Direction8::South);
        // Screen West: PI radians (180 deg)
        assert_eq!(Direction8::from_angle_rad(std::f64::consts::PI), Direction8::West);
        // Screen North: -PI / 2 radians (-90 deg) or 3*PI/2 (270 deg)
        assert_eq!(Direction8::from_angle_rad(-std::f64::consts::FRAC_PI_2), Direction8::North);
        assert_eq!(Direction8::from_angle_rad(3.0 * std::f64::consts::FRAC_PI_2), Direction8::North);
    }

    #[test]
    fn test_direction_conversions() {
        assert_eq!(Direction8::North.frame_offset(), 0);
        assert_eq!(Direction8::NorthEast.frame_offset(), 1);
        assert_eq!(Direction8::East.frame_offset(), 2);
        assert_eq!(Direction8::SouthEast.frame_offset(), 3);
        assert_eq!(Direction8::South.frame_offset(), 4);
        assert_eq!(Direction8::SouthWest.frame_offset(), 5);
        assert_eq!(Direction8::West.frame_offset(), 6);
        assert_eq!(Direction8::NorthWest.frame_offset(), 7);

        for i in 0..8 {
            let dir = Direction8::from_frame_offset(i);
            assert_eq!(dir.frame_offset(), i);
        }

        assert_eq!(Direction8::North.opposite(), Direction8::South);
        assert_eq!(Direction8::East.opposite(), Direction8::West);
        assert_eq!(Direction8::North.rotate_cw(), Direction8::NorthEast);
        assert_eq!(Direction8::North.rotate_ccw(), Direction8::NorthWest);
    }

    #[test]
    fn test_vehicle_detection() {
        assert_eq!(VehicleType::from_unit_name("Lynx Scout #1"), Some(VehicleType::Lynx));
        assert_eq!(VehicleType::from_unit_name("Panther Tank #2"), Some(VehicleType::Panther));
        assert_eq!(VehicleType::from_unit_name("Tiger Heavy #3"), Some(VehicleType::Tiger));
        assert_eq!(VehicleType::from_unit_name("Cargo Truck #4"), Some(VehicleType::CargoTruck));
        assert_eq!(VehicleType::from_unit_name("Robo-Miner #5"), Some(VehicleType::RoboMiner));
        assert_eq!(VehicleType::from_unit_name("Earthworker #6"), Some(VehicleType::Earthworker));
        assert_eq!(VehicleType::from_unit_name("ConVec #7"), Some(VehicleType::ConVec));
    }

    #[test]
    fn vehicle_tread_frames_are_inside_directional_prt_groups() {
        let art = crate::archive::prt::PrtFile::parse(include_bytes!("../../assets/op2_art.prt")).unwrap();
        let north = UnitSpriteManager::calculate_sprite_group(VehicleType::Lynx, Faction::Eden, Direction8::North);
        let east = UnitSpriteManager::calculate_sprite_group(VehicleType::Lynx, Faction::Eden, Direction8::East);
        assert_eq!(north, 1202);
        assert_eq!(east, 1206);
        assert_eq!(art.animations[north as usize].frames.len(), 6);
        assert_eq!(art.animations[east as usize].frames.len(), 6);
        assert_eq!(art.animations[north as usize + 1].frames.len(), 1);
    }

    #[test]
    fn unit_animation_uses_elapsed_time_and_preserves_fractional_remainder() {
        let mut colony = crate::sim::ColonyState::new(Faction::Eden);
        let mut unit = colony.units.iter_mut().find(|u| u.name.contains("Lynx")).unwrap().clone();
        unit.path.push((unit.tile_x + 1, unit.tile_y));
        let mut manager = UnitSpriteManager::new(SpriteAtlas::new());
        manager.update(0.25, &[&unit]);
        assert_eq!(manager.unit_states[&unit.id].tread_frame, 2);
        manager.update(0.11, &[&unit]);
        assert_eq!(manager.unit_states[&unit.id].tread_frame, 3);
        unit.path.clear();
        manager.update(0.0, &[&unit]);
        assert_eq!(manager.unit_states[&unit.id].tread_frame, 0);
    }

    #[test]
    fn test_explosion_stages() {
        let mut exp = ExplosionAnimation::new(1, 10.0, 10.0, 1.0);
        assert_eq!(exp.stage(), ExplosionStage::Ignition);

        // Advance into Expansion
        exp.update(0.15);
        assert_eq!(exp.stage(), ExplosionStage::Expansion);

        // Advance into Peak
        exp.update(0.15);
        assert_eq!(exp.stage(), ExplosionStage::Peak);

        // Advance into Dissipation
        exp.update(0.15);
        assert_eq!(exp.stage(), ExplosionStage::Dissipation);

        // Finish
        exp.update(0.2);
        assert!(exp.is_finished);
    }

    #[test]
    fn test_buffer_blitting_and_alpha() {
        let mut atlas = SpriteAtlas::new();
        let mut buf = vec![0u8; 16 * 16 * 4];

        // Fill test sprite
        let mut sprite = DecodedSprite::new(4, 4);
        sprite.set_pixel(0, 0, 255, 0, 0, 255);
        sprite.set_pixel(1, 1, 0, 255, 0, 128);
        atlas.decoded_cache.insert(100, sprite);

        let ok = atlas.draw_to_rgba_buffer(100, &mut buf, 16, 16, 16 * 4, 2, 2, None);
        assert!(ok);

        // Check pixel at (2, 2) has red 255
        let idx = (2 * 16 + 2) * 4;
        assert_eq!(buf[idx], 255);
        assert_eq!(buf[idx + 3], 255);
    }
}
