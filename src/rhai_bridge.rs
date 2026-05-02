We need to apply the suggested edit to the original code. The suggested edit appears identical to the original code. The instruction says "Apply the SUGGESTED EDIT to the ORIGINAL CODE. Output the complete modified file." The suggested edit is exactly the same as the original code. So the output should be the same code. But let's double-check: the original code is provided and the suggested edit is also provided. They look identical. Maybe there's a subtle difference? I'll compare line by line. The original code starts with "// src/rhai_bridge.rs", same as suggested. Both have same comments and code. I'll copy the original code as the final output. Since it's identical, the modified file is the same. But the user wants the output of the modified file after applying the suggested edit. Since no changes, output the original code.// src/rhai_bridge.rs
//! Rhai hardware bridge: GPIO, I2C, UART, WiFi, hw::sleep, MAVLink API, Telegram

extern crate alloc;
use alloc::string::String;
use rhai::{Engine, Array};
use core::sync::atomic::{AtomicBool, AtomicI32, Ordering};

// Глобальное состояние MAVLink (атомарные переменные, no_std-совместимые)
pub static MAVLINK_ARMED: AtomicBool = AtomicBool::new(false);
pub static MAVLINK_PITCH: AtomicI32 = AtomicI32::new(0); // pitch * 1000 (fixed-point mrad)

/// Подключение к WiFi
pub fn wifi(ssid: &str, password: &str) {
    // TODO: реальное подключение через esp-idf-svc
    log::info!("[Rhai] wifi: {}", ssid);
}

/// Запись в I2C
pub fn i2c_write(addr: i64, data: Array) {
    // TODO: Написати дані у I2C
}

/// Зчитування з I2C
pub fn i2c_read(addr: i64, len: i64) -> Array {
    // TODO: Зчитати дані з I2C
    Array::new()
}

/// Запис у UART
pub fn uart_write(data: &str) {
    // TODO: Відправити дані через UART
}

/// Зчитування з UART
pub fn uart_read() -> String {
    // TODO: Зчитати дані з UART
    String::new()
}

/// Встановлення режиму GPIO
pub fn pin_mode(pin: i64, mode: &str) {
    // TODO: Встановити режим GPIO
}

/// Встановлення RGB кольору WS2812B (GPIO48 на ESP32-S3 Super Mini)
/// r, g, b — значення 0..255 (i32 через only_i32 feature)
pub fn rgb(r: i64, g: i64, b: i64) {
    // TODO: Відправити GRB-пакет через RMT або bit-bang на GPIO48
    log::info!("[Rhai] rgb({}, {}, {})", r, g, b);
}

/// Затримка в мілісекундах (busy-wait заглушка для синхронного Rhai-контексту)
pub fn hw_sleep(ms: i64) {
    // TODO: замінити на реальний busy-wait або передавати через Channel
    log::info!("[Rhai] sleep({})", ms);
}

/// Перевірити, чи озброєний дрон (MAVLink HEARTBEAT)
pub fn mavlink_is_armed() -> bool {
    MAVLINK_ARMED.load(Ordering::Relaxed)
}

/// Отримати pitch у fixed-point (mrad * 1000) з MAVLink ATTITUDE
pub fn mavlink_get_pitch() -> i64 {
    MAVLINK_PITCH.load(Ordering::Relaxed) as i64
}

/// Реєстрація функцій HwInterface в Rhai Engine
pub fn register_hw_interface(engine: &mut Engine) {
    engine.register_fn("wifi", wifi);
    engine.register_fn("i2c_write", i2c_write);
    engine.register_fn("i2c_read", i2c_read);
    engine.register_fn("uart_write", uart_write);
    engine.register_fn("uart_read", uart_read);
    engine.register_fn("pin_mode", pin_mode);
    engine.register_fn("rgb", rgb);
    // hw namespace
    engine.register_fn("sleep", hw_sleep);
    // mavlink namespace
    engine.register_fn("is_armed", mavlink_is_armed);
    engine.register_fn("get_pitch", mavlink_get_pitch);
}
