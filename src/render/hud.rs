//! Outpost 2 Modern WebHUD & Tactical Command Dashboard Subsystem
//!
//! Provides the authentic Outpost 2 Divided Destiny aesthetic HUD and command interface:
//! - Tactical Minimap Radar Display with disaster radar blips, color-coded factions, and interactive panning
//! - Colony Command Dashboard & Resource Status Bar with ore rates, food sustenance, power grid brownout/blackout alerts, morale gauge, and Sol counter
//! - Factory Production & ConVec Kit Queue Panel with hulls, weapons, kits, progress ring/bar, resource validation, and rally-point dispatch
//! - University Research & Tech Tree Browser with prerequisite gating, cost evaluation, descriptions, and progress tracking
//! - Tactical Notification Dispatcher & In-Game Chat HUD with priority-coded entries, radio transcripts, timestamps, and log scrolling
//! - Responsive Keyboard Shortcut System with Space (jump to alert), P (pause/resume), 1-9 (unit control groups), F (factory view), R (research view), B (base overview), and M (minimap detail toggle)

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use web_sys::CanvasRenderingContext2d;

use crate::sim::{
    combat::WeaponType,
    fog::FogState,
    tech::TechTree,
    ColonyState, Faction,
};

// =========================================================================
// 1. Tactical Minimap Radar Projection & Math
// =========================================================================

/// Projects isometric game coordinates to and from 2D radar minimap pixels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarProjection {
    pub map_cols: i32,
    pub map_rows: i32,
    pub radar_width: f64,
    pub radar_height: f64,
    pub show_detail: bool,
}

impl RadarProjection {
    /// Construct a new projection instance with map dimensions and minimap pixel bounds.
    pub fn new(cols: i32, rows: i32, width: f64, height: f64) -> Self {
        Self {
            map_cols: cols.max(1),
            map_rows: rows.max(1),
            radar_width: width.max(1.0),
            radar_height: height.max(1.0),
            show_detail: false,
        }
    }

    /// Convert tile grid coordinate to minimap pixel coordinate.
    pub fn world_tile_to_minimap(&self, tx: f64, ty: f64) -> (f64, f64) {
        let mx = (tx / (self.map_cols as f64)) * self.radar_width;
        let my = (ty / (self.map_rows as f64)) * self.radar_height;
        (mx, my)
    }

    /// Convert minimap pixel coordinate to tile grid coordinate (clamped).
    pub fn minimap_to_world_tile(&self, mx: f64, my: f64) -> (f64, f64) {
        let norm_x = (mx / self.radar_width).clamp(0.0, 1.0);
        let norm_y = (my / self.radar_height).clamp(0.0, 1.0);
        let tx = norm_x * (self.map_cols as f64);
        let ty = norm_y * (self.map_rows as f64);
        (tx, ty)
    }

    /// Compute camera offsets `(camera_x, camera_y)` to center the viewport on a clicked minimap pixel.
    pub fn calculate_camera_pan(
        click_x: f64,
        click_y: f64,
        radar_w: f64,
        radar_h: f64,
        screen_w: f64,
        screen_h: f64,
        tile_w: f64,
        tile_h: f64,
        zoom: f64,
        cols: i32,
        rows: i32,
    ) -> (f64, f64) {
        let norm_x = (click_x / radar_w.max(1.0)).clamp(0.0, 1.0);
        let norm_y = (click_y / radar_h.max(1.0)).clamp(0.0, 1.0);

        let target_tx = norm_x * (cols as f64);
        let target_ty = norm_y * (rows as f64);

        let camera_x = screen_w * 0.5 - target_tx * tile_w * zoom;
        let camera_y = screen_h * 0.5 - target_ty * tile_h * zoom;

        (camera_x, camera_y)
    }

    /// Calculate the camera frustum bounding box on the minimap in pixels `(x, y, w, h)`.
    pub fn calculate_viewport_frustum_rect(
        camera_x: f64,
        camera_y: f64,
        screen_w: f64,
        screen_h: f64,
        tile_w: f64,
        tile_h: f64,
        zoom: f64,
        radar_w: f64,
        radar_h: f64,
        cols: i32,
        rows: i32,
    ) -> (f64, f64, f64, f64) {
        let min_tx = -camera_x / (tile_w * zoom);
        let min_ty = -camera_y / (tile_h * zoom);
        let max_tx = (screen_w - camera_x) / (tile_w * zoom);
        let max_ty = (screen_h - camera_y) / (tile_h * zoom);

        let scale_x = radar_w / (cols as f64);
        let scale_y = radar_h / (rows as f64);

        let fx = (min_tx * scale_x).clamp(0.0, radar_w);
        let fy = (min_ty * scale_y).clamp(0.0, radar_h);
        let fw = ((max_tx - min_tx) * scale_x).clamp(12.0, radar_w);
        let fh = ((max_ty - min_ty) * scale_y).clamp(8.0, radar_h);

        (fx, fy, fw, fh)
    }

    /// Toggle tactical detail overlay mode on the radar.
    pub fn toggle_detail(&mut self) -> bool {
        self.show_detail = !self.show_detail;
        self.show_detail
    }
}

// =========================================================================
// 2. Colony Command Dashboard & Resource Status Bar
// =========================================================================

/// Energy grid operational condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyGridState {
    Normal,
    Brownout,
    Blackout,
}

impl EnergyGridState {
    /// Evaluate grid state from produced and required power.
    pub fn from_power(produced: u32, required: u32) -> Self {
        if produced >= required {
            EnergyGridState::Normal
        } else if produced == 0 && required > 0 {
            EnergyGridState::Blackout
        } else {
            let deficit = required - produced;
            if deficit >= (required / 2) {
                EnergyGridState::Blackout
            } else {
                EnergyGridState::Brownout
            }
        }
    }

    /// Color hex code for UI display.
    pub fn alert_color(&self) -> &'static str {
        match self {
            EnergyGridState::Normal => "#22c55e",
            EnergyGridState::Brownout => "#f59e0b",
            EnergyGridState::Blackout => "#ef4444",
        }
    }

    /// Text label for HUD status.
    pub fn status_label(&self) -> &'static str {
        match self {
            EnergyGridState::Normal => "GRID OPTIMAL",
            EnergyGridState::Brownout => "BROWNOUT ALERT",
            EnergyGridState::Blackout => "BLACKOUT CRITICAL",
        }
    }

    /// Localized text label for HUD status.
    pub fn status_label_localized(&self, lang: crate::i18n::Language) -> &'static str {
        crate::i18n::get_dictionary(lang).energy_grid_status(*self)
    }
}

/// Colonial morale tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoraleTier {
    Excellent,
    Good,
    Fair,
    Poor,
    Rioting,
}

impl MoraleTier {
    /// Classify morale percentage into canonical Outpost 2 tier.
    pub fn from_percentage(pct: u32) -> Self {
        match pct {
            90..=u32::MAX => MoraleTier::Excellent,
            75..=89 => MoraleTier::Good,
            50..=74 => MoraleTier::Fair,
            25..=49 => MoraleTier::Poor,
            _ => MoraleTier::Rioting,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            MoraleTier::Excellent => "Excellent",
            MoraleTier::Good => "Good",
            MoraleTier::Fair => "Fair",
            MoraleTier::Poor => "Poor",
            MoraleTier::Rioting => "Rioting",
        }
    }

    pub fn label_localized(&self, lang: crate::i18n::Language) -> &'static str {
        crate::i18n::get_dictionary(lang).morale_tier(*self)
    }

    pub fn color(&self) -> &'static str {
        match self {
            MoraleTier::Excellent => "#4ade80",
            MoraleTier::Good => "#a3e635",
            MoraleTier::Fair => "#facc15",
            MoraleTier::Poor => "#fb923c",
            MoraleTier::Rioting => "#ef4444",
        }
    }
}

/// Population sustenance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SustenanceState {
    Surplus,
    Adequate,
    Deficit,
    Starvation,
}

impl SustenanceState {
    pub fn evaluate(food_stock: u32, population: u32) -> Self {
        if population == 0 {
            return SustenanceState::Surplus;
        }
        let per_capita = food_stock / population;
        if food_stock == 0 {
            SustenanceState::Starvation
        } else if per_capita >= 10 {
            SustenanceState::Surplus
        } else if per_capita >= 3 {
            SustenanceState::Adequate
        } else {
            SustenanceState::Deficit
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SustenanceState::Surplus => "Surplus",
            SustenanceState::Adequate => "Adequate",
            SustenanceState::Deficit => "Rationing",
            SustenanceState::Starvation => "STARVATION",
        }
    }

    pub fn label_localized(&self, lang: crate::i18n::Language) -> &'static str {
        crate::i18n::get_dictionary(lang).sustenance_state(*self)
    }

    pub fn color(&self) -> &'static str {
        match self {
            SustenanceState::Surplus => "#4ade80",
            SustenanceState::Adequate => "#38bdf8",
            SustenanceState::Deficit => "#f59e0b",
            SustenanceState::Starvation => "#ef4444",
        }
    }
}

/// Real-time rate telemetry for colonial stockpile income and consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRateTracker {
    pub last_sol: f64,
    pub last_common_ore: u32,
    pub last_rare_ore: u32,
    pub last_food: u32,
    pub common_rate: f64,
    pub rare_rate: f64,
    pub food_rate: f64,
}

impl Default for ResourceRateTracker {
    fn default() -> Self {
        Self {
            last_sol: 0.0,
            last_common_ore: 0,
            last_rare_ore: 0,
            last_food: 0,
            common_rate: 0.0,
            rare_rate: 0.0,
            food_rate: 0.0,
        }
    }
}

impl ResourceRateTracker {
    /// Update tracking metrics and compute per-Sol deltas.
    pub fn update(&mut self, current_sol: f64, common: u32, rare: u32, food: u32) {
        let delta_sol = current_sol - self.last_sol;
        if delta_sol >= 0.05 {
            self.common_rate = (common as f64 - self.last_common_ore as f64) / delta_sol;
            self.rare_rate = (rare as f64 - self.last_rare_ore as f64) / delta_sol;
            self.food_rate = (food as f64 - self.last_food as f64) / delta_sol;

            self.last_sol = current_sol;
            self.last_common_ore = common;
            self.last_rare_ore = rare;
            self.last_food = food;
        }
    }
}

// =========================================================================
// 3. Factory Production & ConVec Kit Queue Panel
// =========================================================================

/// Vehicle chassis hulls manufactured at colonial factories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VehicleHull {
    Lynx,
    Panther,
    Tiger,
    CargoTruck,
    ConVec,
    RoboMiner,
    RoboSurveyor,
    Earthworker,
}

impl VehicleHull {
    pub fn all() -> &'static [VehicleHull] {
        &[
            VehicleHull::Lynx,
            VehicleHull::Panther,
            VehicleHull::Tiger,
            VehicleHull::CargoTruck,
            VehicleHull::ConVec,
            VehicleHull::RoboMiner,
            VehicleHull::RoboSurveyor,
            VehicleHull::Earthworker,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            VehicleHull::Lynx => "Lynx Scout",
            VehicleHull::Panther => "Panther Battle Tank",
            VehicleHull::Tiger => "Tiger Heavy Tank",
            VehicleHull::CargoTruck => "Cargo Truck",
            VehicleHull::ConVec => "ConVec Transport",
            VehicleHull::RoboMiner => "Robo-Miner",
            VehicleHull::RoboSurveyor => "Robo-Surveyor",
            VehicleHull::Earthworker => "Earthworker",
        }
    }

    pub fn display_name_localized(&self, lang: crate::i18n::Language) -> &'static str {
        crate::i18n::get_dictionary(lang).vehicle_name(self)
    }

    pub fn base_cost(&self) -> (u32, u32) {
        match self {
            VehicleHull::Lynx => (140, 0),
            VehicleHull::Panther => (250, 50),
            VehicleHull::Tiger => (350, 100),
            VehicleHull::CargoTruck => (150, 0),
            VehicleHull::ConVec => (150, 0),
            VehicleHull::RoboMiner => (200, 40),
            VehicleHull::RoboSurveyor => (130, 10),
            VehicleHull::Earthworker => (160, 0),
        }
    }

    pub fn max_hp(&self) -> u32 {
        match self {
            VehicleHull::Lynx => 140,
            VehicleHull::Panther => 250,
            VehicleHull::Tiger => 420,
            VehicleHull::CargoTruck => 150,
            VehicleHull::ConVec => 180,
            VehicleHull::RoboMiner => 200,
            VehicleHull::RoboSurveyor => 130,
            VehicleHull::Earthworker => 200,
        }
    }

    pub fn can_mount_weapon(&self) -> bool {
        matches!(self, VehicleHull::Lynx | VehicleHull::Panther | VehicleHull::Tiger)
    }

    pub fn can_carry_kit(&self) -> bool {
        matches!(self, VehicleHull::ConVec)
    }
}

/// Weapon mount attachments for combat chassis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponAttachment {
    Laser,
    Microwave,
    Railgun,
    RPG,
    AcidCloud,
}

impl WeaponAttachment {
    pub fn all() -> &'static [WeaponAttachment] {
        &[
            WeaponAttachment::Laser,
            WeaponAttachment::Microwave,
            WeaponAttachment::Railgun,
            WeaponAttachment::RPG,
            WeaponAttachment::AcidCloud,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            WeaponAttachment::Laser => "High-Energy Pulse Laser",
            WeaponAttachment::Microwave => "Microwave Thermal Emitter",
            WeaponAttachment::Railgun => "Hyper-Velocity Railgun",
            WeaponAttachment::RPG => "Rocket Pod (RPG)",
            WeaponAttachment::AcidCloud => "Acid Cloud Dispenser",
        }
    }

    pub fn cost(&self) -> (u32, u32) {
        match self {
            WeaponAttachment::Laser => (0, 0),
            WeaponAttachment::Microwave => (40, 20),
            WeaponAttachment::Railgun => (80, 40),
            WeaponAttachment::RPG => (60, 10),
            WeaponAttachment::AcidCloud => (70, 30),
        }
    }

    pub fn required_tech(&self) -> Option<u32> {
        match self {
            WeaponAttachment::Laser => None,
            WeaponAttachment::Microwave => Some(5202),
            WeaponAttachment::Railgun => Some(5301),
            WeaponAttachment::RPG => None,
            WeaponAttachment::AcidCloud => None,
        }
    }

    pub fn to_sim_weapon(&self) -> WeaponType {
        match self {
            WeaponAttachment::Laser => WeaponType::Laser,
            WeaponAttachment::Microwave => WeaponType::Microwave,
            WeaponAttachment::Railgun => WeaponType::Railgun,
            WeaponAttachment::RPG => WeaponType::RPG,
            WeaponAttachment::AcidCloud => WeaponType::AcidCloud,
        }
    }
}

/// Kit payloads for ConVec mobile deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KitAttachment {
    CommonSmelter,
    SolarArray,
    GuardPost,
    Geothermal,
    Agrodome,
    Tokamak,
    Residence,
    DirtLab,
    StandardLab,
    RepairFacility,
    VehicleFactory,
    Spaceport,
}

impl KitAttachment {
    pub fn all() -> &'static [KitAttachment] {
        &[
            KitAttachment::CommonSmelter,
            KitAttachment::SolarArray,
            KitAttachment::GuardPost,
            KitAttachment::Geothermal,
            KitAttachment::Agrodome,
            KitAttachment::Tokamak,
            KitAttachment::Residence,
            KitAttachment::DirtLab,
            KitAttachment::StandardLab,
            KitAttachment::RepairFacility,
            KitAttachment::VehicleFactory,
            KitAttachment::Spaceport,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            KitAttachment::CommonSmelter => "Common Ore Smelter Kit",
            KitAttachment::SolarArray => "Solar Array Kit",
            KitAttachment::GuardPost => "Guard Post Kit",
            KitAttachment::Geothermal => "Geothermal Plant Kit",
            KitAttachment::Agrodome => "Bio-Dome Agrodome Kit",
            KitAttachment::Tokamak => "Tokamak Fusion Kit",
            KitAttachment::Residence => "Colonial Residence Kit",
            KitAttachment::DirtLab => "D.I.R.T. Lab Kit",
            KitAttachment::StandardLab => "Standard Lab Kit",
            KitAttachment::RepairFacility => "Repair Depot Kit",
            KitAttachment::VehicleFactory => "Vehicle Factory Kit",
            KitAttachment::Spaceport => "Starship Gantry Kit",
        }
    }

    pub fn kit_name(&self) -> &'static str {
        match self {
            KitAttachment::CommonSmelter => "Common Ore Smelter",
            KitAttachment::SolarArray => "Solar Power Array",
            KitAttachment::GuardPost => "Guard Post",
            KitAttachment::Geothermal => "Geothermal Power Plant",
            KitAttachment::Agrodome => "Agridome",
            KitAttachment::Tokamak => "Tokamak Reactor",
            KitAttachment::Residence => "Residence",
            KitAttachment::DirtLab => "D.I.R.T. Lab",
            KitAttachment::StandardLab => "Standard Lab",
            KitAttachment::RepairFacility => "Repair Facility",
            KitAttachment::VehicleFactory => "Vehicle Factory",
            KitAttachment::Spaceport => "Spaceport",
        }
    }

    pub fn cost(&self) -> (u32, u32) {
        match self {
            KitAttachment::CommonSmelter => (300, 50),
            KitAttachment::SolarArray => (170, 0),
            KitAttachment::GuardPost => (200, 50),
            KitAttachment::Geothermal => (300, 75),
            KitAttachment::Agrodome => (170, 50),
            KitAttachment::Tokamak => (450, 200),
            KitAttachment::Residence => (100, 0),
            KitAttachment::DirtLab => (250, 80),
            KitAttachment::StandardLab => (400, 150),
            KitAttachment::RepairFacility => (200, 50),
            KitAttachment::VehicleFactory => (400, 100),
            KitAttachment::Spaceport => (650, 250),
        }
    }

    pub fn required_tech(&self) -> Option<u32> {
        match self {
            KitAttachment::GuardPost => Some(5401),
            KitAttachment::Geothermal => Some(4301),
            _ => None,
        }
    }
}

/// An enqueued factory production order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionOrder {
    pub id: u64,
    pub hull: VehicleHull,
    pub weapon: Option<WeaponAttachment>,
    pub kit: Option<KitAttachment>,
    pub cost_common: u32,
    pub cost_rare: u32,
    pub rally_point: Option<(i32, i32)>,
}

impl ProductionOrder {
    pub fn title(&self) -> String {
        if let Some(w) = self.weapon {
            format!("{} ({})", self.hull.display_name(), w.display_name())
        } else if let Some(k) = self.kit {
            format!("{} [{}]", self.hull.display_name(), k.display_name())
        } else {
            self.hull.display_name().to_string()
        }
    }
}

/// State for the factory production UI and queue dispatcher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryQueueState {
    pub is_open: bool,
    pub active_factory_id: Option<u32>,
    pub selected_hull: VehicleHull,
    pub selected_weapon: Option<WeaponAttachment>,
    pub selected_kit: Option<KitAttachment>,
    pub queue: Vec<ProductionOrder>,
    pub current_progress: f64, // 0.0 to 100.0
    pub rally_point: Option<(i32, i32)>,
    pub next_order_id: u64,
}

impl Default for FactoryQueueState {
    fn default() -> Self {
        Self {
            is_open: false,
            active_factory_id: None,
            selected_hull: VehicleHull::Panther,
            selected_weapon: Some(WeaponAttachment::Laser),
            selected_kit: None,
            queue: Vec::new(),
            current_progress: 0.0,
            rally_point: None,
            next_order_id: 1,
        }
    }
}

impl FactoryQueueState {
    /// Calculate the combined resource cost for the currently selected configuration.
    pub fn current_selection_cost(&self) -> (u32, u32) {
        let (base_c, base_r) = self.selected_hull.base_cost();
        let (w_c, w_r) = self.selected_weapon.map_or((0, 0), |w| w.cost());
        let (k_c, k_r) = self.selected_kit.map_or((0, 0), |k| k.cost());
        (base_c + w_c + k_c, base_r + w_r + k_r)
    }

    /// Check whether technology prerequisites allow producing the current selection.
    pub fn check_prerequisites(&self, tech_tree: &TechTree) -> Result<(), &'static str> {
        if self.selected_hull == VehicleHull::Tiger && !tech_tree.can_build_railgun_tanks() {
            return Err("Requires Research: Magnetic Acceleration & Railgun Armaments (1400 C)");
        }
        if let Some(w) = self.selected_weapon {
            if let Some(req_id) = w.required_tech() {
                if !tech_tree.is_researched(req_id) {
                    return Err("Weapon requires scientific research node unlocked.");
                }
            }
        }
        if let Some(k) = self.selected_kit {
            if let Some(req_id) = k.required_tech() {
                if !tech_tree.is_researched(req_id) {
                    return Err("Kit structure requires scientific research node unlocked.");
                }
            }
        }
        Ok(())
    }

    /// Validate resource budget and enqueue production order.
    pub fn enqueue_selected(
        &mut self,
        common_ore: &mut u32,
        rare_ore: &mut u32,
        tech_tree: &TechTree,
    ) -> Result<ProductionOrder, String> {
        self.check_prerequisites(tech_tree).map_err(|e| e.to_string())?;

        let (cost_c, cost_r) = self.current_selection_cost();
        if *common_ore < cost_c || *rare_ore < cost_r {
            return Err(format!(
                "Insufficient Ore! Required: {} C, {} R. Available: {} C, {} R.",
                cost_c, cost_r, *common_ore, *rare_ore
            ));
        }

        *common_ore -= cost_c;
        *rare_ore -= cost_r;

        let order = ProductionOrder {
            id: self.next_order_id,
            hull: self.selected_hull,
            weapon: if self.selected_hull.can_mount_weapon() {
                self.selected_weapon
            } else {
                None
            },
            kit: if self.selected_hull.can_carry_kit() {
                self.selected_kit
            } else {
                None
            },
            cost_common: cost_c,
            cost_rare: cost_r,
            rally_point: self.rally_point,
        };

        self.next_order_id = self.next_order_id.wrapping_add(1);
        self.queue.push(order.clone());
        Ok(order)
    }

    /// Cancel an enqueued order and refund 100% of resources.
    pub fn cancel_order(
        &mut self,
        index: usize,
        common_ore: &mut u32,
        rare_ore: &mut u32,
    ) -> Option<ProductionOrder> {
        if index < self.queue.len() {
            let order = self.queue.remove(index);
            *common_ore += order.cost_common;
            *rare_ore += order.cost_rare;
            if index == 0 {
                self.current_progress = 0.0;
            }
            Some(order)
        } else {
            None
        }
    }

    /// Set a tactical rally point coordinate on the map.
    pub fn set_rally_point(&mut self, tx: i32, ty: i32) {
        self.rally_point = Some((tx, ty));
    }

    /// Clear the active rally point.
    pub fn clear_rally_point(&mut self) {
        self.rally_point = None;
    }
}

// =========================================================================
// 4. University Research & Tech Tree Browser
// =========================================================================

/// Detailed tech node model for the interactive tree browser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TechTreeItem {
    pub id: u32,
    pub name: String,
    pub category: &'static str,
    pub cost_common: u32,
    pub prerequisites: Vec<u32>,
    pub description: String,
}

/// State for the research browser modal and active research project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchBrowserState {
    pub is_open: bool,
    pub active_tech_id: Option<u32>,
    pub progress_percent: f64, // 0.0 to 100.0
    pub selected_tech_id: Option<u32>,
}

impl Default for ResearchBrowserState {
    fn default() -> Self {
        Self {
            is_open: false,
            active_tech_id: None,
            progress_percent: 0.0,
            selected_tech_id: Some(2701), // Default Astronomy
        }
    }
}

impl ResearchBrowserState {
    /// Canonical list of all research nodes with prerequisite gating.
    pub fn catalog() -> Vec<TechTreeItem> {
        vec![
            TechTreeItem {
                id: 2106,
                name: "Basic Laboratory".into(),
                category: "Core Science",
                cost_common: 0,
                prerequisites: vec![],
                description: "Essential scientific facility unlocking planetary survival disciplines.".into(),
            },
            TechTreeItem {
                id: 2701,
                name: "Astronomy".into(),
                category: "Space & Orbital",
                cost_common: 400,
                prerequisites: vec![2106],
                description: "Rebuilds satellite telemetry, mapping resource distribution and orbital impact vectors.".into(),
            },
            TechTreeItem {
                id: 3301,
                name: "Hydroponics Automation".into(),
                category: "Life Support",
                cost_common: 650,
                prerequisites: vec![2106],
                description: "Automated nutrient feeding arrays granting +50% food output per Bio-Dome.".into(),
            },
            TechTreeItem {
                id: 3402,
                name: "High-Temperature Smelting".into(),
                category: "Metallurgy",
                cost_common: 800,
                prerequisites: vec![2106],
                description: "Plasma-arc furnaces yielding +25% ore refining throughput.".into(),
            },
            TechTreeItem {
                id: 4201,
                name: "Lava Containment Walls".into(),
                category: "Engineering",
                cost_common: 1100,
                prerequisites: vec![2106],
                description: "Reinforced ceramic thermal barriers designed to redirect slow basaltic lava flows.".into(),
            },
            TechTreeItem {
                id: 4301,
                name: "Vulcanology & Hydrothermal Extraction".into(),
                category: "Engineering",
                cost_common: 750,
                prerequisites: vec![2701],
                description: "Subsurface thermal tap, unlocking continuous +450 kW Geothermal Power Plants.".into(),
            },
            TechTreeItem {
                id: 5104,
                name: "Lynx Reinforced Armor".into(),
                category: "Military Robotics",
                cost_common: 950,
                prerequisites: vec![2106],
                description: "Alloy composite plates granting +50 HP to all manufactured combat vehicles.".into(),
            },
            TechTreeItem {
                id: 5202,
                name: "Pulse Laser Optics".into(),
                category: "Military Robotics",
                cost_common: 1200,
                prerequisites: vec![5104],
                description: "Refined focal crystals granting +15 beam intensity to pulse laser weaponry.".into(),
            },
            TechTreeItem {
                id: 5301,
                name: "Magnetic Acceleration & Railguns".into(),
                category: "Military Robotics",
                cost_common: 1400,
                prerequisites: vec![5104],
                description: "Magnetic accelerator coils unlocking Tiger Heavy Tanks armed with Railguns.".into(),
            },
            TechTreeItem {
                id: 5401,
                name: "Automated Point Defense Turrets".into(),
                category: "Military Robotics",
                cost_common: 600,
                prerequisites: vec![2106],
                description: "Automated targeting arrays enabling perimeter Guard Post defense turrets.".into(),
            },
            TechTreeItem {
                id: 6101,
                name: "Cryogenic Evacuation Modules".into(),
                category: "Space & Orbital",
                cost_common: 1500,
                prerequisites: vec![2701, 3301],
                description: "Passenger stasis units ensuring high colonist payload for the Evacuation Starship.".into(),
            },
        ]
    }

    /// Check if a tech node's prerequisites are all researched.
    pub fn is_unlocked(&self, item: &TechTreeItem, tech_tree: &TechTree) -> bool {
        item.prerequisites.iter().all(|&pre| tech_tree.is_researched(pre))
    }

    /// Retrieve list of missing prerequisite names for display in the tooltip.
    pub fn missing_prerequisites(&self, item: &TechTreeItem, tech_tree: &TechTree) -> Vec<String> {
        let catalog = Self::catalog();
        item.prerequisites
            .iter()
            .filter(|&&pre| !tech_tree.is_researched(pre))
            .map(|&pre| {
                catalog
                    .iter()
                    .find(|t| t.id == pre)
                    .map_or(format!("Tech #{}", pre), |t| t.name.clone())
            })
            .collect()
    }

    /// Begin active research on a node. Deducts common ore and starts progress.
    pub fn start_research(
        &mut self,
        tech_id: u32,
        common_ore: &mut u32,
        tech_tree: &TechTree,
    ) -> Result<String, String> {
        if tech_tree.is_researched(tech_id) {
            return Err("Technology is already researched.".into());
        }

        let catalog = Self::catalog();
        let item = catalog
            .iter()
            .find(|t| t.id == tech_id)
            .ok_or_else(|| "Technology not found in matrix.".to_string())?;

        if !self.is_unlocked(item, tech_tree) {
            let missing = self.missing_prerequisites(item, tech_tree).join(", ");
            return Err(format!("Prerequisites locked: Requires {}", missing));
        }

        if *common_ore < item.cost_common {
            return Err(format!(
                "Insufficient Ore: Requires {} C, available: {} C.",
                item.cost_common, *common_ore
            ));
        }

        *common_ore -= item.cost_common;
        self.active_tech_id = Some(tech_id);
        self.progress_percent = 0.0;
        Ok(format!("Initiated scientific research: {}", item.name))
    }

    /// Progress research forward by delta time. Completes tech when reaching 100%.
    pub fn advance_research(
        &mut self,
        dt_secs: f64,
        labs_count: usize,
        tech_tree: &mut TechTree,
    ) -> Option<String> {
        if let Some(tech_id) = self.active_tech_id {
            if labs_count == 0 {
                return None; // No active laboratories
            }
            // Research rate scales with active labs
            let rate = 12.0 * (labs_count as f64).min(4.0);
            self.progress_percent += dt_secs * rate;

            if self.progress_percent >= 100.0 {
                self.progress_percent = 100.0;
                self.active_tech_id = None;

                if let Some(t) = tech_tree.techs.iter_mut().find(|t| t.id == tech_id) {
                    t.is_researched = true;
                    return Some(t.name.clone());
                }
            }
        }
        None
    }
}

// =========================================================================
// 5. Tactical Notification Dispatcher & In-Game Chat HUD
// =========================================================================

/// Classification priority for in-game radio and system dispatches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AlertPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl AlertPriority {
    pub fn color(&self) -> &'static str {
        match self {
            AlertPriority::Low => "#38bdf8",
            AlertPriority::Medium => "#facc15",
            AlertPriority::High => "#fb923c",
            AlertPriority::Critical => "#ef4444",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            AlertPriority::Low => "[INFO]",
            AlertPriority::Medium => "[CAUTION]",
            AlertPriority::High => "[WARNING]",
            AlertPriority::Critical => "[CRITICAL]",
        }
    }
}

/// A scrolling tactical alert log item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TacticalAlert {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub priority: AlertPriority,
    pub location: Option<(i32, i32)>,
    pub timestamp_sol: f64,
    pub age_secs: f64,
}

/// A chat transmission from teammates, broadcast, or multiplayer opponents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatTransmission {
    pub id: u64,
    pub sender: String,
    pub channel: String,
    pub message: String,
    pub timestamp_sol: f64,
}

/// Dispatches priority alerts, radio transmissions, and chat logs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationDispatcher {
    pub alerts: Vec<TacticalAlert>,
    pub chat: Vec<ChatTransmission>,
    pub max_alerts: usize,
    pub max_chat: usize,
    pub last_alert_location: Option<(i32, i32)>,
    pub next_id: u64,
    pub show_chat: bool,
}

impl Default for NotificationDispatcher {
    fn default() -> Self {
        Self {
            alerts: Vec::new(),
            chat: Vec::new(),
            max_alerts: 40,
            max_chat: 30,
            last_alert_location: None,
            next_id: 1,
            show_chat: true,
        }
    }
}

impl NotificationDispatcher {
    /// Post a new tactical alert.
    pub fn push_alert(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        priority: AlertPriority,
        location: Option<(i32, i32)>,
        sol: f64,
    ) {
        if location.is_some() {
            self.last_alert_location = location;
        }

        let alert = TacticalAlert {
            id: self.next_id,
            title: title.into(),
            message: message.into(),
            priority,
            location,
            timestamp_sol: sol,
            age_secs: 0.0,
        };
        self.next_id = self.next_id.wrapping_add(1);

        self.alerts.insert(0, alert);
        if self.alerts.len() > self.max_alerts {
            self.alerts.pop();
        }
    }

    /// Post an in-game or multiplayer chat message.
    pub fn push_chat(
        &mut self,
        sender: impl Into<String>,
        channel: impl Into<String>,
        message: impl Into<String>,
        sol: f64,
    ) {
        let msg = ChatTransmission {
            id: self.next_id,
            sender: sender.into(),
            channel: channel.into(),
            message: message.into(),
            timestamp_sol: sol,
        };
        self.next_id = self.next_id.wrapping_add(1);

        self.chat.insert(0, msg);
        if self.chat.len() > self.max_chat {
            self.chat.pop();
        }
    }

    /// Age alerts and clean up old entries.
    pub fn tick(&mut self, dt_secs: f64) {
        for a in &mut self.alerts {
            a.age_secs += dt_secs;
        }
    }

    /// Retrieve the coordinates of the most recent tactical notification or disaster.
    pub fn jump_to_last_alert(&self) -> Option<(i32, i32)> {
        self.last_alert_location
    }
}

// =========================================================================
// 5B. Real-Time Network Telemetry & Tactical Multiplayer Chat
// =========================================================================

/// Real-time WebRTC multiplayer network telemetry state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTelemetryState {
    pub is_multiplayer: bool,
    pub is_connected: bool,
    pub is_host: bool,
    pub peer_name: Option<String>,
    pub ping_ms: u32,
    pub lockstep_turn: u64,
    pub is_in_sync: bool,
    pub host_faction: Faction,
    pub guest_faction: Option<Faction>,
}

impl Default for NetworkTelemetryState {
    fn default() -> Self {
        Self {
            is_multiplayer: false,
            is_connected: false,
            is_host: true,
            peer_name: None,
            ping_ms: 18,
            lockstep_turn: 1240,
            is_in_sync: true,
            host_faction: Faction::Eden,
            guest_faction: Some(Faction::Plymouth),
        }
    }
}

impl NetworkTelemetryState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Formatted peer connection status badge:
    /// e.g. "P2P ACTIVE: Host & Guest"
    pub fn peer_status_badge(&self) -> String {
        if self.is_connected {
            "P2P ACTIVE: Host & Guest".to_string()
        } else {
            "P2P STANDBY".to_string()
        }
    }

    /// Formatted roundtrip ping / latency telemetry meter:
    /// e.g. "Ping: 18ms"
    pub fn ping_meter(&self) -> String {
        format!("Ping: {}ms", self.ping_ms)
    }

    /// Formatted turn-buffered lockstep sync status:
    /// e.g. "In Sync: Turn 1240"
    pub fn lockstep_sync_status(&self) -> String {
        if self.is_in_sync {
            format!("In Sync: Turn {}", self.lockstep_turn)
        } else {
            format!("Buffering: Turn {}", self.lockstep_turn)
        }
    }

    /// Update telemetry variables from live network packet stream.
    pub fn update_telemetry(&mut self, ping_ms: u32, turn: u64, in_sync: bool) {
        self.ping_ms = ping_ms;
        self.lockstep_turn = turn;
        self.is_in_sync = in_sync;
    }
}

/// Tactical in-game chat message exchanged over WebRTC DataChannel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TacticalChatMessage {
    pub id: u64,
    pub sender: String,
    pub faction: Option<Faction>,
    pub text: String,
    pub timestamp_sol: f64,
    pub is_system: bool,
}

impl TacticalChatMessage {
    /// Color-coded faction prefix:
    /// e.g. "[Eden] Player1:" or "[Plymouth] Sarah:"
    pub fn formatted_prefix(&self) -> String {
        match self.faction {
            Some(Faction::Eden) => format!("[Eden] {}:", self.sender),
            Some(Faction::Plymouth) => format!("[Plymouth] {}:", self.sender),
            None => format!("{}:", self.sender),
        }
    }

    /// Primary accent color based on faction allegiance.
    pub fn faction_color(&self) -> &'static str {
        match self.faction {
            Some(Faction::Eden) => "#38bdf8",     // Cyan
            Some(Faction::Plymouth) => "#f87171", // Coral / Red
            None => "#facc15",                    // Amber / Yellow
        }
    }

    /// Full formatted message line:
    /// e.g. "[Eden] Player1: Need assistance at south ridge!"
    pub fn full_line(&self) -> String {
        format!("{} {}", self.formatted_prefix(), self.text)
    }
}

/// In-game tactical chat controller and text input buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalChatState {
    pub is_open: bool,
    pub input_buffer: String,
    pub cursor_blink: f64,
    pub messages: Vec<TacticalChatMessage>,
    pub max_messages: usize,
    pub next_id: u64,
}

impl Default for TacticalChatState {
    fn default() -> Self {
        Self {
            is_open: false,
            input_buffer: String::new(),
            cursor_blink: 0.0,
            messages: Vec::new(),
            max_messages: 50,
            next_id: 1,
        }
    }
}

impl TacticalChatState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.cursor_blink = 0.0;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.input_buffer.clear();
    }

    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn type_char(&mut self, c: char) {
        if self.input_buffer.len() < 120 {
            self.input_buffer.push(c);
        }
    }

    pub fn backspace(&mut self) {
        self.input_buffer.pop();
    }

    /// Commit the current input buffer as an outgoing message and clear buffer.
    pub fn commit_message(
        &mut self,
        sol: f64,
        sender: &str,
        faction: Faction,
    ) -> Option<TacticalChatMessage> {
        let text = self.input_buffer.trim().to_string();
        if text.is_empty() {
            self.close();
            return None;
        }

        let msg = TacticalChatMessage {
            id: self.next_id,
            sender: sender.to_string(),
            faction: Some(faction),
            text,
            timestamp_sol: sol,
            is_system: false,
        };
        self.next_id = self.next_id.wrapping_add(1);
        self.input_buffer.clear();
        self.is_open = false;

        self.messages.insert(0, msg.clone());
        if self.messages.len() > self.max_messages {
            self.messages.pop();
        }
        Some(msg)
    }

    /// Ingest an incoming message from the peer commander.
    pub fn push_incoming(
        &mut self,
        sender: impl Into<String>,
        text: impl Into<String>,
        faction: Option<Faction>,
        sol: f64,
    ) {
        let msg = TacticalChatMessage {
            id: self.next_id,
            sender: sender.into(),
            faction,
            text: text.into(),
            timestamp_sol: sol,
            is_system: false,
        };
        self.next_id = self.next_id.wrapping_add(1);

        self.messages.insert(0, msg);
        if self.messages.len() > self.max_messages {
            self.messages.pop();
        }
    }

    pub fn tick(&mut self, dt_secs: f64) {
        self.cursor_blink += dt_secs;
        if self.cursor_blink >= 1.0 {
            self.cursor_blink -= 1.0;
        }
    }
}

// =========================================================================
// 6. Responsive Keyboard Shortcut System
// =========================================================================

/// Action result dispatched by the keyboard input listener.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HotkeyAction {
    JumpToLastAlert,
    TogglePause,
    AssignControlGroup(u8),
    RecallControlGroup(u8),
    ToggleFactoryView,
    ToggleResearchView,
    JumpToBaseOverview,
    ToggleMinimapDetail,
    QuickSave,
    QuickLoad,
    ToggleChat,
    CloseChat,
    None,
}

/// Unit control groups and responsive hotkey mapping system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeySystem {
    pub control_groups: [Vec<u32>; 10], // Groups 0-9
    pub is_paused: bool,
    pub last_group_pressed: Option<u8>,
}

impl Default for HotkeySystem {
    fn default() -> Self {
        Self {
            control_groups: Default::default(),
            is_paused: false,
            last_group_pressed: None,
        }
    }
}

impl HotkeySystem {
    /// Process a keyboard event keycode and modifiers.
    pub fn handle_key(&mut self, key: &str, ctrl: bool, _shift: bool) -> HotkeyAction {
        match key {
            " " => HotkeyAction::JumpToLastAlert,
            "p" | "P" => {
                self.is_paused = !self.is_paused;
                HotkeyAction::TogglePause
            }
            "f" | "F" => HotkeyAction::ToggleFactoryView,
            "r" | "R" => HotkeyAction::ToggleResearchView,
            "b" | "B" => HotkeyAction::JumpToBaseOverview,
            "m" | "M" => HotkeyAction::ToggleMinimapDetail,
            "F5" | "f5" => HotkeyAction::QuickSave,
            "F9" | "f9" => HotkeyAction::QuickLoad,
            "Enter" => HotkeyAction::ToggleChat,
            "Escape" => HotkeyAction::CloseChat,
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" => {
                let digit = key.chars().next().unwrap().to_digit(10).unwrap() as u8;
                if ctrl {
                    HotkeyAction::AssignControlGroup(digit)
                } else {
                    self.last_group_pressed = Some(digit);
                    HotkeyAction::RecallControlGroup(digit)
                }
            }
            _ => HotkeyAction::None,
        }
    }

    /// Bind a list of unit IDs to a designated control group index.
    pub fn assign_group(&mut self, group: u8, unit_ids: Vec<u32>) {
        if (group as usize) < self.control_groups.len() {
            self.control_groups[group as usize] = unit_ids;
        }
    }

    /// Retrieve unit IDs belonging to a control group.
    pub fn get_group(&self, group: u8) -> &[u32] {
        if (group as usize) < self.control_groups.len() {
            &self.control_groups[group as usize]
        } else {
            &[]
        }
    }
}

// =========================================================================
// 7. Full HUD State & Canvas HudRenderer
// =========================================================================

/// Unified HUD state holding all tactical overlays and panels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudState {
    pub radar: RadarProjection,
    pub rates: ResourceRateTracker,
    pub factory: FactoryQueueState,
    pub research: ResearchBrowserState,
    pub notifications: NotificationDispatcher,
    pub hotkeys: HotkeySystem,
    pub telemetry: NetworkTelemetryState,
    pub chat: TacticalChatState,
    pub language: String,
}

impl HudState {
    pub fn new(cols: i32, rows: i32, radar_w: f64, radar_h: f64) -> Self {
        Self {
            radar: RadarProjection::new(cols, rows, radar_w, radar_h),
            rates: ResourceRateTracker::default(),
            factory: FactoryQueueState::default(),
            research: ResearchBrowserState::default(),
            notifications: NotificationDispatcher::default(),
            hotkeys: HotkeySystem::default(),
            telemetry: NetworkTelemetryState::default(),
            chat: TacticalChatState::default(),
            language: "en".into(),
        }
    }
}

/// Canvas rendering subsystem for modern tactical Outpost 2 HUD.
pub struct HudRenderer {
    pub state: HudState,
}

impl HudRenderer {
    pub fn new(cols: i32, rows: i32, radar_w: f64, radar_h: f64) -> Self {
        Self {
            state: HudState::new(cols, rows, radar_w, radar_h),
        }
    }

    /// Update frame time, telemetry rates, and research.
    pub fn update(&mut self, dt_secs: f64, colony: &mut ColonyState) -> Option<String> {
        self.state.rates.update(
            colony.sol,
            colony.common_ore,
            colony.rare_ore,
            colony.food_units,
        );

        let active_labs = colony
            .buildings
            .iter()
            .filter(|b| b.is_active && (b.name.contains("Lab") || b.name.contains("D.I.R.T.")))
            .count();

        let mut research_completed = None;
        if let Some(tech_name) =
            self.state
                .research
                .advance_research(dt_secs, active_labs, &mut colony.tech_tree)
        {
            self.state.notifications.push_alert(
                "RESEARCH COMPLETED",
                format!("New breakthrough achieved: {}", tech_name),
                AlertPriority::Low,
                None,
                colony.sol,
            );
            research_completed = Some(tech_name);
        }

        self.state.notifications.tick(dt_secs);
        self.state.chat.tick(dt_secs);
        research_completed
    }

    /// Render the interactive tactical minimap radar display.
    pub fn render_radar(
        &self,
        ctx: &CanvasRenderingContext2d,
        colony: &ColonyState,
        camera_x: f64,
        camera_y: f64,
        screen_w: f64,
        screen_h: f64,
        tile_w: f64,
        tile_h: f64,
        zoom: f64,
        radar_w: f64,
        radar_h: f64,
    ) {
        // Deep tactical background
        ctx.set_fill_style_str("#030712");
        ctx.fill_rect(0.0, 0.0, radar_w, radar_h);

        let scale_x = radar_w / (self.state.radar.map_cols as f64);
        let scale_y = radar_h / (self.state.radar.map_rows as f64);

        // 1. Miniaturized terrain render respecting Fog of War
        for y in 0..self.state.radar.map_rows {
            for x in 0..self.state.radar.map_cols {
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
                ctx.fill_rect(
                    (x as f64) * scale_x,
                    (y as f64) * scale_y,
                    scale_x + 0.5,
                    scale_y + 0.5,
                );
            }
        }

        // 2. Transport Tubes
        ctx.set_fill_style_str("#64748b");
        for &(tx, ty) in &colony.tubes {
            let fog = colony.fog.get(tx as usize, ty as usize);
            if fog != FogState::Unexplored {
                let px = (tx as f64) * scale_x;
                let py = (ty as f64) * scale_y;
                ctx.fill_rect(px, py, 1.5, 1.5);
            }
        }

        // 3. Buildings
        ctx.set_fill_style_str("#38bdf8");
        for b in &colony.buildings {
            let fog = colony.fog.get(b.tile_x as usize, b.tile_y as usize);
            if fog != FogState::Unexplored {
                let bx = (b.tile_x as f64) * scale_x;
                let by = (b.tile_y as f64) * scale_y;
                let bw = (b.width as f64) * scale_x + 1.0;
                let bh = (b.height as f64) * scale_y + 1.0;
                ctx.fill_rect(bx, by, bw, bh);
            }
        }

        // 4. Color-coded units:
        //    Blue = Eden, Red = Plymouth, Yellow = Neutral/ruins
        for u in &colony.units {
            let fog = colony.fog.get(u.tile_x as usize, u.tile_y as usize);
            if fog == FogState::Visible {
                let color = match u.faction {
                    Faction::Eden => "#38bdf8",     // Blue Eden
                    Faction::Plymouth => "#ef4444", // Red Plymouth
                };
                ctx.set_fill_style_str(color);
                let ux = (u.tile_x as f64) * scale_x;
                let uy = (u.tile_y as f64) * scale_y;
                ctx.fill_rect(ux - 1.0, uy - 1.0, 3.0, 3.0);
            }
        }

        // Neutral / Ruin Ore Deposits (Yellow)
        for dep in &colony.ore_deposits {
            let fog = colony.fog.get(dep.tile_x as usize, dep.tile_y as usize);
            if fog != FogState::Unexplored {
                let dx = (dep.tile_x as f64) * scale_x;
                let dy = (dep.tile_y as f64) * scale_y;
                ctx.set_fill_style_str("#facc15"); // Yellow neutral/ruins
                ctx.fill_rect(dx - 1.0, dy - 1.0, 2.5, 2.5);
            }
        }

        // 5. Active Disaster Radar Blips:
        // Pulsing orange/red for volcanic fissures
        let pulse_alpha = 0.5 + 0.5 * (colony.sol * 8.0).sin().abs();
        ctx.set_fill_style_str(&format!("rgba(249, 115, 22, {:.2})", pulse_alpha));
        for &(lx, ly) in &colony.disasters.active_lava_tiles {
            let fog = colony.fog.get(lx as usize, ly as usize);
            if fog != FogState::Unexplored {
                let px = (lx as f64) * scale_x;
                let py = (ly as f64) * scale_y;
                ctx.fill_rect(px - 1.0, py - 1.0, 3.0, 3.0);
            }
        }

        // Yellow swirling icons for vortices
        ctx.set_stroke_style_str("#fde047");
        ctx.set_line_width(1.2);
        for v in &colony.disasters.active_vortices {
            let px = v.x * scale_x;
            let py = v.y * scale_y;
            let spin = colony.sol * 10.0;
            ctx.begin_path();
            let _ = ctx.arc(px, py, 3.5, spin, spin + std::f64::consts::PI);
            ctx.stroke();
            ctx.begin_path();
            let _ = ctx.arc(px, py, 2.0, spin + std::f64::consts::PI, spin + std::f64::consts::TAU);
            ctx.stroke();
        }

        // Red reticles for meteor impact sites
        ctx.set_stroke_style_str("#ef4444");
        ctx.set_line_width(1.0);
        for m in &colony.disasters.pending_meteors {
            let px = m.x * scale_x;
            let py = m.y * scale_y;
            ctx.begin_path();
            let _ = ctx.arc(px, py, 4.0, 0.0, std::f64::consts::TAU);
            ctx.stroke();
            ctx.begin_path();
            ctx.move_to(px - 6.0, py);
            ctx.line_to(px + 6.0, py);
            ctx.move_to(px, py - 6.0);
            ctx.line_to(px, py + 6.0);
            ctx.stroke();
        }

        // Rally-Point Blip
        if let Some((rx, ry)) = self.state.factory.rally_point {
            let rpx = (rx as f64) * scale_x;
            let rpy = (ry as f64) * scale_y;
            ctx.set_stroke_style_str("#4ade80");
            ctx.set_line_width(1.5);
            ctx.begin_path();
            ctx.move_to(rpx - 3.0, rpy);
            ctx.line_to(rpx + 3.0, rpy);
            ctx.move_to(rpx, rpy - 3.0);
            ctx.line_to(rpx, rpy + 3.0);
            ctx.stroke();
        }

        // 6. Viewport camera bounding rectangle indicating current screen coverage
        let (fx, fy, fw, fh) = RadarProjection::calculate_viewport_frustum_rect(
            camera_x,
            camera_y,
            screen_w,
            screen_h,
            tile_w,
            tile_h,
            zoom,
            radar_w,
            radar_h,
            self.state.radar.map_cols,
            self.state.radar.map_rows,
        );

        ctx.set_stroke_style_str("#38bdf8");
        ctx.set_line_width(1.2);
        ctx.stroke_rect(fx, fy, fw, fh);

        // Radar Detail Mode Overlay (if enabled via 'M')
        if self.state.radar.show_detail {
            ctx.set_stroke_style_str("rgba(56, 189, 248, 0.15)");
            ctx.set_line_width(0.5);
            for x in (0..self.state.radar.map_cols).step_by(8) {
                let px = (x as f64) * scale_x;
                ctx.begin_path();
                ctx.move_to(px, 0.0);
                ctx.line_to(px, radar_h);
                ctx.stroke();
            }
            for y in (0..self.state.radar.map_rows).step_by(8) {
                let py = (y as f64) * scale_y;
                ctx.begin_path();
                ctx.move_to(0.0, py);
                ctx.line_to(radar_w, py);
                ctx.stroke();
            }
        }
    }

    /// Render the Colony Command Dashboard & Resource Status Bar across the top of the canvas.
    pub fn render_command_bar(
        &self,
        ctx: &CanvasRenderingContext2d,
        colony: &ColonyState,
        screen_w: f64,
    ) {
        let bar_h = 28.0;
        ctx.set_fill_style_str("rgba(11, 15, 23, 0.94)");
        ctx.fill_rect(0.0, 0.0, screen_w, bar_h);

        ctx.set_stroke_style_str("#1e293b");
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(0.0, bar_h);
        ctx.line_to(screen_w, bar_h);
        ctx.stroke();

        let lang = crate::i18n::Language::from_code(&self.state.language);
        let dict = crate::i18n::get_dictionary(lang);

        ctx.set_font("bold 11px system-ui, -apple-system, sans-serif");
        let mut x = 12.0;
        let y = 18.0;

        // Faction Badge
        let faction_color = match colony.faction {
            Faction::Eden => "#38bdf8",
            Faction::Plymouth => "#ef4444",
        };
        ctx.set_fill_style_str(faction_color);
        let faction_text = match colony.faction {
            Faction::Eden => "EDEN",
            Faction::Plymouth => "PLYMOUTH",
        };
        let _ = ctx.fill_text(faction_text, x, y);
        x += 68.0;

        // Sol Counter
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(dict.sol, x, y);
        ctx.set_fill_style_str("#f1f5f9");
        let sol_str = format!("{:.2}", colony.sol);
        let _ = ctx.fill_text(&sol_str, x + 38.0, y);
        x += 80.0;

        // Common Ore + Rate
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(dict.common_ore, x, y);
        ctx.set_fill_style_str("#facc15");
        let rate_c = self.state.rates.common_rate;
        let c_sign = if rate_c >= 0.0 { "+" } else { "" };
        let c_str = format!("{} t ({}{:.0}/s)", colony.common_ore, c_sign, rate_c);
        let _ = ctx.fill_text(&c_str, x + 65.0, y);
        x += 150.0;

        // Rare Ore + Rate
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(dict.rare_ore, x, y);
        ctx.set_fill_style_str("#c084fc");
        let rate_r = self.state.rates.rare_rate;
        let r_sign = if rate_r >= 0.0 { "+" } else { "" };
        let r_str = format!("{} t ({}{:.0}/s)", colony.rare_ore, r_sign, rate_r);
        let _ = ctx.fill_text(&r_str, x + 55.0, y);
        x += 140.0;

        // Food & Sustenance
        let sust = SustenanceState::evaluate(colony.food_units, colony.population);
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(dict.food, x, y);
        ctx.set_fill_style_str(sust.color());
        let f_str = format!("{} [{}]", colony.food_units, sust.label_localized(lang));
        let _ = ctx.fill_text(&f_str, x + 48.0, y);
        x += 130.0;

        // Energy Grid Status (Brownout / Blackout Alert)
        let grid = EnergyGridState::from_power(colony.power_produced, colony.power_consumed);
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(dict.power, x, y);
        ctx.set_fill_style_str(grid.alert_color());
        let p_str = format!(
            "{}/{} MW ({})",
            colony.power_produced,
            colony.power_consumed,
            grid.status_label_localized(lang)
        );
        let _ = ctx.fill_text(&p_str, x + 58.0, y);
        x += 190.0;

        // Morale Gauge
        let morale = MoraleTier::from_percentage(colony.morale_pct);
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(dict.pop, x, y);
        ctx.set_fill_style_str(morale.color());
        let m_str = format!("{}% - {}", colony.morale_pct, morale.label_localized(lang));
        let _ = ctx.fill_text(&m_str, x + 45.0, y);

        // Live Multiplayer HUD Widget (Connection Badge, Ping Meter, Lockstep Sync Status)
        if self.state.telemetry.is_multiplayer || self.state.telemetry.is_connected {
            let widget_x = (screen_w - 430.0).max(x + 130.0);
            self.render_network_telemetry_widget(ctx, widget_x, 3.0);
        }
    }

    /// Render the live multiplayer HUD widget:
    /// - Peer connection status badge (`P2P ACTIVE: Host & Guest`).
    /// - Real-time roundtrip ping / latency telemetry meter (`Ping: 18ms`).
    /// - Turn-buffered lockstep sync status (`In Sync: Turn 1240`).
    pub fn render_network_telemetry_widget(
        &self,
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
    ) {
        let t = &self.state.telemetry;
        let badge_text = t.peer_status_badge();
        let ping_text = t.ping_meter();
        let sync_text = t.lockstep_sync_status();

        ctx.set_font("bold 10px monospace, system-ui");

        // 1. Peer Connection Status Badge
        let badge_bg = if t.is_connected {
            "rgba(34, 197, 94, 0.2)"
        } else {
            "rgba(239, 68, 68, 0.2)"
        };
        let badge_border = if t.is_connected {
            "#22c55e"
        } else {
            "#ef4444"
        };
        let badge_color = if t.is_connected {
            "#4ade80"
        } else {
            "#f87171"
        };

        let badge_w = 175.0;
        let badge_h = 22.0;

        ctx.set_fill_style_str(badge_bg);
        ctx.fill_rect(x, y, badge_w, badge_h);
        ctx.set_stroke_style_str(badge_border);
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x, y, badge_w, badge_h);

        // Status indicator circle
        ctx.set_fill_style_str(badge_color);
        ctx.begin_path();
        let _ = ctx.arc(x + 10.0, y + 11.0, 3.5, 0.0, std::f64::consts::PI * 2.0);
        ctx.fill();

        ctx.set_fill_style_str(badge_color);
        let _ = ctx.fill_text(&badge_text, x + 20.0, y + 15.0);

        // 2. Real-time Roundtrip Ping / Latency telemetry meter
        let ping_x = x + badge_w + 12.0;
        let ping_color = if t.ping_ms < 50 {
            "#4ade80" // Fast - Green
        } else if t.ping_ms < 150 {
            "#facc15" // Moderate - Amber
        } else {
            "#ef4444" // High - Red
        };
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text("RTT:", ping_x, y + 15.0);
        ctx.set_fill_style_str(ping_color);
        let _ = ctx.fill_text(&ping_text, ping_x + 28.0, y + 15.0);

        // 3. Turn-buffered Lockstep Sync status
        let sync_x = ping_x + 95.0;
        let sync_color = if t.is_in_sync {
            "#38bdf8"
        } else {
            "#fb923c"
        };
        ctx.set_fill_style_str(sync_color);
        let _ = ctx.fill_text(&sync_text, sync_x, y + 15.0);
    }

    /// Render scrolling tactical alerts and in-game chat overlay.
    pub fn render_alerts_and_chat(
        &self,
        ctx: &CanvasRenderingContext2d,
        screen_w: f64,
        screen_h: f64,
    ) {
        let max_visible = 5;
        let start_y = if self.state.chat.is_open {
            screen_h - 135.0
        } else {
            screen_h - 100.0
        };
        let line_h = 18.0;

        ctx.set_font("11px monospace, system-ui");

        // 1. Render tactical alerts
        for (idx, alert) in self.state.notifications.alerts.iter().take(max_visible).enumerate() {
            let y = start_y - (idx as f64 * line_h);
            if y < 40.0 {
                break;
            }

            // Priority badge
            ctx.set_fill_style_str(alert.priority.color());
            let badge = alert.priority.badge();
            let _ = ctx.fill_text(badge, 16.0, y);

            // Message text
            ctx.set_fill_style_str("#f8fafc");
            let text = format!("[Sol {:.2}] {}: {}", alert.timestamp_sol, alert.title, alert.message);
            let _ = ctx.fill_text(&text, 80.0, y);
        }

        // 2. Render Tactical Chat messages (incoming & outgoing with color-coded faction names)
        let chat_count = self.state.chat.messages.len().min(5);
        for (idx, msg) in self.state.chat.messages.iter().take(chat_count).enumerate() {
            let y = start_y - ((max_visible + idx) as f64 * line_h);
            if y < 40.0 {
                break;
            }

            // Color-coded faction prefix (e.g. "[Eden] Player1:" or "[Plymouth] Sarah:")
            ctx.set_fill_style_str(msg.faction_color());
            let prefix = msg.formatted_prefix();
            let _ = ctx.fill_text(&prefix, 16.0, y);

            // Message body
            ctx.set_fill_style_str("#f1f5f9");
            let prefix_w = prefix.len() as f64 * 7.5;
            let _ = ctx.fill_text(&format!(" {}", msg.text), 16.0 + prefix_w, y);
        }

        // 3. Render Tactical Chat Input Prompt when open (Press Enter to focus)
        if self.state.chat.is_open {
            let box_x = 16.0;
            let box_y = screen_h - 90.0;
            let box_w = (screen_w * 0.55).clamp(380.0, 720.0);
            let box_h = 32.0;

            // Translucent glass prompt container
            ctx.set_fill_style_str("rgba(15, 23, 42, 0.94)");
            ctx.fill_rect(box_x, box_y, box_w, box_h);

            ctx.set_stroke_style_str("#38bdf8");
            ctx.set_line_width(1.5);
            ctx.stroke_rect(box_x, box_y, box_w, box_h);

            // Prompt label & buffer
            ctx.set_font("bold 11px monospace, system-ui");
            ctx.set_fill_style_str("#38bdf8");
            let prompt_label = "💬 COMM [ENTER: Send | ESC: Close] > ";
            let _ = ctx.fill_text(prompt_label, box_x + 10.0, box_y + 20.0);

            let label_w = prompt_label.len() as f64 * 7.0;
            ctx.set_fill_style_str("#f8fafc");
            let cursor = if self.state.chat.cursor_blink < 0.5 { "_" } else { " " };
            let display_text = format!("{}{}", self.state.chat.input_buffer, cursor);
            let _ = ctx.fill_text(&display_text, box_x + 10.0 + label_w, box_y + 20.0);
        }
    }

    /// Render the in-canvas Factory Production & ConVec Kit Queue Panel.
    pub fn render_factory_panel(
        &self,
        ctx: &CanvasRenderingContext2d,
        _colony: &ColonyState,
        screen_w: f64,
        screen_h: f64,
    ) {
        if !self.state.factory.is_open {
            return;
        }

        let lang = crate::i18n::Language::from_code(&self.state.language);
        let dict = crate::i18n::get_dictionary(lang);

        let panel_w = 340.0;
        let panel_h = 380.0;
        let panel_x = (screen_w - panel_w) * 0.5;
        let panel_y = (screen_h - panel_h) * 0.5;

        // Backdrop
        ctx.set_fill_style_str("rgba(15, 23, 42, 0.95)");
        ctx.fill_rect(panel_x, panel_y, panel_w, panel_h);

        ctx.set_stroke_style_str("#38bdf8");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(panel_x, panel_y, panel_w, panel_h);

        // Header
        ctx.set_fill_style_str("#38bdf8");
        ctx.set_font("bold 13px system-ui, sans-serif");
        let _ = ctx.fill_text(dict.hud.factory_title, panel_x + 16.0, panel_y + 24.0);

        // Selection info
        ctx.set_font("11px system-ui, sans-serif");
        ctx.set_fill_style_str("#e2e8f0");
        let hull_text = format!("{}: {}", dict.hud.factory_hull, self.state.factory.selected_hull.display_name_localized(lang));
        let _ = ctx.fill_text(&hull_text, panel_x + 16.0, panel_y + 54.0);

        let (cost_c, cost_r) = self.state.factory.current_selection_cost();
        let cost_text = format!("{}: {} {}, {} {}", dict.hud.factory_cost, cost_c, dict.common_ore, cost_r, dict.rare_ore);
        ctx.set_fill_style_str("#facc15");
        let _ = ctx.fill_text(&cost_text, panel_x + 16.0, panel_y + 74.0);

        // Rally-point status
        let rally_str = match self.state.factory.rally_point {
            Some((rx, ry)) => format!("{}: ({}, {})", dict.hud.factory_rally, rx, ry),
            None => dict.hud.factory_unassigned_rally.to_string(),
        };
        ctx.set_fill_style_str("#4ade80");
        let _ = ctx.fill_text(&rally_str, panel_x + 16.0, panel_y + 94.0);

        // Production progress bar
        let bar_x = panel_x + 16.0;
        let bar_y = panel_y + 110.0;
        let bar_w = panel_w - 32.0;
        let bar_h = 14.0;

        ctx.set_fill_style_str("#1e293b");
        ctx.fill_rect(bar_x, bar_y, bar_w, bar_h);

        let progress = (self.state.factory.current_progress / 100.0).clamp(0.0, 1.0);
        ctx.set_fill_style_str("#38bdf8");
        ctx.fill_rect(bar_x, bar_y, bar_w * progress, bar_h);

        ctx.set_stroke_style_str("#64748b");
        ctx.stroke_rect(bar_x, bar_y, bar_w, bar_h);

        // Queue listing
        ctx.set_fill_style_str("#94a3b8");
        let _ = ctx.fill_text(dict.hud.factory_queue, panel_x + 16.0, panel_y + 148.0);

        for (idx, order) in self.state.factory.queue.iter().take(6).enumerate() {
            let qy = panel_y + 170.0 + (idx as f64 * 22.0);
            ctx.set_fill_style_str(if idx == 0 { "#38bdf8" } else { "#cbd5e1" });
            let q_str = format!("{}. {}", idx + 1, order.title());
            let _ = ctx.fill_text(&q_str, panel_x + 20.0, qy);
        }

        // Close hotkey helper
        ctx.set_fill_style_str("#64748b");
        ctx.set_font("10px system-ui, sans-serif");
        let _ = ctx.fill_text(dict.hud.factory_footer_hint, panel_x + 16.0, panel_y + panel_h - 12.0);
    }

    /// Render the in-canvas University Research & Tech Tree Browser modal.
    pub fn render_research_browser(
        &self,
        ctx: &CanvasRenderingContext2d,
        colony: &ColonyState,
        screen_w: f64,
        screen_h: f64,
    ) {
        if !self.state.research.is_open {
            return;
        }

        let lang = crate::i18n::Language::from_code(&self.state.language);
        let dict = crate::i18n::get_dictionary(lang);

        let panel_w = 420.0;
        let panel_h = 440.0;
        let panel_x = (screen_w - panel_w) * 0.5;
        let panel_y = (screen_h - panel_h) * 0.5;

        ctx.set_fill_style_str("rgba(15, 23, 42, 0.96)");
        ctx.fill_rect(panel_x, panel_y, panel_w, panel_h);

        ctx.set_stroke_style_str("#38bdf8");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(panel_x, panel_y, panel_w, panel_h);

        // Header
        ctx.set_fill_style_str("#38bdf8");
        ctx.set_font("bold 13px system-ui, sans-serif");
        let _ = ctx.fill_text(dict.hud.research_title, panel_x + 16.0, panel_y + 24.0);

        // Active project progress bar
        let bar_x = panel_x + 16.0;
        let bar_y = panel_y + 40.0;
        let bar_w = panel_w - 32.0;
        let bar_h = 14.0;

        ctx.set_fill_style_str("#1e293b");
        ctx.fill_rect(bar_x, bar_y, bar_w, bar_h);

        let prog = (self.state.research.progress_percent / 100.0).clamp(0.0, 1.0);
        ctx.set_fill_style_str("#4ade80");
        ctx.fill_rect(bar_x, bar_y, bar_w * prog, bar_h);

        ctx.set_stroke_style_str("#64748b");
        ctx.stroke_rect(bar_x, bar_y, bar_w, bar_h);

        let status_str = match self.state.research.active_tech_id {
            Some(tid) => format!("{}: Tech #{} ({:.0}%)", dict.hud.research_active, tid, self.state.research.progress_percent),
            None => dict.hud.research_none.to_string(),
        };
        ctx.set_fill_style_str("#e2e8f0");
        ctx.set_font("10px system-ui, sans-serif");
        let _ = ctx.fill_text(&status_str, bar_x + 6.0, bar_y + 11.0);

        // Catalog of nodes
        let catalog = ResearchBrowserState::catalog();
        let list_y = panel_y + 75.0;

        ctx.set_font("11px system-ui, sans-serif");
        for (idx, item) in catalog.iter().take(9).enumerate() {
            let row_y = list_y + (idx as f64 * 34.0);
            let is_done = colony.tech_tree.is_researched(item.id);
            let is_unlocked = self.state.research.is_unlocked(item, &colony.tech_tree);

            let color = if is_done {
                "#4ade80" // Green researched
            } else if is_unlocked {
                "#38bdf8" // Blue unlocked
            } else {
                "#64748b" // Gray locked
            };

            ctx.set_fill_style_str(color);
            let node_str = format!("{} [{} C]", item.name, item.cost_common);
            let _ = ctx.fill_text(&node_str, panel_x + 16.0, row_y);

            ctx.set_fill_style_str("#94a3b8");
            ctx.set_font("9px system-ui, sans-serif");
            let cat_str = format!("{}: {}", item.category, item.description);
            let _ = ctx.fill_text(&cat_str, panel_x + 16.0, row_y + 12.0);
            ctx.set_font("11px system-ui, sans-serif");
        }

        ctx.set_fill_style_str("#64748b");
        ctx.set_font("10px system-ui, sans-serif");
        let _ = ctx.fill_text(dict.hud.research_footer_hint, panel_x + 16.0, panel_y + panel_h - 12.0);
    }
}

// =========================================================================
// 8. Unit Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radar_projection_conversions() {
        let proj = RadarProjection::new(64, 64, 128.0, 128.0);

        // Origin
        assert_eq!(proj.world_tile_to_minimap(0.0, 0.0), (0.0, 0.0));
        assert_eq!(proj.minimap_to_world_tile(0.0, 0.0), (0.0, 0.0));

        // Center
        let (mx, my) = proj.world_tile_to_minimap(32.0, 32.0);
        assert!((mx - 64.0).abs() < 1e-4);
        assert!((my - 64.0).abs() < 1e-4);

        let (tx, ty) = proj.minimap_to_world_tile(64.0, 64.0);
        assert!((tx - 32.0).abs() < 1e-4);
        assert!((ty - 32.0).abs() < 1e-4);
    }

    #[test]
    fn test_radar_camera_pan_math() {
        let (cam_x, cam_y) = RadarProjection::calculate_camera_pan(
            64.0, 64.0, // clicked center of 128x128 radar
            128.0, 128.0, 800.0, 600.0, // screen 800x600
            32.0, 32.0, 1.0, 64, 64,
        );

        // Center tile for 64x64 is (32, 32).
        assert!((cam_x - (-624.0)).abs() < 1e-4);
        assert!((cam_y - (-724.0)).abs() < 1e-4);
    }

    #[test]
    fn test_energy_grid_states() {
        assert_eq!(
            EnergyGridState::from_power(500, 300),
            EnergyGridState::Normal
        );
        assert_eq!(
            EnergyGridState::from_power(250, 300),
            EnergyGridState::Brownout
        );
        assert_eq!(
            EnergyGridState::from_power(100, 300),
            EnergyGridState::Blackout
        );
        assert_eq!(
            EnergyGridState::from_power(0, 300),
            EnergyGridState::Blackout
        );
    }

    #[test]
    fn test_morale_tiers() {
        assert_eq!(MoraleTier::from_percentage(95), MoraleTier::Excellent);
        assert_eq!(MoraleTier::from_percentage(85), MoraleTier::Good);
        assert_eq!(MoraleTier::from_percentage(65), MoraleTier::Fair);
        assert_eq!(MoraleTier::from_percentage(35), MoraleTier::Poor);
        assert_eq!(MoraleTier::from_percentage(15), MoraleTier::Rioting);
    }

    #[test]
    fn test_factory_production_queue_and_cancellation() {
        let mut factory = FactoryQueueState::default();
        let tech = TechTree::new();
        let mut common = 1000;
        let mut rare = 500;

        factory.selected_hull = VehicleHull::Panther;
        factory.selected_weapon = Some(WeaponAttachment::Laser);

        // Enqueue 1
        let order = factory
            .enqueue_selected(&mut common, &mut rare, &tech)
            .expect("should enqueue panther");
        assert_eq!(order.hull, VehicleHull::Panther);
        assert_eq!(factory.queue.len(), 1);
        assert_eq!(common, 750); // 1000 - 250
        assert_eq!(rare, 450); // 500 - 50

        // Cancel order
        let cancelled = factory
            .cancel_order(0, &mut common, &mut rare)
            .expect("should cancel");
        assert_eq!(cancelled.hull, VehicleHull::Panther);
        assert_eq!(factory.queue.len(), 0);
        assert_eq!(common, 1000);
        assert_eq!(rare, 500);
    }

    #[test]
    fn test_tech_browser_prerequisites() {
        let tech_tree = TechTree::new(); // Starts with Basic Lab (2106)
        let state = ResearchBrowserState::default();
        let catalog = ResearchBrowserState::catalog();

        let astronomy = catalog.iter().find(|t| t.id == 2701).unwrap();
        assert!(state.is_unlocked(astronomy, &tech_tree));

        let vulcanology = catalog.iter().find(|t| t.id == 4301).unwrap();
        // Requires Astronomy, which is not yet researched
        assert!(!state.is_unlocked(vulcanology, &tech_tree));
    }

    #[test]
    fn test_hotkey_dispatch() {
        let mut hotkeys = HotkeySystem::default();

        assert_eq!(
            hotkeys.handle_key(" ", false, false),
            HotkeyAction::JumpToLastAlert
        );
        assert_eq!(
            hotkeys.handle_key("p", false, false),
            HotkeyAction::TogglePause
        );
        assert!(hotkeys.is_paused);
        assert_eq!(
            hotkeys.handle_key("f", false, false),
            HotkeyAction::ToggleFactoryView
        );
        assert_eq!(
            hotkeys.handle_key("r", false, false),
            HotkeyAction::ToggleResearchView
        );
        assert_eq!(
            hotkeys.handle_key("b", false, false),
            HotkeyAction::JumpToBaseOverview
        );
        assert_eq!(
            hotkeys.handle_key("m", false, false),
            HotkeyAction::ToggleMinimapDetail
        );
        assert_eq!(
            hotkeys.handle_key("F5", false, false),
            HotkeyAction::QuickSave
        );
        assert_eq!(
            hotkeys.handle_key("F9", false, false),
            HotkeyAction::QuickLoad
        );

        // Control group assignment & recall
        assert_eq!(
            hotkeys.handle_key("1", true, false),
            HotkeyAction::AssignControlGroup(1)
        );
        hotkeys.assign_group(1, vec![101, 102]);
        assert_eq!(hotkeys.get_group(1), &[101, 102]);

        assert_eq!(
            hotkeys.handle_key("1", false, false),
            HotkeyAction::RecallControlGroup(1)
        );
    }

    #[test]
    fn test_notification_dispatcher_and_jump() {
        let mut dispatcher = NotificationDispatcher::default();
        dispatcher.push_alert(
            "MAGMA ERUPTION",
            "Volcanic fissure detected near sector 4",
            AlertPriority::Critical,
            Some((45, 28)),
            1.25,
        );

        assert_eq!(dispatcher.jump_to_last_alert(), Some((45, 28)));
        assert_eq!(dispatcher.alerts.len(), 1);
        assert_eq!(dispatcher.alerts[0].priority, AlertPriority::Critical);
    }

    #[test]
    fn test_network_telemetry_formatting() {
        let mut telemetry = NetworkTelemetryState::default();

        // Disconnected state
        telemetry.is_connected = false;
        assert_eq!(telemetry.peer_status_badge(), "P2P STANDBY");

        // Connected state
        telemetry.is_connected = true;
        assert_eq!(telemetry.peer_status_badge(), "P2P ACTIVE: Host & Guest");

        // Ping telemetry meter
        telemetry.ping_ms = 18;
        assert_eq!(telemetry.ping_meter(), "Ping: 18ms");

        telemetry.ping_ms = 24;
        assert_eq!(telemetry.ping_meter(), "Ping: 24ms");

        // Turn-buffered lockstep sync status
        telemetry.lockstep_turn = 1240;
        telemetry.is_in_sync = true;
        assert_eq!(telemetry.lockstep_sync_status(), "In Sync: Turn 1240");

        // Desync / buffering state
        telemetry.is_in_sync = false;
        assert_eq!(telemetry.lockstep_sync_status(), "Buffering: Turn 1240");

        // update_telemetry method
        telemetry.update_telemetry(32, 1300, true);
        assert_eq!(telemetry.ping_meter(), "Ping: 32ms");
        assert_eq!(telemetry.lockstep_sync_status(), "In Sync: Turn 1300");
    }

    #[test]
    fn test_tactical_chat_message_dispatch_and_queue_management() {
        let mut chat = TacticalChatState::default();
        chat.max_messages = 3;

        // 1. Interactive chat prompt buffer input
        assert!(!chat.is_open);
        chat.open();
        assert!(chat.is_open);

        // Type characters
        for c in "Need assistance at south ridge!".chars() {
            chat.type_char(c);
        }
        assert_eq!(chat.input_buffer, "Need assistance at south ridge!");

        // Backspace
        chat.backspace();
        assert_eq!(chat.input_buffer, "Need assistance at south ridge");
        chat.type_char('!');

        // Commit message
        let committed = chat
            .commit_message(10.5, "Player1", Faction::Eden)
            .expect("Committed message");
        assert_eq!(committed.sender, "Player1");
        assert_eq!(committed.faction, Some(Faction::Eden));
        assert_eq!(committed.text, "Need assistance at south ridge!");
        assert_eq!(committed.formatted_prefix(), "[Eden] Player1:");
        assert_eq!(committed.faction_color(), "#38bdf8");
        assert_eq!(
            committed.full_line(),
            "[Eden] Player1: Need assistance at south ridge!"
        );
        assert!(!chat.is_open);
        assert!(chat.input_buffer.is_empty());
        assert_eq!(chat.messages.len(), 1);

        // 2. Incoming peer message (Plymouth)
        chat.push_incoming(
            "Sarah",
            "Affirmative! Dispatching Lynxes to south ridge.",
            Some(Faction::Plymouth),
            10.6,
        );
        assert_eq!(chat.messages.len(), 2);
        assert_eq!(chat.messages[0].formatted_prefix(), "[Plymouth] Sarah:");
        assert_eq!(chat.messages[0].faction_color(), "#f87171");
        assert_eq!(
            chat.messages[0].full_line(),
            "[Plymouth] Sarah: Affirmative! Dispatching Lynxes to south ridge."
        );

        // 3. Queue overflow / eviction check (max_messages = 3)
        chat.push_incoming("BaseAI", "Common Ore mine depleted.", None, 10.7);
        assert_eq!(chat.messages.len(), 3);

        chat.push_incoming("Player1", "Under heavy railgun fire!", Some(Faction::Eden), 10.8);
        assert_eq!(chat.messages.len(), 3);
        // The newest message is at index 0
        assert_eq!(chat.messages[0].sender, "Player1");
        assert_eq!(chat.messages[0].text, "Under heavy railgun fire!");
    }
}
