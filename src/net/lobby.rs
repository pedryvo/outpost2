//! Outpost 2 Multiplayer & Skirmish Match Lobby Subsystem
//!
//! Provides the match lobby state machine, room configuration negotiation,
//! peer handshake serialization, and synchronization across WebRTC DataChannels.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::sim::Faction;

/// Game modes supported in Outpost 2 matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameMode {
    /// Authentic narrative campaign missions with story events and objectives.
    Campaign,
    /// Free-form planetary skirmish vs. hostile AI commanders or rival colonies.
    Skirmish,
    /// Head-to-head peer-to-peer tactical multiplayer over WebRTC.
    P2P,
}

impl GameMode {
    /// Display name of the game mode.
    pub fn name(&self) -> &'static str {
        match self {
            GameMode::Campaign => "Campaign Scenario",
            GameMode::Skirmish => "Planetary Skirmish",
            GameMode::P2P => "Peer-to-Peer Tactical Multiplayer",
        }
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Lifecycle states of the multiplayer / skirmish match lobby.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LobbyState {
    /// Lobby is idle; no active host room or connection attempts in progress.
    Idle,
    /// Host has generated the WebRTC Offer SDP; awaiting distribution to guest.
    HostingOfferGenerated,
    /// Guest has ingested the Host Offer and generated the Answer SDP; awaiting transmission back.
    GuestAnswerGenerated,
    /// WebRTC DataChannel connection established; players in room synchronizing setup.
    Connected,
    /// Both commanders are ready; match countdown started and simulation launching.
    GameStarting,
}

impl LobbyState {
    /// Human-readable description of current state.
    pub fn description(&self) -> &'static str {
        match self {
            LobbyState::Idle => "Idle / Disconnected",
            LobbyState::HostingOfferGenerated => "Host Room Open - Offer SDP Generated",
            LobbyState::GuestAnswerGenerated => "Joining Outpost - Answer SDP Generated",
            LobbyState::Connected => "Commanders Linked - WebRTC DataChannel Active",
            LobbyState::GameStarting => "Launching Operational Theater - Synchronizing Simulation",
        }
    }

    /// Validate whether a state transition from `self` to `next` is permissible.
    pub fn can_transition_to(&self, next: LobbyState) -> bool {
        match (*self, next) {
            // Any state can return to Idle on disconnect or cancellation
            (_, LobbyState::Idle) => true,
            // Idle -> Host Offer
            (LobbyState::Idle, LobbyState::HostingOfferGenerated) => true,
            // Idle -> Guest Answer
            (LobbyState::Idle, LobbyState::GuestAnswerGenerated) => true,
            // Direct connection from Idle or active signaling states
            (LobbyState::Idle, LobbyState::Connected) => true,
            (LobbyState::HostingOfferGenerated, LobbyState::Connected) => true,
            (LobbyState::GuestAnswerGenerated, LobbyState::Connected) => true,
            // Connected -> GameStarting when both commanders ready
            (LobbyState::Connected, LobbyState::GameStarting) => true,
            _ => false,
        }
    }
}

impl fmt::Display for LobbyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Tactical map theaters supported in Outpost 2 multiplayer matchmaking.
pub const MULTIPLAYER_MAPS: &[&str] = &[
    "Eden 1: The New Frontier",
    "Eden 2: Discovery at Mount Plymouth",
    "Eden 3: The Gathering Storm",
    "Plymouth 1: Seeds of Dissent",
    "Plymouth 2: Red Planet Crucible",
    "Plymouth 3: Final Exodus",
    "Procedural Skirmish",
];

/// Interactive match lobby room configuration and participant state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LobbyRoom {
    /// Unique identifier for this match room session.
    pub room_id: String,
    /// Display name or callsign of the hosting colony commander.
    pub host_name: String,
    /// Map or terrain theater designated for the engagement.
    pub map_name: String,
    /// Faction chosen by the host commander (Eden or Plymouth).
    pub faction: Faction,
    /// Designated operational game mode.
    pub game_mode: GameMode,
    /// Environmental disaster intensity level (0 = None, 1 = Low, 2 = Standard, 3 = Extreme).
    pub disaster_intensity: u32,
    /// Starting metal stockpiles (Common and Rare ore).
    pub starting_resources: u32,

    /// Display name of the guest commander (if joined).
    #[serde(default)]
    pub guest_name: Option<String>,
    /// Faction chosen by the guest commander.
    #[serde(default)]
    pub guest_faction: Option<Faction>,
    /// Current lobby lifecycle state.
    #[serde(default = "default_lobby_state")]
    pub state: LobbyState,
    /// Ready status of host commander.
    #[serde(default)]
    pub is_host_ready: bool,
    /// Ready status of guest commander.
    #[serde(default)]
    pub is_guest_ready: bool,
    /// Shared deterministic random seed for simulation parity.
    #[serde(default)]
    pub match_seed: u64,

    /// Base64 SDP Offer string generated by the host.
    #[serde(default)]
    pub host_offer_sdp: Option<String>,
    /// Base64 SDP Answer string pasted by the host (received from guest).
    #[serde(default)]
    pub host_answer_sdp: Option<String>,
    /// Base64 SDP Offer string pasted by the guest (received from host).
    #[serde(default)]
    pub guest_offer_sdp: Option<String>,
    /// Base64 SDP Answer string generated by the guest.
    #[serde(default)]
    pub guest_answer_sdp: Option<String>,
    /// Roundtrip network latency to peer in milliseconds.
    #[serde(default)]
    pub latency_ms: Option<u32>,
    /// In-memory clipboard status notification (e.g. "Offer copied to clipboard!").
    #[serde(default)]
    pub clipboard_status: Option<String>,
}

fn default_lobby_state() -> LobbyState {
    LobbyState::Idle
}

impl Default for LobbyRoom {
    fn default() -> Self {
        Self::new(
            "NEW-TERRA-42",
            "Host Commander",
            MULTIPLAYER_MAPS[0],
            Faction::Eden,
            GameMode::P2P,
            2,
            5000,
        )
    }
}

impl LobbyRoom {
    /// Create a new host lobby room with default parameters.
    pub fn new(
        room_id: impl Into<String>,
        host_name: impl Into<String>,
        map_name: impl Into<String>,
        faction: Faction,
        game_mode: GameMode,
        disaster_intensity: u32,
        starting_resources: u32,
    ) -> Self {
        Self {
            room_id: room_id.into(),
            host_name: host_name.into(),
            map_name: map_name.into(),
            faction,
            game_mode,
            disaster_intensity,
            starting_resources,
            guest_name: None,
            guest_faction: None,
            state: LobbyState::Idle,
            is_host_ready: false,
            is_guest_ready: false,
            match_seed: 0xDEADBEEF,
            host_offer_sdp: None,
            host_answer_sdp: None,
            guest_offer_sdp: None,
            guest_answer_sdp: None,
            latency_ms: None,
            clipboard_status: None,
        }
    }

    /// Set host generated WebRTC SDP offer and transition to hosting state.
    pub fn set_host_offer(&mut self, sdp: impl Into<String>) {
        self.host_offer_sdp = Some(sdp.into());
        let _ = self.transition_to(LobbyState::HostingOfferGenerated);
    }

    /// Single-click copy of generated host offer SDP to clipboard.
    pub fn copy_host_offer(&mut self) -> Result<String, String> {
        if let Some(ref offer) = self.host_offer_sdp {
            self.clipboard_status = Some("Offer copied to clipboard!".into());
            Ok(offer.clone())
        } else {
            Err("No host offer SDP available to copy".into())
        }
    }

    /// Host ingests guest SDP answer, completing the handshake and establishing connection.
    pub fn paste_guest_answer(&mut self, answer: impl Into<String>) -> Result<(), String> {
        let ans = answer.into();
        if ans.trim().is_empty() {
            return Err("Guest answer payload cannot be empty".into());
        }
        self.host_answer_sdp = Some(ans);
        let _ = self.transition_to(LobbyState::Connected);
        Ok(())
    }

    /// Guest ingests host SDP offer.
    pub fn paste_host_offer(&mut self, offer: impl Into<String>) -> Result<(), String> {
        let off = offer.into();
        if off.trim().is_empty() {
            return Err("Host offer payload cannot be empty".into());
        }
        self.guest_offer_sdp = Some(off);
        Ok(())
    }

    /// Guest generates WebRTC SDP answer and transitions to ready-to-reply state.
    pub fn generate_guest_answer(&mut self, answer: impl Into<String>) -> Result<(), String> {
        let ans = answer.into();
        if ans.trim().is_empty() {
            return Err("Generated answer cannot be empty".into());
        }
        self.guest_answer_sdp = Some(ans);
        let _ = self.transition_to(LobbyState::GuestAnswerGenerated);
        Ok(())
    }

    /// Single-click copy of generated guest answer SDP to clipboard.
    pub fn copy_guest_answer(&mut self) -> Result<String, String> {
        if let Some(ref ans) = self.guest_answer_sdp {
            self.clipboard_status = Some("Answer copied to clipboard!".into());
            Ok(ans.clone())
        } else {
            Err("No guest answer SDP available to copy".into())
        }
    }

    /// Toggle or set commander readiness state.
    pub fn toggle_ready(&mut self, is_host: bool) {
        if is_host {
            self.is_host_ready = !self.is_host_ready;
        } else {
            self.is_guest_ready = !self.is_guest_ready;
        }
    }

    /// Record live roundtrip ping latency in milliseconds.
    pub fn set_latency(&mut self, latency_ms: u32) {
        self.latency_ms = Some(latency_ms);
    }

    /// Connection status banner text:
    /// e.g. "Awaiting Guest Connection..." -> "Peer Connected (Latency: 24ms)"
    pub fn connection_status_banner(&self) -> String {
        if self.state == LobbyState::Connected {
            if let Some(lat) = self.latency_ms {
                format!("Peer Connected (Latency: {}ms)", lat)
            } else {
                "Peer Connected".to_string()
            }
        } else if self.state == LobbyState::HostingOfferGenerated {
            "Awaiting Guest Connection...".to_string()
        } else if self.state == LobbyState::GuestAnswerGenerated {
            "Answer Generated - Transmitting to Host...".to_string()
        } else {
            "Awaiting Guest Connection...".to_string()
        }
    }

    /// Cycle through multiplayer tactical maps (Eden 1-3, Plymouth 1-3, Procedural Skirmish).
    pub fn cycle_map(&mut self, forward: bool) {
        let current_pos = MULTIPLAYER_MAPS
            .iter()
            .position(|&m| m == self.map_name)
            .unwrap_or(0);
        let next_pos = if forward {
            (current_pos + 1) % MULTIPLAYER_MAPS.len()
        } else if current_pos == 0 {
            MULTIPLAYER_MAPS.len() - 1
        } else {
            current_pos - 1
        };
        self.map_name = MULTIPLAYER_MAPS[next_pos].to_string();
    }

    /// Select map by exact or friendly name.
    pub fn select_map(&mut self, map_name: impl Into<String>) {
        self.map_name = map_name.into();
    }

    /// Advance the lobby state machine, enforcing valid transition pathways.
    pub fn transition_to(&mut self, next: LobbyState) -> Result<(), String> {
        if self.state.can_transition_to(next) {
            self.state = next;
            Ok(())
        } else {
            Err(format!(
                "Invalid lobby transition from {:?} to {:?}",
                self.state, next
            ))
        }
    }

    /// Record a guest commander joining the match room.
    pub fn register_guest(
        &mut self,
        guest_name: impl Into<String>,
        guest_faction: Faction,
    ) -> Result<(), String> {
        self.guest_name = Some(guest_name.into());
        self.guest_faction = Some(guest_faction);
        self.is_guest_ready = false;
        Ok(())
    }

    /// Toggle or set commander readiness state.
    pub fn set_ready(&mut self, is_host: bool, ready: bool) {
        if is_host {
            self.is_host_ready = ready;
        } else {
            self.is_guest_ready = ready;
        }
    }

    /// Verify whether both commanders are ready to initiate the game simulation.
    pub fn is_ready_to_launch(&self) -> bool {
        self.state == LobbyState::Connected
            && self.is_host_ready
            && self.is_guest_ready
            && self.guest_name.is_some()
    }

    /// Initiate match launch countdown, generating random seed if needed.
    pub fn launch_match(&mut self, seed: u64) -> Result<(), String> {
        if !self.is_ready_to_launch() {
            return Err("Cannot launch match: both commanders must be linked and ready".into());
        }
        self.match_seed = seed;
        self.transition_to(LobbyState::GameStarting)
    }

    /// Apply an incoming handshake packet to update the room configuration.
    pub fn handle_handshake(
        &mut self,
        handshake: &LobbyHandshake,
    ) -> Result<Option<LobbyHandshake>, String> {
        match handshake {
            LobbyHandshake::RoomConfig { room } => {
                // Guest adopts room configuration from Host
                self.room_id = room.room_id.clone();
                self.host_name = room.host_name.clone();
                self.map_name = room.map_name.clone();
                self.faction = room.faction;
                self.game_mode = room.game_mode;
                self.disaster_intensity = room.disaster_intensity;
                self.starting_resources = room.starting_resources;
                self.match_seed = room.match_seed;
                self.is_host_ready = room.is_host_ready;
                let _ = self.transition_to(LobbyState::Connected);
                Ok(None)
            }
            LobbyHandshake::GuestJoin {
                guest_name,
                guest_faction,
            } => {
                // Host receives guest identification
                self.register_guest(guest_name, *guest_faction)?;
                let _ = self.transition_to(LobbyState::Connected);
                // Host replies with current room state confirmation
                Ok(Some(LobbyHandshake::RoomConfig { room: self.clone() }))
            }
            LobbyHandshake::PlayerReady { is_host, ready } => {
                self.set_ready(*is_host, *ready);
                Ok(None)
            }
            LobbyHandshake::StartMatch {
                seed,
                map_name,
                tick_rate: _,
            } => {
                self.map_name = map_name.clone();
                self.match_seed = *seed;
                let _ = self.transition_to(LobbyState::GameStarting);
                Ok(None)
            }
            LobbyHandshake::LobbyChat { .. } => Ok(None),
        }
    }

    /// Serialize room configuration to JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize room configuration from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Network handshake packet exchanged across the WebRTC DataChannel during lobby setup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LobbyHandshake {
    /// Host transmits complete room setup configuration to newly connected guest.
    RoomConfig { room: LobbyRoom },
    /// Guest transmits player designation and faction selection to the host.
    GuestJoin {
        guest_name: String,
        guest_faction: Faction,
    },
    /// Either commander signals readiness status change.
    PlayerReady { is_host: bool, ready: bool },
    /// Host signals start of match with synchronized seed and map choice.
    StartMatch {
        seed: u64,
        map_name: String,
        tick_rate: u32,
    },
    /// In-lobby tactical communication exchange.
    LobbyChat {
        sender: String,
        text: String,
        timestamp_secs: f64,
    },
}

impl LobbyHandshake {
    /// Serialize handshake message to compact JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Parse handshake message from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize handshake message to binary UTF-8 byte slice.
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Parse handshake message from binary UTF-8 byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    /// Transmit this handshake packet across an active `WebRtcSession`.
    pub fn send_over_session(
        &self,
        session: &crate::net::session::WebRtcSession,
    ) -> Result<(), wasm_bindgen::JsValue> {
        let json = self
            .to_json()
            .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("Handshake serialization failed: {}", e)))?;
        session.send_raw(&json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lobby_room_instantiation_and_fields() {
        let room = LobbyRoom::new(
            "NEW-TERRA-42",
            "Commander Vance",
            "eden01.map",
            Faction::Eden,
            GameMode::Skirmish,
            2,
            5000,
        );

        assert_eq!(room.room_id, "NEW-TERRA-42");
        assert_eq!(room.host_name, "Commander Vance");
        assert_eq!(room.map_name, "eden01.map");
        assert_eq!(room.faction, Faction::Eden);
        assert_eq!(room.game_mode, GameMode::Skirmish);
        assert_eq!(room.disaster_intensity, 2);
        assert_eq!(room.starting_resources, 5000);
        assert_eq!(room.state, LobbyState::Idle);
        assert_eq!(room.guest_name, None);
        assert_eq!(room.guest_faction, None);
        assert!(!room.is_host_ready);
        assert!(!room.is_guest_ready);
    }

    #[test]
    fn test_lobby_state_transitions() {
        let mut room = LobbyRoom::new(
            "ROOM-1",
            "Host",
            "eden01.map",
            Faction::Eden,
            GameMode::P2P,
            1,
            3000,
        );

        // Valid: Idle -> HostingOfferGenerated
        assert!(room.transition_to(LobbyState::HostingOfferGenerated).is_ok());
        assert_eq!(room.state, LobbyState::HostingOfferGenerated);

        // Valid: HostingOfferGenerated -> Connected
        assert!(room.transition_to(LobbyState::Connected).is_ok());
        assert_eq!(room.state, LobbyState::Connected);

        // Invalid: Connected -> HostingOfferGenerated
        assert!(room.transition_to(LobbyState::HostingOfferGenerated).is_err());

        // Valid: Connected -> Idle (reset)
        assert!(room.transition_to(LobbyState::Idle).is_ok());

        // Valid: Idle -> GuestAnswerGenerated
        assert!(room.transition_to(LobbyState::GuestAnswerGenerated).is_ok());
        assert_eq!(room.state, LobbyState::GuestAnswerGenerated);

        // Valid: GuestAnswerGenerated -> Connected
        assert!(room.transition_to(LobbyState::Connected).is_ok());
    }

    #[test]
    fn test_lobby_handshake_serde_roundtrip() {
        let room = LobbyRoom::new(
            "ROOM-99",
            "Eden Prime",
            "ply01.map",
            Faction::Eden,
            GameMode::Campaign,
            3,
            6000,
        );

        let handshake = LobbyHandshake::RoomConfig { room: room.clone() };
        let json = handshake.to_json().expect("Serialize LobbyHandshake");
        let decoded: LobbyHandshake = LobbyHandshake::from_json(&json).expect("Deserialize LobbyHandshake");

        assert_eq!(handshake, decoded);

        // Binary roundtrip
        let bytes = handshake.to_bytes().expect("To bytes");
        let decoded_bytes = LobbyHandshake::from_bytes(&bytes).expect("From bytes");
        assert_eq!(handshake, decoded_bytes);
    }

    #[test]
    fn test_guest_join_and_readiness_launch_flow() {
        let mut host_room = LobbyRoom::new(
            "MATCH-77",
            "Host Vance",
            "eden02.map",
            Faction::Eden,
            GameMode::P2P,
            2,
            4000,
        );

        let mut guest_room = LobbyRoom::new(
            "TMP",
            "Unknown",
            "tmp",
            Faction::Plymouth,
            GameMode::P2P,
            0,
            0,
        );

        // 1. Host transitions to HostingOfferGenerated
        host_room.transition_to(LobbyState::HostingOfferGenerated).unwrap();

        // 2. Peer channel links -> Guest sends GuestJoin handshake
        let join_msg = LobbyHandshake::GuestJoin {
            guest_name: "Commander Sarah".into(),
            guest_faction: Faction::Plymouth,
        };

        // Host receives GuestJoin
        let reply = host_room.handle_handshake(&join_msg).unwrap();
        assert_eq!(host_room.state, LobbyState::Connected);
        assert_eq!(host_room.guest_name.as_deref(), Some("Commander Sarah"));
        assert_eq!(host_room.guest_faction, Some(Faction::Plymouth));

        // Guest receives reply RoomConfig
        if let Some(LobbyHandshake::RoomConfig { room }) = reply {
            guest_room.handle_handshake(&LobbyHandshake::RoomConfig { room }).unwrap();
        }

        assert_eq!(guest_room.state, LobbyState::Connected);
        assert_eq!(guest_room.room_id, "MATCH-77");
        assert_eq!(guest_room.host_name, "Host Vance");

        // Cannot launch yet: not both ready
        assert!(!host_room.is_ready_to_launch());

        // Host marks ready
        host_room.set_ready(true, true);
        assert!(!host_room.is_ready_to_launch());

        // Guest marks ready
        let guest_ready_msg = LobbyHandshake::PlayerReady { is_host: false, ready: true };
        host_room.handle_handshake(&guest_ready_msg).unwrap();
        assert!(host_room.is_ready_to_launch());

        // Host launches match
        host_room.launch_match(0x1337BEEF).unwrap();
        assert_eq!(host_room.state, LobbyState::GameStarting);
        assert_eq!(host_room.match_seed, 0x1337BEEF);
    }

    #[test]
    fn test_game_mode_display_and_names() {
        assert_eq!(GameMode::Campaign.name(), "Campaign Scenario");
        assert_eq!(GameMode::Skirmish.name(), "Planetary Skirmish");
        assert_eq!(GameMode::P2P.name(), "Peer-to-Peer Tactical Multiplayer");

        assert_eq!(format!("{}", GameMode::P2P), "Peer-to-Peer Tactical Multiplayer");
    }

    #[test]
    fn test_multiplayer_lobby_offer_answer_copy_workflow() {
        let mut host_room = LobbyRoom::new(
            "OP2-MATCH-88",
            "Host Commander",
            MULTIPLAYER_MAPS[0],
            Faction::Eden,
            GameMode::P2P,
            2,
            5000,
        );

        let mut guest_room = LobbyRoom::new(
            "OP2-JOIN-88",
            "Guest Commander",
            MULTIPLAYER_MAPS[0],
            Faction::Plymouth,
            GameMode::P2P,
            2,
            5000,
        );

        // Initial banner
        assert_eq!(host_room.connection_status_banner(), "Awaiting Guest Connection...");

        // 1. Host creates Base64 offer SDP
        let sample_offer = "data:application/sdp;base64,eyJ0eXBlIjoib2ZmZXIiLCJzZHAiOiJ2PTB..." ;
        host_room.set_host_offer(sample_offer);
        assert_eq!(host_room.state, LobbyState::HostingOfferGenerated);
        assert_eq!(host_room.connection_status_banner(), "Awaiting Guest Connection...");

        // Host single-click copies offer
        let copied_offer = host_room.copy_host_offer().expect("Copy offer");
        assert_eq!(copied_offer, sample_offer);
        assert_eq!(host_room.clipboard_status.as_deref(), Some("Offer copied to clipboard!"));

        // 2. Guest pastes Host Offer
        guest_room.paste_host_offer(&copied_offer).expect("Paste host offer");
        assert_eq!(guest_room.guest_offer_sdp.as_deref(), Some(sample_offer));

        // 3. Guest generates Answer SDP
        let sample_answer = "data:application/sdp;base64,eyJ0eXBlIjoiYW5zd2VyIiwic2RwIjoidj0wLi4uIn0=";
        guest_room.generate_guest_answer(sample_answer).expect("Generate guest answer");
        assert_eq!(guest_room.state, LobbyState::GuestAnswerGenerated);
        assert_eq!(guest_room.connection_status_banner(), "Answer Generated - Transmitting to Host...");

        // Guest single-click copies answer
        let copied_answer = guest_room.copy_guest_answer().expect("Copy answer");
        assert_eq!(copied_answer, sample_answer);
        assert_eq!(guest_room.clipboard_status.as_deref(), Some("Answer copied to clipboard!"));

        // 4. Host pastes Guest Answer
        host_room.paste_guest_answer(&copied_answer).expect("Paste guest answer");
        assert_eq!(host_room.state, LobbyState::Connected);

        // Status banner updates on connection & latency
        assert_eq!(host_room.connection_status_banner(), "Peer Connected");
        host_room.set_latency(24);
        assert_eq!(host_room.connection_status_banner(), "Peer Connected (Latency: 24ms)");

        // 5. Readiness and Match Launch
        host_room.register_guest("Guest Sarah", Faction::Plymouth).unwrap();
        assert!(!host_room.is_ready_to_launch());

        // Host toggles ready
        host_room.toggle_ready(true);
        assert!(host_room.is_host_ready);
        assert!(!host_room.is_ready_to_launch());

        // Guest toggles ready
        host_room.toggle_ready(false);
        assert!(host_room.is_guest_ready);
        assert!(host_room.is_ready_to_launch());

        // Launch match
        assert!(host_room.launch_match(0xCAFEBABE).is_ok());
        assert_eq!(host_room.state, LobbyState::GameStarting);
        assert_eq!(host_room.match_seed, 0xCAFEBABE);
    }

    #[test]
    fn test_map_selector_and_cycle() {
        let mut room = LobbyRoom::default();
        assert_eq!(room.map_name, "Eden 1: The New Frontier");

        // Cycle forward through all maps
        room.cycle_map(true);
        assert_eq!(room.map_name, "Eden 2: Discovery at Mount Plymouth");

        room.cycle_map(true);
        assert_eq!(room.map_name, "Eden 3: The Gathering Storm");

        room.cycle_map(true);
        assert_eq!(room.map_name, "Plymouth 1: Seeds of Dissent");

        room.cycle_map(true);
        assert_eq!(room.map_name, "Plymouth 2: Red Planet Crucible");

        room.cycle_map(true);
        assert_eq!(room.map_name, "Plymouth 3: Final Exodus");

        room.cycle_map(true);
        assert_eq!(room.map_name, "Procedural Skirmish");

        // Wraps around to Eden 1
        room.cycle_map(true);
        assert_eq!(room.map_name, "Eden 1: The New Frontier");

        // Cycle backward
        room.cycle_map(false);
        assert_eq!(room.map_name, "Procedural Skirmish");

        // Select directly
        room.select_map("Procedural Skirmish");
        assert_eq!(room.map_name, "Procedural Skirmish");
    }
}
