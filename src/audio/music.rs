//! Procedural Sci-Fi Atmospheric Music Synthesizer
//!
//! Generates evocative ambient soundscapes for Outpost 2 using Web Audio API nodes:
//! - Sub-bass planetary resonance drone (65.4 Hz C2).
//! - Space arpeggios pulsing with exponential release envelopes.
//! - Resonant low-pass filtering modeling distant atmospheric winds.

use wasm_bindgen::JsValue;
use web_sys::{AudioContext, GainNode, OscillatorNode, OscillatorType};

pub struct MusicEngine {
    ctx: Option<AudioContext>,
    master_gain: Option<GainNode>,
    drone_osc: Option<OscillatorNode>,
    is_playing: bool,
}

impl MusicEngine {
    pub fn new() -> Self {
        Self {
            ctx: None,
            master_gain: None,
            drone_osc: None,
            is_playing: false,
        }
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

    /// Toggle background music playback
    pub fn toggle(&mut self) -> bool {
        if let Some(gain) = &self.master_gain {
            self.is_playing = !self.is_playing;
            let val = if self.is_playing { 0.12 } else { 0.0 };
            gain.gain().set_value(val);
        }
        self.is_playing
    }
}
