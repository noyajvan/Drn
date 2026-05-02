// src/lib.rs
// ESP32-S3 Rhai+MAVLink firmware library

pub mod shared_state;
pub mod logging;
pub mod status_queue;
pub mod init_rhai;
pub mod rhai_bridge;
pub mod rhai_engine;
pub mod mavlink_handler;
pub mod mavlink;
