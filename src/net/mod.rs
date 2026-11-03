//! Outpost 2 Peer-to-Peer WebRTC Multiplayer Subsystem
//!
//! Provides deterministic networking, lockstep turn packet protocol,
//! and serverless zero-signaling WebRTC peer-to-peer communication.

pub mod lobby;
pub mod protocol;
pub mod session;

pub use lobby::{GameMode, LobbyHandshake, LobbyRoom, LobbyState};
pub use protocol::{NetMessage, NetOrder, NetPacket, SignalingMessage};
pub use session::WebRtcSession;
pub use crate::sim::Faction;
