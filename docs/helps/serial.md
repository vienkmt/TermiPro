# Giao tiếp Serial - Hướng dẫn Hoàn chỉnh

Chào mừng bạn đến với hướng dẫn về **Giao tiếp Serial trong TermiPro**. Tài liệu này sẽ giúp bạn hiểu rõ về Serial Port communication và cách sử dụng nó một cách hiệu quả.

---

## Mục lục

1. [Giới thiệu](#giới-thiệu)
2. [Kết nối Serial](#kết-nối-serial)
3. [Tín hiệu Điều khiển](#tín-hiệu-điều-khiển)
4. [Gửi Dữ liệu](#gửi-dữ-liệu)
5. [Auto Send](#auto-send)
6. [Terminal](#terminal)
7. [Mẹo Sử dụng](#mẹo-sử-dụng)

---

## Giới thiệu

### Serial Port Communication là gì?

**Serial Port** (cổng Serial) là một phương pháp **giao tiếp đơn giản** giữa máy tính và thiết bị điện tử, gửi dữ liệu **từng bit một, theo thứ tự** (serial = tuần tự).

Đây là một trong những cách giao tiếp **cổ nhất nhưng hiệu quả nhất** được sử dụng rộng rãi trong:
- **Microcontroller**: Arduino, STM32, ESP32, PIC
- **Thiết bị nhúng**: Cảm biến, module WiFi, GPS
- **Thiết bị công nghiệp**: PLC, biến tần, bộ điều khiển
- **Thiết bị viễn thông**: Modem, bộ chuyển đổi

### Tại sao sử dụng Serial Port?

**Ưu điểm:**
- **Đơn giản**: Chỉ cần 3 dây cơ bản (TX, RX, GND)
- **Phổ biến**: Hầu hết thiết bị điện tử đều hỗ trợ
- **Tin cậy**: Đã được sử dụng hàng chục năm
- **Dễ debug**: Nhìn thấy trực tiếp dữ liệu gửi/nhận

**Nhược điểm:**
- Tốc độ không quá cao (so với Ethernet, USB)
- Cần cấu hình đúng baud rate

### Khi nào cần sử dụng Serial Port?

- **Lập trình embedded systems**: Tương tác với Arduino, ESP32
- **Debug thiết bị**: Xem thông báo lỗi, dữ liệu cảm biến
- **Gửi lệnh**: Điều khiển thiết bị từ xa
- **Theo dõi dữ liệu**: Xem dòng dữ liệu thời gian thực
- **Kiểm tra thiết bị**: Xác minh thiết bị hoạt động đúng

> **Mẹo:** Hầu hết các embedded developer sử dụng Serial Port hàng ngày để debug và kiểm tra code.

---

## Kết nối Serial

### Bước 1: Chuẩn bị Thiết bị

Trước khi kết nối, hãy chuẩn bị:

**Thiết bị của bạn:**
- Được cấp điện (pin đủ hoặc USB power)
- Được cắm vào máy tính bằng cáp USB
- Các dây nối đúng chỗ (nếu sử dụng breakout board)

**Kiểm tra:**
1. Thiết bị có đèn LED sáng không? (nếu có)
2. Máy tính nhận biết thiết bị không? (kiểm tra Device Manager trên Windows)

### Bước 2: Chọn Cổng Serial

Cổng Serial là **địa chỉ** mà máy tính sử dụng để nói chuyện với thiết bị:

**Cách chọn:**
1. Mở **TermiPro**
2. Tìm dropdown **"Cổng Serial"** ở sidebar bên trái
3. Nhấp vào dropdown
4. **Lưu ý:** Danh sách cổng sẽ **tự động cập nhật** khi bạn click

**Tên cổng trên các hệ điều hành:**

| Hệ điều hành | Tên cổng | Ví dụ |
|---|---|---|
| **Windows** | COM + số | `COM3`, `COM4`, `COM5` |
| **macOS** | `/dev/tty.usbserial` | `/dev/tty.usbserial-14210` |
| **Linux** | `/dev/ttyUSB` | `/dev/ttyUSB0`, `/dev/ttyUSB1` |

> **Mẹo:** Nếu không biết cổng nào là thiết bị của bạn: ghi nhớ danh sách cổng → cắm thiết bị → mở dropdown lại → cổng **mới xuất hiện** chính là cổng của thiết bị!

### Bước 3: Cấu hình Baud Rate

**Baud Rate** là **tốc độ truyền dữ liệu**, đo bằng **bits per second (bps)**.

> **Tương tự như:** Tốc độ xe. Nếu bạn lái 60 km/h nhưng đường quy định 100 km/h, sẽ có vấn đề. Tương tự, baud rate của máy tính phải **khớp với thiết bị**!

**Các tốc độ phổ biến:**

| Tốc độ | Sử dụng | Ghi chú |
|---|---|---|
| **9600** | Arduino cũ, cảm biến | Tốc độ **mặc định** của Arduino Uno |
| **115200** | Arduino mới, ESP32 | Tốc độ **hiện đại**, phổ biến nhất |
| **19200, 38400** | Thiết bị công nghiệp | Ít dùng hơn |
| **230400, 460800** | Truyền dữ liệu lớn | Rất hiếm |

**Cách cấu hình:**
1. Mở dropdown **"Baud Rate"**
2. Chọn tốc độ
3. **Quan trọng:** Tốc độ phải **trùng khớp với thiết bị**

**Làm thế nào để biết baud rate của thiết bị?**

- **Nếu bạn viết code:** Tìm dòng `Serial.begin()` trong code Arduino
  ```cpp
  Serial.begin(9600);    // Baud rate là 9600
  ```

- **Nếu dùng thiết bị có sẵn:** Xem tài liệu hoặc datasheet

- **Nếu không chắc:** Thử `9600` hoặc `115200` (hai tốc độ phổ biến nhất)

### Bước 4: Cấu hình Data Bits, Stop Bits, Parity

Sau khi cấu hình baud rate, bạn có thể cấu hình các tham số khác:

#### Data Bits (Số Bit Dữ liệu)
Số bit dùng để mã hóa mỗi ký tự:
- **8 bits**: Phổ biến nhất (99% các trường hợp)
- **7 bits**: Hiếm, chỉ thiết bị cũ
- **5, 6 bits**: Rất hiếm

> **Khuyến cáo:** Chọn **8 bits** nếu không chắc chắn.

#### Stop Bits (Bit Dừng)
Số bit báo hiệu kết thúc ký tự:
- **1 bit**: Phổ biến nhất (99% các trường hợp)
- **1.5, 2 bits**: Hiếm, chỉ thiết bị cũ

> **Khuyến cáo:** Chọn **1 bit** nếu không chắc chắn.

#### Parity (Kiểm tra Chẵn Lẻ)
Dùng để **phát hiện lỗi** khi truyền dữ liệu:
- **None**: KHÔNG kiểm tra (phổ biến nhất, 99% các trường hợp)
- **Even**: Kiểm tra để tổng bit "1" là chẵn
- **Odd**: Kiểm tra để tổng bit "1" là lẻ

> **Khuyến cáo:** Chọn **None** nếu không chắc chắn.

**Cấu hình tiêu chuẩn cho 99% thiết bị:**

```
Baud Rate: 115200 (hoặc 9600)
Data Bits: 8
Stop Bits: 1
Parity: None
```

### Bước 5: Kết nối

**Khi đã cấu hình xong:**

1. Nhấp nút **"Kết nối"** (Connect) ở sidebar
2. **Dấu hiệu thành công:**
   - Nút thay đổi thành **"Ngắt kết nối"** (Disconnect)
   - Header hiển thị trạng thái **xanh** (Connected)
   - Terminal có thể bắt đầu nhận dữ liệu

3. **Nếu lỗi:**
   - Kiểm tra lại baud rate
   - Kiểm tra cáp USB
   - Thử cổng USB khác
   - Khởi động lại ứng dụng

---

## Tín hiệu Điều khiển

### DTR và RTS là gì?

**DTR (Data Terminal Ready)** và **RTS (Request To Send)** là những **tín hiệu điều khiển** trong giao tiếp Serial. Chúng không phải là dữ liệu mà là những **dây điều khiển** dùng để quản lý luồng truyền.

> **Tương tự như:** Nếu dữ liệu TX/RX là **dây nói chuyện**, thì DTR/RTS là **dây bắt tay** giữa hai bên.

### DTR (Data Terminal Ready)

**Ý nghĩa:**
- Báo hiệu máy tính **sẵn sàng** để giao tiếp
- Khi DTR **cao (1)**: Máy tính sẵn sàng
- Khi DTR **thấp (0)**: Máy tính không sẵn sàng

**Sử dụng thực tế:**
- **Arduino**: Khi bạn mở Serial Port từ Arduino IDE, DTR được **kéo thấp** để **reset board** (khởi động lại code)
- **ESP32**: Một số board sử dụng DTR để chuyển vào chế độ download firmware
- **Thiết bị công nghiệp**: Sử dụng để kiểm tra xem máy tính còn sống hay không

### RTS (Request To Send)

**Ý nghĩa:**
- Báo hiệu máy tính **muốn gửi dữ liệu**
- Khi RTS **cao (1)**: Máy tính muốn gửi
- Khi RTS **thấp (0)**: Máy tính không gửi

**Sử dụng thực tế:**
- **Điều khiển luồng**: Thiết bị có thể nói với máy tính "đợi, tôi bận"
- **RS485 networks**: Sử dụng RTS để chuyển giữa chế độ gửi/nhận

### Ảnh hưởng đến ứng dụng của bạn

**Trong hầu hết trường hợp, bạn không cần lo lắng về DTR/RTS:**

- TermiPro **tự động quản lý** những tín hiệu này
- Arduino IDE cũng **tự động quản lý**
- Chỉ những **ứng dụng đặc biệt** mới cần tuỳ chỉnh DTR/RTS

**Khi nào cần chú ý?**

1. **Board Arduino cũ**: Một số board tự động reset khi DTR thay đổi (đây là tính năng!)
2. **Firmware upload**: Một số board dùng DTR/RTS để vào chế độ bootloader
3. **Giao thức đặc biệt**: Các giao thức RS485 hoặc xác nhận luồng (flow control)

> **Lưu ý:** TermiPro xử lý DTR/RTS tự động. Nếu bạn gặp vấn đề reset không mong muốn, hãy kiểm tra tài liệu board của bạn.

---

## Gửi Dữ liệu

### Text Mode (Chế độ Văn bản)

**Text Mode** là chế độ gửi dữ liệu ở dạng **ký tự bình thường**.

#### Cách sử dụng

1. **Chắc chắn đã kết nối** (nút kết nối phải là "Ngắt kết nối")
2. **Nhập dữ liệu** vào ô input ở dưới cùng
3. **Nhấn "Gửi"** hoặc bấm **Enter**

#### Ví dụ

| Bạn nhập | Thiết bị nhận |
|---|---|
| `Hello` | `Hello` + ký tự xuống dòng |
| `AT` | `AT` + ký tự xuống dòng |
| `START` | `START` + ký tự xuống dòng |

#### Ký tự Xuống dòng (Line Ending)

> **Quan trọng:** TermiPro **tự động thêm ký tự xuống dòng** vào cuối mỗi tin nhắn trong Text mode.

**Các loại ký tự xuống dòng:**

| Loại | Ký hiệu | Ví dụ | Sử dụng |
|---|---|---|---|
| **None** | (không có) | `Hello` | Hiếm, chỉ thiết bị đặc biệt |
| **LF** | `\n` | `Hello\n` | Phổ biến trên Linux, Arduino |
| **CR** | `\r` | `Hello\r` | Hiếm |
| **CRLF** | `\r\n` | `Hello\r\n` | Windows, một số thiết bị |

**TermiPro thường sử dụng CRLF hoặc LF** (tùy hệ điều hành).

**Nếu thiết bị của bạn yêu cầu kiểu xuống dòng khác:**
- Sử dụng **Hex Mode** để gửi chính xác
- Ví dụ: `48 65 6C 6C 6F` (Hello mà không có xuống dòng)

---

### Hex Mode (Chế độ Thập Lục Phân)

**Hex Mode** cho phép bạn gửi dữ liệu dưới dạng **byte thập lục phân**, hữu ích khi:
- Gửi **dữ liệu nhị phân** (không phải ký tự)
- Gửi **lệnh điều khiển** với byte cụ thể
- Làm việc với **giao thức nhị phân**

#### Định dạng Hex

Nhập các byte thập lục phân **cách nhau bằng khoảng trắng**:

```
48 65 6C 6C 6F
```

Tương đương:
- `48` hex = 72 decimal = ký tự 'H'
- `65` hex = 101 decimal = ký tự 'e'
- `6C` hex = 108 decimal = ký tự 'l'
- `6C` hex = 108 decimal = ký tự 'l'
- `6F` hex = 111 decimal = ký tự 'o'

**Kết quả:** Gửi từ "Hello" nhưng **không có ký tự xuống dòng** thêm vào.

#### Cách sử dụng

1. **Chắc chắn đã kết nối**
2. **Chuyển sang Hex Mode** (nhấp nút Hex ở phần footer)
3. **Nhập dữ liệu hex** với byte cách nhau bằng khoảng trắng
   - Ví dụ: `01 02 03 FF 00`
4. **Nhấn "Gửi"** hoặc **Enter**

#### Ví dụ thực tế

| Bạn nhập | Kết quả gửi | Mô tả |
|---|---|---|
| `48 65 6C 6C 6F` | 5 byte | Từ "Hello" ở dạng hex |
| `01 02 03` | 3 byte | Lệnh điều khiển tùy chỉnh |
| `FF 00` | 2 byte: 255, 0 | Reset lệnh |

#### Định dạng Hex được hỗ trợ

Tất cả các định dạng dưới đây đều hoạt động:

```
48 65 6C 6C 6F          (cách nhau bằng khoảng trắng) ✓
0x48 0x65 0x6C 0x6C 0x6F   (với tiền tố 0x) ✓
0x48 65 0x6C 6C 0x6F    (hỗn hợp) ✓
486c656c6f              (KHÔNG cách) ✗
```

> **Mẹo:** Sử dụng **cách nhau bằng khoảng trắng** để dễ đọc nhất.

---

### Lựa chọn Line Ending (Ký tự Xuống dòng)

**Trong Text Mode**, TermiPro có thể cấu hình loại ký tự xuống dòng:

| Lựa chọn | Ý nghĩa | Sử dụng |
|---|---|---|
| **None** | Không thêm gì | Thiết bị không yêu cầu xuống dòng |
| **CR** | Thêm `\r` | Thiết bị cũ, một số máy fax/modem |
| **LF** | Thêm `\n` | Arduino, Linux, phổ biến |
| **CRLF** | Thêm `\r\n` | Windows, thường an toàn nhất |

**Cách cấu hình:**
1. Mở **Settings** hoặc **Preferences** (nếu có)
2. Tìm **"Line Ending"** hoặc **"Xuống dòng"**
3. Chọn loại mong muốn

> **Khuyến cáo:** Nếu không chắc, chọn **LF** (phổ biến cho Arduino)

---

## Auto Send

**Auto Send** cho phép bạn **gửi dữ liệu tự động, lặp lại** theo chu kỳ.

### Khi nào dùng Auto Send?

- **Gửi lênh kiểm tra liên tục**: Ví dụ gửi "ping" mỗi 100ms
- **Thu thập dữ liệu**: Gửi lệnh "read_sensor" mỗi 1 giây
- **Stress test**: Gửi dữ liệu liên tục để kiểm tra thiết bị
- **Chờ phản hồi**: Gửi lệnh và chờ kết quả

### Cách sử dụng

**Bước 1: Chuẩn bị dữ liệu**
1. Nhập dữ liệu muốn gửi vào ô input
2. Ví dụ: `Get Status` hoặc `01 02 03` (Hex mode)

**Bước 2: Cấu hình Interval**
1. Tìm phần **"Auto Send"** ở sidebar
2. Nhập **khoảng thời gian** giữa các lần gửi
3. Đơn vị: **miliseconds (ms)**
   - `100` = gửi mỗi 100ms (10 lần/giây)
   - `1000` = gửi mỗi 1 giây
   - `5000` = gửi mỗi 5 giây

**Bước 3: Bắt đầu**
1. Nhấp nút **"Start"** (hoặc tương tự)
2. Dữ liệu sẽ bắt đầu gửi lặp lại

**Bước 4: Dừng**
1. Nhấp nút **"Stop"** để dừng
2. Counter sẽ hiển thị **số lần đã gửi**

### Ví dụ thực tế

**Ví dụ 1: Giám sát cảm biến**

```
Dữ liệu: "READ_SENSOR"
Interval: 1000ms (1 giây)
Kết quả: Mỗi giây gửi "READ_SENSOR", thiết bị gửi lại dữ liệu cảm biến
```

**Ví dụ 2: Kiểm tra kết nối**

```
Dữ liệu: "PING"
Interval: 500ms
Kết quả: Mỗi 500ms gửi "PING", nếu thiết bị sống sẽ phản hồi "PONG"
```

**Ví dụ 3: Stress test**

```
Dữ liệu: "01 02 03 04 05" (Hex mode)
Interval: 50ms (tốc độ cao)
Kết quả: Gửi liên tục để kiểm tra thiết bị có chịu được không
```

### Tips sử dụng Auto Send

**Tip 1: Chọn Interval hợp lý**
- Quá nhanh (< 50ms): Có thể làm thiết bị quá tải
- Bình thường (100ms - 5s): Hợp lý cho hầu hết trường hợp
- Quá chậm (> 10s): Chỉ dùng khi cần giám sát lâu dài

**Tip 2: Xem Counter**
- Counter hiển thị số lần đã gửi
- Giúp bạn biết Auto Send đang hoạt động

**Tip 3: Kết hợp với Terminal**
- Xem dữ liệu TX (gửi) và RX (nhận) trong terminal
- Giúp debug nếu có vấn đề

**Tip 4: Dừng trước khi ngắt kết nối**
- Luôn nhấp "Stop" trước khi ngắt kết nối
- Tránh dữ liệu bị rối

---

## Terminal

**Terminal** là phần **hiển thị dữ liệu** gửi/nhận thời gian thực.

### Hiểu về TX và RX

| Ký hiệu | Tên đầy đủ | Ý nghĩa | Màu |
|---|---|---|---|
| **TX** | **Transmit** | Dữ liệu **bạn gửi** | Vàng |
| **RX** | **Receive** | Dữ liệu **thiết bị gửi** | Xanh |

**Ví dụ:**

```
[TX] Hello
[RX] Hello World
[TX] Get Status
[RX] OK
```

- Dòng 1: Bạn gửi "Hello" (vàng)
- Dòng 2: Thiết bị phản hồi "Hello World" (xanh)
- Dòng 3: Bạn gửi "Get Status" (vàng)
- Dòng 4: Thiết bị phản hồi "OK" (xanh)

### Thống kê TX/RX

Terminal hiển thị **số lượng gói tin**:

```
TX: 5 packets
RX: 5 packets
```

Giúp bạn:
- Kiểm tra xem dữ liệu có được gửi không
- Kiểm tra xem thiết bị có phản hồi không
- Phát hiện nếu một bên không gửi gì

### Chế độ hiển thị

#### Text Mode

Hiển thị dữ liệu dưới dạng **ký tự bình thường**:

```
[TX] Hello World
[RX] OK
[RX] Temperature: 25.5°C
```

**Lợi ích:**
- Dễ đọc
- Hiểu ngay nội dung

**Vấn đề:**
- Dữ liệu nhị phân (byte không thể hiển thị) sẽ bị **lỗi** hoặc **mất**
- Ký tự kiểm soát (control characters) sẽ hiển thị lạ

#### Hex Mode

Hiển thị dữ liệu dưới dạng **byte thập lục phân**:

```
[TX] 48 65 6C 6C 6F
[RX] 4F 4B
```

**Lợi ích:**
- Thấy **chính xác mỗi byte**
- Dữ liệu nhị phân hiển thị rõ ràng
- Dễ debug giao thức nhị phân

**Vấn đề:**
- Khó đọc nếu chỉ là text thường

### Cách chuyển đổi chế độ hiển thị

1. Tìm nút **"Text"** hoặc **"Hex"** ở phía trên terminal
2. Nhấp để chuyển đổi
3. Terminal sẽ cập nhật ngay lập tức

> **Lưu ý:** Chuyển đổi chế độ hiển thị **không ảnh hưởng** đến dữ liệu gửi/nhận. Chỉ thay đổi cách hiển thị mà thôi.

### Auto Scroll

**Auto Scroll** tự động **cuộn xuống** để bạn thấy dữ liệu mới nhất.

**Cách sử dụng:**
- Tìm nút **"Auto Scroll"** hoặc checkbox
- Bật để tự động cuộn xuống
- Tắt nếu bạn muốn xem dữ liệu cũ

**Lợi ích:**
- Luôn nhìn thấy dữ liệu mới
- Không phải cuộn tay

**Vấn đề:**
- Nếu dữ liệu quá nhiều, có thể làm chậm ứng dụng

### Clear Terminal

**Clear** xóa **tất cả dữ liệu** trên terminal.

**Cách sử dụng:**
1. Tìm nút **"Clear"**
2. Nhấp để xóa

**Khi dùng:**
- Terminal có quá nhiều dữ liệu cũ
- Muốn bắt đầu ghi log mới
- Thiết bị bị lỗi và gửi dữ liệu rác

> **Cảnh báo:** Clear **không thể hoàn tác**. Dữ liệu cũ sẽ mất vĩnh viễn.

### Giới hạn dữ liệu

Terminal có **giới hạn số dòng** để tránh chương trình bị chậm:

- Thường là **1000-5000 dòng**
- Khi vượt quá, dòng **cũ nhất sẽ bị xóa**
- Nếu cần lưu dữ liệu, hãy **export** (nếu tính năng có)

---

## Mẹo Sử dụng

### Mẹo 1: Arduino (Xác minh kết nối)

**Bạn muốn:** Kiểm tra Arduino có hoạt động không

**Cách làm:**
1. Cài đặt code Arduino:
   ```cpp
   void setup() {
     Serial.begin(115200);
   }
   void loop() {
     Serial.println("Arduino is alive");
     delay(1000);
   }
   ```

2. Mở TermiPro:
   - Chọn cổng (COM3, /dev/ttyUSB0, v.v.)
   - Chọn baud rate **115200**
   - Kết nối

3. **Kết quả:** Mỗi giây sẽ thấy "Arduino is alive"

### Mẹo 2: ESP32 (Xem debug message)

**Bạn muốn:** Xem thông báo debug khi ESP32 chạy

**Cách làm:**
1. Cài đặt code ESP32:
   ```cpp
   void setup() {
     Serial.begin(115200);
     Serial.println("ESP32 Started");
   }
   ```

2. Mở TermiPro:
   - Chọn cổng
   - Chọn baud rate **115200**
   - Bật **Auto Scroll**
   - Kết nối

3. **Kết quả:** Xem tất cả debug message từ ESP32

### Mẹo 3: Giao tiếp hai chiều (Request-Response)

**Bạn muốn:** Gửi lệnh và chờ phản hồi

**Cách làm:**
1. Cài đặt code thiết bị để lắng nghe và phản hồi:
   ```cpp
   void loop() {
     if (Serial.available()) {
       String cmd = Serial.readStringUntil('\n');
       if (cmd == "ON") {
         Serial.println("TURNED ON");
         // Bật LED
       } else if (cmd == "OFF") {
         Serial.println("TURNED OFF");
         // Tắt LED
       }
     }
   }
   ```

2. Mở TermiPro:
   - Kết nối
   - Nhập `ON` → gửi → thấy "TURNED ON"
   - Nhập `OFF` → gửi → thấy "TURNED OFF"

3. **Kết quả:** Điều khiển thiết bị từ TermiPro

### Mẹo 4: Gửi lệnh hex cho module GSM/GPRS

**Bạn muốn:** Gửi lệnh AT (điều khiển modem/GSM)

**Cách làm:**
1. Chuyển sang **Hex Mode**
2. Gửi lệnh AT (ví dụ: kiểm tra kết nối):
   - Tìm mã hex của "AT\r\n"
   - AT = `41 54`
   - \r = `0D`
   - \n = `0A`
   - Gửi: `41 54 0D 0A`

3. **Kết quả:** Module sẽ phản hồi `OK`

> **Lưu ý:** Hầu hết module AT cũng hiểu Text mode, nên chỉ cần gõ "AT" và gửi là đủ.

### Mẹo 5: Monitoring dữ liệu cảm biến với Auto Send

**Bạn muốn:** Đọc cảm biến mỗi giây

**Cách làm:**
1. Cài đặt code để gửi dữ liệu khi nhận lệnh "READ":
   ```cpp
   void loop() {
     if (Serial.available()) {
       String cmd = Serial.readStringUntil('\n');
       if (cmd == "READ") {
         int sensorValue = analogRead(A0);
         Serial.println(sensorValue);
       }
     }
   }
   ```

2. Mở TermiPro:
   - Nhập `READ` vào ô input
   - Mở **Auto Send**
   - Chọn **Interval: 1000ms** (1 giây)
   - Nhấp **Start**

3. **Kết quả:** Mỗi giây gửi "READ", cảm biến trả về giá trị

### Mẹo 6: Ghi lại dữ liệu để phân tích

**Bạn muốn:** Lưu tất cả dữ liệu để phân tích sau

**Cách làm:**
1. Kết nối và để terminal chạy (dữ liệu sẽ được hiển thị)
2. Nếu TermiPro có tính năng **"Export"** hoặc **"Save Log"**:
   - Nhấp **"Export"**
   - Chọn file để lưu
   - Dữ liệu sẽ được lưu dưới dạng text hoặc CSV

3. Nếu không có tính năng này:
   - Dùng **Select All** (`Ctrl+A`) → **Copy** (`Ctrl+C`)
   - Dán vào file text hoặc Excel

> **Mẹo:** Nếu bạn muốn phân tích dữ liệu trong Excel, hãy sao chép từ terminal và dán vào spreadsheet. Các dòng TX/RX sẽ được phân tách rõ.

### Mẹo 7: Debug khi dữ liệu lỗi

**Bạn muốn:** Xem chính xác byte nào bị lỗi

**Cách làm:**
1. Chuyển terminal sang **Hex Mode**
2. Gửi dữ liệu test (ví dụ: "Hello")
3. Xem byte hex thực tế được gửi/nhận
4. So sánh với mong đợi
5. Tìm ra byte nào khác

**Ví dụ:**
- Mong đợi: `48 65 6C 6C 6F` (Hello)
- Thực tế: `48 65 6C 6C 6F 0A` (Hello + newline)
- **Kết luận:** TermiPro đã thêm `0A` (newline)

### Mẹo 8: Kiểm tra baud rate sai

**Bạn muốn:** Xác định baud rate thiết bị nếu không biết

**Cách làm:**
1. Kế nối thiết bị ở baud rate **115200** (phổ biến)
2. Nhập dữ liệu test "TEST"
3. Xem kết quả:
   - **Dữ liệu bình thường:** Baud rate đúng
   - **Dữ liệu rác (ký tự lạ):** Baud rate sai
4. Nếu sai, thử baud rate khác (9600, 19200, v.v.)

**Dấu hiệu baud rate sai:**
```
Bạn gửi: TEST
Nhận được: ÊŤĘÜ (hoặc ký tự lạ khác)
```

### Mẹo 9: Kiểm tra kết nối bằng echo

**Bạn muốn:** Kiểm tra kết nối có tốt không

**Cách làm:**
1. Cài đặt code echo trên thiết bị:
   ```cpp
   void loop() {
     if (Serial.available()) {
       char c = Serial.read();
       Serial.write(c);  // Gửi lại ký tự
     }
   }
   ```

2. Mở TermiPro:
   - Gửi "Hello"
   - Nếu thấy "Hello" phản hồi lại → kết nối OK
   - Nếu không thấy gì → có vấn đề

### Mẹo 10: Sử dụng Terminal Monitor thay vì Serial Plotter

**Khi nào dùng Serial Monitor (TermiPro):**
- Gửi/nhận lệnh text
- Xem debug message
- Giao tiếp hai chiều

**Khi nào dùng Serial Plotter:**
- Hiển thị biểu đồ dữ liệu số
- Theo dõi giá trị cảm biến theo thời gian

> **Lưu ý:** TermiPro là **Serial Monitor**, không phải Serial Plotter. Để vẽ biểu đồ, bạn cần công cụ khác.

---

## Những Lỗi Thường Gặp

### Lỗi 1: "Dữ liệu được nhận nhưng bị lỗi"

**Nguyên nhân:** Baud rate không khớp

**Giải pháp:**
1. Kiểm tra baud rate thiết bị
2. Thử các baud rate phổ biến: 9600, 115200
3. Xem dữ liệu ở Hex Mode để tìm ra quy luật

### Lỗi 2: "Thiết bị không phản hồi"

**Nguyên nhân:** Thiết bị không gửi dữ liệu

**Giải pháp:**
1. Kiểm tra code thiết bị có gửi dữ liệu không
2. Kiểm tra thiết bị có được cấp điện không
3. Thử thiết bị khác để xác nhận

### Lỗi 3: "Dữ liệu gửi không được"

**Nguyên nhân:** Chưa kết nối hoặc dữ liệu trống

**Giải pháp:**
1. Kiểm tra nút kết nối có hiển thị "Ngắt kết nối" không
2. Nhập dữ liệu vào ô input
3. Nhấn Gửi

### Lỗi 4: "Terminal hiển thị dữ liệu rác"

**Nguyên nhân:** Baud rate sai hoặc kết nối bị nhiễu

**Giải pháp:**
1. Kiểm tra baud rate
2. Thử cáp USB khác
3. Thử cổng USB khác

---

## Tóm Tắt

| Tính năng | Mô tả |
|---|---|
| **Baud Rate** | Tốc độ truyền - phải khớp với thiết bị (9600, 115200 phổ biến) |
| **Data Bits** | Số bit dữ liệu (8 là mặc định) |
| **Stop Bits** | Bit dừng (1 là mặc định) |
| **Parity** | Kiểm tra lỗi (None là mặc định) |
| **Text Mode** | Gửi văn bản thường, thêm xuống dòng |
| **Hex Mode** | Gửi byte hex, không thêm gì |
| **TX (Transmit)** | Dữ liệu bạn gửi |
| **RX (Receive)** | Dữ liệu thiết bị gửi |
| **Auto Send** | Gửi dữ liệu lặp lại theo chu kỳ |
| **DTR/RTS** | Tín hiệu điều khiển (TermiPro tự động quản lý) |

---

## Liên Hệ Hỗ Trợ

Nếu gặp vấn đề:

1. **Kiểm tra tài liệu thiết bị:** Luôn tìm baud rate, pinout, giao thức
2. **Xem phần "Mẹo Sử dụng":** Có thể là use case giống bạn
3. **Kiểm tra GitHub Issues:** Xem nếu ai khác gặp vấn đề tương tự
4. **Thử lại từ đầu:** Ngắt kết nối → kết nối lại → test dữ liệu

---

**Cập nhật lần cuối: 28/12/2025**

*Chúc bạn sử dụng TermiPro vui vẻ!*
