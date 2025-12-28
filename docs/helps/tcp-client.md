# Hướng Dẫn Sử Dụng TCP Client

Hướng dẫn chi tiết cách sử dụng tính năng TCP Client trong TermiPro để kết nối và giao tiếp với các máy chủ TCP từ xa.

---

## Mục lục

1. [Giới thiệu](#giới-thiệu)
2. [Kết nối TCP Server](#kết-nối-tcp-server)
3. [Gửi dữ liệu](#gửi-dữ-liệu)
4. [Auto Send - Gửi Tự động](#auto-send---gửi-tự-động)
5. [Terminal Hiển thị](#terminal-hiển-thị)
6. [Trạng thái Kết nối](#trạng-thái-kết-nối)
7. [Mẹo và Thủ thuật](#mẹo-và-thủ-thuật)
8. [Xử lý Lỗi](#xử-lý-lỗi)

---

## Giới thiệu

### TCP Client là gì?

**TCP Client** là một công cụ cho phép máy tính của bạn **kết nối đến một máy chủ TCP từ xa** để gửi và nhận dữ liệu qua mạng. Điều này rất hữu ích khi:

- Bạn cần kiểm tra giao tiếp với một máy chủ (server) qua mạng
- Kiểm tra các API hoặc dịch vụ chạy trên máy chủ
- Debug ứng dụng phân tán (distributed applications)
- Giao tiếp với thiết bị IoT hoặc embedded systems qua TCP
- Kiểm tra các giao thức tùy chỉnh

### Khi nào sử dụng TCP Client?

| Tình huống | Sử dụng TCP Client? |
|-----------|------------------|
| Kết nối đến server qua mạng | ✓ Có |
| Giao tiếp với cổng serial USB | ✗ Không (dùng Serial Connection) |
| Kiểm tra API web | ✓ Có (nếu cần giao tiếp TCP thô) |
| Kiểm tra IoT device qua mạng | ✓ Có |
| Giao tiếp Modbus TCP | ✓ Có |

---

## Kết nối TCP Server

### Chuẩn bị trước khi kết nối

Trước khi bắt đầu, bạn cần biết:

1. **Địa chỉ Host**: IP address hoặc tên miền của máy chủ
   - Ví dụ: `192.168.1.100`, `localhost`, `example.com`
2. **Port**: Cổng TCP mà máy chủ đang lắng nghe
   - Ví dụ: `8080`, `5000`, `3000`, `9600`

> **Lưu ý:** Máy chủ phải đang chạy và lắng nghe trên địa chỉ IP và cổng bạn cung cấp. Nếu máy chủ không chạy, kết nối sẽ thất bại.

### Các bước kết nối

1. **Tìm phần "TCP Client"** ở phía bên trái màn hình (sidebar)
2. **Nhập Host Address** (địa chỉ máy chủ)
   - Nhập IP: `192.168.1.100` hoặc `localhost`
   - Hoặc nhập tên miền: `example.com`
3. **Nhập Port** (cổng)
   - Ví dụ: `8080` hoặc `5000`
4. **Nhấp nút "Connect"** (nút xanh)
5. **Chờ kết nối thành công**
   - Nút sẽ thay đổi thành "Disconnect" (đỏ)
   - Terminal sẽ hiển thị "Đã kết nối"

### Ví dụ thực tế

#### Ví dụ 1: Kết nối đến máy chủ cục bộ (Local)

```
Host: localhost
Port: 8080
```

Sử dụng khi máy chủ đang chạy trên máy tính của bạn.

#### Ví dụ 2: Kết nối đến máy chủ trên cùng mạng

```
Host: 192.168.1.50
Port: 5000
```

Sử dụng khi máy chủ chạy trên máy tính khác trong cùng mạng WiFi/LAN.

#### Ví dụ 3: Kết nối đến máy chủ từ xa

```
Host: api.example.com
Port: 443
```

Sử dụng khi máy chủ chạy trên internet công cộng.

### Các tính năng khi kết nối

- **Host và Port bị khóa**: Khi đã kết nối, bạn không thể thay đổi Host/Port. Bạn phải **Disconnect** trước
- **Trạng thái hiển thị**: Màn hình sẽ cho biết đang kết nối hay không
- **Tự động kết nối lại**: Nếu mất kết nối, TermiPro sẽ tự động thử kết nối lại

---

## Gửi dữ liệu

### Hiểu về hai chế độ gửi

TermiPro hỗ trợ hai cách gửi dữ liệu:

#### 1. Text Mode (Chế độ Văn bản)

- **Sử dụng khi**: Gửi text bình thường
- **Ví dụ**: `Hello`, `GET /api/data HTTP/1.1`, `AT+GMMI`
- **Tính năng**: Tự động thêm ký tự xuống dòng (line ending) nếu được cấu hình

#### 2. Hex Mode (Chế độ Thập lục phân)

- **Sử dụng khi**: Gửi dữ liệu nhị phân hoặc byte cụ thể
- **Ví dụ**: `48 65 6C 6C 6F`, `01 02 FF`
- **Tính năng**: Không thêm ký tự xuống dòng tự động

### Gửi dữ liệu Text mode

1. **Chắc chắn đã kết nối** (nút Connect phải là "Disconnect" - đỏ)
2. **Chắc chắn toggle "Hex" tắt** (không được bật)
3. **Nhập tin nhắn** vào ô input ở dưới cùng
   - Ví dụ: `Hello Server`
4. **Nhấn nút "Send"** hoặc **Enter**
5. **Xem kết quả** trong Terminal

**Ví dụ thực tế:**

```
Bạn gửi: Hello
Terminal hiển thị: TX: Hello
```

### Gửi dữ liệu Hex mode

1. **Chắc chắn đã kết nối**
2. **Bật toggle "Hex"** (sẽ sáng lên)
3. **Nhập byte hex** cách nhau bằng **khoảng trắng**
   - Ví dụ: `48 65 6C 6C 6F` (là từ "Hello" ở dạng hex)
4. **Nhấn nút "Send"** hoặc **Enter**

**Định dạng Hex được hỗ trợ:**

| Định dạng | Ví dụ | Ghi chú |
|-----------|-------|---------|
| Cách khoảng trắng | `48 65 6C 6C 6F` | Được khuyến khích |
| Với tiền tố 0x | `0x48 0x65 0x6C 0x6F` | Cũng được hỗ trợ |
| Kết hợp cả hai | `48 0x65 6C 0x6F` | Flexible nhất |

**Bảng chuyển đổi Hex phổ biến:**

| Ký tự | Hex | | Ký tự | Hex |
|-------|-----|---|-------|-----|
| A | 41 | | a | 61 |
| H | 48 | | h | 68 |
| CR (xuống dòng) | 0D | | LF (line feed) | 0A |

> **Lưu ý:** Nếu muốn gửi "Hello" ở Hex mode **không có xuống dòng**, hãy gửi `48 65 6C 6C 6F`. Nếu muốn có xuống dòng, thêm `0D 0A` vào cuối.

### Cấu hình Line Ending (Ký tự Xuống dòng)

Bạn có thể chọn cách thêm ký tự xuống dòng trong **Text mode**:

| Tùy chọn | Ký hiệu | Mô tả | Khi nào dùng |
|---------|---------|-------|------------|
| **None** | N | Không thêm | Máy chủ không yêu cầu |
| **CR** | `\r` | Carriage Return (byte 0D) | Một số thiết bị lỗi thời |
| **LF** | `\n` | Line Feed (byte 0A) | Unix/Linux |
| **CRLF** | `\r\n` | CR + LF (byte 0D 0A) | Windows, HTTP, SMTP |

**Ví dụ:**
```
Text mode, Line Ending = CRLF:
Bạn gửi: Hello
Máy chủ nhận: Hello\r\n (byte: 48 65 6C 6C 6F 0D 0A)

Text mode, Line Ending = None:
Bạn gửi: Hello
Máy chủ nhận: Hello (byte: 48 65 6C 6C 6F)
```

> **Khuyến nghị:** Hầu hết các máy chủ yêu cầu **CRLF** (Windows-style) hoặc **LF** (Unix-style). Kiểm tra tài liệu máy chủ để chắc chắn.

### Các phím tắt hữu ích

| Phím tắt | Chức năng |
|---------|---------|
| **Enter** | Gửi dữ liệu |
| **Shift + Enter** | Xuống dòng trong ô input (nếu cần gửi đa dòng) |
| **Ctrl + A** | Chọn tất cả |
| **Ctrl + C** | Sao chép |
| **Ctrl + V** | Dán |

> **Lưu ý Mac:** Thay `Ctrl` bằng `Cmd`

### Ô Input - Giữ nội dung sau khi gửi

**Đặc điểm quan trọng:** Ô input **không bị xóa tự động** sau khi gửi. Điều này có lợi:

- Bạn có thể **gửi lại** cùng dữ liệu mà không cần nhập lại
- Bạn có thể **chỉnh sửa nhẹ** rồi gửi lại
- Dữ liệu sẽ được **dùng cho Auto Send**

**Nút Clear (X):** Nhấp vào dấu `X` ở bên phải ô input để xóa nội dung.

---

## Auto Send - Gửi Tự động

### Giới thiệu Auto Send

**Auto Send** cho phép bạn gửi dữ liệu **liên tục theo chu kỳ** mà không cần nhấn nút gửi lần lần.

### Khi nào sử dụng Auto Send

- Gửi lệnh PING định kỳ để kiểm tra kết nối
- Cập nhật dữ liệu cảm biến liên tục
- Test hiệu suất máy chủ với dữ liệu lặp lại
- Giám sát máy chủ với interval cố định

### Cách sử dụng Auto Send

1. **Nhập dữ liệu** vào ô input ở dưới cùng
   - Ví dụ: `GET /status`
2. **Chắc chắn đã kết nối**
3. **Nhấp nút "Auto"** (xanh)
4. **Điều chỉnh tần suất** (nếu cần):
   - Mở phần **"Auto Send Settings"** ở sidebar
   - Thay đổi **"Frequency"** (từ 50ms đến 60.000ms)
5. **Xem dữ liệu được gửi** trong Terminal
   - Mỗi lần gửi sẽ hiển thị một dòng TX

### Cấu hình Tần suất (Frequency)

| Giá trị | Mô tả | Sử dụng khi |
|--------|-------|-----------|
| 50ms | Rất nhanh | Kiểm tra hiệu suất, test stress |
| 100ms | Nhanh | Giám sát thời gian thực nhanh |
| 500ms | Bình thường | Kiểm tra lệnh thông thường |
| 1000ms (1s) | Chậm | Cập nhật định kỳ, giám sát |
| 5000ms (5s) | Rất chậm | Kiểm tra kết nối, lệnh hiếm |

> **Lưu ý:** Bạn **không thể thay đổi tần suất khi Auto Send đang chạy**. Bạn phải nhấp nút "Stop" trước.

### Dừng Auto Send

1. **Nhấp nút "Stop"** (đỏ) ở dưới cùng
2. Auto Send sẽ dừng ngay lập tức
3. Bạn có thể thay đổi dữ liệu và tần suất

### Thông tin Auto Send

Khi Auto Send chạy, bạn sẽ thấy:

- **Biểu tượng "ON"** ở phần **Auto Send Settings**
- **Số lần đã gửi** hiển thị trong Auto Send Settings
- **Terminal cập nhật** mỗi lần gửi

---

## Terminal Hiển thị

### TX vs RX

Terminal hiển thị hai loại dữ liệu:

#### TX - Dữ liệu Bạn Gửi

- **Viết tắt:** Transmit (truyền)
- **Màu sắc:** Cam/vàng
- **Ý nghĩa:** Dữ liệu từ máy tính bạn gửi đi
- **Khi nào xuất hiện:** Mỗi khi bạn nhấn Send hoặc Auto Send gửi dữ liệu

**Ví dụ:**
```
TX: GET /api/status
TX: Hello Server
```

#### RX - Dữ liệu Máy chủ Gửi về

- **Viết tắt:** Receive (nhận)
- **Màu sắc:** Xanh
- **Ý nghĩa:** Dữ liệu từ máy chủ gửi về cho bạn
- **Khi nào xuất hiện:** Khi máy chủ gửi dữ liệu

**Ví dụ:**
```
RX: HTTP/1.1 200 OK
RX: {"status": "OK"}
```

### Thống kê TX/RX

Ở phần trên cùng Terminal, bạn sẽ thấy thống kê:

```
TX ↑ 15  |  RX ↓ 23
```

| Thống kê | Ý nghĩa | Sử dụng |
|---------|---------|--------|
| **TX ↑ 15** | Bạn đã gửi 15 lần | Kiểm tra số lần gửi |
| **RX ↓ 23** | Bạn đã nhận 23 lần | Kiểm tra máy chủ có phản hồi |

**Cách đọc thống kê:**
- TX cao nhưng RX thấp → Máy chủ có thể không phản hồi
- RX cao nhưng TX thấp → Máy chủ gửi dữ liệu mà không yêu cầu
- Cả hai bằng 0 → Chưa có giao tiếp

### Hai chế độ hiển thị: Text vs Hex

#### Text View (Hiển thị Chữ)

- **Hiển thị:** Dữ liệu dưới dạng ký tự
- **Dùng khi:** Gửi/nhận text bình thường
- **Ví dụ:**
  ```
  TX: GET /status
  RX: {"status": "OK"}
  ```
- **Lợi ích:** Dễ đọc
- **Hạn chế:** Không hiển thị ký tự kiểm soát

#### Hex View (Hiển thị Hex)

- **Hiển thị:** Dữ liệu dưới dạng thập lục phân
- **Dùng khi:** Cần xem chi tiết từng byte
- **Ví dụ:**
  ```
  TX: 48 65 6C 6C 6F
  RX: 52 45 41 44 59
  ```
- **Lợi ích:** Thấy chính xác từng byte, phát hiện dữ liệu ẩn
- **Hạn chế:** Khó đọc hơn

**Cách chuyển đổi:**
- Tìm nút **"Text"** hoặc **"Hex"** ở phần **Display** trong sidebar
- Nhấp để chuyển đổi

### Auto Scroll - Tự động Cuộn

Terminal sẽ **tự động cuộn xuống** để luôn hiển thị dữ liệu mới nhất. Điều này rất hữu ích khi có nhiều dữ liệu.

**Tạm dừng Auto Scroll:**
- Kéo lên phía trên để xem dữ liệu cũ
- Kéo xuống dưới cùng để trở lại theo dõi thời gian thực

### Nút Clear - Xóa Terminal

- **Vị trí:** Phần trên cùng Terminal, nút dấu thùng rác
- **Chức năng:** Xóa tất cả dữ liệu hiển thị
- **Khi nào dùng:**
  - Muốn bắt đầu lại theo dõi
  - Terminal có quá nhiều dữ liệu
  - Muốn reset thống kê TX/RX

---

## Trạng thái Kết nối

### Các trạng thái kết nối

| Trạng thái | Biểu tượng | Ý nghĩa |
|-----------|-----------|--------|
| **Connected** | Nút đỏ "Disconnect" | Đã kết nối thành công, có thể gửi/nhận dữ liệu |
| **Disconnected** | Nút xanh "Connect" | Không kết nối, không thể gửi dữ liệu |
| **Reconnecting** | Nút vàng "Reconnecting" với icon quay tròn | Đang thử kết nối lại sau khi mất kết nối |
| **Error** | Thông báo lỗi đỏ | Lỗi kết nối (máy chủ không tìm thấy, từ chối kết nối, v.v.) |

### Thông báo Trạng thái

Ở phần **TCP Client** sidebar, bạn sẽ thấy thông báo:

**Kết nối thành công:**
```
✓ Connected
```

**Đang thử kết nối lại (mất kết nối tạm thời):**
```
⟳ Reconnecting... (1/3)
```

Điều này có nghĩa máy chủ đã ngắt kết nối, TermiPro đang tự động thử lại. Nó sẽ cố gắng tối đa 3 lần.

**Lỗi kết nối:**
```
⚠ Connection refused on localhost:8080
```

Máy chủ không chấp nhận kết nối. Lý do có thể:
- Máy chủ không chạy
- Port sai
- Máy chủ từ chối kết nối
- Tường lửa chặn

### Tự động Kết nối lại

Nếu mất kết nối với máy chủ, TermiPro sẽ **tự động cố gắng kết nối lại**:

1. **Lần 1:** Sau 1 giây
2. **Lần 2:** Sau 1 giây nữa
3. **Lần 3:** Sau 1 giây nữa
4. **Nếu thất bại:** Hiển thị lỗi, dừng kết nối

Bạn có thể nhấp nút "Disconnect" bất kỳ lúc nào để dừng việc kết nối lại.

---

## Mẹo và Thủ thuật

### Mẹo 1: Kiểm tra kết nối nhanh

Gửi một tin nhắn đơn giản để kiểm tra máy chủ có phản hồi:

1. Nhập: `PING` hoặc `Hello`
2. Nhấn Send
3. Nếu thấy RX → Máy chủ hoạt động
4. Nếu không thấy RX → Máy chủ không phản hồi (hoặc không xử lý tin nhắn)

### Mẹo 2: Debug giao thức HTTP

Để kiểm tra máy chủ HTTP thô:

```
Gửi:
GET /api/data HTTP/1.1
Host: example.com

Nhấn Send
```

Bạn sẽ thấy phản hồi HTTP từ máy chủ.

### Mẹo 3: Sử dụng Auto Send để giám sát

Để kiểm tra máy chủ còn sống không:

1. Nhập: `PING`
2. Đặt Auto Send = 5000ms (5 giây)
3. Nhấp Auto
4. Terminal sẽ gửi PING mỗi 5 giây
5. Nếu thấy RX → Máy chủ còn hoạt động

### Mẹo 4: Sao chép và chỉnh sửa dữ liệu

1. Xem dữ liệu RX trong Terminal
2. Sao chép (Ctrl+C)
3. Dán vào ô input (Ctrl+V)
4. Chỉnh sửa nếu cần
5. Gửi lại (Enter)

### Mẹo 5: Kiểm tra Hex view khi dữ liệu lộn xộn

Nếu dữ liệu hiển thị sai ký tự:

1. Chuyển sang **Hex View**
2. Bạn sẽ thấy byte thực tế
3. Giúp phát hiện vấn đề encoding hoặc dữ liệu nhị phân

### Mẹo 6: Tìm tần suất Auto Send phù hợp

| Mục tiêu | Tần suất |
|---------|---------|
| Kiểm tra kết nối | 5-10 giây (5000-10000ms) |
| Cập nhật thường xuyên | 1 giây (1000ms) |
| Kiểm tra hiệu suất | 100-500ms |
| Test stress | 50ms |

---

## Xử lý Lỗi

### Lỗi 1: "Connection refused"

**Nguyên nhân:**
- Máy chủ không chạy
- Port sai
- Máy chủ từ chối kết nối
- Tường lửa chặn

**Cách khắc phục:**
1. Kiểm tra host và port có đúng không
2. Chắc chắn máy chủ đang chạy
3. Thử kết nối từ máy khác xem có vấn đề hay không
4. Kiểm tra tường lửa

### Lỗi 2: "Host not found" hoặc "Unknown host"

**Nguyên nhân:**
- Tên miền không tồn tại hoặc sai tên
- Không có kết nối internet
- DNS không phân giải được

**Cách khắc phục:**
1. Kiểm tra tên miền (spelling)
2. Thử ping trong terminal: `ping example.com`
3. Thử dùng IP address thay vì tên miền
4. Kiểm tra kết nối internet

### Lỗi 3: "Connection timeout"

**Nguyên nhân:**
- Máy chủ không phản hồi (có thể chạy quá chậm)
- Mạng quá chậm
- Máy chủ bị crash

**Cách khắc phục:**
1. Chờ một lúc và thử lại
2. Kiểm tra kết nối mạng
3. Thử kết nối từ máy khác
4. Kiểm tra máy chủ (CPU, RAM)

### Lỗi 4: Gửi dữ liệu nhưng không nhận được phản hồi

**Nguyên nhân có thể:**
- Máy chủ không xử lý dữ liệu
- Định dạng dữ liệu sai
- Máy chủ không gửi phản hồi
- Dữ liệu bị mất trên đường

**Cách khắc phục:**
1. Kiểm tra Hex View xem dữ liệu TX có đúng không
2. Kiểm tra line ending (CRLF, LF, hoặc None)
3. Đọc tài liệu máy chủ để hiểu format yêu cầu
4. Thử gửi dữ liệu đơn giản trước (ví dụ: `PING`)
5. Kiểm tra máy chủ có log để xem có nhận dữ liệu không

### Lỗi 5: Dữ liệu nhận được lộn xộn hoặc sai ký tự

**Nguyên nhân:**
- Encoding sai (UTF-8, ASCII, Latin-1)
- Dữ liệu nhị phân, không phải text
- Dữ liệu bị hỏng

**Cách khắc phục:**
1. Chuyển sang **Hex View** để xem byte thực tế
2. Kiểm tra tài liệu máy chủ về encoding
3. Nếu dữ liệu nhị phân, dùng Hex View để xem
4. Thử lại kết nối

### Lỗi 6: Input bị disable (không gõ được)

**Nguyên nhân:**
- Auto Send đang chạy
- Chưa kết nối

**Cách khắc phục:**
1. Nếu Auto Send chạy: Nhấp nút **Stop**
2. Nếu chưa kết nối: Nhấp nút **Connect** trước

### Lỗi 7: Hex mode không chấp nhận dữ liệu

**Nguyên nhân:**
- Format hex sai
- Byte không phải hex hợp lệ (00-FF)
- Không cách bằng khoảng trắng

**Cách khắc phục:**
1. Kiểm tra mỗi byte phải là `00` - `FF`
2. Đảm bảo các byte cách nhau bằng **1 khoảng trắng**
3. Không dùng dấu phẩy hoặc dấu khác

**Ví dụ sai:**
```
Sai: 48,65,6C,6C,6F (dấu phẩy)
Sai: 486c65c6c6f (không có khoảng trắng)
Sai: 48 65 6C 6L 6F (6L không phải hex)
```

**Ví dụ đúng:**
```
Đúng: 48 65 6C 6C 6F
Đúng: 0x48 0x65 0x6C 0x6F
Đúng: 48 0x65 6C 0x6F
```

---

## Tóm tắt Các Tính Năng

| Tính năng | Mô tả |
|----------|-------|
| **Host + Port** | Nhập địa chỉ máy chủ và cổng |
| **Connect/Disconnect** | Kết nối/ngắt kết nối máy chủ |
| **Text Mode** | Gửi dữ liệu text bình thường |
| **Hex Mode** | Gửi dữ liệu hex/nhị phân |
| **Line Ending** | Chọn ký tự xuống dòng (None, CR, LF, CRLF) |
| **Send** | Gửi dữ liệu một lần |
| **Auto Send** | Gửi dữ liệu liên tục theo chu kỳ |
| **Terminal** | Hiển thị TX/RX real-time |
| **Text/Hex View** | Chuyển đổi cách hiển thị dữ liệu |
| **TX/RX Stats** | Thống kê số lần gửi/nhận |
| **Clear Terminal** | Xóa dữ liệu terminal |
| **Status Message** | Hiển thị trạng thái kết nối |
| **Auto Reconnect** | Tự động kết nối lại khi mất kết nối |

---

## Liên hệ Hỗ trợ

Nếu vẫn gặp vấn đề:

1. Kiểm tra phần "Xử lý Lỗi" ở trên
2. Đảm bảo:
   - Host address đúng (hoặc IP của máy chủ)
   - Port đúng
   - Máy chủ đang chạy
   - Line ending phù hợp
3. Kiểm tra tài liệu máy chủ hoặc dịch vụ
4. Thử từ máy khác để xác nhận vấn đề

**Cập nhật lần cuối:** Tháng 12 năm 2025
