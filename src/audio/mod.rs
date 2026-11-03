//! Web Audio API sound manager for Outpost 2 sound effects

use wasm_bindgen::JsValue;
use web_sys::{AudioContext, GainNode, OscillatorNode};

pub struct AudioManager {
    ctx: Option<AudioContext>,
    master_gain: Option<GainNode>,
    muted: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            ctx: None,
            master_gain: None,
            muted: false,
        }
    }

    /// Initialize the AudioContext (must be triggered by a user gesture)
    pub fn init(&mut self) -> Result<(), JsValue> {
        if self.ctx.is_none() {
            let ctx = AudioContext::new()?;
            let gain = ctx.create_gain()?;
            gain.gain().set_value(0.35); // 35% comfortable master volume
            gain.connect_with_audio_node(&ctx.destination())?;

            self.ctx = Some(ctx);
            self.master_gain = Some(gain);
        }
        Ok(())
    }

    pub fn toggle_mute(&mut self) -> bool {
        self.muted = !self.muted;
        if let Some(gain) = &self.master_gain {
            let val = if self.muted { 0.0 } else { 0.35 };
            gain.gain().set_value(val);
        }
        self.muted
    }

    /// Play futuristic sci-fi UI confirmation beep
    pub fn play_ui_beep(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            let _ = self.play_tone(ctx, gain, 880.0, 0.04);
        }
    }

    /// Play tactical unit acknowledgement radio chime
    pub fn play_unit_ack(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            let _ = self.play_tone(ctx, gain, 587.33, 0.06); // D5
        }
    }

    /// Play building placement / power grid confirmation sound
    pub fn play_build_confirm(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            let _ = self.play_tone(ctx, gain, 440.0, 0.08); // A4
        }
    }

    fn play_tone(&self, ctx: &AudioContext, out: &GainNode, freq: f32, duration: f64) -> Result<(), JsValue> {
        let osc: OscillatorNode = ctx.create_oscillator()?;
        let note_gain: GainNode = ctx.create_gain()?;

        osc.frequency().set_value(freq);
        osc.set_type(web_sys::OscillatorType::Sine);

        let now = ctx.current_time();
        note_gain.gain().set_value_at_time(0.5, now)?;
        note_gain.gain().exponential_ramp_to_value_at_time(0.001, now + duration)?;

        osc.connect_with_audio_node(&note_gain)?;
        note_gain.connect_with_audio_node(out)?;

        osc.start()?;
        osc.stop_with_when(now + duration)?;

        Ok(())
    }
}
