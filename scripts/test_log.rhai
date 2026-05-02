"// Тест системы логирования Rhai
print("=== Rhai Logging Test ===");

let armed = is_armed();
print("Armed status: " + armed.to_string());

if armed {
    print("Drone is ARMED!");
    Telegram.send("ALERT: Drone armed while script running");
} else {
    print("Drone disarmed — safe");
    Telegram.send("INFO: Test script completed, disarmed");
}

// Мигаем LED для визуального подтверждения
rgb(0, 255, 0);  // зелёный — OK
sleep(200);
rgb(0, 0, 0);
sleep(200);
rgb(0, 255, 0);
sleep(200);
rgb(0, 0, 0);

print("Test complete! Log buffer available.");
"