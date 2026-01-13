/* * TEST HIỆU NĂNG: Gửi duy nhất 1 số (Single Value)
 * 8 giây đổi dạng sóng để test khả năng đáp ứng của Flutter Chart
 */

unsigned long lastTime = 0;
unsigned long lastChangeTime = 0;
int sampleRate = 10; // Gửi mỗi 5ms (200Hz) - Tốc độ này đủ để test độ mượt của App
int signalMode = 0;

void setup() {
  Serial.begin(115200); // Tốc độ cao để tránh nghẽn data
}

void loop() {
  unsigned long currentTime = millis();

  // Chuyển chế độ sóng mỗi 8 giây
  if (currentTime - lastChangeTime >= 8000) {
    signalMode = (signalMode + 1) % 4;
    lastChangeTime = currentTime;
  }

  // Gửi dữ liệu theo chu kỳ
  if (currentTime - lastTime >= sampleRate) {
    lastTime = currentTime;
    float value = 0;

    switch (signalMode) {
      case 0: // SIN: Kiểm tra độ mượt của animation
        value = sin(currentTime / 200.0) * 50 + 50;
        break;
      case 1: // SQUARE: Kiểm tra khả năng nhảy vọt của trục Y (Auto-scale)
        value = ((currentTime / 500) % 2 == 0) ? 90 : 10;
        break;
      case 2: // SAWTOOTH: Kiểm tra tính tuyến tính
        value = (currentTime % 1000) / 10.0;
        break;
      case 3: // NOISE: Test stress CPU/GPU (Vẽ đường zigzag liên tục)
        value = random(0, 100);
        break;
    }

    // CHỈ GỬI DUY NHẤT GIÁ TRỊ SỐ
    // Việc này giúp Flutter chỉ cần: double.parse(event) là xong.
    Serial.println(value); 
  }
}