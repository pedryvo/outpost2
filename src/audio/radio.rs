//! Tactical Computer Speech & Radio Dispatch Engine
//!
//! Recreates Outpost 2's signature computerized tactical audio dispatch system:
//! Announces disaster warnings, base incursions, research completions, and kit deployments.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
export function speakTacticalAlert(text) {
    try {
        if (typeof window !== 'undefined' && 'speechSynthesis' in window) {
            window.speechSynthesis.cancel();
            const utter = new SpeechSynthesisUtterance(text);
            utter.rate = 1.05;
            utter.pitch = 0.90; // Authoritative sci-fi military tone
            utter.volume = 0.85;
            window.speechSynthesis.speak(utter);
        }
    } catch (e) {
        // Graceful fallback if audio permissions are restricted
    }
}
"#)]
extern "C" {
    pub fn speakTacticalAlert(text: &str);
}

pub struct RadioDispatcher;

impl RadioDispatcher {
    pub fn announce(msg: &str) {
        speakTacticalAlert(msg);
    }
}
