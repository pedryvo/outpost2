//! Mobile Touch Ergonomics & High-DPI Display Scaling Engine
//!
//! Provides gesture tracking for single-touch tap, two-finger panning,
//! two-finger pinch-to-zoom, and long-press contextual actions.
//! Also provides High-DPI Retina display resolution calculations with
//! crisp pixel alignment.

#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum TouchGestureEvent {
    /// Single finger tap (used for unit selection or move/attack/repair orders)
    Tap { x: f64, y: f64 },
    /// Two finger drag / smooth map panning
    TwoFingerPan { dx: f64, dy: f64 },
    /// Two finger pinch zoom (smooth zoom in / zoom out)
    PinchZoom {
        current_dist: f64,
        dist_delta: f64,
        zoom_factor: f64,
    },
    /// Single finger held without moving for > 500ms (context action / rally point)
    LongPress { x: f64, y: f64 },
    /// Single finger drag across the map
    SingleFingerDrag {
        x: f64,
        y: f64,
        dx: f64,
        dy: f64,
    },
    /// No gesture recognized for this frame/update
    None,
}

#[derive(Debug, Clone)]
pub struct TouchTrackerConfig {
    pub long_press_threshold_ms: f64,
    pub tap_max_duration_ms: f64,
    pub tap_max_movement_sq: f64,
    pub pinch_min_distance: f64,
}

impl Default for TouchTrackerConfig {
    fn default() -> Self {
        Self {
            long_press_threshold_ms: 500.0,
            tap_max_duration_ms: 400.0,
            tap_max_movement_sq: 225.0, // 15 pixels squared
            pinch_min_distance: 20.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TouchGestureTracker {
    config: TouchTrackerConfig,
    touch_start_time: f64,
    start_positions: Vec<(f64, f64)>,
    last_positions: Vec<(f64, f64)>,
    initial_pinch_dist: Option<f64>,
    last_pinch_dist: Option<f64>,
    long_press_fired: bool,
    active_touch_count: usize,
}

impl TouchGestureTracker {
    pub fn new() -> Self {
        Self::with_config(TouchTrackerConfig::default())
    }

    pub fn with_config(config: TouchTrackerConfig) -> Self {
        Self {
            config,
            touch_start_time: 0.0,
            start_positions: Vec::new(),
            last_positions: Vec::new(),
            initial_pinch_dist: None,
            last_pinch_dist: None,
            long_press_fired: false,
            active_touch_count: 0,
        }
    }

    /// Process touchstart event
    pub fn on_touch_start(&mut self, touches: &[(f64, f64)], timestamp_ms: f64) {
        self.touch_start_time = timestamp_ms;
        self.start_positions = touches.to_vec();
        self.last_positions = touches.to_vec();
        self.long_press_fired = false;
        self.active_touch_count = touches.len();

        if touches.len() == 2 {
            let dist = distance(touches[0], touches[1]);
            self.initial_pinch_dist = Some(dist);
            self.last_pinch_dist = Some(dist);
        } else {
            self.initial_pinch_dist = None;
            self.last_pinch_dist = None;
        }
    }

    /// Process touchmove event
    pub fn on_touch_move(&mut self, touches: &[(f64, f64)], _timestamp_ms: f64) -> TouchGestureEvent {
        self.active_touch_count = touches.len();

        if touches.is_empty() {
            return TouchGestureEvent::None;
        }

        if touches.len() == 1 {
            let cur = touches[0];
            let start = self.start_positions.first().copied().unwrap_or(cur);
            let last = self.last_positions.first().copied().unwrap_or(cur);

            let move_dist_sq = (cur.0 - start.0).powi(2) + (cur.1 - start.1).powi(2);
            let dx = cur.0 - last.0;
            let dy = cur.1 - last.1;
            self.last_positions = touches.to_vec();

            if move_dist_sq > self.config.tap_max_movement_sq {
                // Movement exceeds tap threshold: cancel long-press eligibility
                self.long_press_fired = true; // Prevents delayed long-press from triggering

                if dx.abs() > 0.5 || dy.abs() > 0.5 {
                    TouchGestureEvent::SingleFingerDrag {
                        x: cur.0,
                        y: cur.1,
                        dx,
                        dy,
                    }
                } else {
                    TouchGestureEvent::None
                }
            } else {
                TouchGestureEvent::None
            }
        } else if touches.len() >= 2 {
            // Cancel long press if multiple fingers are placed
            self.long_press_fired = true;

            let t0 = touches[0];
            let t1 = touches[1];
            let cur_dist = distance(t0, t1);

            let last_t0 = self.last_positions.first().copied().unwrap_or(t0);
            let last_t1 = self.last_positions.get(1).copied().unwrap_or(t1);

            let mid_cur_x = (t0.0 + t1.0) * 0.5;
            let mid_cur_y = (t0.1 + t1.1) * 0.5;
            let mid_last_x = (last_t0.0 + last_t1.0) * 0.5;
            let mid_last_y = (last_t0.1 + last_t1.1) * 0.5;

            let pan_dx = mid_cur_x - mid_last_x;
            let pan_dy = mid_cur_y - mid_last_y;

            let prev_dist = self.last_pinch_dist.unwrap_or(cur_dist);
            let dist_delta = cur_dist - prev_dist;

            self.last_positions = touches.to_vec();
            self.last_pinch_dist = Some(cur_dist);

            // If distance delta is notable, prioritize pinch-zoom
            if dist_delta.abs() > 2.0 && cur_dist > self.config.pinch_min_distance {
                let zoom_factor = if prev_dist > 0.0 {
                    cur_dist / prev_dist
                } else {
                    1.0
                };
                TouchGestureEvent::PinchZoom {
                    current_dist: cur_dist,
                    dist_delta,
                    zoom_factor,
                }
            } else if pan_dx.abs() > 0.5 || pan_dy.abs() > 0.5 {
                TouchGestureEvent::TwoFingerPan {
                    dx: pan_dx,
                    dy: pan_dy,
                }
            } else {
                TouchGestureEvent::None
            }
        } else {
            TouchGestureEvent::None
        }
    }

    /// Process touchend event
    pub fn on_touch_end(&mut self, timestamp_ms: f64) -> TouchGestureEvent {
        let duration = timestamp_ms - self.touch_start_time;

        if self.active_touch_count == 1 && !self.long_press_fired {
            if let Some(start) = self.start_positions.first() {
                let last = self.last_positions.first().unwrap_or(start);
                let move_dist_sq = (last.0 - start.0).powi(2) + (last.1 - start.1).powi(2);

                if duration <= self.config.tap_max_duration_ms && move_dist_sq <= self.config.tap_max_movement_sq {
                    self.active_touch_count = 0;
                    return TouchGestureEvent::Tap {
                        x: start.0,
                        y: start.1,
                    };
                }
            }
        }

        self.active_touch_count = 0;
        self.initial_pinch_dist = None;
        self.last_pinch_dist = None;
        TouchGestureEvent::None
    }

    /// Check if long press (> 500ms) has triggered during continuous hold
    pub fn check_long_press(&mut self, current_time_ms: f64) -> Option<TouchGestureEvent> {
        if self.active_touch_count != 1 || self.long_press_fired {
            return None;
        }

        let elapsed = current_time_ms - self.touch_start_time;
        if elapsed >= self.config.long_press_threshold_ms {
            if let Some(start) = self.start_positions.first() {
                let last = self.last_positions.first().unwrap_or(start);
                let move_dist_sq = (last.0 - start.0).powi(2) + (last.1 - start.1).powi(2);
                if move_dist_sq <= self.config.tap_max_movement_sq {
                    self.long_press_fired = true;
                    return Some(TouchGestureEvent::LongPress {
                        x: start.0,
                        y: start.1,
                    });
                }
            }
        }

        None
    }

    pub fn is_active(&self) -> bool {
        self.active_touch_count > 0
    }
}

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

/// Canvas scaling dimensions for High-DPI (Retina / 4K) displays
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HighDpiDimensions {
    pub physical_width: u32,
    pub physical_height: u32,
    pub logical_width: f64,
    pub logical_height: f64,
    pub dpr: f64,
}

/// Compute crisp backing store buffer dimensions from CSS client dimensions and DPR
pub fn compute_high_dpi_canvas_dimensions(
    client_width: f64,
    client_height: f64,
    device_pixel_ratio: f64,
) -> HighDpiDimensions {
    let dpr = device_pixel_ratio.max(1.0);
    let physical_width = (client_width * dpr).round().max(1.0) as u32;
    let physical_height = (client_height * dpr).round().max(1.0) as u32;

    HighDpiDimensions {
        physical_width,
        physical_height,
        logical_width: client_width,
        logical_height: client_height,
        dpr,
    }
}

/// Snap float coordinate to nearest physical pixel boundary to eliminate antialiased blur
pub fn snap_to_pixel_boundary(coord: f64, dpr: f64) -> f64 {
    let safe_dpr = dpr.max(1.0);
    (coord * safe_dpr).round() / safe_dpr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_touch_tap_detection() {
        let mut tracker = TouchGestureTracker::new();
        let touch_point = (120.0, 240.0);

        tracker.on_touch_start(&[touch_point], 1000.0);
        assert!(tracker.is_active());

        // Small stationary movement
        let move_ev = tracker.on_touch_move(&[(122.0, 241.0)], 1100.0);
        assert_eq!(move_ev, TouchGestureEvent::None);

        // Released within tap duration (< 400ms)
        let end_ev = tracker.on_touch_end(1250.0);
        assert_eq!(
            end_ev,
            TouchGestureEvent::Tap {
                x: 120.0,
                y: 240.0,
            }
        );
        assert!(!tracker.is_active());
    }

    #[test]
    fn test_tap_cancelled_by_large_movement() {
        let mut tracker = TouchGestureTracker::new();
        tracker.on_touch_start(&[(100.0, 100.0)], 1000.0);

        // Drag finger significantly (> 15px)
        let move_ev = tracker.on_touch_move(&[(140.0, 100.0)], 1100.0);
        assert!(matches!(move_ev, TouchGestureEvent::SingleFingerDrag { .. }));

        // Release: should NOT register as tap
        let end_ev = tracker.on_touch_end(1200.0);
        assert_eq!(end_ev, TouchGestureEvent::None);
    }

    #[test]
    fn test_long_press_recognition() {
        let mut tracker = TouchGestureTracker::new();
        let touch_point = (300.0, 450.0);

        tracker.on_touch_start(&[touch_point], 1000.0);

        // At 300ms, not yet 500ms
        let check_300 = tracker.check_long_press(1300.0);
        assert_eq!(check_300, None);

        // At 550ms without movement: fires LongPress
        let check_550 = tracker.check_long_press(1550.0);
        assert_eq!(
            check_550,
            Some(TouchGestureEvent::LongPress {
                x: 300.0,
                y: 450.0,
            })
        );

        // Subsequent checks should not re-fire
        let check_600 = tracker.check_long_press(1600.0);
        assert_eq!(check_600, None);
    }

    #[test]
    fn test_two_finger_panning() {
        let mut tracker = TouchGestureTracker::new();
        let t1 = (100.0, 100.0);
        let t2 = (200.0, 100.0);

        tracker.on_touch_start(&[t1, t2], 1000.0);

        // Both fingers move right by 25px (distance remains 100px)
        let t1_moved = (125.0, 100.0);
        let t2_moved = (225.0, 100.0);

        let ev = tracker.on_touch_move(&[t1_moved, t2_moved], 1050.0);
        assert_eq!(ev, TouchGestureEvent::TwoFingerPan { dx: 25.0, dy: 0.0 });
    }

    #[test]
    fn test_two_finger_pinch_zoom() {
        let mut tracker = TouchGestureTracker::new();
        let t1 = (100.0, 100.0);
        let t2 = (200.0, 100.0); // Initial dist = 100px

        tracker.on_touch_start(&[t1, t2], 1000.0);

        // Fingers spread apart to 150px (pinch-zoom out / magnifying)
        let t1_spread = (75.0, 100.0);
        let t2_spread = (225.0, 100.0); // New dist = 150px

        let ev = tracker.on_touch_move(&[t1_spread, t2_spread], 1050.0);
        match ev {
            TouchGestureEvent::PinchZoom {
                current_dist,
                dist_delta,
                zoom_factor,
            } => {
                assert!((current_dist - 150.0).abs() < 1e-4);
                assert!((dist_delta - 50.0).abs() < 1e-4);
                assert!((zoom_factor - 1.5).abs() < 1e-4);
            }
            other => panic!("Expected PinchZoom, got {:?}", other),
        }
    }

    #[test]
    fn test_high_dpi_canvas_dimensions() {
        // Standard 1080p (DPR = 1.0)
        let dim_1x = compute_high_dpi_canvas_dimensions(1920.0, 1080.0, 1.0);
        assert_eq!(dim_1x.physical_width, 1920);
        assert_eq!(dim_1x.physical_height, 1080);
        assert_eq!(dim_1x.dpr, 1.0);

        // Apple Retina / 4K (DPR = 2.0)
        let dim_2x = compute_high_dpi_canvas_dimensions(1440.0, 900.0, 2.0);
        assert_eq!(dim_2x.physical_width, 2880);
        assert_eq!(dim_2x.physical_height, 1800);
        assert_eq!(dim_2x.dpr, 2.0);

        // Fractional scaling (e.g. Windows 125% scale, DPR = 1.25)
        let dim_frac = compute_high_dpi_canvas_dimensions(1000.0, 600.0, 1.25);
        assert_eq!(dim_frac.physical_width, 1250);
        assert_eq!(dim_frac.physical_height, 750);
    }

    #[test]
    fn test_snap_to_pixel_boundary() {
        // On DPR 2.0, coordinates should align to 0.5 physical steps
        let snapped = snap_to_pixel_boundary(10.23, 2.0);
        assert_eq!(snapped, 10.0);

        let snapped_half = snap_to_pixel_boundary(10.26, 2.0);
        assert_eq!(snapped_half, 10.5);
    }
}
