//! Procedural Sci-Fi Atmospheric Music Synthesizer & Dynamic Soundtrack
//!
//! Generates evocative ambient soundscapes for Outpost 2 using Web Audio API nodes:
//! - Sub-bass planetary resonance drone (65.4 Hz C2).
//! - Dynamic soundtrack sequencer switching between Exploration, Combat, and Red Alert modes.
//! - Shimmering pentatonic arpeggios and rhythmic industrial pulses.

use wasm_bindgen::JsValue;
use web_sys::{AudioContext, GainNode, OscillatorNode, OscillatorType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicMood {
    Exploration,
    Combat,
    Emergency,
}

pub struct MusicEngine {
    ctx: Option<AudioContext>,
    master_gain: Option<GainNode>,
    drone_osc: Option<OscillatorNode>,
    is_playing: bool,
    pub mood: MusicMood,
    step_timer: f64,
    step_index: usize,
}

impl MusicEngine {
    pub fn new() -> Self {
        Self {
            ctx: None,
            master_gain: None,
            drone_osc: None,
            is_playing: false,
            mood: MusicMood::Exploration,
            step_timer: 0.0,
            step_index: 0,
        }
    }

    /// Initialize procedural music synthesizer connecting to a designated output node (e.g. music_gain)
    pub fn init_with_output(&mut self, ctx: &AudioContext, output: &GainNode) -> Result<(), JsValue> {
        if self.ctx.is_none() {
            let gain = ctx.create_gain()?;
            gain.gain().set_value(0.12); // Soft ambient volume
            gain.connect_with_audio_node(output)?;

            // Deep planetary drone oscillator
            let drone = ctx.create_oscillator()?;
            drone.set_type(OscillatorType::Sine);
            drone.frequency().set_value(65.41); // C2

            // Low-pass filter for deep space atmosphere
            let filter = ctx.create_biquad_filter()?;
            filter.set_type(web_sys::BiquadFilterType::Lowpass);
            filter.frequency().set_value(220.0);

            drone.connect_with_audio_node(&filter)?;
            filter.connect_with_audio_node(&gain)?;
            drone.start()?;

            self.ctx = Some(ctx.clone());
            self.master_gain = Some(gain);
            self.drone_osc = Some(drone);
            self.is_playing = true;
        }
        Ok(())
    }

    /// Initialize procedural music synthesizer
    pub fn init(&mut self, ctx: &AudioContext) -> Result<(), JsValue> {
        if self.ctx.is_none() {
            let gain = ctx.create_gain()?;
            gain.gain().set_value(0.12); // Soft ambient volume
            gain.connect_with_audio_node(&ctx.destination())?;

            // Deep planetary drone oscillator
            let drone = ctx.create_oscillator()?;
            drone.set_type(OscillatorType::Sine);
            drone.frequency().set_value(65.41); // C2

            // Low-pass filter for deep space atmosphere
            let filter = ctx.create_biquad_filter()?;
            filter.set_type(web_sys::BiquadFilterType::Lowpass);
            filter.frequency().set_value(220.0);

            drone.connect_with_audio_node(&filter)?;
            filter.connect_with_audio_node(&gain)?;
            drone.start()?;

            self.ctx = Some(ctx.clone());
            self.master_gain = Some(gain);
            self.drone_osc = Some(drone);
            self.is_playing = true;
        }
        Ok(())
    }

    /// Update musical sequencer tick according to current tactical mood
    pub fn update(&mut self, dt_secs: f64, is_combat: bool, is_alert: bool) {
        if !self.is_playing {
            return;
        }

        let target_mood = if is_alert {
            MusicMood::Emergency
        } else if is_combat {
            MusicMood::Combat
        } else {
            MusicMood::Exploration
        };
        self.mood = target_mood;

        let interval = match self.mood {
            MusicMood::Combat => 0.28,
            MusicMood::Emergency => 0.35,
            MusicMood::Exploration => 0.85,
        };

        self.step_timer += dt_secs;
        if self.step_timer >= interval {
            self.step_timer = 0.0;
            self.step_index = (self.step_index + 1) % 16;
            self.trigger_step();
        }
    }

    fn trigger_step(&self) {
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            let (freq, osc_type, dur, vol) = match self.mood {
                MusicMood::Exploration => {
                    // Shimmering sci-fi minor pentatonic arpeggio (C4, Eb4, F4, G4, Bb4, C5)
                    let scale = [261.63, 311.13, 349.23, 392.00, 466.16, 523.25, 392.00, 311.13];
                    let f = scale[self.step_index % scale.len()];
                    (f, OscillatorType::Sine, 0.70, 0.06)
                }
                MusicMood::Combat => {
                    // Driving staccato bassline (C2, C2, Eb2, G2, F2, C2)
                    let bassline = [65.41, 65.41, 77.78, 98.00, 87.31, 65.41, 77.78, 98.00];
                    let f = bassline[self.step_index % bassline.len()];
                    (f, OscillatorType::Triangle, 0.22, 0.14)
                }
                MusicMood::Emergency => {
                    // Tense tritone pulsing alert motifs
                    let alert_notes = [130.81, 185.00, 130.81, 196.00]; // C3, F#3, C3, G3
                    let f = alert_notes[self.step_index % alert_notes.len()];
                    (f, OscillatorType::Sawtooth, 0.25, 0.10)
                }
            };

            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(note_gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(osc_type);
                    let _ = osc.frequency().set_value(freq as f32);
                    let _ = note_gain.gain().set_value_at_time(vol, now);
                    let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.001, now + dur);
                    let _ = osc.connect_with_audio_node(&note_gain);
                    let _ = note_gain.connect_with_audio_node(gain);
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + dur);
                }
            }
        }
    }

    /// Toggle background music playback
    pub fn toggle(&mut self) -> bool {
        if let Some(gain) = &self.master_gain {
            self.is_playing = !self.is_playing;
            let val = if self.is_playing { 0.12 } else { 0.0 };
            gain.gain().set_value(val);
        }
        self.is_playing
    }

    /// Update synthesizer volume attenuation
    pub fn set_volume(&mut self, vol: f32) {
        if let Some(gain) = &self.master_gain {
            if self.is_playing {
                let val = (vol.clamp(0.0, 1.0) * 0.15).max(0.0);
                gain.gain().set_value(val);
            }
        }
    }
}
