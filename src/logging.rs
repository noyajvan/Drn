// src/logging.rs
// Система логирования для ESP32-S3 (FreeRTOS, два ядра)
// 1. Циклический буфер 4KB в RAM (НЕ пишем на Flash!)
// 2. Перенаправление Rhai print() в системный лог
// 3. Очередь MAVLink STATUSTEXT для полётного контроллера
// 4. Telegram-логи (событийные) — вызов из скриптов

use core::sync::atomic::{AtomicUsize, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use log::{Log, Metadata, Record, LevelFilter};

const BUF_SIZE: usize = 4096;
const MSG_MAX: usize = 256; // максимальная длина одного лог-сообщения

/// Состояние циклического буфера
struct LogBuffer {
    data: [u8; BUF_SIZE],
    write_pos: AtomicUsize,
    overflow_count: AtomicUsize, // сколько раз произошло переполнение (для отладки)
}

static LOG_BUF: Mutex<CriticalSectionRawMutex, LogBuffer> = Mutex::new(LogBuffer {
    data: [0u8; BUF_SIZE],
    write_pos: AtomicUsize::new(0),
    overflow_count: AtomicUsize::new(0),
});

/// Глобальный индикатор инициализации (чтобы не инициализировать дважды)
static LOG_INITIALIZED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Записать строку в циклический буфер (внутренняя, вызывается из логгера)
fn write_to_buffer(text: &str) {
    let bytes = text.as_bytes();
    let len = bytes.len().min(BUF_SIZE - 2); // оставляем место для '\n\0'
    
    let mut buf = LOG_BUF.lock();
    let mut pos = buf.write_pos.load(Ordering::Relaxed);
    
    for &b in bytes.iter().take(len) {
        buf.data[pos] = b;
        pos = (pos + 1) % BUF_SIZE;
    }
    // добавляем перевод строки
    buf.data[pos] = b'\n';
    pos = (pos + 1) % BUF_SIZE;
    // завершающий ноль для удобства дампа
    buf.data[pos] = 0;
    
    buf.write_pos.store(pos, Ordering::Relaxed);
}

/// Получить содержимое буфера как строку (для выгрузки в Telegram/Serial)
pub fn dump_buffer() -> heapless::String<4096> {
    let mut result = heapless::String::new();
    let buf = LOG_BUF.lock();
    let write = buf.write_pos.load(Ordering::Relaxed);
    let start = 0;
    
    // Простой дамп от начала до write позиции
    for i in 0..write {
        let c = buf.data[i];
        if c == 0 { break; }
        let _ = result.push(c as char);
    }
    result
}

/// Очистить буфер
pub fn clear_buffer() {
    let mut buf = LOG_BUF.lock();
    buf.data = [0u8; BUF_SIZE];
    buf.write_pos.store(0, Ordering::Relaxed);
    buf.overflow_count.store(0, Ordering::Relaxed);
}

/// Пользовательский логгер для интеграции с крейтом `log`
pub struct EspLogger;

impl Log for EspLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true // фильтрация через esp-println (LOG_LEVEL)
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        
        // Получаем ID ядра из FreeRTOS
        let core_id = unsafe { esp_idf_sys::xPortGetCoreID() };
        
        // Форматируем сообщение с меткой ядра
        let mut msg_buf: heapless::String<MSG_MAX> = heapless::String::new();
        let level = record.level();
        let prefix = match level {
            log::Level::Error => "ERROR",
            log::Level::Warn => "WARN",
            log::Level::Info => "INFO",
            log::Level::Debug => "DEBUG",
            log::Level::Trace => "TRACE",
        };
        
        // Формат: [Core 0] ERROR: сообщение
        let _ = core::fmt::write(&mut msg_buf, format_args!("[Core {}] {}: {}", core_id, prefix, record.args()));
        
        // Выводим в UART (usb-serial-jtag)
        // esp-println уже делает println с фильтром, но мы хотим гарантировать вывод
        // Поэтому используем прямую печать через esp_println
        esp_println::println!("{}", msg_buf.as_str());
        
        // Пишем в циклический буфер
        write_to_buffer(msg_buf.as_str());
    }

    fn flush(&self) {
        // no-op
    }
}

/// Инициализировать систему логирования
pub fn init() {
    if LOG_INITIALIZED.load(Ordering::Relaxed) {
        return; // уже инициализировано
    }
    
    // Устанавливаем глобальный логгер
    static LOGGER: EspLogger = EspLogger;
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(LevelFilter::Info); // debug/trace отключены в релизе
    
    LOG_INITIALIZED.store(true, Ordering::Relaxed);
    log::info!("Logging system initialized: [Core {}]", unsafe { esp_idf_sys::xPortGetCoreID() });
}

/// Обработчик вывода print() из Rhai (регистрируется в init_rhai.rs)
/// Перенаправляет вывод скрипта в системный лог с префиксом [Script]
pub fn rhai_print_handler(text: &str) {
    log::info!("[Script] {}", text);
}

/// Отправить событийное сообщение в Telegram (заглушка — пока через лог)
/// В будущем: вызов HTTP POST к Telegram Bot API
pub fn telegram_send(text: &str) {
    log::warn!("[Telegram] {}", text);
    // Отправляем в очередь STATUSTEXT для отображения на OSD/пульте
    crate::status_queue::STATUS_QUEUE.push_str(text);
}

/// Отправить сообщение в MAVLink STATUSTEXT очередь
pub fn send_statustext(severity: u8, text: &str) {
    // Формат: [severity] текст (severity: 0-EMERGENCY, 1-ALERT, ..., 6-INFO, 7-DEBUG)
    let mut msg = heapless::String::<64>::new();
    let _ = core::fmt::write(&mut msg, format_args!("[Sev:{}] {}", severity, text));
    crate::status_queue::STATUS_QUEUE.push(msg);
}