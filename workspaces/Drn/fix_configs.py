import os

config_toml_content = '''# .cargo/config.toml
# Конфигурация для ESP32-S3 с ESP-IDF (std)

[build]
target = "xtensa-esp32s3-espidf"
jobs = 8

[target.xtensa-esp32s3-espidf]
linker = "xtensa-esp32s3-elf-gcc"
rustflags = [
    "-C", "opt-level=s",
]
'''

cargo_toml_content = '''[package]
name = "esp32-s3-rhai-mavlink"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
esp-idf-svc = { version = "0.49", default-features = false, features = ["std"] }
esp-idf-sys = { version = "0.35", features = ["std"] }
esp-idf-hal = { version = "0.44", features = ["std"] }
esp-backtrace = { version = "0.19", features = ["esp32s3", "panic-handler", "println"] }
esp-println = { version = "0.17", features = ["esp32s3", "log-04", "jtag-serial"] }

rhai = { version = "1.21", features = ["sync", "f32_float", "only_i32", "no_index", "no_object"] }

mavlink = { version = "0.12", features = ["std"] }

anyhow = "1"
thiserror = "2"
log = "0.4"
heapless = "0.8"

embassy-executor = { version = "0.7", features = ["executor-thread", "task-arena-size-32768"] }
embassy-time = { version = "0.4", features = ["generic-queue-8"] }
embassy-sync = "0.6"
embassy-futures = "0.1"

serde = { version = "1", features = ["derive"] }
serde_json = "1"
atomic-polyfill = "1.0"
crossbeam-channel = "0.5"

[profile.dev]
opt-level = 1
lto = false
codegen-units = 16
debug = false
incremental = true

[profile.release]
opt-level = "s"
lto = true
codegen-units = 1
debug = false
'''

with open('/workspaces/Drn/.cargo/config.toml', 'w') as f:
    f.write(config_toml_content)
with open('/workspaces/Drn/Cargo.toml', 'w') as f:
    f.write(cargo_toml_content)
print('Both config files written successfully.')
