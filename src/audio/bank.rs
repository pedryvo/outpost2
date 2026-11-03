//! Outpost 2 Digital Sample Bank & Waveform Audio Subsystem
//!
//! Provides high-performance sample caching, 8-bit mono unsigned PCM WAV decoding,
//! channel pooling, volume attenuation, and authentic legacy event dispatching.
#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::{AudioBuffer, AudioContext, GainNode};

use crate::archive::VolArchive;

/// Complete legacy Outpost 2 SoundID / VoiceID mapping table (252 slots).
/// Reconstructed from Outpost2.exe RVA 0x4e0170..0x4e0560.
pub const LEGACY_SOUND_TABLE: [&str; 252] = [
    /*   0 */ "bld_fal3.wav",
    /*   1 */ "bld_fal1.wav",
    /*   2 */ "bld_fal2.wav",
    /*   3 */ "bld_make.wav",
    /*   4 */ "ag_sel.wav",
    /*   5 */ "tokamsel.wav",
    /*   6 */ "pantheon.wav",
    /*   7 */ "commsel.wav",
    /*   8 */ "dirt.wav",
    /*   9 */ "fac_andr.wav",
    /*  10 */ "fac_cons.wav",
    /*  11 */ "fac_sel.wav",
    /*  12 */ "fac_sel.wav",
    /*  13 */ "gar_sel.wav",
    /*  14 */ "geo_sel.wav",
    /*  15 */ "gorf.wav",
    /*  16 */ "guardsel.wav",
    /*  17 */ "lab_3.wav",
    /*  18 */ "lab_1.wav",
    /*  19 */ "lab_2.wav",
    /*  20 */ "light.wav",
    /*  21 */ "mag_sel.wav",
    /*  22 */ "medisel.wav",
    /*  23 */ "dirt.wav",
    /*  24 */ "mine_1.wav",
    /*  25 */ "mine_2.wav",
    /*  26 */ "nur_sel.wav",
    /*  27 */ "observat.wav",
    /*  28 */ "rec_fac.wav",
    /*  29 */ "res1_sel.wav",
    /*  30 */ "res2_sel.wav",
    /*  31 */ "res3_sel.wav",
    /*  32 */ "robosel.wav",
    /*  33 */ "smelter1.wav",
    /*  34 */ "smelter1.wav",
    /*  35 */ "tokamsel.wav",
    /*  36 */ "spaceprt.wav",
    /*  37 */ "storage1.wav",
    /*  38 */ "storage2.wav",
    /*  39 */ "tokamsel.wav",
    /*  40 */ "pantheon.wav",
    /*  41 */ "uni_sel.wav",
    /*  42 */ "consvhc.wav",
    /*  43 */ "dozr_sel.wav",
    /*  44 */ "evac_trn.wav",
    /*  45 */ "earthwrk.wav",
    /*  46 */ "robot4.wav",
    /*  47 */ "spidsel.wav",
    /*  48 */ "infantry.wav",
    /*  49 */ "repvhc.wav",
    /*  50 */ "robmine.wav",
    /*  51 */ "robsurv.wav",
    /*  52 */ "scout.wav",
    /*  53 */ "combhev.wav",
    /*  54 */ "comblit.wav",
    /*  55 */ "combmed.wav",
    /*  56 */ "truk_sel.wav",
    /*  57 */ "bld_exp.wav",
    /*  58 */ "uni_grad.wav",
    /*  59 */ "tokaexp.wav",
    /*  60 */ "geyser.wav",
    /*  61 */ "gar_doc.wav",
    /*  62 */ "doc_dor1.wav",
    /*  63 */ "doc_dor2.wav",
    /*  64 */ "doc_grab.wav",
    /*  65 */ "sulv.wav",
    /*  66 */ "robdep.wav",
    /*  67 */ "reprep.wav",
    /*  68 */ "consrep.wav",
    /*  69 */ "dump.wav",
    /*  70 */ "micwarn.wav",
    /*  71 */ "micfire.wav",
    /*  72 */ "bigexp1.wav",
    /*  73 */ "bigexp2.wav",
    /*  74 */ "bigexp3.wav",
    /*  75 */ "medexp1.wav",
    /*  76 */ "medexp2.wav",
    /*  77 */ "smalxp1.wav",
    /*  78 */ "smalxp2.wav",
    /*  79 */ "smalxp3.wav",
    /*  80 */ "quake.wav",
    /*  81 */ "tornado2.wav",
    /*  82 */ "thunder.wav",
    /*  83 */ "microbe.wav",
    /*  84 */ "fuse.wav",
    /*  85 */ "beep2.wav",
    /*  86 */ "beep2.wav",
    /*  87 */ "beep5.wav",
    /*  88 */ "bld_not.wav",
    /*  89 */ "beep6.wav",
    /*  90 */ "beep9.wav",
    /*  91 */ "beep8.wav",
    /*  92 */ "beep5.wav",
    /*  93 */ "message2.wav",
    /*  94 */ "savnt212.wav",
    /*  95 */ "savant2.wav",
    /*  96 */ "savant3.wav",
    /*  97 */ "savant4.wav",
    /*  98 */ "savant6.wav",
    /*  99 */ "savant7.wav",
    /* 100 */ "savant8.wav",
    /* 101 */ "savant9.wav",
    /* 102 */ "savant10.wav",
    /* 103 */ "savant11.wav",
    /* 104 */ "savnt206.wav",
    /* 105 */ "savant13.wav",
    /* 106 */ "savant14.wav",
    /* 107 */ "savant15.wav",
    /* 108 */ "savnt204.wav",
    /* 109 */ "savant17.wav",
    /* 110 */ "savant18.wav",
    /* 111 */ "savant19.wav",
    /* 112 */ "savant20.wav",
    /* 113 */ "savant21.wav",
    /* 114 */ "savant22.wav",
    /* 115 */ "savant23.wav",
    /* 116 */ "savant24.wav",
    /* 117 */ "savant25.wav",
    /* 118 */ "savant26.wav",
    /* 119 */ "savant27.wav",
    /* 120 */ "savant28.wav",
    /* 121 */ "savant29.wav",
    /* 122 */ "savant30.wav",
    /* 123 */ "savant32.wav",
    /* 124 */ "savant33.wav",
    /* 125 */ "savant46.wav",
    /* 126 */ "savant47.wav",
    /* 127 */ "savant48.wav",
    /* 128 */ "savant49.wav",
    /* 129 */ "savnt221.wav",
    /* 130 */ "savant51.wav",
    /* 131 */ "savant53.wav",
    /* 132 */ "savant58.wav",
    /* 133 */ "savnt202.wav",
    /* 134 */ "savnt227.wav",
    /* 135 */ "savant65.wav",
    /* 136 */ "savant66.wav",
    /* 137 */ "savant67.wav",
    /* 138 */ "savant68.wav",
    /* 139 */ "savant69.wav",
    /* 140 */ "savant70.wav",
    /* 141 */ "savant71.wav",
    /* 142 */ "savant72.wav",
    /* 143 */ "savant73.wav",
    /* 144 */ "savant74.wav",
    /* 145 */ "savant75.wav",
    /* 146 */ "savant76.wav",
    /* 147 */ "savant77.wav",
    /* 148 */ "savant78.wav",
    /* 149 */ "savant79.wav",
    /* 150 */ "savant80.wav",
    /* 151 */ "savant93.wav",
    /* 152 */ "savnt278.wav",
    /* 153 */ "savnt207.wav",
    /* 154 */ "savant96.wav",
    /* 155 */ "savant97.wav",
    /* 156 */ "savant98.wav",
    /* 157 */ "savant99.wav",
    /* 158 */ "savnt100.wav",
    /* 159 */ "savnt101.wav",
    /* 160 */ "savnt201.wav",
    /* 161 */ "savnt203.wav",
    /* 162 */ "savnt205.wav",
    /* 163 */ "savnt208.wav",
    /* 164 */ "savnt209.wav",
    /* 165 */ "savnt210.wav",
    /* 166 */ "savnt211.wav",
    /* 167 */ "savnt213.wav",
    /* 168 */ "savnt214.wav",
    /* 169 */ "savnt215.wav",
    /* 170 */ "savnt216.wav",
    /* 171 */ "savnt218.wav",
    /* 172 */ "savnt219.wav",
    /* 173 */ "savnt220.wav",
    /* 174 */ "savnt223.wav",
    /* 175 */ "savnt224.wav",
    /* 176 */ "savnt225.wav",
    /* 177 */ "savnt226.wav",
    /* 178 */ "savnt228.wav",
    /* 179 */ "savnt229.wav",
    /* 180 */ "savnt230.wav",
    /* 181 */ "savnt231.wav",
    /* 182 */ "savnt232.wav",
    /* 183 */ "savnt233.wav",
    /* 184 */ "savnt234.wav",
    /* 185 */ "savnt235.wav",
    /* 186 */ "savnt236.wav",
    /* 187 */ "savnt237.wav",
    /* 188 */ "savnt238.wav",
    /* 189 */ "savnt239.wav",
    /* 190 */ "savnt240.wav",
    /* 191 */ "savnt241.wav",
    /* 192 */ "savnt242.wav",
    /* 193 */ "savnt243.wav",
    /* 194 */ "savnt244.wav",
    /* 195 */ "savnt245.wav",
    /* 196 */ "savnt246.wav",
    /* 197 */ "savnt247.wav",
    /* 198 */ "savnt248.wav",
    /* 199 */ "savnt249.wav",
    /* 200 */ "savnt250.wav",
    /* 201 */ "savnt251.wav",
    /* 202 */ "savnt252.wav",
    /* 203 */ "savnt253.wav",
    /* 204 */ "savnt254.wav",
    /* 205 */ "savnt255.wav",
    /* 206 */ "savnt256.wav",
    /* 207 */ "savnt257.wav",
    /* 208 */ "savnt258.wav",
    /* 209 */ "savnt259.wav",
    /* 210 */ "savnt260.wav",
    /* 211 */ "savnt261.wav",
    /* 212 */ "savnt262.wav",
    /* 213 */ "savnt263.wav",
    /* 214 */ "savnt264.wav",
    /* 215 */ "savnt265.wav",
    /* 216 */ "savnt266.wav",
    /* 217 */ "savnt267.wav",
    /* 218 */ "savnt268.wav",
    /* 219 */ "savnt269.wav",
    /* 220 */ "savnt270.wav",
    /* 221 */ "savnt271.wav",
    /* 222 */ "savnt272.wav",
    /* 223 */ "savnt273.wav",
    /* 224 */ "savnt274.wav",
    /* 225 */ "savnt275.wav",
    /* 226 */ "savnt276.wav",
    /* 227 */ "savnt277.wav",
    /* 228 */ "mis_fire.wav",
    /* 229 */ "acid2.wav",
    /* 230 */ "mis_fire.wav",
    /* 231 */ "geo_sel.wav",
    /* 232 */ "laser.wav",
    /* 233 */ "medexp1.wav",
    /* 234 */ "microwve.wav",
    /* 235 */ "medexp1.wav",
    /* 236 */ "railfir.wav",
    /* 237 */ "railtar.wav",
    /* 238 */ "bigexp1.wav",
    /* 239 */ "bigexp2.wav",
    /* 240 */ "mis_fire.wav",
    /* 241 */ "spam1.wav",
    /* 242 */ "mis_fire.wav",
    /* 243 */ "foam.wav",
    /* 244 */ "thor.wav",
    /* 245 */ "medexp1.wav",
    /* 246 */ "mis_fire.wav",
    /* 247 */ "medexp2.wav",
    /* 248 */ "laser.wav",
    /* 249 */ "smalxp1.wav",
    /* 250 */ "gun1.wav",
    /* 251 */ "medexp1.wav",
];

/// Canonical sound sample holding PCM float data and an optional cached WebAudio buffer.
#[derive(Clone)]
pub struct SoundSample {
    pub name: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub pcm_data: Vec<f32>,
    pub buffer: Option<AudioBuffer>,
}

/// Dynamic SoundBank holding loaded audio clips, channel pooling state,
/// and volume attenuation controls.
pub struct SoundBank {
    samples: RefCell<HashMap<String, SoundSample>>,
    ctx: RefCell<Option<AudioContext>>,
    output_node: RefCell<Option<GainNode>>,
    recent_play_times: RefCell<HashMap<String, f64>>,
    max_channels: usize,
    active_voices: RefCell<usize>,
}

impl SoundBank {
    /// Create a new SoundBank initialized with embedded representative waveforms.
    pub fn new() -> Self {
        let bank = Self {
            samples: RefCell::new(HashMap::new()),
            ctx: RefCell::new(None),
            output_node: RefCell::new(None),
            recent_play_times: RefCell::new(HashMap::new()),
            max_channels: 16,
            active_voices: RefCell::new(0),
        };

        // Preload core representative SFX and Voice lines embedded into the WASM binary:
        bank.preload_embedded_samples();
        bank
    }

    /// Preload the essential unit acknowledgements, tactical combat alerts,
    /// building completion chimes, and UI beeps.
    fn preload_embedded_samples(&self) {
        // Unit Acknowledgements & Movement confirmations
        self.register_embedded("truk_sel.wav", include_bytes!("../../assets/audio/truk_sel.wav"));
        self.register_embedded("dozr_sel.wav", include_bytes!("../../assets/audio/dozr_sel.wav"));
        self.register_embedded("consvhc.wav", include_bytes!("../../assets/audio/consvhc.wav"));
        self.register_embedded("combmed.wav", include_bytes!("../../assets/audio/combmed.wav"));
        self.register_embedded("comblit.wav", include_bytes!("../../assets/audio/comblit.wav"));
        self.register_embedded("combhev.wav", include_bytes!("../../assets/audio/combhev.wav"));
        self.register_embedded("scout.wav", include_bytes!("../../assets/audio/scout.wav"));
        self.register_embedded("repvhc.wav", include_bytes!("../../assets/audio/repvhc.wav"));
        self.register_embedded("robmine.wav", include_bytes!("../../assets/audio/robmine.wav"));
        self.register_embedded("ag_sel.wav", include_bytes!("../../assets/audio/ag_sel.wav"));

        // Tactical Combat Warnings
        self.register_embedded("savant4.wav", include_bytes!("../../assets/audio/savant4.wav")); // "We are under attack!"
        self.register_embedded("savant3.wav", include_bytes!("../../assets/audio/savant3.wav")); // "Structure destroyed"
        self.register_embedded("savnt206.wav", include_bytes!("../../assets/audio/savnt206.wav")); // "Vehicle destroyed"
        self.register_embedded("savant17.wav", include_bytes!("../../assets/audio/savant17.wav")); // "Warning: Missile launch detected"
        self.register_embedded("savant67.wav", include_bytes!("../../assets/audio/savant67.wav")); // "Lava alert / Volcanic eruption warning"
        self.register_embedded("savant68.wav", include_bytes!("../../assets/audio/savant68.wav")); // "Danger: Volcanic eruption detected"
        self.register_embedded("savant71.wav", include_bytes!("../../assets/audio/savant71.wav")); // "Vortex detected"
        self.register_embedded("savnt101.wav", include_bytes!("../../assets/audio/savnt101.wav")); // "Evacuate the colony: Volcanic eruption imminent"

        // Building & Research Completion Chimes
        self.register_embedded("bld_make.wav", include_bytes!("../../assets/audio/bld_make.wav")); // Structure kit deployed
        self.register_embedded("uni_grad.wav", include_bytes!("../../assets/audio/uni_grad.wav")); // Research fanfare chime
        self.register_embedded("savant2.wav", include_bytes!("../../assets/audio/savant2.wav")); // "Research completed"
        self.register_embedded("savnt210.wav", include_bytes!("../../assets/audio/savnt210.wav")); // "Bulldozing completed"

        // Core UI & Combat SFX
        self.register_embedded("beep2.wav", include_bytes!("../../assets/audio/beep2.wav"));
        self.register_embedded("beep5.wav", include_bytes!("../../assets/audio/beep5.wav"));
        self.register_embedded("beep8.wav", include_bytes!("../../assets/audio/beep8.wav"));
        self.register_embedded("beep9.wav", include_bytes!("../../assets/audio/beep9.wav"));
        self.register_embedded("bld_not.wav", include_bytes!("../../assets/audio/bld_not.wav"));
        self.register_embedded("message2.wav", include_bytes!("../../assets/audio/message2.wav"));
        self.register_embedded("laser.wav", include_bytes!("../../assets/audio/laser.wav"));
        self.register_embedded("bigexp1.wav", include_bytes!("../../assets/audio/bigexp1.wav"));
        self.register_embedded("quake.wav", include_bytes!("../../assets/audio/quake.wav"));
    }

    fn register_embedded(&self, name: &str, wav_bytes: &[u8]) {
        let _ = self.register_sample(name, wav_bytes);
    }

    /// Attach AudioContext and master destination GainNode
    pub fn init(&self, ctx: &AudioContext, output: &GainNode) {
        *self.ctx.borrow_mut() = Some(ctx.clone());
        *self.output_node.borrow_mut() = Some(output.clone());

        // Warm up and pre-decode cached buffers
        let mut samples = self.samples.borrow_mut();
        for sample in samples.values_mut() {
            if sample.buffer.is_none() {
                if let Ok(buf) = Self::create_audio_buffer(ctx, sample.sample_rate, sample.channels, &sample.pcm_data) {
                    sample.buffer = Some(buf);
                }
            }
        }
    }

    /// Register a WAV sample from raw bytes
    pub fn register_sample(&self, name: &str, wav_bytes: &[u8]) -> Result<(), String> {
        let (sample_rate, channels, pcm_data) = parse_wav(wav_bytes)?;
        let key = name.to_ascii_lowercase();

        let mut buffer = None;
        if let Some(ctx) = self.ctx.borrow().as_ref() {
            if let Ok(buf) = Self::create_audio_buffer(ctx, sample_rate, channels, &pcm_data) {
                buffer = Some(buf);
            }
        }

        self.samples.borrow_mut().insert(
            key,
            SoundSample {
                name: name.to_string(),
                sample_rate,
                channels,
                pcm_data,
                buffer,
            },
        );

        Ok(())
    }

    /// Register an uncompressed WAV member directly from an Outpost 2 .vol archive
    pub fn register_from_vol(&self, archive: &VolArchive, entry_name: &str) -> Result<(), String> {
        if let Some(bytes) = archive.get_file(entry_name) {
            self.register_sample(entry_name, bytes)
        } else {
            Err(format!("File '{}' not found in .vol archive", entry_name))
        }
    }

    /// Play a sound effect sample by name at default comfortable volume (0.85)
    pub fn play_sample(&self, name: &str) {
        self.play_sample_with_volume(name, 0.85);
    }

    /// Play a sound effect sample with custom volume attenuation (0.0 to 1.0)
    pub fn play_sample_with_volume(&self, name: &str, volume: f32) {
        let key = name.to_ascii_lowercase();
        let ctx_opt = self.ctx.borrow();
        let ctx = match ctx_opt.as_ref() {
            Some(c) => c,
            None => return,
        };

        let now = ctx.current_time();

        // Channel pooling & debouncing: prevent duplicate trigger within 40ms
        {
            let mut recent = self.recent_play_times.borrow_mut();
            if let Some(&last_time) = recent.get(&key) {
                if now - last_time < 0.040 {
                    return; // Dropped to avoid ear-splitting phase stacking
                }
            }
            recent.insert(key.clone(), now);
        }

        // Active voice limit
        let mut active = self.active_voices.borrow_mut();
        if *active >= self.max_channels {
            return;
        }

        let mut samples = self.samples.borrow_mut();
        let sample = match samples.get_mut(&key) {
            Some(s) => s,
            None => return,
        };

        // Lazy-instantiate AudioBuffer if not already created
        if sample.buffer.is_none() {
            match Self::create_audio_buffer(ctx, sample.sample_rate, sample.channels, &sample.pcm_data) {
                Ok(buf) => sample.buffer = Some(buf),
                Err(_) => return,
            }
        }

        let buffer = match sample.buffer.as_ref() {
            Some(b) => b,
            None => return,
        };

        if let Ok(source) = ctx.create_buffer_source() {
            source.set_buffer(Some(buffer));

            if let Ok(gain) = ctx.create_gain() {
                gain.gain().set_value(volume.clamp(0.0, 1.5));

                if let Some(master) = self.output_node.borrow().as_ref() {
                    let _ = source.connect_with_audio_node(&gain);
                    let _ = gain.connect_with_audio_node(master);
                    let _ = source.start();
                    *active += 1;
                }
            }
        }
    }

    /// Play a legacy Outpost 2 Voice Dispatch / Sound ID event.
    /// Maps 0..=251 to the authentic audio waveform or synthesizes speech dispatch.
    pub fn play_voice_event(&self, event_id: u32) {
        if let Some(filename) = Self::get_event_filename(event_id) {
            let key = filename.to_ascii_lowercase();
            if self.samples.borrow().contains_key(&key) {
                self.play_sample(filename);
                return;
            }
        }

        // Fallback: announce transcript via speech synthesis dispatcher if available
        if let Some(transcript) = Self::get_event_transcript(event_id) {
            crate::audio::radio::RadioDispatcher::announce(transcript);
        }
    }

    /// Check if a sample is loaded in memory
    pub fn is_sample_loaded(&self, name: &str) -> bool {
        self.samples.borrow().contains_key(&name.to_ascii_lowercase())
    }

    /// Return total loaded samples count
    pub fn sample_count(&self) -> usize {
        self.samples.borrow().len()
    }

    /// Map legacy Outpost 2 SoundID/VoiceID to canonical filename
    pub fn get_event_filename(event_id: u32) -> Option<&'static str> {
        let idx = event_id as usize;
        if idx < LEGACY_SOUND_TABLE.len() {
            let name = LEGACY_SOUND_TABLE[idx];
            if !name.is_empty() {
                return Some(name);
            }
        }
        None
    }

    /// Map legacy Outpost 2 Voice/Sound ID to canonical text transcript
    pub fn get_event_transcript(event_id: u32) -> Option<&'static str> {
        match event_id {
            2 => Some("Structure construction failed"),
            3 => Some("Structure construction completed"),
            4 => Some("AgriDome selected"),
            42 => Some("ConVec construction vehicle selected"),
            43 => Some("Earthworker bulldozer selected"),
            52 => Some("Scout vehicle selected"),
            53 => Some("Tiger heavy assault vehicle selected"),
            54 => Some("Lynx light combat vehicle selected"),
            55 => Some("Panther medium combat vehicle selected"),
            56 => Some("Cargo truck selected"),
            58 => Some("Scientific research objective completed"),
            72 | 73 | 74 => Some("Explosion detonation"),
            80 => Some("Earthquake tremor detected"),
            85 | 86 => Some("UI Confirmation beep"),
            88 => Some("Structure placement obstructed"),
            91 => Some("Radio dispatch tone"),
            93 => Some("Incoming priority transmission"),
            95 => Some("Research completed"),
            96 => Some("Structure destroyed"),
            97 => Some("We are under attack!"),
            106 => Some("Enemy units sighted"),
            109 => Some("Warning: Missile launch detected"),
            110 => Some("Warning: Incoming missile"),
            111 => Some("Morale is excellent"),
            112 => Some("Morale is good"),
            113 => Some("Morale is fair"),
            115 => Some("Morale is terrible"),
            116 => Some("Food production in surplus"),
            117 => Some("Food storage diminishing"),
            118 => Some("Warning: People are starving"),
            119 => Some("Power levels optimal"),
            120 => Some("Caution: Power levels marginal"),
            121 => Some("Warning: Power shortage"),
            122 => Some("New mission objective"),
            136 => Some("Caution: Volcano watch initiated"),
            137 => Some("Warning: Volcanic eruption expected"),
            138 => Some("Danger: Volcanic eruption detected"),
            140 => Some("Alert: Atmospheric vortex imminent"),
            141 => Some("Warning: Atmospheric vortex detected"),
            144 => Some("Warning: Electrical storm detected"),
            145 => Some("Caution: Meteor watch initiated"),
            146 => Some("Alert: Meteor impact imminent"),
            147 => Some("Warning: Meteor approaching"),
            149 => Some("Alert: Seismic event imminent"),
            150 => Some("Warning: Seismic event detected"),
            159 => Some("Commander, a volcanic eruption is imminent! Evacuate the colony and rendezvous north."),
            162 => Some("Reinforcements have arrived"),
            104 | 164 => Some("Vehicle destroyed"),
            165 => Some("Bulldozing completed"),
            166 => Some("Vehicle infected by Blight"),
            170 => Some("Vehicle captured"),
            174 => Some("Starship module manufactured"),
            177 => Some("Food stores are plentiful"),
            178 => Some("You have failed: Our colony is doomed"),
            217 => Some("Fueling systems deployed"),
            223 => Some("Phoenix module deployed"),
            224 => Some("Rare metal storage module deployed"),
            226 => Some("Food storage module deployed"),
            227 => Some("You have failed, our colony is doomed"),
            232 => Some("Laser discharge"),
            234 => Some("Microwave thermal emitter discharge"),
            236 => Some("Railgun kinetic discharge"),
            237 => Some("Railgun target impact"),
            243 => Some("Foam projector discharge"),
            244 => Some("Thor hammer detonation"),
            250 => Some("Kinetic weapon discharge"),
            _ => None,
        }
    }

    /// Construct a WebAudio AudioBuffer from raw float samples
    fn create_audio_buffer(
        ctx: &AudioContext,
        sample_rate: u32,
        channels: u16,
        pcm_floats: &[f32],
    ) -> Result<AudioBuffer, JsValue> {
        let frame_count = (pcm_floats.len() / channels as usize) as u32;
        let buffer = ctx.create_buffer(channels as u32, frame_count, sample_rate as f32)?;

        if channels == 1 {
            buffer.copy_to_channel(pcm_floats, 0)?;
        } else {
            // De-interleave channels
            let ch_len = frame_count as usize;
            for ch in 0..channels as usize {
                let mut ch_data = Vec::with_capacity(ch_len);
                for i in 0..ch_len {
                    ch_data.push(pcm_floats[i * channels as usize + ch]);
                }
                buffer.copy_to_channel(&ch_data, ch as i32)?;
            }
        }

        Ok(buffer)
    }
}

/// Robust RIFF/WAVE parser for Outpost 2 8-bit unsigned and 16-bit signed PCM audio.
pub fn parse_wav(data: &[u8]) -> Result<(u32, u16, Vec<f32>), String> {
    if data.len() < 44 {
        return Err("WAV data too small for RIFF header".into());
    }
    if &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return Err("Invalid RIFF/WAVE container signature".into());
    }

    let mut pos = 12;
    let mut sample_rate = 22050u32;
    let mut channels = 1u16;
    let mut bits_per_sample = 8u16;
    let mut pcm_floats = Vec::new();

    while pos + 8 <= data.len() {
        let chunk_id = &data[pos..pos + 4];
        let chunk_size = u32::from_le_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]) as usize;
        let data_start = pos + 8;
        let data_end = (data_start + chunk_size).min(data.len());

        if chunk_id == b"fmt " && data_end - data_start >= 16 {
            let fmt = &data[data_start..data_end];
            channels = u16::from_le_bytes([fmt[2], fmt[3]]);
            sample_rate = u32::from_le_bytes([fmt[4], fmt[5], fmt[6], fmt[7]]);
            bits_per_sample = u16::from_le_bytes([fmt[14], fmt[15]]);
        } else if chunk_id == b"data" {
            let raw_data = &data[data_start..data_end];
            if bits_per_sample == 8 {
                // Outpost 2 standard: 8-bit unsigned PCM (128 is silence)
                pcm_floats = raw_data
                    .iter()
                    .map(|&b| (b as f32 - 128.0) / 128.0)
                    .collect();
            } else if bits_per_sample == 16 {
                // 16-bit signed PCM little-endian
                pcm_floats = raw_data
                    .chunks_exact(2)
                    .map(|chunk| {
                        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                        sample as f32 / 32768.0
                    })
                    .collect();
            }
        }

        pos += 8 + ((chunk_size + 1) & !1); // Chunks are 2-byte aligned
    }

    if pcm_floats.is_empty() {
        return Err("No PCM audio samples found in WAV".into());
    }

    Ok((sample_rate, channels, pcm_floats))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_sound_table_integrity() {
        assert_eq!(LEGACY_SOUND_TABLE.len(), 252);
        assert_eq!(SoundBank::get_event_filename(3), Some("bld_make.wav"));
        assert_eq!(SoundBank::get_event_filename(56), Some("truk_sel.wav"));
        assert_eq!(SoundBank::get_event_filename(58), Some("uni_grad.wav"));
        assert_eq!(SoundBank::get_event_filename(95), Some("savant2.wav"));
        assert_eq!(SoundBank::get_event_filename(96), Some("savant3.wav"));
        assert_eq!(SoundBank::get_event_filename(97), Some("savant4.wav"));
        assert_eq!(SoundBank::get_event_filename(104), Some("savnt206.wav"));
        assert_eq!(SoundBank::get_event_filename(137), Some("savant67.wav"));
        assert_eq!(SoundBank::get_event_filename(232), Some("laser.wav"));
    }

    #[test]
    fn test_legacy_event_transcripts() {
        assert_eq!(SoundBank::get_event_transcript(95), Some("Research completed"));
        assert_eq!(SoundBank::get_event_transcript(96), Some("Structure destroyed"));
        assert_eq!(SoundBank::get_event_transcript(97), Some("We are under attack!"));
        assert_eq!(SoundBank::get_event_transcript(137), Some("Warning: Volcanic eruption expected"));
        assert_eq!(SoundBank::get_event_transcript(164), Some("Vehicle destroyed"));
    }

    #[test]
    fn test_parse_wav_8bit_mono() {
        // Construct a synthetic 8-bit mono WAV buffer (22050 Hz, 4 samples)
        let mut wav = Vec::new();
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(36u32 + 4u32).to_le_bytes());
        wav.extend_from_slice(b"WAVE");
        // fmt chunk
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
        wav.extend_from_slice(&1u16.to_le_bytes());  // PCM
        wav.extend_from_slice(&1u16.to_le_bytes());  // Mono
        wav.extend_from_slice(&22050u32.to_le_bytes()); // Sample rate
        wav.extend_from_slice(&22050u32.to_le_bytes()); // Byte rate
        wav.extend_from_slice(&1u16.to_le_bytes());  // Block align
        wav.extend_from_slice(&8u16.to_le_bytes());  // Bits per sample
        // data chunk
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&4u32.to_le_bytes());
        wav.extend_from_slice(&[128, 255, 0, 128]); // Silence, Max+, Max-, Silence

        let (rate, channels, pcm) = parse_wav(&wav).expect("Valid WAV parsing");
        assert_eq!(rate, 22050);
        assert_eq!(channels, 1);
        assert_eq!(pcm.len(), 4);
        assert!((pcm[0] - 0.0).abs() < 0.01);
        assert!((pcm[1] - 0.992).abs() < 0.01);
        assert!((pcm[2] - (-1.0)).abs() < 0.01);
        assert!((pcm[3] - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_sound_bank_embedded_preloads() {
        let bank = SoundBank::new();
        assert!(bank.sample_count() >= 23);
        assert!(bank.is_sample_loaded("beep2.wav"));
        assert!(bank.is_sample_loaded("truk_sel.wav"));
        assert!(bank.is_sample_loaded("dozr_sel.wav"));
        assert!(bank.is_sample_loaded("combmed.wav"));
        assert!(bank.is_sample_loaded("comblit.wav"));
        assert!(bank.is_sample_loaded("combhev.wav"));
        assert!(bank.is_sample_loaded("scout.wav"));
        assert!(bank.is_sample_loaded("repvhc.wav"));
        assert!(bank.is_sample_loaded("robmine.wav"));
        assert!(bank.is_sample_loaded("bld_make.wav"));
        assert!(bank.is_sample_loaded("uni_grad.wav"));
        assert!(bank.is_sample_loaded("savant2.wav"));
        assert!(bank.is_sample_loaded("savant3.wav"));
        assert!(bank.is_sample_loaded("savant4.wav"));
        assert!(bank.is_sample_loaded("savant67.wav"));
        assert!(bank.is_sample_loaded("savant68.wav"));
        assert!(bank.is_sample_loaded("savant71.wav"));
        assert!(bank.is_sample_loaded("savnt206.wav"));
    }

    #[test]
    fn test_custom_sample_registration() {
        let bank = SoundBank::new();
        let initial_count = bank.sample_count();

        let mut wav = Vec::new();
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&40u32.to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&22050u32.to_le_bytes());
        wav.extend_from_slice(&22050u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&8u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&2u32.to_le_bytes());
        wav.extend_from_slice(&[128, 128]);

        bank.register_sample("custom_test.wav", &wav).expect("Registration succeeded");
        assert_eq!(bank.sample_count(), initial_count + 1);
        assert!(bank.is_sample_loaded("custom_test.wav"));
    }
}
