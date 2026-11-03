//! Outpost 2 Multiplayer Protocol
//!
//! Deterministic packet protocol and data structures for peer-to-peer multiplayer
//! synchronization over WebRTC RTCDataChannel.

use serde::{Deserialize, Serialize};
use crate::sim::Faction;

/// Order types that can be issued to units across the network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetOrder {
    /// Move selected unit(s) to target coordinates (tile_x, tile_y)
    MoveTo(i32, i32),
    /// Attack a specific target unit by its global ID
    Attack(u32),
    /// Patrol between two coordinates (start_x, start_y) -> (end_x, end_y)
    Patrol(i32, i32, i32, i32),
    /// Anchor defense perimeter guard at (anchor_x, anchor_y)
    Guard(i32, i32),
    /// Repair a targeted friendly damaged unit or structure
    Repair(u32),
    /// Bulldoze target hazard/terrain tile at (tile_x, tile_y)
    Bulldoze(i32, i32),
    /// Deploy ConVec structure kit at (tile_x, tile_y)
    DeployConVec(i32, i32),
}

impl NetOrder {
    /// Create a MoveTo order
    pub fn move_to(x: i32, y: i32) -> Self {
        NetOrder::MoveTo(x, y)
    }

    /// Create an Attack order targeting unit_id
    pub fn attack(target_id: u32) -> Self {
        NetOrder::Attack(target_id)
    }

    /// Create a Patrol order between (x1, y1) and (x2, y2)
    pub fn patrol(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        NetOrder::Patrol(x1, y1, x2, y2)
    }

    /// Create a Guard order anchored at (x, y)
    pub fn guard(x: i32, y: i32) -> Self {
        NetOrder::Guard(x, y)
    }

    /// Create a Repair order targeting friendly unit_id
    pub fn repair(target_id: u32) -> Self {
        NetOrder::Repair(target_id)
    }

    /// Create a Bulldoze order at (x, y)
    pub fn bulldoze(x: i32, y: i32) -> Self {
        NetOrder::Bulldoze(x, y)
    }

    /// Create a DeployConVec order at (x, y)
    pub fn deploy_convec(x: i32, y: i32) -> Self {
        NetOrder::DeployConVec(x, y)
    }

    /// Convert network order to the internal engine `UnitOrder`.
    pub fn to_unit_order(&self) -> crate::sim::pathfinding::UnitOrder {
        match self {
            NetOrder::MoveTo(x, y) => crate::sim::pathfinding::UnitOrder::MoveTo {
                target_x: *x,
                target_y: *y,
            },
            NetOrder::Attack(target_id) => crate::sim::pathfinding::UnitOrder::Attack {
                target_unit_id: *target_id,
            },
            NetOrder::Patrol(x1, y1, x2, y2) => crate::sim::pathfinding::UnitOrder::Patrol {
                start_x: *x1,
                start_y: *y1,
                end_x: *x2,
                end_y: *y2,
                heading_to_end: true,
            },
            NetOrder::Guard(x, y) => crate::sim::pathfinding::UnitOrder::Guard {
                anchor_x: *x,
                anchor_y: *y,
            },
            NetOrder::Repair(target_id) => crate::sim::pathfinding::UnitOrder::Repair {
                target_unit_id: *target_id,
            },
            NetOrder::Bulldoze(x, y) => crate::sim::pathfinding::UnitOrder::Bulldoze {
                target_x: *x,
                target_y: *y,
            },
            NetOrder::DeployConVec(x, y) => {
                crate::sim::pathfinding::UnitOrder::DeployStructure {
                    kit_name: "Default".to_string(),
                    tile_x: *x,
                    tile_y: *y,
                }
            }
        }
    }
}

impl TryFrom<&crate::sim::pathfinding::UnitOrder> for NetOrder {
    type Error = &'static str;

    fn try_from(order: &crate::sim::pathfinding::UnitOrder) -> Result<Self, Self::Error> {
        match order {
            crate::sim::pathfinding::UnitOrder::MoveTo { target_x, target_y } => {
                Ok(NetOrder::MoveTo(*target_x, *target_y))
            }
            crate::sim::pathfinding::UnitOrder::Attack { target_unit_id } => {
                Ok(NetOrder::Attack(*target_unit_id))
            }
            crate::sim::pathfinding::UnitOrder::Patrol {
                start_x,
                start_y,
                end_x,
                end_y,
                ..
            } => Ok(NetOrder::Patrol(*start_x, *start_y, *end_x, *end_y)),
            crate::sim::pathfinding::UnitOrder::Guard { anchor_x, anchor_y } => {
                Ok(NetOrder::Guard(*anchor_x, *anchor_y))
            }
            crate::sim::pathfinding::UnitOrder::Repair { target_unit_id } => {
                Ok(NetOrder::Repair(*target_unit_id))
            }
            crate::sim::pathfinding::UnitOrder::Bulldoze { target_x, target_y } => {
                Ok(NetOrder::Bulldoze(*target_x, *target_y))
            }
            crate::sim::pathfinding::UnitOrder::DeployStructure {
                tile_x, tile_y, ..
            } => Ok(NetOrder::DeployConVec(*tile_x, *tile_y)),
            _ => Err("UnitOrder type cannot be mapped to NetOrder"),
        }
    }
}

/// Deterministic, high-performance packet protocol for peer-to-peer multiplayer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NetMessage {
    /// Initial handshake to synchronize player profile, faction, and scenario.
    Handshake {
        player_name: String,
        faction: Faction,
        scenario_id: usize,
    },
    /// Tactical order issued to one or multiple selected units.
    UnitOrder {
        unit_ids: Vec<u32>,
        order: NetOrder,
    },
    /// Queue production of vehicle or structure kit at building factory.
    QueueProduction {
        building_id: u32,
        item: String,
    },
    /// Mark a structure for dismantling/recycling to recover metals.
    RecycleStructure {
        building_id: u32,
    },
    /// In-game chat message exchanged between players.
    ChatMessage {
        sender: String,
        text: String,
    },
    /// Lockstep tick heartbeat and game Sol timestamp for desync detection.
    SyncHeartbeat {
        tick: u64,
        sol: f64,
    },
}

impl NetMessage {
    /// Serialize message to standard JSON format.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Parse message from standard JSON string.
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Serialize message to binary format (JSON byte slice).
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Parse message from binary byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

/// Enveloped packet carrying sequencing metadata and simulation tick timing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetPacket {
    /// Monotonically increasing packet sequence number
    pub seq: u64,
    /// Client performance timestamp (milliseconds)
    pub timestamp_ms: f64,
    /// Target execution tick in lockstep simulation
    pub target_tick: u64,
    /// Core game net message
    pub message: NetMessage,
}

impl NetPacket {
    /// Create a new enveloped network packet
    pub fn new(seq: u64, timestamp_ms: f64, target_tick: u64, message: NetMessage) -> Self {
        Self {
            seq,
            timestamp_ms,
            target_tick,
            message,
        }
    }

    /// Serialize packet to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Parse packet from JSON
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

/// Session Description Protocol (SDP) exchange container for zero-server P2P signaling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalingMessage {
    #[serde(rename = "type")]
    pub sdp_type: String, // "offer" or "answer"
    pub sdp: String,
}

impl SignalingMessage {
    /// Create a new signaling message
    pub fn new(sdp_type: impl Into<String>, sdp: impl Into<String>) -> Self {
        Self {
            sdp_type: sdp_type.into(),
            sdp: sdp.into(),
        }
    }

    /// Encode signaling payload into Base64 string for copy/paste or URL query hash.
    pub fn to_base64(&self) -> Result<String, serde_json::Error> {
        use base64::Engine;
        let json_str = serde_json::to_string(self)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(json_str.as_bytes()))
    }

    /// Decode signaling payload from either a Base64 string or raw JSON string.
    pub fn from_base64_or_json(input: &str) -> Result<Self, String> {
        use base64::Engine;
        let trimmed = input.trim();
        if trimmed.starts_with('{') {
            serde_json::from_str(trimmed).map_err(|e| format!("Invalid JSON signaling payload: {}", e))
        } else {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(trimmed)
                .map_err(|e| format!("Invalid Base64 string: {}", e))?;
            let json_str = String::from_utf8(bytes)
                .map_err(|e| format!("Decoded payload is not valid UTF-8: {}", e))?;
            serde_json::from_str(&json_str)
                .map_err(|e| format!("Decoded payload is not valid JSON: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_serde() {
        let msg = NetMessage::Handshake {
            player_name: "EdenCommander".to_string(),
            faction: Faction::Eden,
            scenario_id: 1,
        };
        let json = msg.to_json().expect("Serialize handshake");
        assert!(json.contains("\"type\":\"Handshake\""));
        assert!(json.contains("\"player_name\":\"EdenCommander\""));

        let decoded = NetMessage::from_json(&json).expect("Deserialize handshake");
        assert_eq!(msg, decoded);
    }

    #[test]
    fn test_unit_order_all_variants() {
        let orders = vec![
            NetOrder::move_to(10, 20),
            NetOrder::attack(42),
            NetOrder::patrol(5, 5, 25, 25),
            NetOrder::guard(14, 18),
            NetOrder::repair(99),
            NetOrder::bulldoze(12, 16),
            NetOrder::deploy_convec(30, 40),
        ];

        for order in orders {
            let msg = NetMessage::UnitOrder {
                unit_ids: vec![1, 2, 3],
                order: order.clone(),
            };
            let json = msg.to_json().expect("Serialize unit order");
            let decoded = NetMessage::from_json(&json).expect("Deserialize unit order");
            assert_eq!(msg, decoded);

            let unit_order = order.to_unit_order();
            let round_trip = NetOrder::try_from(&unit_order).expect("Convert back");
            assert_eq!(order, round_trip);
        }
    }

    #[test]
    fn test_queue_production_and_recycle() {
        let q_msg = NetMessage::QueueProduction {
            building_id: 105,
            item: "Lynx Laser Tank".to_string(),
        };
        let q_json = q_msg.to_json().expect("QueueProduction json");
        assert_eq!(q_msg, NetMessage::from_json(&q_json).unwrap());

        let r_msg = NetMessage::RecycleStructure { building_id: 105 };
        let r_json = r_msg.to_json().expect("RecycleStructure json");
        assert_eq!(r_msg, NetMessage::from_json(&r_json).unwrap());
    }

    #[test]
    fn test_chat_and_heartbeat() {
        let chat = NetMessage::ChatMessage {
            sender: "Player2".to_string(),
            text: "Enemy Lynx spotted at eastern ridge!".to_string(),
        };
        let chat_json = chat.to_json().expect("ChatMessage json");
        assert_eq!(chat, NetMessage::from_json(&chat_json).unwrap());

        let hb = NetMessage::SyncHeartbeat {
            tick: 14400,
            sol: 28.5,
        };
        let hb_json = hb.to_json().expect("SyncHeartbeat json");
        assert_eq!(hb, NetMessage::from_json(&hb_json).unwrap());
    }

    #[test]
    fn test_signaling_base64_roundtrip() {
        let sig = SignalingMessage::new("offer", "v=0\r\no=- 1234 2 IN IP4 127.0.0.1\r\n");
        let b64 = sig.to_base64().expect("Encode base64");
        let decoded = SignalingMessage::from_base64_or_json(&b64).expect("Decode base64");
        assert_eq!(sig, decoded);

        // Also test raw JSON input fallback
        let json = serde_json::to_string(&sig).expect("To json");
        let decoded_json = SignalingMessage::from_base64_or_json(&json).expect("Decode json");
        assert_eq!(sig, decoded_json);
    }
}
