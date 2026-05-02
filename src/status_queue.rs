// src/status_queue.rs
// Thread-safe status message queue for MAVLink STATUSTEXT and Telegram

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use heapless::String;

/// Maximum number of pending status messages
const QUEUE_SIZE: usize = 8;
/// Maximum length of a single status message
const MSG_MAX: usize = 128;

/// Global status queue — used by logging, MAVLink handler, and Rhai scripts
pub static STATUS_QUEUE: Channel<CriticalSectionRawMutex, String<MSG_MAX>, QUEUE_SIZE> =
    Channel::new();

/// Push a formatted status message to the queue (non-blocking)
pub fn push_str(text: &str) {
    let mut msg: String<MSG_MAX> = String::new();
    if core::fmt::Write::write_str(&mut msg, text).is_ok() {
        // Try to send; if full, silently drop (or log warning)
        let _ = STATUS_QUEUE.try_send(msg);
    }
}

/// Push a pre-formatted String to the queue
pub fn push(msg: String<MSG_MAX>) {
    let _ = STATUS_QUEUE.try_send(msg);
}

/// Pop a message from the queue (non-blocking)
pub fn pop() -> Option<String<MSG_MAX>> {
    STATUS_QUEUE.try_recv()
}