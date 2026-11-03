//! Web Audio API sound manager for Outpost 2 sound effects

pub mod music;
pub mod radio;

use wasm_bindgen::JsValue;
use web_sys::{AudioContext, GainNode, OscillatorNode};
use music::MusicEngine;

pub struct AudioManager {
    pub ctx: Option<AudioContext>,
    master_gain: Option<GainNode>,
    muted: bool,
    pub music: MusicEngine,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            ctx: None,
            master_gain: None,
            muted: false,
            music: MusicEngine::new(),
        }
    }

    /// Initialize the AudioContext (must be triggered by a user gesture)
    pub fn init(&mut self) -> Result<(), JsValue> {
        if self.ctx.is_none() {
            let ctx = AudioContext::new()?;
            let gain = ctx.create_gain()?;
            gain.gain().set_value(0.35); // 35% comfortable master volume
            gain.connect_with_audio_node(&ctx.destination())?;

            let _ = self.music.init(&ctx);

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

    pub fn is_muted(&self) -> bool {
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

    /// Play high-energy laser beam discharge sound
    pub fn play_laser(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(note_gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(web_sys::OscillatorType::Sawtooth);
                    let _ = osc.frequency().set_value_at_time(880.0, now);
                    let _ = osc.frequency().exponential_ramp_to_value_at_time(110.0, now + 0.12);
                    let _ = note_gain.gain().set_value_at_time(0.4, now);
                    let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.01, now + 0.12);
                    let _ = osc.connect_with_audio_node(&note_gain);
                    let _ = note_gain.connect_with_audio_node(gain);
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + 0.12);
                }
            }
        }
    }

    /// Play heavy vehicle or structure destruction explosion
    pub fn play_explosion(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(note_gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(web_sys::OscillatorType::Triangle);
                    let _ = osc.frequency().set_value_at_time(140.0, now);
                    let _ = osc.frequency().exponential_ramp_to_value_at_time(28.0, now + 0.35);
                    let _ = note_gain.gain().set_value_at_time(0.6, now);
                    let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.01, now + 0.35);
                    let _ = osc.connect_with_audio_node(&note_gain);
                    let _ = note_gain.connect_with_audio_node(gain);
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + 0.35);
                }
            }
        }
    }

    /// Play triumphant victory fanfare upon completing campaign objectives or launching starship
    pub fn play_victory_fanfare(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            let notes = [
                (523.25, 0.00, 0.12), // C5
                (392.00, 0.12, 0.12), // G4
                (523.25, 0.24, 0.12), // C5
                (659.25, 0.36, 0.15), // E5
                (783.99, 0.52, 0.18), // G5
                (1046.50, 0.72, 0.55), // C6
            ];
            for &(freq, start_offset, dur) in &notes {
                if let Ok(osc) = ctx.create_oscillator() {
                    if let Ok(note_gain) = ctx.create_gain() {
                        let now = ctx.current_time() + start_offset;
                        osc.set_type(web_sys::OscillatorType::Triangle);
                        let _ = osc.frequency().set_value_at_time(freq as f32, now);
                        let _ = note_gain.gain().set_value_at_time(0.4, now);
                        let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.001, now + dur);
                        let _ = osc.connect_with_audio_node(&note_gain);
                        let _ = note_gain.connect_with_audio_node(gain);
                        let _ = osc.start_with_when(now);
                        let _ = osc.stop_with_when(now + dur);
                    }
                }
            }
        }
    }

    /// Play ominous defeat alert when Command Center is lost
    pub fn play_defeat_alert(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            let notes = [220.0, 196.0, 174.61, 146.83]; // A3, G3, F3, D3
            for (i, &freq) in notes.iter().enumerate() {
                if let Ok(osc) = ctx.create_oscillator() {
                    if let Ok(note_gain) = ctx.create_gain() {
                        let now = ctx.current_time() + (i as f64 * 0.22);
                        osc.set_type(web_sys::OscillatorType::Sawtooth);
                        let _ = osc.frequency().set_value_at_time(freq as f32, now);
                        let _ = note_gain.gain().set_value_at_time(0.35, now);
                        let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.001, now + 0.35);
                        let _ = osc.connect_with_audio_node(&note_gain);
                        let _ = note_gain.connect_with_audio_node(gain);
                        let _ = osc.start_with_when(now);
                        let _ = osc.stop_with_when(now + 0.35);
                    }
                }
            }
        }
    }

    /// Play triumphant structure commissioning / ConVec kit deployment arpeggio
    pub fn play_construction_deploy(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            let notes = [523.25, 659.25, 783.99]; // C5, E5, G5
            for (i, &freq) in notes.iter().enumerate() {
                if let Ok(osc) = ctx.create_oscillator() {
                    if let Ok(note_gain) = ctx.create_gain() {
                        let now = ctx.current_time() + (i as f64 * 0.08);
                        osc.set_type(web_sys::OscillatorType::Sine);
                        let _ = osc.frequency().set_value_at_time(freq, now);
                        let _ = note_gain.gain().set_value_at_time(0.35, now);
                        let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.001, now + 0.22);
                        let _ = osc.connect_with_audio_node(&note_gain);
                        let _ = note_gain.connect_with_audio_node(gain);
                        let _ = osc.start_with_when(now);
                        let _ = osc.stop_with_when(now + 0.22);
                    }
                }
            }
        }
    }

    /// Play atmospheric dust vortex whistling howling wind gust
    pub fn play_vortex_howl(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(note_gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(web_sys::OscillatorType::Sine);
                    let _ = osc.frequency().set_value_at_time(180.0, now);
                    let _ = osc.frequency().exponential_ramp_to_value_at_time(85.0, now + 0.35);
                    let _ = osc.frequency().exponential_ramp_to_value_at_time(160.0, now + 0.65);
                    let _ = note_gain.gain().set_value_at_time(0.25, now);
                    let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.001, now + 0.65);
                    let _ = osc.connect_with_audio_node(&note_gain);
                    let _ = note_gain.connect_with_audio_node(gain);
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + 0.65);
                }
            }
        }
    }

    /// Play building placement error / obstructed buzzer
    pub fn play_build_error(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, &self.master_gain) {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(note_gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(web_sys::OscillatorType::Square);
                    let _ = osc.frequency().set_value_at_time(130.0, now);
                    let _ = note_gain.gain().set_value_at_time(0.3, now);
                    let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.01, now + 0.15);
                    let _ = osc.connect_with_audio_node(&note_gain);
                    let _ = note_gain.connect_with_audio_node(gain);
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + 0.15);
                }
            }
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
