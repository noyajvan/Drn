"src/shared_state.rs"
// Cross-core shared state for ESP32-S3
// All fields are atomic for lock-free access from both cores

use core::sync::atomic::{AtomicU32, AtomicBool, AtomicI32, Ordering};

/// LED state — доступен из Core0 (MAVLink) и Core1 (Rhai)
pub struct LedState {
    pub color: AtomicU32,      // 0x00RRGGBB
    pub brightness: AtomicU32, // 0-255
    pub mode: AtomicU32,       // 0=off, 1=solid, 2=blink
    pub blink_period_ms: AtomicU32,
    pub blink_pulse_ms: AtomicU32,
    pub dirty: AtomicBool,
}

impl LedState {
    pub const fn new() -> Self {
        Self {
            color: AtomicU32::new(0),
            brightness: AtomicU32::new(80),
            mode: AtomicU32::new(0),
            blink_period_ms: AtomicU32::new(500),
            blink_pulse_ms: AtomicU32::new(100),
            dirty: AtomicBool::new(false),
        }
    }
}

/// MAVLink telemetry — чтение из Rhai, запись из MAVLink handler
pub struct MavlinkState {
    pub heartbeat_received: AtomicBool,
    pub last_seq: AtomicU32,
    pub sys_id: AtomicU32,
    pub component_id: AtomicU32,
}

impl MavlinkState {
    pub const fn new() -> Self {
        Self {
            heartbeat_received: AtomicBool::new(false),
            last_seq: AtomicU32::new(0),
            sys_id: AtomicU32::new(1),
            component_id: AtomicU32::new(1),
        }
    }
}

/// Telegram command queue — Core1 отправляет команды из Telegram в MAVLink (Core0)
pub struct CommandState {
    pub arm: AtomicBool,
    pub disarm: AtomicBool,
    pub takeoff_alt: AtomicI32, // -1 = нет команды, >0 = высота в см
    pub rtl: AtomicBool,
    pub land: AtomicBool,
    pub emergency: AtomicBool,
    pub rhai_command: AtomicI32, // ID скрипта для запуска, -1 = нет команды
}

impl CommandState {
    pub const fn new() -> Self {
        Self {
            arm: AtomicBool::new(false),
            disarm: AtomicBool::new(false),
            takeoff_alt: AtomicI32::new(-1),
            rtl: AtomicBool::new(false),
            land: AtomicBool::new(false),
            emergency: AtomicBool::new(false),
            rhai_command: AtomicI32::new(-1),
        }
    }
}

/// Firebase logging state
pub struct FirebaseState {
    pub connected: AtomicBool,
    pub last_sync_ms: AtomicU32,
    pub pending_logs: AtomicU32, // количество ожидающих отправки логов
}

impl FirebaseState {
    pub const fn new() -> Self {
        Self {
            connected: AtomicBool::new(false),
            last_sync_ms: AtomicU32::new(0),
            pending_logs: AtomicU32::new(0),
        }
    }
}

/// Reset command flags after processing
pub fn reset_commands(state: &CommandState) {
    state.arm.store(false, Ordering::Relaxed);
    state.disarm.store(false, Ordering::Relaxed);
    state.takeoff_alt.store(-1, Ordering::Relaxed);
    state.rtl.store(false, Ordering::Relaxed);
    state.land.store(false, Ordering::Relaxed);
    state.emergency.store(false, Ordering::Relaxed);
    state.rhai_command.store(-1, Ordering::Relaxed);
}