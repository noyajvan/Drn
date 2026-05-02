// build.rs
// Скрипт збірки для ESP32-S3 (ESP-IDF + std + FreeRTOS)
// Використовує esp-idf-svc/esp-idf-sys з підтримкою стандартної бібліотеки

fn main() {
    // Повідомити cargo про перезбірку при зміні build.rs
    println!("cargo:rerun-if-changed=build.rs");

    // Перевірити target — тільки ESP32-S3 підтримується
    let target = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target != "xtensa" {
        println!("cargo:warning=Non-Xtensa target detected: {}. Some features may not work.", target);
    }
}
