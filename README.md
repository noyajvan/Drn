# ESP32-S3 Super Mini — Rhai Firmware (no_std + embassy)

Каркас прошивки для **ESP32-S3 Super Mini** з підтримкою скриптів Rhai, MAVLink, WiFi та LittleFS.

## Апаратна специфікація (ESP32-S3 Super Mini)

| Параметр | Значення |
|----------|----------|
| Flash | 4 MB (QIO) |
| PSRAM | Немає |
| RGB LED | GPIO48 (WS2812B, 1 піксель, GRB порядок) |
| BOOT кнопка | GPIO0 (LOW = Safe Mode) |
| USB-CDC | Native USB (не CH340) |
| I2C SDA | GPIO8 |
| I2C SCL | GPIO9 |
| UART TX | GPIO5 (MAVLink) |
| UART RX | GPIO6 (MAVLink) |

## Архітектура

```
src/
├── main.rs              # Точка входу, State Machine (BOOT_MODE / RUN_MODE)
├── init_peripherals.rs  # Ініціалізація GPIO, I2C, UART, deep sleep
├── init_rhai.rs         # Глобальний рушій Rhai (StaticCell), esp-println
├── rhai_bridge.rs       # Hardware API для Rhai: gpio, i2c, uart, sleep, mavlink
└── mavlink_handler.rs   # Async task: читання UART, парсинг MAVLink, Signal

examples/
└── test_led.rhai        # Тестовий скрипт: мигання RGB + print()
```

## Встановлення toolchain

```bash
# 1. Встановити espup (менеджер Xtensa toolchain)
cargo install espup

# 2. Встановити Xtensa Rust toolchain
espup install

# 3. Активувати змінні середовища
. $HOME/export-esp.sh

# 4. Перевірити
rustup show
```

## Компіляція

```bash
# Перевірка без лінкування
cargo check

# Збірка
cargo build --release

# Прошивка через espflash
cargo install espflash
espflash flash --monitor target/xtensa-esp32s3-none-elf/release/esp32_s3_rhai
```

## State Machine

| Стан | Умова | Дія |
|------|-------|-----|
| `BOOT_MODE` | GPIO0 = LOW при старті | HTTP сервер для налаштування |
| `RUN_MODE` | GPIO0 = HIGH | Виконання `script.rhai` з LittleFS |
| `FALLBACK` | Помилка Rhai | Відкат на `backup.rhai` |

## Rhai API

```rhai
// Затримка
sleep(500);          // 500 мс

// GPIO
pin_mode(2, "output");
rgb(255, 0, 0);      // RGB LED

// I2C
i2c_write(0x68, [0x01, 0x02]);
let data = i2c_read(0x68, 6);

// UART
uart_write("hello");
let s = uart_read();

// WiFi
wifi("SSID", "password");

// MAVLink
let armed = is_armed();
let pitch = get_pitch();  // mrad * 1000 (i32)
```

## Залежності (Cargo.toml)

- `embassy-executor` — async runtime
- `embassy-time` — таймери
- `embassy-sync` — Signal/Channel між задачами
- `rhai` — скриптовий рушій (f32, only_i32, no_std)
- `littlefs2` — файлова система у flash
- `esp-wifi` — WiFi драйвер
- `esp-println` — вивід у USB-UART
- `mavlink` — MAVLink протокол
