//! WebRTC Peer-to-Peer Session Management
//!
//! Provides a serverless, zero-signaling-infrastructure WebRTC DataChannel connection
//! between two players (Host and Guest) using copy/paste Base64 SDP or URL query hashes.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    RtcConfiguration, RtcDataChannel, RtcDataChannelInit, RtcDataChannelState,
    RtcPeerConnection, RtcSdpType, RtcSessionDescriptionInit,
};
use crate::net::protocol::{NetMessage, SignalingMessage};

/// Internal session state shared across callbacks and public session handles.
struct SessionInner {
    is_host: bool,
    peer_connection: RtcPeerConnection,
    data_channel: RefCell<Option<RtcDataChannel>>,
    incoming_messages: Rc<RefCell<Vec<NetMessage>>>,
    raw_incoming_messages: Rc<RefCell<Vec<String>>>,
    pending_outgoing: Rc<RefCell<Vec<String>>>,
    is_connected: Rc<Cell<bool>>,
    state_desc: Rc<RefCell<String>>,
    _callbacks: Rc<RefCell<Vec<Box<dyn std::any::Any>>>>,
}

/// WebRTC Peer-to-Peer Session for Outpost 2 Multiplayer.
///
/// Supports serverless manual SDP exchange (copy/paste or URL query hash),
/// deterministic ordered data channels, and seamless integration with the lockstep game loop.
#[derive(Clone)]
pub struct WebRtcSession {
    inner: Rc<SessionInner>,
}

impl WebRtcSession {
    /// Create a new session configured as the Host.
    ///
    /// The host creates the data channel and generates the initial WebRTC Offer.
    pub fn new_host() -> Result<Self, JsValue> {
        Self::create_session(true)
    }

    /// Create a new session configured as the Guest.
    ///
    /// The guest accepts the host's offer, creates an Answer, and receives the data channel.
    pub fn new_guest() -> Result<Self, JsValue> {
        Self::create_session(false)
    }

    fn create_session(is_host: bool) -> Result<Self, JsValue> {
        let rtc_config = create_default_rtc_configuration()?;
        let pc = RtcPeerConnection::new_with_configuration(&rtc_config)?;

        let incoming_messages = Rc::new(RefCell::new(Vec::new()));
        let raw_incoming_messages = Rc::new(RefCell::new(Vec::new()));
        let pending_outgoing = Rc::new(RefCell::new(Vec::new()));
        let is_connected = Rc::new(Cell::new(false));
        let initial_state = if is_host {
            "Host: Initialized, awaiting create_offer()".to_string()
        } else {
            "Guest: Initialized, awaiting offer from host".to_string()
        };
        let state_desc = Rc::new(RefCell::new(initial_state));
        let callbacks: Rc<RefCell<Vec<Box<dyn std::any::Any>>>> = Rc::new(RefCell::new(Vec::new()));

        let data_channel = RefCell::new(None);

        if is_host {
            // Host creates the ordered reliable data channel
            let dc_init = RtcDataChannelInit::new();
            dc_init.set_ordered(true);
            let dc = pc.create_data_channel_with_data_channel_dict("op2-data", &dc_init);
            setup_datachannel_listeners(
                &dc,
                &incoming_messages,
                &raw_incoming_messages,
                &pending_outgoing,
                &is_connected,
                &state_desc,
                &callbacks,
            );
            *data_channel.borrow_mut() = Some(dc);
        } else {
            // Guest listens for the data channel event from host
            let incoming_clone = incoming_messages.clone();
            let raw_clone = raw_incoming_messages.clone();
            let pending_clone = pending_outgoing.clone();
            let is_conn_clone = is_connected.clone();
            let state_clone = state_desc.clone();
            let callbacks_clone = callbacks.clone();

            let dc_holder = Rc::new(RefCell::new(None));
            let dc_holder_clone = dc_holder.clone();

            let on_datachannel = Closure::wrap(Box::new(move |event: web_sys::RtcDataChannelEvent| {
                let dc = event.channel();
                web_sys::console::log_1(&JsValue::from_str("[WebRTC] Guest received RTCDataChannel from Host!"));
                setup_datachannel_listeners(
                    &dc,
                    &incoming_clone,
                    &raw_clone,
                    &pending_clone,
                    &is_conn_clone,
                    &state_clone,
                    &callbacks_clone,
                );
                *dc_holder_clone.borrow_mut() = Some(dc);
            }) as Box<dyn FnMut(web_sys::RtcDataChannelEvent)>);

            pc.set_ondatachannel(Some(on_datachannel.as_ref().unchecked_ref()));
            callbacks.borrow_mut().push(Box::new(on_datachannel));
        }

        // Setup peer connection state change logging
        let state_clone = state_desc.clone();
        let pc_clone = pc.clone();
        let on_connectionstatechange = Closure::wrap(Box::new(move || {
            let state = pc_clone.connection_state();
            let desc = format!("PeerConnection: {:?}", state);
            web_sys::console::log_1(&JsValue::from_str(&format!("[WebRTC] {}", desc)));
            *state_clone.borrow_mut() = desc;
        }) as Box<dyn FnMut()>);
        pc.set_onconnectionstatechange(Some(on_connectionstatechange.as_ref().unchecked_ref()));
        callbacks.borrow_mut().push(Box::new(on_connectionstatechange));

        Ok(Self {
            inner: Rc::new(SessionInner {
                is_host,
                peer_connection: pc,
                data_channel,
                incoming_messages,
                raw_incoming_messages,
                pending_outgoing,
                is_connected,
                state_desc,
                _callbacks: callbacks,
            }),
        })
    }

    /// Host Step 1: Create an Offer and return a Base64-encoded SDP string to share with Guest.
    ///
    /// This method starts ICE candidate gathering and waits for it to complete or time out,
    /// ensuring all gathered candidates are embedded directly in the single SDP string.
    pub async fn create_offer(&self) -> Result<String, JsValue> {
        if !self.inner.is_host {
            return Err(JsValue::from_str("Only the Host can initiate create_offer()"));
        }
        let pc = &self.inner.peer_connection;

        let offer_promise = pc.create_offer();
        let offer_val = wasm_bindgen_futures::JsFuture::from(offer_promise).await?;
        let offer_sdp = js_sys::Reflect::get(&offer_val, &JsValue::from_str("sdp"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Failed to get SDP string from offer"))?;

        let desc = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        desc.set_sdp(&offer_sdp);
        let set_local = pc.set_local_description(&desc);
        wasm_bindgen_futures::JsFuture::from(set_local).await?;

        // Wait for ICE candidate gathering so SDP is fully self-contained
        wait_for_ice_gathering_complete(pc, 1500).await?;

        let final_sdp = pc.local_description().map(|d| d.sdp()).unwrap_or(offer_sdp);
        let signaling = SignalingMessage::new("offer", final_sdp);
        signaling
            .to_base64()
            .map_err(|e| JsValue::from_str(&format!("Base64 serialization failed: {}", e)))
    }

    /// Guest Step: Accept the Host's Offer SDP and return a Base64-encoded Answer SDP string.
    ///
    /// The offer string can be either Base64-encoded or raw JSON format.
    pub async fn accept_offer_and_create_answer(&self, offer_sdp: &str) -> Result<String, JsValue> {
        let pc = &self.inner.peer_connection;

        let sig_offer = SignalingMessage::from_base64_or_json(offer_sdp)
            .map_err(|e| JsValue::from_str(&e))?;

        if sig_offer.sdp_type != "offer" {
            return Err(JsValue::from_str("Signaling payload must be of type 'offer'"));
        }

        let remote_desc = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        remote_desc.set_sdp(&sig_offer.sdp);
        let set_remote = pc.set_remote_description(&remote_desc);
        wasm_bindgen_futures::JsFuture::from(set_remote).await?;

        let answer_promise = pc.create_answer();
        let answer_val = wasm_bindgen_futures::JsFuture::from(answer_promise).await?;
        let answer_sdp = js_sys::Reflect::get(&answer_val, &JsValue::from_str("sdp"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Failed to get SDP string from answer"))?;

        let local_desc = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
        local_desc.set_sdp(&answer_sdp);
        let set_local = pc.set_local_description(&local_desc);
        wasm_bindgen_futures::JsFuture::from(set_local).await?;

        // Wait for ICE candidate gathering
        wait_for_ice_gathering_complete(pc, 1500).await?;

        let final_sdp = pc.local_description().map(|d| d.sdp()).unwrap_or(answer_sdp);
        let signaling = SignalingMessage::new("answer", final_sdp);
        signaling
            .to_base64()
            .map_err(|e| JsValue::from_str(&format!("Base64 serialization failed: {}", e)))
    }

    /// Host Step 2: Accept the Guest's Answer SDP to complete the serverless peer-to-peer link.
    pub async fn accept_answer(&self, answer_sdp: &str) -> Result<(), JsValue> {
        let pc = &self.inner.peer_connection;

        let sig_answer = SignalingMessage::from_base64_or_json(answer_sdp)
            .map_err(|e| JsValue::from_str(&e))?;

        if sig_answer.sdp_type != "answer" {
            return Err(JsValue::from_str("Signaling payload must be of type 'answer'"));
        }

        let remote_desc = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
        remote_desc.set_sdp(&sig_answer.sdp);
        let set_remote = pc.set_remote_description(&remote_desc);
        wasm_bindgen_futures::JsFuture::from(set_remote).await?;

        web_sys::console::log_1(&JsValue::from_str(
            "[WebRTC] Host accepted Answer. Peer-to-peer connection established!",
        ));
        Ok(())
    }

    /// Send a typed game packet over the data channel.
    ///
    /// If the channel is still opening, the message is queued and flushed upon connection.
    pub fn send_message(&self, message: &NetMessage) -> Result<(), JsValue> {
        let json_str = message
            .to_json()
            .map_err(|e| JsValue::from_str(&format!("NetMessage serialization failed: {}", e)))?;
        self.send_raw(&json_str)
    }

    /// Send raw string payload across data channel.
    pub fn send_raw(&self, text: &str) -> Result<(), JsValue> {
        if let Some(dc) = self.inner.data_channel.borrow().as_ref() {
            if dc.ready_state() == RtcDataChannelState::Open {
                return dc.send_with_str(text);
            }
        }
        // Channel not ready yet; buffer for dispatch on open
        self.inner.pending_outgoing.borrow_mut().push(text.to_string());
        Ok(())
    }

    /// Poll and drain all received typed messages since the last poll.
    ///
    /// Intended to be invoked once per game frame / simulation tick.
    pub fn poll_messages(&self) -> Vec<NetMessage> {
        let mut queue = self.inner.incoming_messages.borrow_mut();
        queue.drain(..).collect()
    }

    /// Poll and drain all raw string messages since the last poll.
    pub fn poll_raw_messages(&self) -> Vec<String> {
        let mut queue = self.inner.raw_incoming_messages.borrow_mut();
        queue.drain(..).collect()
    }

    /// Check if the WebRTC DataChannel is open and ready for bidirectional communication.
    pub fn is_connected(&self) -> bool {
        if let Some(dc) = self.inner.data_channel.borrow().as_ref() {
            dc.ready_state() == RtcDataChannelState::Open
        } else {
            false
        }
    }

    /// Returns true if this session was configured as the Host.
    pub fn is_host(&self) -> bool {
        self.inner.is_host
    }

    /// Returns human-readable connection status string.
    pub fn connection_status(&self) -> String {
        self.inner.state_desc.borrow().clone()
    }

    /// Close data channel and peer connection.
    pub fn close(&self) {
        if let Some(dc) = self.inner.data_channel.borrow().as_ref() {
            dc.close();
        }
        self.inner.peer_connection.close();
        self.inner.is_connected.set(false);
        *self.inner.state_desc.borrow_mut() = "Connection Closed".to_string();
    }
}

/// Configure STUN servers for WebRTC NAT traversal.
fn create_default_rtc_configuration() -> Result<RtcConfiguration, JsValue> {
    let config = RtcConfiguration::new();
    let ice_servers = js_sys::Array::new();

    let server1 = js_sys::Object::new();
    let urls = js_sys::Array::new();
    urls.push(&JsValue::from_str("stun:stun.l.google.com:19302"));
    urls.push(&JsValue::from_str("stun:stun1.l.google.com:19302"));
    urls.push(&JsValue::from_str("stun:stun2.l.google.com:19302"));
    js_sys::Reflect::set(&server1, &JsValue::from_str("urls"), &urls)?;
    ice_servers.push(&server1);

    config.set_ice_servers(&ice_servers.into());
    Ok(config)
}

/// Attach standard callbacks to RTCDataChannel.
fn setup_datachannel_listeners(
    dc: &RtcDataChannel,
    incoming: &Rc<RefCell<Vec<NetMessage>>>,
    raw_incoming: &Rc<RefCell<Vec<String>>>,
    pending: &Rc<RefCell<Vec<String>>>,
    is_connected: &Rc<Cell<bool>>,
    state_desc: &Rc<RefCell<String>>,
    callbacks: &Rc<RefCell<Vec<Box<dyn std::any::Any>>>>,
) {
    let dc_clone_for_open = dc.clone();
    let pending_for_open = pending.clone();
    let is_conn_for_open = is_connected.clone();
    let state_for_open = state_desc.clone();

    let onopen = Closure::wrap(Box::new(move || {
        is_conn_for_open.set(true);
        *state_for_open.borrow_mut() = "Connected (DataChannel Open)".to_string();
        web_sys::console::log_1(&JsValue::from_str("[WebRTC] RTCDataChannel 'op2-data' is OPEN!"));

        // Flush buffered pending messages
        let mut queued = pending_for_open.borrow_mut();
        for msg in queued.drain(..) {
            let _ = dc_clone_for_open.send_with_str(&msg);
        }
    }) as Box<dyn FnMut()>);
    dc.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    callbacks.borrow_mut().push(Box::new(onopen));

    let is_conn_for_close = is_connected.clone();
    let state_for_close = state_desc.clone();
    let onclose = Closure::wrap(Box::new(move || {
        is_conn_for_close.set(false);
        *state_for_close.borrow_mut() = "Disconnected (DataChannel Closed)".to_string();
        web_sys::console::log_1(&JsValue::from_str("[WebRTC] RTCDataChannel closed"));
    }) as Box<dyn FnMut()>);
    dc.set_onclose(Some(onclose.as_ref().unchecked_ref()));
    callbacks.borrow_mut().push(Box::new(onclose));

    let onerror = Closure::wrap(Box::new(move |err: JsValue| {
        web_sys::console::error_2(&JsValue::from_str("[WebRTC] RTCDataChannel error:"), &err);
    }) as Box<dyn FnMut(JsValue)>);
    dc.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    callbacks.borrow_mut().push(Box::new(onerror));

    let incoming_for_msg = incoming.clone();
    let raw_for_msg = raw_incoming.clone();
    let onmessage = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        let data = event.data();
        if let Some(text) = data.as_string() {
            raw_for_msg.borrow_mut().push(text.clone());
            if let Ok(msg) = NetMessage::from_json(&text) {
                incoming_for_msg.borrow_mut().push(msg);
            } else {
                web_sys::console::warn_2(
                    &JsValue::from_str("[WebRTC] Failed to parse NetMessage JSON:"),
                    &JsValue::from_str(&text),
                );
            }
        } else if js_sys::ArrayBuffer::is_view(&data) || data.is_instance_of::<js_sys::ArrayBuffer>() {
            let uint8 = js_sys::Uint8Array::new(&data);
            let bytes = uint8.to_vec();
            if let Ok(msg) = NetMessage::from_bytes(&bytes) {
                incoming_for_msg.borrow_mut().push(msg);
            }
        }
    }) as Box<dyn FnMut(web_sys::MessageEvent)>);
    dc.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    callbacks.borrow_mut().push(Box::new(onmessage));
}

/// Helper that waits for ICE gathering to complete or times out after `timeout_ms`.
async fn wait_for_ice_gathering_complete(
    pc: &RtcPeerConnection,
    timeout_ms: i32,
) -> Result<(), JsValue> {
    if pc.ice_gathering_state() == web_sys::RtcIceGatheringState::Complete {
        return Ok(());
    }

    let pc_clone = pc.clone();
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let is_resolved = Rc::new(Cell::new(false));
        let resolve_rc = Rc::new(RefCell::new(Some(resolve)));

        let do_resolve = {
            let is_resolved = is_resolved.clone();
            let resolve_rc = resolve_rc.clone();
            move || {
                if !is_resolved.get() {
                    is_resolved.set(true);
                    if let Some(res) = resolve_rc.borrow_mut().take() {
                        let _ = res.call0(&JsValue::NULL);
                    }
                }
            }
        };

        // Safety timeout
        let do_resolve_timeout = do_resolve.clone();
        let timeout_closure = Closure::once(move || {
            do_resolve_timeout();
        });
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout_closure.as_ref().unchecked_ref(),
                timeout_ms,
            );
        } else {
            do_resolve();
            return;
        }
        timeout_closure.forget();

        // Candidate listener: candidate == None means ICE gathering is complete
        let do_resolve_ice = do_resolve.clone();
        let pc_check = pc_clone.clone();
        let ice_closure = Closure::wrap(Box::new(move |event: web_sys::RtcPeerConnectionIceEvent| {
            if event.candidate().is_none()
                || pc_check.ice_gathering_state() == web_sys::RtcIceGatheringState::Complete
            {
                do_resolve_ice();
            }
        }) as Box<dyn FnMut(web_sys::RtcPeerConnectionIceEvent)>);

        pc_clone.set_onicecandidate(Some(ice_closure.as_ref().unchecked_ref()));
        ice_closure.forget();
    });

    wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(())
}
