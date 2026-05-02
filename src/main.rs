We need to apply the suggested edit to the original code. The suggested edit shows modifications to the `main.rs` file. The original code uses `esp_idf_sys::xTaskCreatePinnedToCore` with function pointers and raw FFI. The suggested edit replaces that with `std::thread::spawn` plus `thread_pin_to_core` and removes the `xTaskCreatePinnedToCore` approach. Also, the suggested edit changes comments, changes imports, and basically simplifies.

We must output the complete modified file, exactly as the suggested edit, but note that the suggested edit says "// Заглушка — позже вызов mavlink_handler::run();" and "// Заглушка — позже вызов rhai_engine::run();". That is placeholder. However, the original code had mavlink_handler::run_mavlink_task and rhai_engine::run_rhai_task as function pointers. The suggested edit likely intends to later call those functions directly within the thread closures. But the suggested edit code as shown does not actually call them; it has comments as placeholders. The instruction: "Apply the SUGGESTED EDIT to the ORIGINAL CODE. Output the complete modified file." So we should output the modified file exactly as the suggested edit shows. The modifications are: remove diff line markers, show the final code. The suggested edit is a code block with `// src/main.rs` at top, then the full file content. We should output that entire file as the modified code. It includes everything from `use std::sync::Arc;` to the end. But note: the original code had some comments like `// Global states (link-time initialized because they're in shared_state.rs) // We'll just use them directly.` which are not in the suggested edit. The suggested edit completely replaces the file. So the output should be the suggested edit content.

Check for any missing parts. The suggested edit includes only the code from the block. It includes the `fn main()` function with `thread_pin_to_core` calls. It does not include the original's `core0_thread.join().ok();` but uses `core0_handle.join().ok();`. The original had `thread::Builder` with stack size, but the suggested edit just `std::thread::spawn`. So we output the suggested edit exactly.

Thus, final answer is the code block, without any additional text.```rust
// src/main.rs
// ESP32-S3 Super Mini — FreeRTOS + std
// Core0: MAVLink handler (UART/SocketCAN)
// Core1: Rhai engine + Telegram bot + Firebase logger

use std::sync::Arc;
use anyhow::Result;
use log::info;
use esp_idf_svc::sys::{self as esp_idf_sys, ThreadSpawnConfiguration};
use esp_idf_svc::hal::task::thread_pin_to_core;
mod shared_state;
mod mavlink_handler;
mod rhai_engine;
mod telegram_bot;
mod firebase_logger;

use shared_state::{
    LedState, MavlinkState, CommandState, FirebaseState,
    MAVLINK_STATE, COMMAND_STATE, FIREBASE_STATE, LED_STATE,
};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    info!("ESP32-S3 Rhai+MAVLink firmware starting...");

    // Pin Core0 thread (Protocol CPU) — MAVLink
    let core0_handle = std::thread::spawn(move || {
        thread_pin_to_core(0);
        info!("MAVLink handler pinned to Core0");
        // Заглушка — позже вызов mavlink_handler::run();
    });

    // Pin Core1 thread (Application CPU) — Rhai + Telegram + Firebase
    let core1_handle = std::thread::spawn(move || {
        thread_pin_to_core(1);
        info!("Rhai engine pinned to Core1");
        // Заглушка — позже вызов rhai_engine::run();
    });

    core0_handle.join().ok();
    core1_handle.join().ok();
    Ok(())
}