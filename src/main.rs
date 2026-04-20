#![no_std]
#![no_main]

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, delay::Delay, main, rng::Rng};
use esp_println::println;
use rhai::{Dynamic, Engine, Scope};

extern crate alloc;

fn init_heap() {
    const HEAP_SIZE: usize = 72 * 1024;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    unsafe {
        #[allow(static_mut_refs)]
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr(),
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
    println!("Heap initialized: {} bytes available", HEAP_SIZE);
}

// Simple function to register with Rhai - logs a message
fn rhai_log(message: &str) {
    println!("[Rhai] {}", message);
}

fn rhai_double(n: i64) -> i64 {
    n * 2
}

fn rhai_add(a: i64, b: i64) -> i64 {
    a + b
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Initialize delay
    let delay = Delay::new();

    // Initialize RNG (useful for Rhai if needed)
    let _ = peripherals.RNG;
    let mut rng = Rng::new();

    println!("\n\n");
    println!("========================================");
    println!(" ESP32-S3 Super Mini + Rhai v0.1.0     ");
    println!(" Embedded Scripting Environment        ");
    println!("========================================");
    println!();

    // Initialize the heap for Rhai (which requires dynamic allocation)
    init_heap();

    // Create Rhai scripting engine
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    // Register native Rust functions with the Rhai engine
    engine.register_fn("log", rhai_log);
    engine.register_fn("double", rhai_double);
    engine.register_fn("add", rhai_add);

    // Register a function that uses the hardware (example: get random number)
    engine.register_fn("random", move || -> i64 {
        let random_value = rng.random();
        random_value as i64
    });

    println!("Rhai engine initialized successfully!");
    println!("Registered functions: log(), double(), add(), random()");
    println!();

    // Sample Rhai script to demonstrate capabilities
    let script = r#"
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
    "#;

    println!("Executing sample Rhai script...");
    match engine.eval_with_scope::<Dynamic>(&mut scope, script) {
        Ok(result) => {
            println!("Rhai script returned: {:?}", result);
        }
        Err(e) => {
            println!("Rhai script error: {}", e);
        }
    }

    println!();
    println!("Rhai runtime is now active.");
    println!("You can extend this with GPIO control, sensors, WiFi, etc.");
    println!("Heartbeat started - the board is running Rhai scripts!");

    let mut counter = 0u32;

    loop {
        counter += 1;

        println!("ESP32-S3 heartbeat #{} - Rhai engine ready", counter);
        delay.delay_millis(2500);
    }
}
