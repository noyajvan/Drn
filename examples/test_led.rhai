





















We need to output only the modified code file. The instruction says: "Apply the SUGGESTED EDIT to the ORIGINAL CODE. Output the complete modified file." Since the SUGGESTED EDIT is a completely different script, the modified file should be that script. The original code is entirely replaced. There are no parts to preserve; the suggested edit replaces everything. So output the suggested edit code exactly. But note that the SUGGESTED EDIT code snippet includes backticks in the prompt; we need to output the code without the surrounding backticks. The instruction says: "Output ONLY code." So I'll output the code contents. The original file had a comment at the top with filename and description. The suggested edit has its own comments. So output that.// Тестовый скрипт для ESP32-S3 Super Mini
// Мигание RGB + проверка MAVLink

print("=== Starting LED Test ===");

// Помигаем разными цветами
rgb(255, 0, 0);    // красный
sleep(500);
rgb(0, 255, 0);    // зелёный
sleep(500);
rgb(0, 0, 255);    // синий
sleep(500);
rgb(255, 255, 0);  // жёлтый
sleep(500);
rgb(0, 255, 255);  // циан
sleep(500);
rgb(255, 0, 255);  // пурпурный
sleep(500);

// Проверим MAVLink состояние
let armed = is_armed();
print("Armed status: " + armed.to_string());

if armed {
    print("Drone is ARMED — monitoring attitude");
    loop {
        let pitch = get_pitch() / 1000;
        print("Pitch (deg): " + pitch.to_string());
        sleep(1000);
    }
} else {
    print("Drone is DISARMED — turning LED off");
    rgb(0, 0, 0);
}

print("=== Test Complete ===");