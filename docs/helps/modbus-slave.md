# Hướng dẫn sử dụng Modbus Slave

Chào mừng bạn đến với hướng dẫn **Modbus Slave** của TermiPro. Tính năng này cho phép bạn **mô phỏng một thiết bị Modbus Slave** để kiểm tra và phát triển các hệ thống SCADA, lập trình PLC, hoặc các ứng dụng giao tiếp Modbus.

---

## Mục lục

1. [Giới thiệu](#giới-thiệu)
2. [Chế độ hoạt động](#chế-độ-hoạt-động)
3. [Cấu hình Modbus Slave](#cấu-hình-modbus-slave)
4. [Vùng dữ liệu Modbus](#vùng-dữ-liệu-modbus)
5. [Chỉnh sửa dữ liệu](#chỉnh-sửa-dữ-liệu)
6. [Mô phỏng tự động](#mô-phỏng-tự-động)
7. [Cấu hình Delay phản hồi](#cấu-hình-delay-phản-hồi)
8. [Ánh xạ Exception](#ánh-xạ-exception)
9. [Thống kê và giám sát](#thống-kê-và-giám-sát)
10. [Lưu và tải dữ liệu](#lưu-và-tải-dữ-liệu)
11. [Mẹo sử dụng](#mẹo-sử-dụng)
12. [Xử lý sự cố](#xử-lý-sự-cố)

---

## Giới thiệu

### Modbus Slave là gì?

**Modbus Slave** (Mô tô Slave) là tính năng cho phép TermiPro hoạt động như một thiết bị đầu cuối Modbus. Thay vì **đọc dữ liệu từ thiết bị khác** (như Modbus Master), Modbus Slave **lưu trữ dữ liệu** và **trả lời các yêu cầu** từ các thiết bị khác muốn đọc hoặc ghi dữ liệu.

### Tại sao sử dụng Modbus Slave?

Modbus Slave rất hữu ích trong các tình huống:

| Tình huống | Lợi ích |
|-----------|---------|
| **Phát triển hệ thống SCADA** | Kiểm tra logic SCADA mà không cần thiết bị thật |
| **Lập trình PLC** | Mô phỏng các cảm biến hoặc thiết bị không có sẵn |
| **Kiểm tra giao tiếp Modbus** | Xác minh rằng ứng dụng của bạn gửi/nhận dữ liệu đúng |
| **Đào tạo** | Học về Modbus protocol mà không cần thiết bị đắt tiền |
| **Gỡ lỗi** | Mô phỏng các trường hợp lỗi hoặc ngoại lệ |

### Chế độ Modbus Slave vs Master

- **Modbus Master** (chế độ chính): Ứng dụng **gửi yêu cầu** và **nhận dữ liệu** từ các thiết bị khác
- **Modbus Slave** (chế độ đầu cuối): TermiPro **lưu trữ dữ liệu** và **trả lời các yêu cầu** từ các ứng dụng khác

---

## Chế độ hoạt động

Modbus Slave hỗ trợ 2 chế độ chính:

### 1. RTU Slave (Serial)

**RTU Slave** giao tiếp thông qua **cổng Serial (COM port)**.

| Thông số | Mô tả |
|---------|-------|
| **Cổng Serial** | Cổng USB serial mà bạn muốn sử dụng (VD: `/dev/ttyUSB0`, `COM3`) |
| **Baud Rate** | Tốc độ truyền (9600, 115200, v.v.) |
| **Data Bits** | Số bit dữ liệu (thường là 8) |
| **Stop Bits** | Số bit dừng (thường là 1) |
| **Parity** | Kiểm tra chẵn lẻ (None, Odd, Even) |
| **Slave ID** | Định danh slave (0-247, thường là 1) |

**Ưu điểm:**
- Giao tiếp qua cáp Serial vật lý
- Thích hợp cho các thiết bị cũ
- Không cần mạng

**Nhược điểm:**
- Chỉ có thể kết nối một Master
- Tốc độ truyền chậm hơn

### 2. TCP Slave (Mạng)

**TCP Slave** giao tiếp thông qua **mạng Ethernet (TCP/IP)**.

| Thông số | Mô tả |
|---------|-------|
| **Địa chỉ Bind** | Địa chỉ IP để lắng nghe (0.0.0.0 = tất cả) |
| **Cổng Lắng nghe** | Cổng TCP (mặc định: 502 cho Modbus) |
| **Unit ID** | Định danh đơn vị (0-247, thường là 1) |

**Ưu điểm:**
- Giao tiếp qua mạng Ethernet
- Tốc độ truyền nhanh hơn
- Có thể kết nối nhiều Masters cùng lúc
- Thích hợp cho các hệ thống hiện đại

**Nhược điểm:**
- Cần kết nối mạng
- Cần quản lý địa chỉ IP

---

## Cấu hình Modbus Slave

### Bước 1: Chọn chế độ (RTU hoặc TCP)

1. Mở tab **Modbus Slave** (hoặc tạo mới nếu chưa có)
2. Chọn chế độ:
   - **RTU**: Giao tiếp qua Serial port
   - **TCP**: Giao tiếp qua mạng

### Bước 2: Cấu hình RTU Slave

Nếu chọn **RTU**:

1. **Chọn cổng Serial**:
   - Nhấp vào dropdown "Chọn cổng"
   - Chọn cổng mong muốn (VD: `/dev/ttyUSB0`)

2. **Cấu hình các thông số**:
   - **Baud Rate**: Chọn tốc độ (mặc định: 9600)
   - **Data Bits**: Để mặc định là 8
   - **Stop Bits**: Để mặc định là 1
   - **Parity**: Để mặc định là None

3. **Đặt Slave ID**:
   - Nhập Slave ID (thường là 1)
   - Master sẽ dùng ID này để giao tiếp với bạn

### Bước 3: Cấu hình TCP Slave

Nếu chọn **TCP**:

1. **Cấu hình địa chỉ Bind**:
   - Nhập địa chỉ IP (mặc định: `0.0.0.0` = tất cả)
   - Để `0.0.0.0` nếu muốn chấp nhận kết nối từ bất kỳ đâu

2. **Cấu hình cổng Lắng nghe**:
   - Nhập cổng TCP (mặc định: 502 cho Modbus)
   - Nếu cổng 502 được bảo vệ, thử cổng 5020 hoặc 8502

3. **Đặt Unit ID**:
   - Nhập Unit ID (thường là 1)
   - Tương tự Slave ID ở RTU

### Bước 4: Bắt đầu Modbus Slave

1. Nhấp nút **"Bắt đầu"** hoặc **"Start"**
2. Đợi thông báo xác nhận (màu xanh = thành công)
3. Bây giờ Modbus Slave đã sẵn sàng nhận yêu cầu từ Master

---

## Vùng dữ liệu Modbus

Modbus Slave lưu trữ dữ liệu trong **4 vùng chính**. Mỗi vùng có mục đích khác nhau:

### 1. Coils (Cuộn dây - Địa chỉ 0-9999)

**Coils** là những biến **ON/OFF (1 bit)** có thể **đọc và ghi**.

| Thông số | Mô tả |
|---------|-------|
| **Kiểu dữ liệu** | Boolean (Đúng/Sai) |
| **Địa chỉ** | 0 - 9999 |
| **Kích thước** | 1 bit mỗi địa chỉ |
| **Quyền hạn** | Đọc và Ghi |
| **Function Codes** | FC01 (đọc), FC05 (ghi 1), FC15 (ghi nhiều) |

**Ví dụ sử dụng:**
- Trạng thái bóng đèn (ON/OFF)
- Trạng thái máy bơm (chạy/dừng)
- Trạng thái cơ chế (mở/đóng)

### 2. Discrete Inputs (Đầu vào rời rạc - Địa chỉ 0-9999)

**Discrete Inputs** là những biến **ON/OFF (1 bit)** **chỉ đọc từ Master**, nhưng bạn có thể **chỉnh sửa trong giao diện TermiPro**.

| Thông số | Mô tả |
|---------|-------|
| **Kiểu dữ liệu** | Boolean (Đúng/Sai) |
| **Địa chỉ** | 0 - 9999 |
| **Kích thước** | 1 bit mỗi địa chỉ |
| **Quyền hạn** | Chỉ đọc từ Master (Master không thể ghi) |
| **Function Codes** | FC02 (đọc) |

**Ví dụ sử dụng:**
- Trạng thái cảm biến (phát hiện/không phát hiện)
- Trạng thái nút bấm (nhấn/không nhấn)
- Trạng thái báo động (có/không)

### 3. Holding Registers (Thanh ghi cung cấp - Địa chỉ 0-9999)

**Holding Registers** là những **giá trị số 16-bit** có thể **đọc và ghi**.

| Thông số | Mô tả |
|---------|-------|
| **Kiểu dữ liệu** | Số nguyên 16-bit (0 - 65535) |
| **Địa chỉ** | 0 - 9999 |
| **Kích thước** | 2 bytes mỗi địa chỉ |
| **Quyền hạn** | Đọc và Ghi |
| **Function Codes** | FC03 (đọc), FC06 (ghi 1), FC16 (ghi nhiều) |

**Ví dụ sử dụng:**
- Nhiệt độ thiết lập (setpoint)
- Tốc độ quạt (0-1000 RPM)
- Thời gian chờ (milliseconds)

### 4. Input Registers (Thanh ghi đầu vào - Địa chỉ 0-9999)

**Input Registers** là những **giá trị số 16-bit** **chỉ đọc từ Master**, nhưng bạn có thể **chỉnh sửa trong giao diện TermiPro**.

| Thông số | Mô tả |
|---------|-------|
| **Kiểu dữ liệu** | Số nguyên 16-bit (0 - 65535) |
| **Địa chỉ** | 0 - 9999 |
| **Kích thước** | 2 bytes mỗi địa chỉ |
| **Quyền hạn** | Chỉ đọc từ Master (Master không thể ghi) |
| **Function Codes** | FC04 (đọc) |

**Ví dụ sử dụng:**
- Giá trị cảm biến nhiệt độ (C°)
- Giá trị cảm biến áp suất (bar)
- Giá trị cảm biến độ ẩm (%)

### Bảng tóm tắt 4 vùng dữ liệu

| Vùng | Kiểu | Địa chỉ | Quyền | FC Đọc | FC Ghi |
|------|------|---------|------|--------|--------|
| **Coils** | Bit | 0-9999 | R/W | FC01 | FC05, FC15 |
| **Discrete Inputs** | Bit | 0-9999 | R | FC02 | - |
| **Holding Registers** | u16 | 0-9999 | R/W | FC03 | FC06, FC16 |
| **Input Registers** | u16 | 0-9999 | R | FC04 | - |

---

## Chỉnh sửa dữ liệu

### Chỉnh sửa thủ công

#### Coils và Discrete Inputs

1. Mở tab **"Coils"** hoặc **"Discrete Inputs"**
2. Tìm dòng cần chỉnh sửa (theo địa chỉ)
3. Nhấp vào trạng thái để chuyển đổi **ON ↔ OFF**
4. Thay đổi áp dụng **ngay lập tức**

#### Holding Registers và Input Registers

1. Mở tab **"Holding Registers"** hoặc **"Input Registers"**
2. Tìm dòng cần chỉnh sửa (theo địa chỉ)
3. Nhấp vào giá trị để sửa
4. Nhập giá trị mới (0 - 65535)
5. Nhấn **Enter** để lưu

### Chỉnh sửa nhiều giá trị cùng lúc

Một số phiên bản có hỗ trợ chỉnh sửa hàng loạt:

1. Chọn **phạm vi địa chỉ** (VD: 0-10)
2. Nhập **giá trị khởi đầu** hoặc **mẫu**
3. Nhấp **"Áp dụng"** để cập nhật tất cả

---

## Mô phỏng tự động

**Mô phỏng tự động** cho phép các giá trị **thay đổi tự động theo thời gian** mà không cần bạn chỉnh sửa thủ công. Điều này rất hữu ích cho việc kiểm tra các hệ thống phản ứng với dữ liệu thay đổi.

### 3 Chế độ Mô phỏng

#### Chế độ 1: Sin Wave (Sóng hình sin)

**Sin Wave** tạo ra một giá trị **dao động giữa min và max** theo **hình sóng sin**.

| Tham số | Mô tả |
|--------|-------|
| **Min** | Giá trị nhỏ nhất (VD: 0) |
| **Max** | Giá trị lớn nhất (VD: 100) |
| **Chu kỳ (ms)** | Thời gian một chu kỳ hoàn chỉnh (VD: 2000ms = 2 giây) |

**Ví dụ:**
- Min: 0, Max: 100, Chu kỳ: 2000ms
- Giá trị sẽ dao động: 0 → 50 → 100 → 50 → 0 → ... (lặp lại)

**Khi sử dụng:**
- Mô phỏng cảm biến nhiệt độ biến đổi mịn
- Mô phỏng dòng điện dao động
- Mô phỏng signal AC

#### Chế độ 2: Ramp (Dốc)

**Ramp** tạo ra một giá trị **tăng hoặc giảm tuyến tính** (theo bước).

| Tham số | Mô tả |
|--------|-------|
| **Min** | Giá trị nhỏ nhất |
| **Max** | Giá trị lớn nhất |
| **Bước (Step)** | Mỗi bước tăng/giảm bao nhiêu (VD: +5 hoặc -10) |
| **Khoảng thời gian (ms)** | Thời gian giữa mỗi bước |
| **Đảo chiều tại ranh giới** | Khi đạt min/max, có quay ngược không? |

**Ví dụ 1: Tăng từ 0 đến 100**
- Min: 0, Max: 100, Bước: +10, Khoảng: 500ms
- Dãy: 0 → 10 → 20 → 30 → ... → 100
- Mỗi bước cách nhau 500ms

**Ví dụ 2: Tăng rồi giảm (Palindrome)**
- Min: 0, Max: 100, Bước: +5, Đảo chiều: Có
- Dãy: 0 → 5 → 10 → ... → 100 → 95 → 90 → ... → 0 → 5 → ... (lặp)

**Khi sử dụng:**
- Mô phỏng tốc độ động cơ tăng dần
- Mô phỏng nước được đổ vào bể (tăng liên tục)
- Mô phỏng chu kỳ nén-giãn của piston

#### Chế độ 3: Random (Ngẫu nhiên)

**Random** tạo ra một **giá trị ngẫu nhiên** trong khoảng min-max.

| Tham số | Mô tả |
|--------|-------|
| **Min** | Giá trị nhỏ nhất |
| **Max** | Giá trị lớn nhất |
| **Khoảng thời gian (ms)** | Thời gian tạo giá trị mới |

**Ví dụ:**
- Min: 15, Max: 35, Khoảng: 1000ms
- Cứ 1 giây, một giá trị ngẫu nhiên từ 15-35 được tạo ra

**Khi sử dụng:**
- Mô phỏng cảm biến nhiễu
- Mô phỏng lỗi ngẫu nhiên
- Mô phỏng giá trị biến động không dự đoán được

### Cách bật Mô phỏng

1. Chọn vùng dữ liệu (Coils, Holding Registers, v.v.)
2. Chọn địa chỉ cần mô phỏng
3. Nhấp **"Cấu hình mô phỏng"** hoặc **"Setup Simulation"**
4. Chọn chế độ (Sin Wave, Ramp, Random)
5. Nhập tham số
6. Nhấp **"Bắt đầu"** hoặc **"Start"**

Giá trị sẽ tự động thay đổi theo mô phỏng. Bạn có thể **dừng lại bất kỳ lúc nào** bằng cách nhấp **"Dừng"** hoặc **"Stop"**.

---

## Cấu hình Delay phản hồi

**Delay phản hồi** là thời gian mà Modbus Slave sẽ **chờ trước khi trả lời** yêu cầu từ Master. Điều này hữu ích để **kiểm tra cách ứng dụng xử lý các thiết bị chậm**.

### Tại sao sử dụng Delay?

| Tình huống | Lợi ích |
|-----------|---------|
| **Kiểm tra timeout** | Xem ứng dụng có hết thời chờ không |
| **Kiểm tra timeout logic** | Kiểm tra mã xử lý khi slave chậm |
| **Mô phỏng thiết bị chậm** | Thiết bị thật có thể chậm |
| **Mô phỏng mạng yếu** | Mô phỏng độ trễ mạng |

### Các loại Delay

#### Global Delay (Delay toàn cục)

Áp dụng **delay cho tất cả yêu cầu**.

1. Nhập **Global Delay (ms)**
   - VD: 500ms = mọi phản hồi chậm 0.5 giây

#### Per-Function-Code Delay (Delay theo Function Code)

Áp dụng **delay riêng cho mỗi Function Code** (FC).

| FC | Tên | Delay |
|----|-----|-------|
| 01 | Read Coils | 100ms |
| 02 | Read Discrete Inputs | 100ms |
| 03 | Read Holding Registers | 50ms |
| 04 | Read Input Registers | 50ms |
| 05 | Write Single Coil | 200ms |
| 06 | Write Single Register | 200ms |

**Ví dụ:**
- FC03 (đọc) = 50ms
- FC06 (ghi) = 200ms
- Các FC khác sẽ dùng global delay (nếu có)

#### Random Delay (Delay ngẫu nhiên)

Thêm **delay ngẫu nhiên** để mô phỏng **mạng không ổn định**.

| Tham số | Mô tả |
|--------|-------|
| **Min (ms)** | Delay tối thiểu |
| **Max (ms)** | Delay tối đa |

**Ví dụ:**
- Min: 0ms, Max: 100ms
- Mỗi phản hồi sẽ có delay ngẫu nhiên từ 0-100ms

### Cách cấu hình Delay

1. Mở **"Cấu hình Delay"** hoặc **"Response Delay Settings"**
2. Chọn loại delay:
   - Global Delay: Nhập giá trị (ms)
   - Per-FC Delay: Chọn FC, nhập delay
   - Random Delay: Nhập min-max
3. Nhấp **"Áp dụng"** hoặc **"Apply"**

---

## Ánh xạ Exception

**Exception mapping** cho phép bạn **mô phỏng lỗi hoặc trạng thái ngoại lệ**. Thay vì trả lời bình thường, Modbus Slave sẽ trả lại mã **lỗi Modbus**.

### Tại sao sử dụng Exception Mapping?

| Tình huống | Lợi ích |
|-----------|---------|
| **Kiểm tra xử lý lỗi** | Xem ứng dụng xử lý lỗi Modbus như thế nào |
| **Kiểm tra vùng bị hỏng** | Mô phỏng các địa chỉ không khả dụng |
| **Kiểm tra slave bận** | Mô phỏng slave đang bận không thể trả lời |
| **Kiểm tra cơ chế phục hồi** | Kiểm tra ứng dụng có thử lại không |

### Các mã Exception Modbus

| Mã | Tên | Mô tả |
|----|-----|-------|
| **01** | Illegal Function | Function Code không được hỗ trợ |
| **02** | Illegal Data Address | Địa chỉ không hợp lệ (vượt quá phạm vi) |
| **03** | Illegal Data Value | Giá trị không hợp lệ |
| **04** | Device Failure | Thiết bị bị lỗi |
| **05** | Acknowledge | Nhận được yêu cầu, đang xử lý (chưa xong) |
| **06** | Device Busy | Thiết bị đang bận |

### Ví dụ Exception Mapping

**Ví dụ 1: Địa chỉ 100-110 không khả dụng**
- Vùng dữ liệu: Holding Registers
- Địa chỉ từ: 100
- Địa chỉ đến: 110
- Mã Exception: 02 (Illegal Data Address)
- Kết quả: Master yêu cầu bất kỳ địa chỉ nào trong 100-110 sẽ nhận được lỗi 02

**Ví dụ 2: Slave bận từ lúc 14:00-14:10**
- Tất cả các vùng dữ liệu
- Địa chỉ: 0-9999
- Mã Exception: 06 (Device Busy)
- Kết quả: Mọi yêu cầu đều trả về lỗi "bận"

### Cách cấu hình Exception Mapping

1. Mở **"Ánh xạ Exception"** hoặc **"Exception Mapping"**
2. Nhấp **"Thêm"** hoặc **"Add New"**
3. Nhập thông tin:
   - **Vùng dữ liệu**: Coils, Discrete Inputs, Holding Registers, hoặc Input Registers
   - **Địa chỉ từ**: Địa chỉ bắt đầu
   - **Địa chỉ đến**: Địa chỉ kết thúc
   - **Mã Exception**: Chọn mã lỗi (01-06)
4. Nhấp **"Lưu"** hoặc **"Save"**

---

## Thống kê và giám sát

**Thống kê** giúp bạn **theo dõi hoạt động của Modbus Slave** và **phát hiện vấn đề**.

### Các thông số thống kê

| Thông số | Mô tả |
|---------|-------|
| **Tổng yêu cầu** | Số lượng yêu cầu đã nhận từ lúc bắt đầu |
| **Yêu cầu thành công** | Số lượng yêu cầu được xử lý thành công |
| **Lỗi** | Số lượng yêu cầu bị lỗi |
| **Tốc độ yêu cầu (RPS)** | Số lượng yêu cầu mỗi giây |
| **Thời gian phản hồi TB** | Thời gian trung bình để trả lời (ms) |
| **Thời gian phản hồi min** | Phản hồi nhanh nhất (ms) |
| **Thời gian phản hồi max** | Phản hồi chậm nhất (ms) |

### Bảng chi tiết theo Function Code

| FC | Đọc/Ghi | Thành công | Lỗi | Tỷ lệ |
|----|---------|-----------|-----|-------|
| FC01 | Đọc | 1250 | 5 | 99.6% |
| FC03 | Đọc | 980 | 2 | 99.8% |
| FC06 | Ghi | 450 | 0 | 100% |
| ... | ... | ... | ... | ... |

### Giám sát TCP Clients (Chỉ TCP Slave)

Nếu sử dụng **TCP Slave**, bạn có thể xem danh sách **các Masters đang kết nối**:

| Thông tin | Mô tả |
|----------|-------|
| **ID Client** | Định danh duy nhất của Master |
| **Địa chỉ IP** | Địa chỉ IP của Master |
| **Thời gian kết nối** | Lúc Master kết nối |
| **Số yêu cầu** | Tổng yêu cầu từ Master này |

### Cách xem Thống kê

1. Mở tab **"Thống kê"** hoặc **"Statistics"**
2. Xem các số liệu hiện tại
3. Bảng sẽ **cập nhật thời gian thực** khi có yêu cầu mới

---

## Lưu và tải dữ liệu

**Lưu dữ liệu** cho phép bạn **giữ lại các giá trị** đã cấu hình khi tắt ứng dụng. **Tải dữ liệu** cho phép bạn **khôi phục cấu hình trước đó**.

### Lưu dữ liệu

#### Cách lưu thủ công

1. Nhấp **"Lưu dữ liệu"** hoặc **"Save Data"**
2. Chọn vị trí lưu file (thường là `.json`)
3. Nhập tên file (VD: `slave_config_20251228.json`)
4. Nhấp **"Lưu"** hoặc **"Save"**

**Nội dung file lưu:**
- Tất cả giá trị trong 4 vùng dữ liệu
- Cấu hình mô phỏng (nếu có)
- Cấu hình exception mapping
- Cấu hình delay

#### Tự động lưu

Một số phiên bản hỗ trợ **tự động lưu định kỳ**:

1. Mở **"Cấu hình chung"** hoặc **"Settings"**
2. Bật **"Tự động lưu"**
3. Chọn khoảng thời gian (VD: mỗi 5 phút)

### Tải dữ liệu

#### Cách tải

1. Nhấp **"Tải dữ liệu"** hoặc **"Load Data"**
2. Chọn file JSON đã lưu trước đó
3. Nhấp **"Mở"** hoặc **"Open"**
4. Dữ liệu và cấu hình sẽ được **khôi phục**

**Lưu ý:**
- Dữ liệu cũ sẽ bị **ghi đè** (không thể hoàn tác)
- Hãy **sao lưu** nếu muốn giữ dữ liệu cũ

#### Tải mẫu

Một số bộ dữ liệu mẫu có sẵn:

- **Mẫu Basic**: Giá trị mặc định cho người mới bắt đầu
- **Mẫu HVAC**: Giả lập hệ thống điều hòa không khí
- **Mẫu Water Tank**: Giả lập bể nước

1. Nhấp **"Tải mẫu"** hoặc **"Load Template"**
2. Chọn mẫu
3. Dữ liệu sẽ được tải

---

## Mẹo sử dụng

### Mẹo 1: Kiểm tra SCADA

**Mục tiêu:** Kiểm tra ứng dụng SCADA mà không cần thiết bị thật

**Cách làm:**
1. Cấu hình Modbus Slave với cùng cổng/địa chỉ như thiết bị thật
2. Bật mô phỏng Sin Wave hoặc Ramp cho các giá trị cảm biến
3. Chạy ứng dụng SCADA và xem có đọc được dữ liệu không
4. Kiểm tra biểu đồ, cảnh báo, v.v.

**Lợi ích:**
- Không cần thiết bị thật (tiết kiệm chi phí)
- Có thể tái tạo các tình huống (dự báo thời tiết, mùa, v.v.)
- An toàn hơn (không có rủi ro thiệt hại)

### Mẹo 2: Lập trình PLC

**Mục tiêu:** Kiểm tra code PLC với cảm biến không có sẵn

**Cách làm:**
1. Cấu hình Modbus Slave mô phỏng các cảm biến cần thiết
2. Bật các mô phỏng tương ứng (VD: nhiệt độ tăng dần)
3. Nạp code PLC với logic kiểm tra
4. Theo dõi các output PLC (relay, van, v.v.)

**Ví dụ:**
- PLC cần kiểm tra: "Nếu nhiệt độ > 40°C, mở quạt"
- Dùng Modbus Slave mô phỏng nhiệt độ Ramp từ 20-60°C
- Khi nhiệt độ đạt 40°C, kiểm tra xem PLC mở quạt hay không

### Mẹo 3: Gỡ lỗi giao tiếp

**Mục tiêu:** Tìm ra lỗi trong mã giao tiếp Modbus

**Cách làm:**
1. Bật Modbus Slave ở chế độ TCP
2. Chạy ứng dụng của bạn kết nối đến Slave
3. Xem tab **"Thống kê"** để kiểm tra yêu cầu
4. Xem các lỗi có xảy ra không
5. Dùng Exception Mapping để mô phỏng lỗi và kiểm tra code xử lý lỗi

### Mẹo 4: Kiểm tra Timeout

**Mục tiêu:** Xem ứng dụng xử lý timeout như thế nào

**Cách làm:**
1. Cấu hình Delay phản hồi lớn (VD: 5000ms = 5 giây)
2. Chạy ứng dụng yêu cầu dữ liệu
3. Kiểm tra ứng dụng có:
   - Hết thời chờ (timeout)?
   - Thử lại (retry)?
   - Hiển thị lỗi cho người dùng?

### Mẹo 5: Kiểm tra khôi phục lỗi

**Mục tiêu:** Kiểm tra ứng dụng có khôi phục được khi Slave bị tắt/bật

**Cách làm:**
1. Chạy ứng dụng kết nối đến Modbus Slave
2. Đang hoạt động bình thường, nhấp **"Dừng"** để tắt Slave
3. Quan sát ứng dụng:
   - Có báo lỗi không?
   - Có hiển thị "kết nối bị ngắt"?
4. Bật lại Slave (**"Bắt đầu"**)
5. Kiểm tra ứng dụng có tự kết nối lại không

---

## Xử lý sự cố

### Vấn đề 1: "Không thể bắt đầu RTU Slave"

#### Nguyên nhân có thể

- Cổng Serial không được cắm hoặc không tồn tại
- Cấu hình cổng sai
- Cổng đang được sử dụng bởi ứng dụng khác

#### Cách khắc phục

1. **Kiểm tra cấu hình cổng**:
   - Chắc chắn cổng đã được chọn (không để trống)
   - Chắc chắn cấu hình baud rate, data bits, v.v. đúng

2. **Kiểm tra kết nối USB**:
   - Đảm bảo thiết bị USB được cắm
   - Thử cắm vào cổng USB khác

3. **Đóng ứng dụng khác sử dụng cổng**:
   - Kiểm tra Arduino IDE, PuTTY, hoặc các ứng dụng serial khác
   - Đóng tất cả chúng

4. **Khởi động lại ứng dụng**:
   - Thoát TermiPro hoàn toàn
   - Mở lại và thử lại

### Vấn đề 2: "Không thể bắt đầu TCP Slave - Cổng bị dùng"

#### Nguyên nhân có thể

- Cổng 502 (mặc định) được bảo vệ hoặc đã được sử dụng
- ứng dụng Modbus khác đang sử dụng cùng cổng
- Phiên trước của TermiPro chưa thoát hoàn toàn

#### Cách khắc phục

1. **Đổi cổng**:
   - Thay cổng 502 sang cổng khác (VD: 5020, 8502)
   - Cập nhật Master cũng sử dụng cổng mới

2. **Kiểm tra quyền**:
   - Trên Linux/macOS: Chạy TermiPro với `sudo`
   - Windows: Chạy dưới quyền Administrator

3. **Đóng phiên cũ**:
   - Đảm bảo không còn phiên TermiPro khác chạy
   - Xem Task Manager (Windows) hoặc Activity Monitor (macOS)

4. **Thử lại**:
   - Đợi 10 giây rồi thử bắt đầu Slave lại

### Vấn đề 3: Master không thể kết nối đến TCP Slave

#### Nguyên nhân có thể

- Địa chỉ IP hoặc cổng sai
- Firewall chặn kết nối
- TCP Slave chưa được bắt đầu
- Slave và Master không cùng mạng

#### Cách khắc phục

1. **Kiểm tra TCP Slave đang chạy**:
   - Xem tab TCP Slave có trạng thái "Running" không
   - Kiểm tra cổng lắng nghe

2. **Kiểm tra cấu hình Master**:
   - Địa chỉ IP: `127.0.0.1` hoặc `localhost` (cùng máy)
   - Địa chỉ IP: `192.168.x.x` (mạng khác)
   - Cổng phải giống với cấu hình Slave

3. **Kiểm tra Firewall**:
   - Windows Firewall: Cho phép TermiPro truy cập mạng
   - Linux: `sudo ufw allow 502/tcp` (nếu dùng ufw)
   - macOS: System Preferences > Security & Privacy > Firewall

4. **Kiểm tra kết nối mạng**:
   - Chắc chắn Slave và Master cùng mạng (hoặc có route)
   - Ping từ Master đến Slave để kiểm tra

### Vấn đề 4: Master không nhận được dữ liệu đúng

#### Nguyên nhân có thể

- Địa chỉ Slave/Unit ID sai
- Function Code sai (VD: đọc Coils từ Holding Registers)
- Dữ liệu chưa được nhập vào Slave
- Baud rate RTU không khớp (RTU Slave)

#### Cách khắc phục

1. **Kiểm tra Slave ID/Unit ID**:
   - RTU Slave: Kiểm tra Slave ID
   - TCP Slave: Kiểm tra Unit ID
   - Master phải dùng ID giống hệt

2. **Kiểm tra Function Code**:
   - FC01-FC04: Đọc (Read)
   - FC05, FC15: Viết Coils
   - FC06, FC16: Viết Registers
   - Kiểm tra Master gửi FC nào

3. **Kiểm tra dữ liệu trong Slave**:
   - Mở tab dữ liệu (Coils, Holding Registers, v.v.)
   - Kiểm tra địa chỉ và giá trị có đúng không

4. **Kiểm tra baud rate (RTU Slave)**:
   - RTU Slave và Master phải có baud rate giống hệt
   - Thường dùng: 9600, 115200

### Vấn đề 5: Dữ liệu thay đổi nhưng Master không thấy

#### Nguyên nhân có thể

- Master không cập nhật liên tục (polling)
- Master đang poll địa chỉ khác
- Mô phỏng chưa bắt đầu

#### Cách khắc phục

1. **Kiểm tra Master có polling không**:
   - Kiểm tra Master có gửi yêu cầu liên tục không
   - Xem tab Thống kê của Slave có tăng Tổng yêu cầu không

2. **Kiểm tra địa chỉ polling**:
   - Master đang poll địa chỉ nào?
   - Dữ liệu thay đổi ở địa chỉ nào?
   - Chúng có giống không?

3. **Kiểm tra mô phỏng**:
   - Xem mô phỏng đã bắt đầu không
   - Xem địa chỉ mô phỏng có giống địa chỉ polling không

### Vấn đề 6: Exception hoặc lỗi liên tục

#### Nguyên nhân có thể

- Exception Mapping được cấu hình (cố ý hoặc vô tình)
- Địa chỉ nằm ngoài phạm vi
- Dữ liệu bị hỏng

#### Cách khắc phục

1. **Kiểm tra Exception Mapping**:
   - Mở tab **"Ánh xạ Exception"** hoặc **"Exception Mapping"**
   - Kiểm tra các địa chỉ đó có trong danh sách không
   - Nếu không cần, xóa khỏi danh sách

2. **Kiểm tra phạm vi địa chỉ**:
   - Modbus địa chỉ từ 0-9999
   - Nếu Master yêu cầu > 9999, sẽ lỗi "Illegal Data Address"

3. **Khôi phục dữ liệu**:
   - Nhấp **"Reset"** để khôi phục giá trị mặc định
   - Hoặc tải dữ liệu từ file `.json` đã lưu

---

## Tóm tắt nhanh

| Tác vụ | Bước |
|--------|------|
| **Bắt đầu RTU Slave** | Chọn chế độ RTU → Chọn cổng → Cấu hình → Nhấp Bắt đầu |
| **Bắt đầu TCP Slave** | Chọn chế độ TCP → Nhập cổng → Nhấp Bắt đầu |
| **Chỉnh sửa giá trị** | Mở tab dữ liệu → Tìm địa chỉ → Nhấp và sửa |
| **Bật mô phỏng** | Chọn địa chỉ → Cấu hình mô phỏng → Chọn chế độ → Bắt đầu |
| **Cấu hình delay** | Mở Delay Settings → Nhập giá trị → Áp dụng |
| **Lưu dữ liệu** | Nhấp Lưu dữ liệu → Chọn vị trí → Nhấp Lưu |
| **Tải dữ liệu** | Nhấp Tải dữ liệu → Chọn file → Nhấp Mở |

---

## Hỏi đáp nhanh

**Q: Modbus Slave là gì?**
A: Là tính năng cho phép TermiPro hoạt động như một thiết bị Modbus để kiểm tra các hệ thống khác.

**Q: RTU hay TCP tốt hơn?**
A: RTU tốt cho kết nối Serial cạnh tranh. TCP tốt cho mạng Ethernet và tốc độ cao.

**Q: Có thể chạy RTU và TCP cùng lúc không?**
A: Tùy vào version. Nhiều phiên bản hỗ trợ chạy cả hai trong các tab riêng.

**Q: Modbus Slave sẽ mất dữ liệu khi tắt không?**
A: Có, trừ khi bạn lưu dữ liệu vào file `.json` trước.

**Q: Có thể mô phỏng một Master thay vì Slave không?**
A: Không. Dùng tab **Modbus Master** để làm Master.

**Q: Làm sao để biết Master kết nối được không?**
A: Xem tab **Thống kê**. Nếu "Tổng yêu cầu" tăng, Master đang kết nối.

**Q: Làm sao để kiểm tra TCP Slave từ máy khác?**
A: Dùng Master trên máy khác, nhập địa chỉ IP của Slave (chứ không phải 127.0.0.1).

**Q: Exception Code nào để mô phỏng "không có quyền"?**
A: Modbus tiêu chuẩn không có Exception Code cho "không có quyền". Dùng Code 03 (Illegal Data Value) thay thế.

---

## Liên hệ hỗ trợ

Nếu gặp vấn đề không được giải quyết trong hướng dẫn này:

1. Kiểm tra tab **"FAQ"** hoặc **"Hỏi đáp"**
2. Tìm kiếm trên GitHub Issues
3. Liên hệ nhóm phát triển TermiPro

---

**Cập nhật lần cuối**: Tháng 12 năm 2025

**Phiên bản tài liệu**: 1.0
