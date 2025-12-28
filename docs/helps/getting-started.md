# Bắt đầu sử dụng TermiPro

Chào mừng bạn đến với **TermiPro** - công cụ giao tiếp chuyên nghiệp cho các hệ thống nhúng và IoT. Hướng dẫn này sẽ giúp bạn làm quen với ứng dụng một cách dễ dàng và nhanh chóng.

## Mục lục

1. [TermiPro là gì?](#termipro-là-gì)
2. [Các tính năng chính](#các-tính-năng-chính)
3. [Giao diện tổng quan](#giao-diện-tổng-quan)
4. [Quản lý Tab](#quản-lý-tab)
5. [Bắt đầu nhanh](#bắt-đầu-nhanh)
6. [Các giao thức được hỗ trợ](#các-giao-thức-được-hỗ-trợ)
7. [Liên kết hữu ích](#liên-kết-hữu-ích)

---

## TermiPro là gì?

**TermiPro** là ứng dụng desktop chuyên nghiệp để giao tiếp với các hệ thống nhúng (embedded systems) và thiết bị IoT. Ứng dụng hỗ trợ nhiều giao thức truyền thông khác nhau, cho phép bạn:

- Giao tiếp với microcontroller (Arduino, STM32, ESP32)
- Kết nối TCP/IP với các thiết bị mạng
- Sử dụng giao thức Modbus cho các ứng dụng công nghiệp
- Tương tác với các thiết bị MQTT
- Theo dõi dữ liệu real-time
- Gửi dữ liệu dạng Text hoặc Hex

### Ai nên dùng TermiPro?

- **Kỹ sư embedded systems** - Lập trình và kiểm tra microcontroller
- **Kỹ sư IoT** - Giao tiếp với các thiết bị IoT
- **Lập trình viên** - Phát triển và debug các ứng dụng serial/network
- **Kỹ sư điều khiển** - Làm việc với các thiết bị Modbus công nghiệp
- **Nhà phát triển** - Kiểm tra và theo dõi các kết nối thiết bị

---

## Các tính năng chính

TermiPro hỗ trợ **6 giao thức truyền thông** khác nhau:

### 1. Serial Port (Cổng Serial)
- Giao tiếp trực tiếp với thiết bị qua USB/Serial
- Cấu hình Baud Rate, Data Bits, Stop Bits, Parity
- Lý tưởng cho Arduino, STM32, PLC

### 2. TCP Client
- Kết nối đến máy chủ TCP từ xa
- Gửi/nhận dữ liệu thông qua mạng
- Phù hợp cho các thiết bị network có sẵn

### 3. TCP Server
- Lắng nghe kết nối TCP từ các client khác
- Quản lý nhiều kết nối đồng thời
- Dùng cho các ứng dụng cơ sở dữ liệu tập trung

### 4. Modbus Master
- Giao tiếp với các thiết bị Modbus RTU/TCP
- Đọc/ghi thanh ghi từ thiết bị công nghiệp
- Hỗ trợ đầy đủ các function code Modbus

### 5. Modbus Slave
- Mô phỏng thiết bị Modbus
- Sử dụng để kiểm tra các ứng dụng Modbus Master
- Hỗ trợ RTU/TCP

### 6. MQTT Client
- Kết nối đến broker MQTT
- Publish/Subscribe các topic
- Lý tưởng cho các ứng dụng IoT cloud

---

## Giao diện tổng quan

TermiPro được chia thành các phần chính sau:

```
┌─────────────────────────────────────────────────┐
│  Header - Logo, Tên app, Trạng thái            │
├─────────────┬─────────────────────────────────┤
│             │                                 │
│  Tab Bar    │  Nội dung Tab hiện tại         │
│  (6 tab)    │  (Serial, TCP, Modbus, MQTT)   │
│             │                                 │
├─────────────┴─────────────────────────────────┤
│  Footer - Input, Nút gửi, Tùy chọn           │
└─────────────────────────────────────────────────┘
```

### Header (Tiêu đề - Phía trên)
- **Logo TermiPro**: Nhận dạng ứng dụng
- **Tên ứng dụng**: "TermiPro - Professional Serial Monitor"
- **Trạng thái kết nối**: Hiển thị số lượng kết nối đang hoạt động

### Tab Bar (Thanh tab - Bên trái/trên)
- **Tạo Tab mới**: Nhấp nút "+" để tạo tab cho giao thức mới
- **Quản lý Tab**: Hiển thị danh sách các tab đang mở
- **Đóng Tab**: Nhấp X trên tab để đóng kết nối

### Nội dung Tab (Khu vực chính)
Thay đổi theo loại giao thức được chọn:
- **Cấu hình kết nối**: Nhập hostname, port, baud rate, v.v.
- **Nút Kết nối/Ngắt**: Bắt đầu hoặc dừng kết nối
- **Terminal hiển thị**: Xem dữ liệu nhận/gửi real-time
- **Gửi dữ liệu**: Nhập và gửi dữ liệu đến thiết bị

### Footer (Chân trang - Phía dưới)
- **Ô nhập liệu**: Nhập dữ liệu muốn gửi
- **Nút Gửi (Send)**: Gửi dữ liệu
- **Chế độ Text/Hex**: Chọn định dạng dữ liệu
- **Tùy chọn**: Clear, Auto Scroll, v.v.

---

## Quản lý Tab

TermiPro cho phép bạn mở nhiều kết nối cùng lúc bằng cách sử dụng **Tab**.

### Tạo Tab mới

1. Nhấp nút **"+"** trên thanh tab (bên trái/trên)
2. Cửa sổ "Tab mới" sẽ mở lên
3. Chọn giao thức (Serial, TCP Client, TCP Server, Modbus, MQTT)
4. Nhập tên cho tab (tùy chọn)
5. Nhấp **"Tạo"** để tạo tab mới

### Chuyển đổi giữa các Tab

- Nhấp trên tên tab muốn chuyển đến
- Nội dung sẽ cập nhật để hiển thị tab được chọn
- Mỗi tab duy trì cấu hình riêng

### Đóng Tab

1. Nhấp nút **X** ở góc phải của tab
2. Nếu tab có kết nối đang hoạt động, cửa sổ xác nhận sẽ xuất hiện
3. Nhấp **"Có"** để ngắt kết nối và đóng tab
4. Nhấp **"Không"** để hủy

### Giới hạn Tab

- Tối đa **10 tab** có thể mở cùng lúc
- Nếu đã đạt giới hạn, vui lòng đóng một số tab trước khi tạo mới

---

## Bắt đầu nhanh

Dưới đây là hướng dẫn từng bước để kết nối với Arduino qua Serial Port.

### Chuẩn bị

1. **Chuẩn bị thiết bị**
   - Đảm bảo Arduino được cấp nguồn
   - Arduino chạy chương trình serial (có dòng lệnh Serial.begin())

2. **Kết nối USB**
   - Kết nối Arduino với máy tính bằng cáp USB
   - Chờ 2-3 giây để hệ thống nhận biết thiết bị

3. **Mở TermiPro**
   - Khởi động ứng dụng TermiPro
   - Chờ giao diện tải xong

### Các bước thực hiện

**Bước 1: Chọn giao thức Serial**
- Nếu đây là lần đầu, hãy tạo tab mới bằng nút "+"
- Chọn "Serial Port" từ danh sách giao thức
- Nhấp "Tạo"

**Bước 2: Chọn cổng Serial**
- Tìm phần **"Chọn cổng"** ở Sidebar
- Nhấp vào dropdown
- Chọn cổng mà Arduino của bạn sử dụng:
  - **Windows**: Thường là `COM3`, `COM4`, v.v.
  - **macOS/Linux**: Thường là `/dev/ttyUSB0`, `/dev/ttyACM0`, v.v.

**Bước 3: Cấu hình Baud Rate**
- Tìm phần **"Baud Rate"**
- Chọn baud rate **giống với Arduino của bạn**
  - Nếu không chắc, thử **9600** hoặc **115200** (phổ biến nhất)
  - Kiểm tra code Arduino để biết giá trị chính xác

**Bước 4: Cấu hình các tham số khác (tuỳ chọn)**
- **Data Bits**: Thường để **8** (mặc định)
- **Stop Bits**: Thường để **1** (mặc định)
- **Parity**: Thường để **None** (mặc định)

**Bước 5: Kết nối**
- Nhấp nút **"Kết nối"** (Connect)
- Nếu thành công:
  - Nút sẽ chuyển thành **"Ngắt kết nối"**
  - Terminal sẽ hiển thị dữ liệu từ Arduino
  - Header sẽ hiển thị trạng thái kết nối

**Bước 6: Xem dữ liệu**
- Dữ liệu từ Arduino sẽ xuất hiện trên terminal
- Có badge **RX** (xanh) để phân biệt dữ liệu nhận

**Bước 7: Gửi dữ liệu**
- Nhập dữ liệu vào ô nhập liệu ở footer
- Chọn chế độ **Text** hoặc **Hex**
- Nhấp **"Gửi"** hoặc bấm Enter
- Dữ liệu sẽ xuất hiện trên terminal với badge **TX** (vàng)

### Mẹo cho lần đầu

- Nếu không thấy dữ liệu, hãy kiểm tra Baud Rate
- Hãy có cấu hình của Arduino sẵn để tra cứu
- Nếu không kết nối được, hãy thử kết nối lại USB

---

## Các giao thức được hỗ trợ

### Serial Port

**Dùng cho**: Arduino, STM32, PLC, các bo mạch phát triển

**Cấu hình chính**:
- **Port**: Chọn từ danh sách các cổng USB
- **Baud Rate**: 300 - 921.600 bps
- **Data Bits**: 5, 6, 7, 8
- **Stop Bits**: 1, 1.5, 2
- **Parity**: None, Odd, Even

**Ví dụ**:
```
Arduino UNO: COM3, 9600 bps, 8 data bits, 1 stop bit, No parity
STM32: /dev/ttyUSB0, 115200 bps, 8 data bits, 1 stop bit, No parity
```

Xem chi tiết: [Hướng dẫn Serial Connection](serial-connection.md)

### TCP Client

**Dùng cho**: Kết nối đến các server TCP từ xa

**Cấu hình chính**:
- **Hostname/IP**: Địa chỉ server (ví dụ: `192.168.1.100`)
- **Port**: Cổng server (ví dụ: `8888`)
- **Mode**: Raw socket hoặc có định dạng riêng

**Ví dụ**:
```
Server IP: 192.168.1.50
Server Port: 9000
```

### TCP Server

**Dùng để**: Lắng nghe và chấp nhận kết nối từ các client khác

**Cấu hình chính**:
- **Listening Port**: Cổng mà server sẽ lắng nghe
- **Max Clients**: Số lượng client tối đa có thể kết nối

**Ví dụ**:
```
Listening Port: 9000
Max Clients: 5
```

### Modbus Master

**Dùng cho**: Giao tiếp với các thiết bị Modbus trong công nghiệp

**Cấu hình chính**:
- **Connection Type**: RTU (qua Serial) hoặc TCP
- **Slave ID**: ID của thiết bị Modbus
- **Baud Rate** (nếu RTU): 9600, 19200, 38400 v.v.
- **Function Code**: Đọc/ghi Coil, Input/Holding Register

**Ví dụ**:
```
Device: RTU slave ID 1
Baud Rate: 9600
Read Holding Register at address 0
```

Xem chi tiết: [Hướng dẫn Modbus](modbus.md)

### Modbus Slave

**Dùng để**: Mô phỏng một thiết bị Modbus cho mục đích kiểm tra

**Cấu hình chính**:
- **Slave ID**: ID của thiết bị mô phỏng
- **Connection Type**: RTU hoặc TCP
- **Registers**: Giá trị của các thanh ghi

### MQTT Client

**Dùng cho**: Kết nối đến MQTT broker cho ứng dụng IoT

**Cấu hình chính**:
- **Broker Address**: Địa chỉ MQTT broker (ví dụ: `mqtt.example.com`)
- **Port**: Thường là `1883` (MQTT) hoặc `8883` (MQTT over SSL)
- **Username/Password**: Thông tin xác thực (nếu cần)
- **Client ID**: ID duy nhất của client
- **Topics**: Các topic để subscribe hoặc publish

**Ví dụ**:
```
Broker: broker.hivemq.com
Port: 1883
Client ID: TermiPro_Device1
Subscribe: sensor/temperature
Publish: device/status
```

---

## Liên kết hữu ích

Để tìm hiểu thêm chi tiết về từng tính năng:

- **[Serial Connection Guide](serial-connection.md)** - Hướng dẫn chi tiết kết nối cổng Serial
- **[Sending Data Guide](sending-data.md)** - Cách gửi dữ liệu Text/Hex
- **[Auto Send Guide](auto-send.md)** - Cấu hình gửi dữ liệu tự động
- **[Terminal Display Guide](terminal-display.md)** - Tùy chọn hiển thị terminal
- **[Modbus Guide](../modbus.md)** - Hướng dẫn sử dụng Modbus
- **[TCP Client/Server Guide](../tcp-client.md)** - Hướng dẫn sử dụng TCP
- **[FAQ & Troubleshooting](faq-troubleshooting.md)** - Câu hỏi thường gặp và xử lý sự cố

---

## Các bước tiếp theo

Sau khi nắm vững các bước cơ bản:

1. **Khám phá các giao thức khác** - Thử TCP Client hoặc MQTT
2. **Tạo nhiều Tab** - Quản lý nhiều kết nối cùng lúc
3. **Sử dụng Hex Mode** - Gửi dữ liệu nhị phân hoặc các lệnh đặc biệt
4. **Cấu hình Auto Send** - Gửi dữ liệu định kỳ cho các tác vụ lặp lại
5. **Đọc tài liệu chi tiết** - Tìm hiểu thêm về từng giao thức

---

## Mẹo nhanh

- **Lưu cấu hình**: Ghi chú cấu hình các thiết bị bạn thường sử dụng
- **Clear terminal**: Nhấp nút Clear khi có quá nhiều dữ liệu cũ
- **Auto scroll**: Bật auto scroll để không bỏ lỡ dữ liệu mới
- **Chuyển đổi Tab**: Sử dụng Tab để quản lý nhiều thiết bị cùng lúc

---

**Chúc bạn sử dụng TermiPro vui vẻ!**

*Cập nhật lần cuối: 28/12/2025*
