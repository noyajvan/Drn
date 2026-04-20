# ESP32-S3 Super Mini + Rhai

Встраиваемая среда скриптинга **Rhai** на плате **ESP32-S3 Super Mini**, написанная на Rust (`no_std`, bare-metal).

Проект демонстрирует, как запустить интерпретатор Rhai на микроконтроллере и регистрировать в нём нативные Rust-функции (включая работу с аппаратурой — например, аппаратным RNG).

---

## 🧩 Возможности

- ✅ `no_std` Rust на ESP32-S3 (Xtensa, bare-metal)
- ✅ Интерпретатор [Rhai](https://rhai.rs) с `no_std` поддержкой
- ✅ Динамическое выделение памяти через `esp-alloc`
- ✅ Вывод через `esp-println` (UART0)
- ✅ Регистрация нативных функций в Rhai (`log`, `double`, `add`, `random`)
- ✅ Пример использования аппаратного RNG ESP32-S3 из Rhai-скрипта
- ✅ Облачная сборка через GitHub Actions (без установки ESP toolchain локально)

---

## 📦 Зависимости

```
esp-hal       = 1.0     (esp32s3, unstable)
esp-backtrace = 0.14    (panic-handler, println)
esp-println   = 0.12    (log)
esp-alloc     = 0.6
rhai          = 1.19    (no_std, f32_float, only_i64)
```

---

## ☁️ Сборка в облаке (рекомендуется)

Локально устанавливать ESP toolchain не нужно — всё собирается в **GitHub Actions**.

### 1. Создай репозиторий на GitHub

```bash
git init
git add .
git commit -m "Initial commit"
git branch -M main
git remote add origin https://github.com/<USER>/esp32-s3-super-mini-rhai.git
git push -u origin main
```

### 2. Запусти сборку

Workflow `.github/workflows/build.yml` запускается автоматически при каждом `push` в `main` / `master`, а также вручную через вкладку **Actions → Build ESP32-S3 Firmware → Run workflow**.

### 3. Скачай прошивку

После завершения сборки в разделе **Actions → <run> → Artifacts** будут доступны:

- `esp32s3-firmware-elf` — ELF-файл (для отладки)
- `esp32s3-firmware-bin` — готовая прошивка `firmware-esp32s3.bin`

### 4. Автоматический release

Если создать git-тег вида `v*.*.*`, workflow автоматически опубликует **GitHub Release** с приложенной прошивкой:

```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## 🔌 Прошивка платы

После того как скачал `firmware-esp32s3.bin` из artifacts:

### Вариант 1: через `espflash`

```bash
cargo install espflash
espflash flash --chip esp32s3 --monitor firmware-esp32s3.bin
```

### Вариант 2: через `esptool.py`

```bash
pip install esptool
esptool.py --chip esp32s3 --port COM3 --baud 921600 write_flash 0x0 firmware-esp32s3.bin
```

> На Linux/macOS порт обычно `/dev/ttyUSB0` или `/dev/ttyACM0`.
> На Windows — `COM3`, `COM4` и т.п. (смотри в Диспетчере устройств).

---

## 💻 Локальная сборка (опционально)

Если всё же хочешь собирать локально:

### 1. Установи ESP toolchain

```bash
cargo install espup --locked
espup install --targets esp32s3
```

### 2. Активируй окружение

**Linux / macOS:**
```bash
source $HOME/export-esp.sh
```

**Windows (PowerShell):**
```powershell
. $env:USERPROFILE\export-esp.ps1
```

### 3. Собери проект

```bash
cargo build --release
```

### 4. Прошей плату

```bash
cargo run --release
```

> Требуется **~5–10 ГБ** свободного места на диске для компиляции.

---

## 📂 Структура проекта

```
esp32-s3-super-mini-rhai/
├── .cargo/
│   └── config.toml              # target = xtensa-esp32s3-none-elf
├── .github/
│   └── workflows/
│       └── build.yml            # GitHub Actions workflow
├── src/
│   └── main.rs                  # основной код + Rhai-скрипт
├── Cargo.toml                   # зависимости
└── README.md
```

---

## 📜 Пример Rhai-скрипта (встроен в `main.rs`)

```rhai
print("Hello from Rhai running on ESP32-S3!");

let x = 21;
let y = double(x);
log("2 * 21 = " + y.to_string());

let sum = add(15, 27);
log("15 + 27 = " + sum.to_string());

let rnd = random();
log("Random value from ESP32 RNG: " + rnd.to_string());

let result = if sum > 40 { "greater than 40" } else { "not greater" };
log("Script evaluation result: " + result);

"Script execution completed successfully!"
```

### Зарегистрированные в Rhai функции

| Функция       | Описание                                     |
|---------------|----------------------------------------------|
| `log(msg)`    | Вывод сообщения через `esp-println`          |
| `double(n)`   | Возвращает `n * 2`                           |
| `add(a, b)`   | Возвращает `a + b`                           |
| `random()`    | Случайное число от аппаратного RNG ESP32-S3  |

---

## 🔧 Аппаратура

- **Плата:** ESP32-S3 Super Mini
- **Чип:** ESP32-S3 (Xtensa LX7, dual-core, 240 MHz)
- **Flash:** 4–8 МБ
- **PSRAM:** опционально
- **USB:** USB-C (встроенный USB-serial)

### Пины (типовая распиновка Super Mini)

| Функция   | GPIO |
|-----------|------|
| LED       | 48   |
| BOOT      | 0    |
| UART0 TX  | 43   |
| UART0 RX  | 44   |

---

## 🚀 Идеи для расширения

- [ ] Управление GPIO из Rhai (`gpio_high(n)`, `gpio_low(n)`)
- [ ] Чтение ADC из Rhai
- [ ] I2C / SPI драйверы, экспонированные в Rhai
- [ ] Wi-Fi + HTTP-клиент
- [ ] Загрузка Rhai-скриптов из SPIFFS / LittleFS
- [ ] REPL через UART (отправляешь Rhai-строку — получаешь результат)
- [ ] MQTT-мост: скрипты реагируют на сообщения

---

## 📄 Лицензия

MIT OR Apache-2.0

---

## 🙏 Благодарности

- [esp-rs](https://github.com/esp-rs) — Rust on ESP
- [Rhai](https://rhai.rs) — embedded scripting language for Rust
- [Espressif](https://www.espressif.com) — за отличные чипы