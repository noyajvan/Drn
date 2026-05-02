// src/rhai_engine.rs
// Core1: Rhai scripting engine + user script runner

use log::info;

/// Run Rhai engine loop (Core1)
pub fn run() {
    info!("[Core1] Rhai engine starting...");
    
    // Инициализация движка (один раз)
    if let Err(e) = crate::init_rhai::init_rhai_engine() {
        log::error!("Failed to init Rhai: {}", e);
        return;
    }
    
    // Загружаем тестовый скрипт
    match crate::init_rhai::mount_script("test_log.rhai") {
        Ok(script) => {
            info!("[Core1] Running test script...");
            let engine = crate::init_rhai::engine();
            let scope = crate::init_rhai::scope();
            match engine.eval_with_scope::<()>(scope, &script) {
                Ok(_) => info!("[Core1] Script completed"),
                Err(e) => log::error!("[Core1] Script error: {}", e),
            }
        }
        Err(e) => log::error!("[Core1] Script mount error: {}", e),
    }
    
    // Дамп циклического буфера после скрипта
    let dump = crate::logging::dump_buffer();
    if !dump.is_empty() {
        info!("[Core1] Log buffer dump:\n{}", dump.as_str());
    }
}