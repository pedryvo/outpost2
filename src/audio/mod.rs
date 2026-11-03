//! Web Audio API sound manager for Outpost 2 sound effects

#![allow(dead_code)]

pub mod bank;
pub mod music;
pub mod radio;

use wasm_bindgen::JsValue;
use web_sys::{AudioContext, GainNode, OscillatorNode};
use music::MusicEngine;
use bank::SoundBank;

pub struct AudioManager {
    pub ctx: Option<AudioContext>,
    master_gain: Option<GainNode>,
    sfx_gain: Option<GainNode>,
    music_gain: Option<GainNode>,
    master_volume: f32,
    sfx_volume: f32,
    music_volume: f32,
    muted: bool,
    pub music: MusicEngine,
    pub bank: SoundBank,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            ctx: None,
            master_gain: None,
            sfx_gain: None,
            music_gain: None,
            master_volume: 0.75,
            sfx_volume: 0.85,
            music_volume: 0.65,
            muted: false,
            music: MusicEngine::new(),
            bank: SoundBank::new(),
        }
    }

    /// Initialize the AudioContext and gain node topology (must be triggered by a user gesture)
    pub fn init(&mut self) -> Result<(), JsValue> {
        if self.ctx.is_none() {
            let ctx = AudioContext::new()?;

            // Master Gain: roots all game audio into destination
            let master_gain = ctx.create_gain()?;
            let effective_master = if self.muted { 0.0 } else { self.master_volume * 0.5 };
            master_gain.gain().set_value(effective_master);
            master_gain.connect_with_audio_node(&ctx.destination())?;

            // SFX Gain: dedicated bus for legacy WAV samples and procedural effects
            let sfx_gain = ctx.create_gain()?;
            sfx_gain.gain().set_value(self.sfx_volume);
            sfx_gain.connect_with_audio_node(&master_gain)?;

            // Music Gain: dedicated bus for procedural music sequencer and ambient drone
            let music_gain = ctx.create_gain()?;
            music_gain.gain().set_value(self.music_volume);
            music_gain.connect_with_audio_node(&master_gain)?;

            let _ = self.music.init_with_output(&ctx, &music_gain);
            self.bank.init(&ctx, &sfx_gain);

            self.ctx = Some(ctx);
            self.master_gain = Some(master_gain);
            self.sfx_gain = Some(sfx_gain);
            self.music_gain = Some(music_gain);
        }
        Ok(())
    }

    /// Get current SFX audio output GainNode, falling back to master gain if uninitialized
    fn sfx_output(&self) -> Option<&GainNode> {
        self.sfx_gain.as_ref().or(self.master_gain.as_ref())
    }

    /// Set Master Volume (clamped 0.0 to 1.0) with immediate Web Audio GainNode adjustment
    pub fn set_master_volume(&mut self, volume: f32) {
        let clamped = volume.clamp(0.0, 1.0);
        self.master_volume = clamped;
        if !self.muted {
            if let Some(gain) = &self.master_gain {
                gain.gain().set_value(clamped * 0.5);
            }
        }
    }

    /// Set SFX Volume (clamped 0.0 to 1.0) with immediate Web Audio GainNode adjustment
    pub fn set_sfx_volume(&mut self, volume: f32) {
        let clamped = volume.clamp(0.0, 1.0);
        self.sfx_volume = clamped;
        if let Some(gain) = &self.sfx_gain {
            gain.gain().set_value(clamped);
        }
    }

    /// Set Dynamic Soundtrack Music Volume (clamped 0.0 to 1.0) with immediate Web Audio GainNode adjustment
    pub fn set_music_volume(&mut self, volume: f32) {
        let clamped = volume.clamp(0.0, 1.0);
        self.music_volume = clamped;
        if let Some(gain) = &self.music_gain {
            gain.gain().set_value(clamped);
        }
        self.music.set_volume(clamped);
    }

    /// Synchronize all volume slider settings simultaneously
    pub fn apply_options(&mut self, master: f32, sfx: f32, music: f32) {
        self.set_master_volume(master);
        self.set_sfx_volume(sfx);
        self.set_music_volume(music);
    }

    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn sfx_volume(&self) -> f32 {
        self.sfx_volume
    }

    pub fn music_volume(&self) -> f32 {
        self.music_volume
    }

    pub fn toggle_mute(&mut self) -> bool {
        self.muted = !self.muted;
        if let Some(gain) = &self.master_gain {
            let val = if self.muted { 0.0 } else { self.master_volume * 0.5 };
            gain.gain().set_value(val);
        }
        self.muted
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }

    /// Advance dynamic procedural soundtrack sequencer
    pub fn update_music(&mut self, dt_secs: f64, is_combat: bool, is_alert: bool) {
        if !self.muted {
            self.music.update(dt_secs, is_combat, is_alert);
        }
    }

    /// Play arbitrary PCM sample by name from the sound bank
    #[allow(dead_code)]
    pub fn play_sample(&self, name: &str) {
        if self.muted {
            return;
        }
        self.bank.play_sample(name);
    }

    /// Play arbitrary PCM sample by name with explicit volume attenuation
    #[allow(dead_code)]
    pub fn play_sample_with_volume(&self, name: &str, volume: f32) {
        if self.muted {
            return;
        }
        self.bank.play_sample_with_volume(name, volume);
    }

    /// Play tactical voice dispatch event by legacy ID (0..=251)
    #[allow(dead_code)]
    pub fn play_voice_event(&self, event_id: u32) {
        if self.muted {
            return;
        }
        self.bank.play_voice_event(event_id);
    }

    /// Play futuristic sci-fi UI confirmation beep
    pub fn play_ui_beep(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("beep2.wav") {
            self.bank.play_sample("beep2.wav");
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
            let _ = self.play_tone(ctx, gain, 880.0, 0.04);
        }
    }

    /// Play tactical unit acknowledgement radio chime
    pub fn play_unit_ack(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("truk_sel.wav") {
            self.bank.play_sample("truk_sel.wav");
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
            let _ = self.play_tone(ctx, gain, 587.33, 0.06); // D5
        }
    }

    /// Play authentic unit audio confirmation when selecting vehicles:
    /// - `dozr_sel.wav` for Earthworker / Bulldozers
    /// - `combmed.wav` for Panther Medium Combat Tanks
    /// - `comblit.wav` for Lynx Light Combat Tanks
    /// - `combhev.wav` for Tiger Heavy Assault Tanks
    /// - `scout.wav` for Scout / RoboSurveyors
    /// - `repvhc.wav` for Repair Vehicles
    /// - `robmine.wav` for Robo-Miners
    /// - `truk_sel.wav` for Cargo Trucks / Haulers / Transports / ConVecs
    pub fn play_vehicle_selection(&self, unit_name: &str) {
        if self.muted {
            return;
        }
        let lower = unit_name.to_lowercase();
        let sample = if lower.contains("earthworker") || lower.contains("dozer") {
            "dozr_sel.wav"
        } else if lower.contains("repair") {
            "repvhc.wav"
        } else if lower.contains("miner") || lower.contains("robmine") {
            "robmine.wav"
        } else if lower.contains("scout") || lower.contains("surveyor") {
            "scout.wav"
        } else if lower.contains("tiger") || lower.contains("heavy") {
            "combhev.wav"
        } else if lower.contains("panther") || lower.contains("medium") {
            "combmed.wav"
        } else if lower.contains("lynx") || lower.contains("light") {
            "comblit.wav"
        } else {
            "truk_sel.wav"
        };

        if self.bank.is_sample_loaded(sample) {
            self.bank.play_sample(sample);
        } else {
            self.play_unit_ack();
        }
    }

    /// Trigger legacy voice dispatch: "We are under attack!" (savant4.wav)
    pub fn play_under_attack(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("savant4.wav") {
            self.bank.play_sample("savant4.wav");
        } else {
            self.bank.play_voice_event(97);
        }
    }

    /// Trigger legacy voice dispatch: "Structure destroyed" (savant3.wav)
    pub fn play_structure_destroyed(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("savant3.wav") {
            self.bank.play_sample("savant3.wav");
        } else {
            self.bank.play_voice_event(96);
        }
        self.play_explosion();
    }

    /// Trigger legacy voice dispatch: "Vehicle destroyed" (savnt206.wav)
    pub fn play_vehicle_destroyed(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("savnt206.wav") {
            self.bank.play_sample("savnt206.wav");
        } else {
            self.bank.play_voice_event(104);
        }
        self.play_explosion();
    }

    /// Trigger legacy voice dispatch: "Danger: Volcanic eruption detected" (savant68.wav)
    pub fn play_volcanic_eruption(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("savant68.wav") {
            self.bank.play_sample("savant68.wav");
        } else {
            self.bank.play_voice_event(138);
        }
    }

    /// Trigger legacy voice dispatch: "Vortex detected" (savant71.wav)
    pub fn play_vortex_detected(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("savant71.wav") {
            self.bank.play_sample("savant71.wav");
        } else {
            self.bank.play_voice_event(141);
        }
    }

    /// Trigger legacy voice dispatch: "Research completed" (savant2.wav) alongside research fanfare (uni_grad.wav)
    pub fn play_research_completed(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("savant2.wav") {
            self.bank.play_sample("savant2.wav");
        } else {
            self.bank.play_voice_event(95);
        }
        if self.bank.is_sample_loaded("uni_grad.wav") {
            self.bank.play_sample("uni_grad.wav");
        } else {
            self.play_victory_fanfare();
        }
    }

    /// Play building placement / power grid confirmation sound
    pub fn play_build_confirm(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("bld_make.wav") {
            self.bank.play_sample("bld_make.wav");
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
            let _ = self.play_tone(ctx, gain, 440.0, 0.08); // A4
        }
    }

    /// Play high-energy laser beam discharge sound
    pub fn play_laser(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("laser.wav") {
            self.bank.play_sample("laser.wav");
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
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

    /// Play heavy hyper-velocity railgun kinetic discharge sound
    pub fn play_railgun(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(note_gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(web_sys::OscillatorType::Square);
                    let _ = osc.frequency().set_value_at_time(320.0, now);
                    let _ = osc.frequency().exponential_ramp_to_value_at_time(42.0, now + 0.24);
                    let _ = note_gain.gain().set_value_at_time(0.5, now);
                    let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.01, now + 0.24);
                    let _ = osc.connect_with_audio_node(&note_gain);
                    let _ = note_gain.connect_with_audio_node(gain);
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + 0.24);
                }
            }
        }
    }

    /// Play microwave thermal beam sizzle discharge
    pub fn play_microwave(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(note_gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(web_sys::OscillatorType::Sawtooth);
                    let _ = osc.frequency().set_value_at_time(1250.0, now);
                    let _ = osc.frequency().linear_ramp_to_value_at_time(380.0, now + 0.11);
                    let _ = note_gain.gain().set_value_at_time(0.25, now);
                    let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.01, now + 0.11);
                    let _ = osc.connect_with_audio_node(&note_gain);
                    let _ = note_gain.connect_with_audio_node(gain);
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + 0.11);
                }
            }
        }
    }

    /// Play microbial acid cloud corrosive hiss
    pub fn play_acid_cloud(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(note_gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(web_sys::OscillatorType::Triangle);
                    let _ = osc.frequency().set_value_at_time(240.0, now);
                    let _ = osc.frequency().linear_ramp_to_value_at_time(90.0, now + 0.18);
                    let _ = note_gain.gain().set_value_at_time(0.32, now);
                    let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.01, now + 0.18);
                    let _ = osc.connect_with_audio_node(&note_gain);
                    let _ = note_gain.connect_with_audio_node(gain);
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + 0.18);
                }
            }
        }
    }

    /// Play heavy vehicle or structure destruction explosion
    pub fn play_explosion(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("bigexp1.wav") {
            self.bank.play_sample("bigexp1.wav");
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
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
        self.play_sample("uni_grad.wav");
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
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
        self.play_voice_event(227);
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
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
        if self.bank.is_sample_loaded("bld_make.wav") {
            self.bank.play_sample("bld_make.wav");
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
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
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
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

    /// Play urgent colony disaster / red alert dual-tone siren
    pub fn play_disaster_siren(&self) {
        if self.muted {
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
            let cycles = 3;
            for c in 0..cycles {
                let start = c as f64 * 0.40;
                if let Ok(osc) = ctx.create_oscillator() {
                    if let Ok(note_gain) = ctx.create_gain() {
                        let now = ctx.current_time() + start;
                        osc.set_type(web_sys::OscillatorType::Sawtooth);
                        let _ = osc.frequency().set_value_at_time(587.33, now); // D5
                        let _ = osc.frequency().linear_ramp_to_value_at_time(880.0, now + 0.20); // A5
                        let _ = osc.frequency().linear_ramp_to_value_at_time(587.33, now + 0.38); // D5
                        let _ = note_gain.gain().set_value_at_time(0.32, now);
                        let _ = note_gain.gain().exponential_ramp_to_value_at_time(0.01, now + 0.38);
                        let _ = osc.connect_with_audio_node(&note_gain);
                        let _ = note_gain.connect_with_audio_node(gain);
                        let _ = osc.start_with_when(now);
                        let _ = osc.stop_with_when(now + 0.38);
                    }
                }
            }
        }
    }

    /// Play building placement error / obstructed buzzer
    pub fn play_build_error(&self) {
        if self.muted {
            return;
        }
        if self.bank.is_sample_loaded("bld_not.wav") {
            self.bank.play_sample("bld_not.wav");
            return;
        }
        if let (Some(ctx), Some(gain)) = (&self.ctx, self.sfx_output()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_manager_initialization_and_preloads() {
        let mgr = AudioManager::new();
        assert!(!mgr.is_muted());
        assert!(mgr.bank.is_sample_loaded("truk_sel.wav"));
        assert!(mgr.bank.is_sample_loaded("dozr_sel.wav"));
        assert!(mgr.bank.is_sample_loaded("combmed.wav"));
        assert!(mgr.bank.is_sample_loaded("comblit.wav"));
        assert!(mgr.bank.is_sample_loaded("combhev.wav"));
        assert!(mgr.bank.is_sample_loaded("scout.wav"));
        assert!(mgr.bank.is_sample_loaded("repvhc.wav"));
        assert!(mgr.bank.is_sample_loaded("robmine.wav"));
        assert!(mgr.bank.is_sample_loaded("savant4.wav"));
        assert!(mgr.bank.is_sample_loaded("savant3.wav"));
        assert!(mgr.bank.is_sample_loaded("savnt206.wav"));
        assert!(mgr.bank.is_sample_loaded("savant68.wav"));
        assert!(mgr.bank.is_sample_loaded("savant71.wav"));
        assert!(mgr.bank.is_sample_loaded("savant2.wav"));
        assert!(mgr.bank.is_sample_loaded("uni_grad.wav"));
    }

    #[test]
    fn test_audio_event_dispatch_methods_when_muted_or_uninitialized() {
        let mut mgr = AudioManager::new();
        // None of these should panic even when uninitialized
        mgr.play_vehicle_selection("Panther Tank #1");
        mgr.play_vehicle_selection("Earthworker #2");
        mgr.play_vehicle_selection("Robo-Miner #3");
        mgr.play_vehicle_selection("Repair Vehicle #4");
        mgr.play_vehicle_selection("Lynx Scout #5");
        mgr.play_under_attack();
        mgr.play_structure_destroyed();
        mgr.play_vehicle_destroyed();
        mgr.play_volcanic_eruption();
        mgr.play_vortex_detected();
        mgr.play_research_completed();

        mgr.toggle_mute();
        assert!(mgr.is_muted());
        mgr.play_under_attack();
        mgr.play_vehicle_destroyed();
    }

    #[test]
    fn test_audio_gain_clamping_and_volume_routing() {
        let mut mgr = AudioManager::new();
        assert_eq!(mgr.master_volume(), 0.75);
        assert_eq!(mgr.sfx_volume(), 0.85);
        assert_eq!(mgr.music_volume(), 0.65);

        // Clamping upper bounds
        mgr.set_master_volume(1.5);
        mgr.set_sfx_volume(2.0);
        mgr.set_music_volume(3.0);
        assert_eq!(mgr.master_volume(), 1.0);
        assert_eq!(mgr.sfx_volume(), 1.0);
        assert_eq!(mgr.music_volume(), 1.0);

        // Clamping lower bounds
        mgr.set_master_volume(-0.5);
        mgr.set_sfx_volume(-1.0);
        mgr.set_music_volume(-2.0);
        assert_eq!(mgr.master_volume(), 0.0);
        assert_eq!(mgr.sfx_volume(), 0.0);
        assert_eq!(mgr.music_volume(), 0.0);

        // Simultaneous options application
        mgr.apply_options(0.40, 0.80, 0.25);
        assert!((mgr.master_volume() - 0.40).abs() < 1e-4);
        assert!((mgr.sfx_volume() - 0.80).abs() < 1e-4);
        assert!((mgr.music_volume() - 0.25).abs() < 1e-4);

        // Mute toggle preserves configured volume level
        assert!(!mgr.is_muted());
        assert!(mgr.toggle_mute());
        assert!(mgr.is_muted());
        assert_eq!(mgr.master_volume(), 0.40);
        assert!(!mgr.toggle_mute());
        assert!(!mgr.is_muted());
        assert_eq!(mgr.master_volume(), 0.40);
    }
}
