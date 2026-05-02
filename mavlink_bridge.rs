//! MAVLink packet parser – mission sync, commands, status messages

use crate::{common::MavMessage, mavlink, SharedData, Point};
use core::sync::atomic::Ordering;
use mavlink::common::MavFrame;
use std::sync::{Arc, RwLock};

/// Parse a buffer of incoming MAVLink bytes
pub fn parse_incoming(data: &[u8], shared: &Arc<RwLock<SharedData>>) {
    // Use mavlink::parse to extract frames
    // Match each frame:
    // - HEARTBEAT -> update armed/mode
    // - GLOBAL_POSITION_INT -> update lat/lon/alt
    // - SYS_STATUS -> update vbat
    // - MISSION_COUNT -> start mission request
    // - MISSION_ITEM_INT -> store points
    // - NAV_DELAY -> extract min_radius
    // - STATUSTEXT -> push to status_queue
    // - COMMAND_ACK -> log
}