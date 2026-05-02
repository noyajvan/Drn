// src/mavlink/state.rs
// Single atomic state struct for MAVLink data — replaces multiple AtomicBool/AtomicI32
// Thread-safe, lock-free reads via AtomicU32/AtomicI32, writes via Mutex

extern crate alloc;

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

/// MAVLink message IDs we care about
pub const MAVLINK_MSG_ID_HEARTBEAT: u32 = 0;
pub const MAVLINK_MSG_ID_GPS_RAW_INT: u32 = 24;
pub const MAVLINK_MSG_ID_ATTITUDE: u32 = 30;
pub const MAVLINK_MSG_ID_MISSION_COUNT: u32 = 44;
pub const MAVLINK_MSG_ID_MISSION_ITEM_INT: u32 = 73;
pub const MAVLINK_MSG_ID_BATTERY_STATUS: u32 = 147;
pub const MAVLINK_MSG_ID_SYS_STATUS: u32 = 1;
pub const MAVLINK_MSG_ID_STATUSTEXT: u32 = 253;

/// MAVLink modes (custom_mode from HEARTBEAT)
pub const MODE_STABILIZE: u32 = 0;
pub const MODE_AKRO: u32 = 1;
pub const MODE_ALT_HOLD: u32 = 2;
pub const MODE_AUTO: u32 = 3;
pub const MODE_LAND: u32 = 9;

/// Atomic state for fast lock-free reads from Rhai
pub struct MavlinkState {
    // HEARTBEAT
    pub armed: AtomicBool,
    pub custom_mode: AtomicU32,
    pub heartbeat_received: AtomicBool,
    // GPS_RAW_INT
    pub lat: AtomicI32,       // degrees * 1e7
    pub lon: AtomicI32,       // degrees * 1e7
    pub gps_fix_type: AtomicU32, // 0-3
    // ATTITUDE
    pub pitch: AtomicI32,     // mrad * 1000 (fixed-point)
    pub roll: AtomicI32,
    pub yaw: AtomicI32,
    // BATTERY_STATUS
    pub battery_remaining: AtomicU32, // 0-100%
    pub battery_voltage: AtomicI32,   // mV
    // MISSION
    pub mission_count: AtomicU32,
    pub mission_loaded: AtomicBool,
    pub min_radius: AtomicI32,  // cm (float * 100)
}

impl MavlinkState {
    pub const fn new() -> Self {
        Self {
            armed: AtomicBool::new(false),
            custom_mode: AtomicU32::new(0),
            heartbeat_received: AtomicBool::new(false),
            lat: AtomicI32::new(0),
            lon: AtomicI32::new(0),
            gps_fix_type: AtomicU32::new(0),
            pitch: AtomicI32::new(0),
            roll: AtomicI32::new(0),
            yaw: AtomicI32::new(0),
            battery_remaining: AtomicU32::new(0),
            battery_voltage: AtomicI32::new(0),
            mission_count: AtomicU32::new(0),
            mission_loaded: AtomicBool::new(false),
            min_radius: AtomicI32::new(5000), // 50.0m default
        }
    }

    // Fast lock-free reads for Rhai
    #[inline]
    pub fn is_armed(&self) -> bool {
        self.armed.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn mode(&self) -> u32 {
        self.custom_mode.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn has_heartbeat(&self) -> bool {
        self.heartbeat_received.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn gps_lat(&self) -> i32 {
        self.lat.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn gps_lon(&self) -> i32 {
        self.lon.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn gps_fix(&self) -> u32 {
        self.gps_fix_type.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn pitch_mrad(&self) -> i32 {
        self.pitch.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn roll_mrad(&self) -> i32 {
        self.roll.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn yaw_mrad(&self) -> i32 {
        self.yaw.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn battery_pct(&self) -> u32 {
        self.battery_remaining.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn battery_mv(&self) -> i32 {
        self.battery_voltage.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn mission_count(&self) -> u32 {
        self.mission_count.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn is_mission_loaded(&self) -> bool {
        self.mission_loaded.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn min_radius_cm(&self) -> i32 {
        self.min_radius.load(Ordering::Relaxed)
    }

    // Updates (called from MAVLink parser task)
    #[inline]
    pub fn update_heartbeat(&self, armed: bool, custom_mode: u32) {
        self.armed.store(armed, Ordering::Relaxed);
        self.custom_mode.store(custom_mode, Ordering::Relaxed);
        self.heartbeat_received.store(true, Ordering::Relaxed);
    }

    #[inline]
    pub fn update_gps(&self, lat: i32, lon: i32, fix_type: u8) {
        self.lat.store(lat, Ordering::Relaxed);
        self.lon.store(lon, Ordering::Relaxed);
        self.gps_fix_type.store(fix_type as u32, Ordering::Relaxed);
    }

    #[inline]
    pub fn update_attitude(&self, pitch: f32, roll: f32, yaw: f32) {
        self.pitch.store((pitch * 1000.0) as i32, Ordering::Relaxed);
        self.roll.store((roll * 1000.0) as i32, Ordering::Relaxed);
        self.yaw.store((yaw * 1000.0) as i32, Ordering::Relaxed);
    }

    #[inline]
    pub fn update_battery(&self, remaining: u8, voltage_mv: u16) {
        self.battery_remaining.store(remaining as u32, Ordering::Relaxed);
        self.battery_voltage.store(voltage_mv as i32, Ordering::Relaxed);
    }

    #[inline]
    pub fn update_mission_count(&self, count: u16) {
        self.mission_count.store(count as u32, Ordering::Relaxed);
        self.mission_loaded.store(false, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_mission_loaded(&self) {
        self.mission_loaded.store(true, Ordering::Relaxed);
    }

    #[inline]
    pub fn update_min_radius(&self, radius_m: f32) {
        self.min_radius.store((radius_m * 100.0) as i32, Ordering::Relaxed);
    }
}

/// Global MAVLink state — initialized once, accessed everywhere
pub static MAVLINK_STATE: MavlinkState = MavlinkState::new();

/// Mission item storage (for zone data — accessed via Mutex)
pub struct MissionItem {
    pub seq: u16,
    pub command: u16,
    pub x: i32,  // lat * 1e7 or param
    pub y: i32,  // lon * 1e7
    pub z: f32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
}

impl MissionItem {
    pub const fn new() -> Self {
        Self { seq: 0, command: 0, x: 0, y: 0, z: 0.0, param1: 0.0, param2: 0.0, param3: 0.0, param4: 0.0 }
    }
}

/// Zone data extracted from mission
#[derive(Clone)]
pub struct Zone {
    pub name: heapless::String<4>,
    pub lat: i32,
    pub lon: i32,
    pub rel: bool,
    pub received: bool,
    pub t_off_seq: u16,
}

impl Zone {
    pub fn new(name: &str) -> Self {
        let mut n = heapless::String::new();
        let _ = n.push_str(name);
        Self { name: n, lat: 0, lon: 0, rel: false, received: false, t_off_seq: 0 }
    }
}

/// Mission zones storage (Mutex-protected for Rhai access)
pub static MISSION_ZONES: Mutex<CriticalSectionRawMutex, heapless::Vec<Zone, 4>> =
    Mutex::new(heapless::Vec::new());

/// Current mission items buffer
pub static MISSION_ITEMS: Mutex<CriticalSectionRawMutex, heapless::Vec<MissionItem, 32>> =
    Mutex::new(heapless::Vec::new());
