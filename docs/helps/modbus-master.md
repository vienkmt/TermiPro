# Hướng dẫn sử dụng Modbus Master

Chào mừng bạn đến với hướng dẫn Modbus Master của TermiPro. Tài liệu này sẽ giúp bạn hiểu và sử dụng tính năng Modbus để giao tiếp với các thiết bị công nghiệp.

## Mục lục

1. [Giới thiệu Modbus](#giới-thiệu-modbus)
2. [Chế độ kết nối](#chế-độ-kết-nối)
3. [Cấu hình RTU (Serial)](#cấu-hình-rtu-serial)
4. [Cấu hình TCP/IP](#cấu-hình-tcpip)
5. [Các Function Codes](#các-function-codes)
6. [Định dạng dữ liệu](#định-dạng-dữ-liệu)
7. [Polling - Đọc dữ liệu tự động](#polling---đọc-dữ-liệu-tự-động)
8. [Hiểu nhật ký giao dịch](#hiểu-nhật-ký-giao-dịch)
9. [Mẹo sử dụng thực tế](#mẹo-sử-dụng-thực-tế)
10. [Xử lý sự cố](#xử-lý-sự-cố)

---

## Giới thiệu Modbus

### Modbus là gì?

**Modbus** là một giao thức truyền thông công nghiệp phổ biến, được sử dụng để giao tiếp giữa các thiết bị điện tử trong hệ thống tự động hóa. Modbus cho phép bạn:

- Đọc dữ liệu từ các thiết bị (như cảm biến, bộ điều khiển)
- Ghi dữ liệu vào các thiết bị (điều khiển các relay, motor, v.v.)
- Giám sát nhiều thiết bị thông qua một kết nối duy nhất

### Master vs Slave

Trong Modbus, có hai vai trò chính:

- **Master (Chủ)**: Là thiết bị hoặc phần mềm bắt đầu yêu cầu (gửi câu hỏi) - TermiPro đóng vai trò này
- **Slave (Tớ)**: Là thiết bị được yêu cầu (trả lời câu hỏi) - Đó là thiết bị công nghiệp của bạn

**Ví dụ thực tế:**
- TermiPro là Master: "Thiết bị A, cho tôi biết nhiệt độ hiện tại là bao nhiêu?"
- Thiết bị A là Slave: "Nhiệt độ hiện tại là 25 độ C"

### Tại sao dùng Modbus?

- **Chuẩn công nghiệp**: Được chấp nhận và sử dụng rộng rãi
- **Đơn giản**: Dễ hiểu, không phức tạp
- **Tin cậy**: Có kiểm tra lỗi (CRC)
- **Nhanh**: Độ trễ thấp, thích hợp cho điều khiển thời gian thực

---

## Chế độ kết nối

TermiPro hỗ trợ hai chế độ Modbus: **RTU** (qua Serial) và **TCP** (qua Ethernet). Chọn chế độ phù hợp với thiết bị của bạn.

### RTU (Serial) - Khi nào dùng?

**RTU = Remote Terminal Unit (qua cổng Serial)**

Sử dụng RTU khi:
- Thiết bị của bạn kết nối qua **cổng Serial (COM port)**
- Thiết bị kết nối qua **USB to Serial adapter**
- Thiết bị không có Ethernet
- Bạn muốn truyền dữ liệu qua dây Serial

**Ưu điểm:**
- Dây nối đơn giản (RS485, RS232)
- Tiết kiệm chi phí
- Phù hợp cho các hệ thống cũ

**Nhược điểm:**
- Khoảng cách giới hạn (thường dưới 1km)
- Tốc độ truyền thấp hơn

### TCP (Ethernet) - Khi nào dùng?

**TCP = Truyền dữ liệu qua mạng Ethernet**

Sử dụng TCP khi:
- Thiết bị của bạn có **cổng Ethernet (RJ45)**
- Thiết bị kết nối qua **WiFi**
- Bạn muốn khoảng cách truyền xa hơn
- Thiết bị hỗ trợ Modbus TCP

**Ưu điểm:**
- Khoảng cách truyền xa (có thể qua Internet)
- Tốc độ cao
- Hỗ trợ nhiều thiết bị cùng một lúc

**Nhược điểm:**
- Cần cấu hình mạng
- Phức tạp hơn

---

## Cấu hình RTU (Serial)

### Bước 1: Chọn chế độ RTU

1. Mở TermiPro
2. Tạo Tab mới -> Chọn "Modbus"
3. Trong sidebar, nhấn nút **[RTU]** (mặc định đã chọn)

### Bước 2: Chọn cổng Serial

1. Kéo xuống danh sách **"Select Port"**
2. Chọn cổng USB mà thiết bị của bạn được kết nối
   - Ví dụ: `/dev/ttyUSB0` (Linux), `COM3` (Windows), `/dev/ttyUSB0` (macOS)
3. Nếu không thấy cổng, kiểm tra:
   - Thiết bị có được kết nối vật lý không?
   - Kabel USB có tốt không?
   - Driver USB to Serial đã được cài không?

### Bước 3: Cấu hình các tham số Serial

Thường thì thiết bị sẽ cung cấp thông số này. Nếu không có, hãy liên hệ nhà sản xuất.

| Tham số | Mặc định | Phạm vi | Ý nghĩa |
|---------|---------|--------|--------|
| **Baud Rate** | 9600 | 300 - 921.600 | Tốc độ truyền (bits/giây). Phải khớp với thiết bị |
| **Data Bits** | 8 | 7 - 8 | Số bits dữ liệu. Thường là 8 |
| **Stop Bits** | 1 | 1 - 2 | Số bit dừng. Thường là 1 |
| **Parity** | Even | None/Odd/Even | Kiểm tra lỗi. Modbus dùng "Even" |

**Ví dụ cấu hình điển hình:**
- Baud Rate: 9600
- Data Bits: 8
- Stop Bits: 1
- Parity: Even

### Bước 4: Nhập Slave ID

**Slave ID** = Mã nhận dạng thiết bị

- Nếu chỉ có 1 thiết bị: Nhập `1`
- Nếu có nhiều thiết bị trên cùng dây Serial: Mỗi thiết bị có ID khác nhau (1-247)
- Liên hệ nhà sản xuất để biết Slave ID của thiết bị

### Bước 5: Kiểm tra Timeout

**Timeout** = Thời gian chờ trả lời từ thiết bị (miligiây)

- Mặc định: 1000ms (1 giây)
- Nếu thiết bị trả lời chậm, tăng lên 2000-3000ms
- Nếu bạn muốn nhanh hơn, giảm xuống 500-800ms

### Bước 6: Kết nối

1. Nhấn nút **[Connect]** xanh
2. Chờ kết nối thành công
3. Nội dung nút sẽ thay đổi thành **[Disconnect]** màu đỏ

---

## Cấu hình TCP/IP

### Bước 1: Chọn chế độ TCP

1. Tạo Tab Modbus mới (hoặc tạo tab mới)
2. Trong sidebar, nhấn nút **[TCP/IP]**

### Bước 2: Nhập địa chỉ Host

**Host** = Địa chỉ IP của thiết bị Modbus TCP

- Nếu thiết bị trên máy tính hiện tại: `localhost` hoặc `127.0.0.1`
- Nếu thiết bị trên mạng khác: Nhập địa chỉ IP, ví dụ `192.168.1.100`

### Bước 3: Nhập cổng

**Port** = Cổng kết nối

- Modbus TCP mặc định: `502`
- Nếu thiết bị sử dụng cổng khác, nhà sản xuất sẽ cho biết
- Thường không cần thay đổi

### Bước 4: Nhập Unit ID

**Unit ID** = Mã nhận dạng thiết bị (tương tự Slave ID trong RTU)

- Nếu chỉ có 1 thiết bị: Nhập `1`
- Nếu có nhiều thiết bị: Mỗi thiết bị có ID khác nhau (1-247)

### Bước 5: Kiểm tra Timeout

Mặc định 1000ms, tương tự RTU.

### Bước 6: Kết nối

Nhấn **[Connect]** và chờ kết nối thành công.

---

## Các Function Codes

**Function Code (FC)** = Lệnh để yêu cầu hoạt động gì từ thiết bị Modbus.

TermiPro hỗ trợ 8 Function Codes tiêu chuẩn:

### Function Codes Đọc (Read)

#### FC01 - Read Coils (Đọc Coils)

- **Ý nghĩa**: Đọc trạng thái ON/OFF (0 hoặc 1)
- **Dữ liệu**: Trả về danh sách các bit (ON/OFF)
- **Dùng khi**: Bạn cần biết các thiết bị điều khiển được bật hay tắt
- **Ví dụ**: Đọc trạng thái của 8 relay (nút bấm, đèn, motor)

| Giá trị | Ý nghĩa |
|--------|---------|
| ON | Coil được kích hoạt / relay bật |
| OFF | Coil không được kích hoạt / relay tắt |

#### FC02 - Read Discrete Inputs (Đọc Discrete Inputs)

- **Ý nghĩa**: Tương tự FC01, nhưng chỉ đọc được (không ghi)
- **Dữ liệu**: Danh sách các bit ON/OFF
- **Dùng khi**: Đọc trạng thái từ các nút bấm, cảm biến chỉ có kết quả đúng/sai
- **Ví dụ**: Kiểm tra xem cửa có mở không, có ánh sáng không

#### FC03 - Read Holding Registers (Đọc Holding Registers)

- **Ý nghĩa**: Đọc giá trị số (0-65535)
- **Dữ liệu**: Danh sách các số 16-bit
- **Dùng khi**: Bạn cần biết giá trị như nhiệt độ, áp suất, tốc độ
- **Ví dụ**: Đọc nhiệt độ từ cảm biến (25.5°C), tốc độ quạt (3000 RPM), công suất (500W)
- **Phổ biến nhất**: Đây là FC được dùng nhiều nhất

#### FC04 - Read Input Registers (Đọc Input Registers)

- **Ý nghĩa**: Tương tự FC03, nhưng chỉ đọc được (không ghi)
- **Dữ liệu**: Danh sách các số 16-bit
- **Dùng khi**: Đọc giá trị từ cảm biến, không thể thay đổi
- **Ví dụ**: Đọc giá trị từ cảm biến áp suất, độ ẩm

### Function Codes Ghi (Write)

#### FC05 - Write Single Coil (Ghi một Coil)

- **Ý nghĩa**: Bật hoặc tắt 1 relay/thiết bị
- **Dữ liệu**: Chỉ 1 giá trị ON hoặc OFF
- **Dùng khi**: Bạn muốn bật/tắt 1 thiết bị (đèn, motor, pump)
- **Ví dụ**: Bật đèn phòng, tắt quạt gió

#### FC06 - Write Single Register (Ghi một Register)

- **Ý nghĩa**: Ghi 1 giá trị số (0-65535) vào 1 địa chỉ
- **Dữ liệu**: Chỉ 1 giá trị số
- **Dùng khi**: Thay đổi cài đặt, đặt tốc độ, điều chỉnh công suất
- **Ví dụ**: Đặt nhiệt độ mục tiêu = 25°C, tốc độ = 1500 RPM

#### FC0F - Write Multiple Coils (Ghi nhiều Coils)

- **Ý nghĩa**: Bật/tắt nhiều relay cùng một lúc
- **Dữ liệu**: Danh sách các giá trị ON/OFF
- **Dùng khi**: Bạn cần điều khiển 5 đèn, 8 relay cùng một lúc
- **Ví dụ**: Tắt tất cả đèn trong nhà (10 đèn), bật các relay cho hệ thống bơm nước

#### FC10 - Write Multiple Registers (Ghi nhiều Registers)

- **Ý nghĩa**: Ghi nhiều giá trị số vào nhiều địa chỉ
- **Dữ liệu**: Danh sách các số 16-bit
- **Dùng khi**: Cài đặt nhiều tham số cùng một lúc
- **Ví dụ**: Đặt nhiệt độ, áp suất, và tốc độ quạt trong cùng 1 yêu cầu

### Hướng dẫn chọn Function Code

| Bạn muốn làm gì? | Function Code |
|------------------|--------------|
| Đọc trạng thái ON/OFF của 1 hoặc nhiều thiết bị | FC01 hoặc FC02 |
| Đọc giá trị số (nhiệt độ, áp suất, v.v.) | FC03 hoặc FC04 |
| Bật/tắt 1 thiết bị | FC05 |
| Thay đổi 1 tham số số | FC06 |
| Bật/tắt nhiều thiết bị | FC0F |
| Thay đổi nhiều tham số cùng lúc | FC10 |

---

## Định dạng dữ liệu

Dữ liệu đọc được từ thiết bị Modbus là các số nguyên 16-bit (0 đến 65535). TermiPro cung cấp nhiều cách để hiển thị những con số này.

### Unsigned (Không dấu)

**Là gì?** Giá trị từ 0 đến 65535

**Khi nào dùng?**
- Đọc nhiệt độ, áp suất, tốc độ, công suất
- Các giá trị luôn dương
- Hầu hết các ứng dụng

**Ví dụ:**
- Giá trị từ thiết bị: 25 = Nhiệt độ 25°C
- Giá trị từ thiết bị: 3000 = Tốc độ 3000 RPM

### Signed (Có dấu)

**Là gì?** Giá trị từ -32768 đến 32767

**Khi nào dùng?**
- Đọc giá trị có thể âm: Nhiệt độ dưới 0°C, độ cao (âm là dưới mực nước)
- Lỗi hoặc độ lệch từ giá trị chuẩn

**Ví dụ:**
- Giá trị từ thiết bị: 65535 = -1 độ C (âm)
- Giá trị từ thiết bị: 32768 = -32768 (âm lớn)

### Hex (Thập lục phân)

**Là gì?** Giá trị dưới dạng hex (0x0000 đến 0xFFFF)

**Khi nào dùng?**
- Kiểm tra lỗi, chứng chỉ
- Các giá trị biểu diễn mã, mã lỗi
- Lập trình, tìm lỗi

**Ví dụ:**
- Giá trị hex: 0x00FF = 255 decimal
- Giá trị hex: 0xFFFF = 65535 decimal

### Binary (Nhị phân)

**Là gì?** Giá trị dưới dạng bit 0 và 1

**Khi nào dùng?**
- Kiểm tra trạng thái bit cụ thể
- Tìm lỗi bit (bit flags)
- Lập trình, tìm lỗi

**Ví dụ:**
- Giá trị nhị phân: 0000000111111111 = 255 (8 bit thấp là 1)
- Giá trị nhị phân: 1111111100000000 = 65280 (8 bit cao là 1)

### Float32 (Số thực)

**Là gì?** Giá trị số thực (có phần thập phân)

**Khi nào dùng?**
- Đọc giá trị có phần thập phân: Nhiệt độ 25.5°C, áp suất 98.6 kPa
- **Lưu ý**: Cần 2 registers liên tiếp để lưu 1 giá trị Float32
- Phải chỉ định "Quantity = 2" để đọc 2 registers

**Ví dụ:**
- 2 registers: [0x4200, 0x0000] = giá trị Float32 = 40.0°C

---

## Polling - Đọc dữ liệu tự động

**Polling** = Đọc dữ liệu từ thiết bị theo một khoảng thời gian lặp lại.

Thay vì bạn phải nhấn [Read] nhiều lần, Polling sẽ tự động đọc dữ liệu liên tục.

### Sử dụng Polling

#### Bước 1: Cấu hình yêu cầu

1. Chọn **Function Code** (ví dụ: FC03 - Read Holding Registers)
2. Nhập **Start Address** (ví dụ: 0)
3. Nhập **Quantity** (ví dụ: 10) - số register cần đọc

#### Bước 2: Cấu hình Polling Interval

1. Tìm **"Poll Int. (ms)"** trong sidebar
2. Nhập khoảng thời gian giữa các lần đọc (miligiây)
   - 1000 = Đọc mỗi 1 giây
   - 500 = Đọc mỗi 0.5 giây (nhanh hơn)
   - 5000 = Đọc mỗi 5 giây (chậm hơn)

**Khuyến nghị:**
- Polling nhanh (100-500ms): Dùng cho thao tác kiểm soát thời gian thực
- Polling trung bình (1000ms): Dùng cho giám sát thông thường
- Polling chậm (5000-10000ms): Dùng cho ghi log, tiết kiệm tài nguyên

#### Bước 3: Bắt đầu Polling

1. Nhấn nút **[Auto Read]** xanh
2. Nút sẽ thay đổi thành **[Stop]** màu đỏ
3. Dữ liệu sẽ tự động cập nhật mỗi khoảng thời gian mà bạn đã cài đặt

#### Bước 4: Dừng Polling

1. Nhấn nút **[Stop]** để dừng đọc tự động
2. Bạn có thể thay đổi Quantity hoặc Start Address
3. Nhấn **[Auto Read]** lại để tiếp tục

### Ví dụ thực tế - Polling một hệ thống

Bạn muốn giám sát:
- Nhiệt độ (từ address 0)
- Áp suất (từ address 1)
- Độ ẩm (từ address 2)

**Cấu hình:**
- Function Code: FC03 (Read Holding Registers)
- Start Address: 0
- Quantity: 3
- Poll Interval: 1000ms (đọc mỗi 1 giây)

**Kết quả:** Mỗi giây, TermiPro sẽ đọc 3 giá trị từ address 0, 1, 2 và hiển thị trên bảng.

---

## Hiểu nhật ký giao dịch

**Transaction Log** (Nhật ký giao dịch) hiển thị tất cả các yêu cầu và phản hồi giữa TermiPro và thiết bị Modbus.

### Cấu trúc một mục Nhật ký

```
14:32:05 FC03 OK 45ms
TX: 01 03 00 00 00 0A C5 CD
RX: 01 03 14 00 0C 00 19 00 26 00 32 00 3E 00 4B 00 57 00 64 01 00 00 01 BC
```

### Giải thích chi tiết

| Phần | Ý nghĩa |
|------|---------|
| **14:32:05** | Thời gian yêu cầu được gửi |
| **FC03** | Function Code (ví dụ: FC03 = Read Holding Registers) |
| **OK** | Kết quả: OK (thành công) hoặc ERR (lỗi) |
| **45ms** | Thời gian phản hồi (bao lâu thiết bị trả lời) |
| **TX:** | Transmitted (dữ liệu TermiPro gửi đi) |
| **RX:** | Received (dữ liệu TermiPro nhận lại) |

### Frame Hex là gì?

**TX Frame (Yêu cầu):**
```
01 03 00 00 00 0A C5 CD
│  │  └────┘ └────┘ └───┘
│  │    │      │     └─ CRC (kiểm tra lỗi)
│  │    │      └─ Số lượng register cần đọc (10)
│  │    └─ Địa chỉ bắt đầu (0)
│  └─ Function Code (03 = Read Holding Registers)
└─ Slave ID / Unit ID (1)
```

**RX Frame (Phản hồi):**
```
01 03 14 00 0C 00 19 00 26 00 32 00 3E 00 4B 00 57 00 64 01 00 00 01 BC
│  │  │  └──────────────────────────────────────────────────────────────┘
│  │  │  │
│  │  │  └─ Dữ liệu (10 registers với giá trị: 12, 25, 38, 50, 62, 75, 87, 100, 256, 1)
│  │  └─ Số byte dữ liệu (20 bytes = 10 registers x 2 bytes/register)
│  └─ Function Code (03)
└─ Slave ID / Unit ID (1)
```

### Khi nào xem Nhật ký?

1. **Kiểm tra kết nối**: Nếu toàn bộ là lỗi, kiểm tra lại cấu hình
2. **Tìm lỗi**: Nếu giá trị lẻ, xem giá trị hex trong log
3. **Kiểm tra tốc độ**: Xem thời phản hồi (response time) để tối ưu
4. **Ghi log**: Lưu data của nhật ký để chứng chỉ, kiểm toán

---

## Mẹo sử dụng thực tế

### Mẹo 1: Kiểm tra kết nối nhanh

**Vấn đề**: Bạn không chắc kết nối có thành công không?

**Cách giải quyết:**
1. Chọn FC01 hoặc FC02 (Read Coils/Discrete Inputs)
2. Nhập Quantity = 1
3. Nhấn [Read]
4. Nếu thành công, kết nối OK. Nếu lỗi, kiểm tra cấu hình.

### Mẹo 2: Đọc giá trị Float32

**Vấn đề**: Cảm biến của bạn cần 2 registers để lưu 1 giá trị thực (25.5°C)

**Cách giải quyết:**
1. Chọn FC03 (Read Holding Registers)
2. Start Address = nơi giá trị bắt đầu
3. **Quantity = 2** (bắt buộc!)
4. Chọn Format = "Float32"
5. Dữ liệu sẽ hiển thị đúng

**Ví dụ:**
- Address 0-1 chứa nhiệt độ: Đọc từ address 0, quantity = 2
- Address 2-3 chứa áp suất: Đọc từ address 2, quantity = 2

### Mẹo 3: Ghi nhiều giá trị

**Vấn đề**: Bạn muốn đặt 3 tham số cùng một lúc (nhiệt độ, áp suất, tốc độ)

**Cách giải quyết:**
1. Chọn FC10 (Write Multiple Registers)
2. Start Address = nơi bắt đầu ghi
3. Nhập 3 giá trị trong các ô nhập liệu
4. Nhấn [Write]

**Ví dụ:**
- Register 0 = 25 (nhiệt độ 25°C)
- Register 1 = 100 (áp suất 100 kPa)
- Register 2 = 1500 (tốc độ 1500 RPM)

### Mẹo 4: Tối ưu tốc độ Polling

**Vấn đề**: Polling quá chậm hoặc quá nhanh?

**Cách giải quyết:**
- **Polling quá chậm**: Giảm Poll Interval (ví dụ: từ 5000 thành 1000ms)
- **Polling quá nhanh**: Tăng Poll Interval (ví dụ: từ 500 thành 2000ms)
- **Điều chỉnh dần**: Không nên thay đổi quá nhiều một lần

**Khuyến nghị:**
- Nếu network bị lag, giảm tốc độ polling
- Nếu cần dữ liệu real-time, tăng tốc độ polling

### Mẹo 5: Ghi log cho kiểm toán

**Vấn đề**: Bạn muốn lưu dữ liệu để kiểm toán hoặc chứng chỉ ISO

**Cách giải quyết:**
1. Bật Polling để tự động đọc dữ liệu
2. Xem Transaction Log (nhật ký giao dịch)
3. Chụp ảnh hoặc Export log (nếu có tính năng)

**Lợi ích**: Chứng minh rằng bạn đã giám sát thiết bị, kiểm soát chất lượng.

### Mẹo 6: Kiểm tra địa chỉ register đúng

**Vấn đề**: Bạn nhận được giá trị lạ hoặc giá trị không thay đổi

**Cách giải quyết:**
1. Xem tài liệu thiết bị -> tìm "Modbus Address Map"
2. Kiểm tra xem địa chỉ bắt đầu có đúng không
3. Thử đọc từ address 0, 1, 2, ... để tìm giá trị cần
4. Ghi chú địa chỉ để sử dụng sau

**Ví dụ (Arduino với Modbus library):**
- Nhiệt độ: Address 0
- Áp suất: Address 1
- Độ ẩm: Address 2
- Trạng thái relay: Address 3

---

## Xử lý sự cố

### Vấn đề 1: Không tìm thấy cổng Serial

**Triệu chứng:**
- Danh sách port trống
- "No ports available"

**Nguyên nhân và cách khắc phục:**

| Nguyên nhân | Cách khắc phục |
|------------|-------------------|
| Thiết bị không được kết nối | Kiểm tra kabel USB, kết nối thiết bị lại |
| Driver USB không cài | Cài driver từ trang web nhà sản xuất hoặc [site:zadig.akeo.ie](http://zadig.akeo.ie) |
| Cổng bị chiếm bởi ứng dụng khác | Đóng phần mềm khác (Arduino IDE, Putty, v.v.) |
| Thiết bị không phải USB Serial | Kiểm tra xem thiết bị có hỗ trợ Serial không |

**Trên Windows:**
- Mở Device Manager (Ctrl+Shift+Esc)
- Tìm "COM Ports"
- Nếu có "Unknown Device", cài driver

**Trên macOS/Linux:**
- Mở Terminal
- Nhập: `ls /dev/tty*`
- Tìm cổng (ví dụ: `/dev/ttyUSB0`)

### Vấn đề 2: Kết nối được nhưng không nhận dữ liệu

**Triệu chứng:**
- Nút [Connect] xanh, nhưng yêu cầu trả về lỗi

**Nguyên nhân và cách khắc phục:**

| Triệu chứng | Nguyên nhân | Cách khắc phục |
|-----------|-----------|-----------|
| Timeout error | Thiết bị trả lời chậm | Tăng Timeout lên 2000-3000ms |
| CRC error (RTU) | Tham số Serial sai | Kiểm tra Baud Rate, Data Bits, Parity |
| Connection refused (TCP) | Thiết bị không hoạt động hoặc IP sai | Ping thiết bị, kiểm tra IP |
| Slave error | Slave ID sai | Kiểm tra Slave ID với nhà sản xuất |

### Vấn đề 3: Giá trị nhận được lạ

**Triệu chứng:**
- Giá trị là 0, 65535, hoặc không bao giờ thay đổi
- Giá trị âm lạ (khi dùng Unsigned format)

**Nguyên nhân và cách khắc phục:**

| Vấn đề | Giải pháp |
|-------|----------|
| Address sai | Kiểm tra Modbus Address Map trong tài liệu thiết bị |
| Quantity sai | Thử quantity = 1, 2, hoặc nhiều hơn |
| Format hiển thị sai | Thử chuyển từ Unsigned sang Signed hoặc Hex |
| Thiết bị không phản hồi | Kiểm tra xem thiết bị đã bật không, kết nối vật lý |

### Vấn đề 4: Polling không hoạt động

**Triệu chứng:**
- Nhấn [Auto Read] nhưng dữ liệu không cập nhật
- [Stop] không xuất hiện

**Nguyên nhân và cách khắc phục:**

| Vấn đề | Giải pháp |
|-------|----------|
| Poll Interval quá cao | Giảm Poll Interval (ví dụ: 5000 thành 1000ms) |
| Kết nối mất | Kiểm tra lại kết nối RTU/TCP |
| Thiết bị không hỗ trợ Polling | Kiểm tra tài liệu thiết bị, thử [Read] thủ công |

### Vấn đề 5: Chậm hoặc lag

**Triệu chứo:**
- Ứng dụng chậm, giao diện không phản hồi
- Response time cao (>100ms)

**Nguyên nhân và cách khắc phục:**

| Vấn đề | Giải pháp |
|-------|----------|
| Polling quá nhanh | Tăng Poll Interval |
| Quantity quá cao | Giảm Quantity (đọc ít register hơn) |
| Network bị tắc (TCP) | Kiểm tra kết nối mạng, giảm tốc độ polling |
| Máy tính quá yếu | Đóng các ứng dụng không cần |

---

## Tổng kết

Hãy nhớ:

1. **Chọn chế độ đúng**: RTU cho Serial, TCP cho Ethernet
2. **Cấu hình đúng**: Baud Rate, Parity, Slave ID, Address
3. **Chọn Function Code phù hợp**: FC03 là phổ biến nhất
4. **Sử dụng Polling để giám sát tự động**
5. **Xem Transaction Log để tìm lỗi**

Nếu gặp vấn đề, hãy:
1. Kiểm tra tài liệu thiết bị Modbus
2. Kiểm tra kết nối vật lý
3. Xem nhật ký giao dịch
4. Xin trợ giúp từ nhà sản xuất thiết bị

Chúc bạn sử dụng TermiPro Modbus Master vui vẻ!
