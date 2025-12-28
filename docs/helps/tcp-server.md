# Hướng dẫn sử dụng TCP Server trong TermiPro

Chào mừng bạn đến với hướng dẫn **TCP Server** của TermiPro. Tính năng này cho phép máy tính của bạn hoạt động như một **máy chủ TCP**, có thể nhận kết nối từ nhiều thiết bị cùng một lúc.

## Mục lục

1. [Giới thiệu TCP Server](#giới-thiệu-tcp-server)
2. [Khi nào sử dụng TCP Server](#khi-nào-sử-dụng-tcp-server)
3. [Cấu hình Server](#cấu-hình-server)
4. [Khởi động Server](#khởi-động-server)
5. [Quản lý Client kết nối](#quản-lý-client-kết-nối)
6. [Chế độ Echo](#chế-độ-echo)
7. [Gửi dữ liệu đến Clients](#gửi-dữ-liệu-đến-clients)
8. [Giám sát Terminal](#giám-sát-terminal)
9. [Mẹo sử dụng](#mẹo-sử-dụng)
10. [Xử lý sự cố](#xử-lý-sự-cố)

---

## Giới thiệu TCP Server

### TCP Server là gì?

**TCP Server** là chế độ cho phép TermiPro trở thành một máy chủ mạng (server). Thay vì kết nối đến một cổng Serial cơ bản, máy tính của bạn sẽ:

- **Mở một cổng mạng** và chờ các kết nối từ các thiết bị khác
- **Nhận kết nối** từ nhiều máy khách (clients) cùng một lúc
- **Gửi và nhận dữ liệu** với từng client một cách độc lập
- **Quản lý** tất cả các kết nối từ một giao diện duy nhất

### Ví dụ đơn giản

Hãy tưởng tượng bạn có một cái chuông cửa:
- **TCP Server**: Là cái chuông được gắn trên cửa (chờ đợi)
- **Clients**: Là các khách hàng bấm chuông (kết nối)
- **Dữ liệu**: Là những gì khách hàng nói khi bấm chuông

---

## Khi nào sử dụng TCP Server

TCP Server rất hữu ích cho các tình huống sau:

### 1. **Kiểm thử nhiều thiết bị cùng lúc**
- Bạn có 3-4 bo mạch phát triển cần giao tiếp với máy tính
- Thay vì mở nhiều cổng COM riêng biệt, bạn dùng một TCP Server

**Ví dụ**: Kiểm thử 3 cảm biến nhiệt độ khác nhau, mỗi cái kết nối qua TCP

### 2. **Mô phỏng thiết bị (Simulation)**
- Bạn muốn giả lập một thiết bị hoặc một hệ thống trên máy tính
- Các ứng dụng khác kết nối đến server của bạn để kiểm thử

**Ví dụ**: Mô phỏng một bộ điều khiển (controller) và các client khác nhau gửi yêu cầu

### 3. **Tập trung dữ liệu từ nhiều nguồn**
- Nhiều thiết bị nhúng khác nhau gửi dữ liệu về một máy tính
- Bạn có thể giám sát tất cả từ TermiPro

**Ví dụ**: 5 robot khác nhau gửi trạng thái về máy tính chủ

### 4. **Phát sóng lệnh đến toàn bộ thiết bị**
- Bạn muốn gửi một lệnh duy nhất đến tất cả các client kết nối
- Ví dụ: "RESET" - tất cả thiết bị nhận được lệnh này

### 5. **Phát triển ứng dụng IoT hoặc mạng nội bộ**
- Luyện tập xây dựng ứng dụng giao tiếp qua mạng
- Tạo prototype đơn giản trước khi phát triển đầy đủ

---

## Cấu hình Server

Trước khi khởi động server, bạn cần cấu hình các tham số cơ bản.

### Bước 1: Mở tab TCP Server

1. Nhấp vào nút **"+"** hoặc **"Thêm tab"** ở trên giao diện
2. Chọn **"TCP Server"** từ danh sách
3. Một tab mới sẽ mở ra với các cấu hình server

### Bước 2: Cấu hình cổng (Port)

**Port** là địa chỉ mà server sẽ lắng nghe. Hãy xem nó như số phòng:

> **Ví dụ**: Nếu máy tính của bạn là tòa nhà, port 5000 là phòng 5000. Clients sẽ "gõ cửa" phòng này để kết nối.

**Cách cấu hình:**

1. Tìm mục **"Port"** ở Sidebar trái
2. Nhập một số từ **1024** đến **65535**
3. Các port phổ biến:
   - **5000**: Mặc định, dễ nhớ
   - **8000-8999**: Thường dùng cho ứng dụng web
   - **10000**: Cổng cao, ít bị chiếm
   - **9600, 10001**: Thường dùng cho IoT

> **Lưu ý**: Tránh các port < 1024 (cần quyền admin). Tránh port đang được sử dụng (ví dụ: port của ứng dụng khác).

**Nếu gặp lỗi "Port already in use"**:
- Thử thay đổi port (ví dụ: từ 5000 sang 5001)
- Hoặc đóng ứng dụng khác đang chiếm port đó

### Bước 3: Cấu hình Bind Address (Địa chỉ lắng nghe)

**Bind Address** xác định máy tính nào có thể kết nối:

| Địa chỉ | Ý nghĩa | Khi nào sử dụng |
|---------|---------|-----------------|
| `0.0.0.0` | **Mọi máy** (mạng nội bộ + local) | Muốn các thiết bị khác kết nối từ mạng |
| `127.0.0.1` | **Chỉ máy tính của bạn** (localhost) | Chỉ test trên máy tính local, không cho mạng khác truy cập |

**Lựa chọn đơn giản:**
- **Để mặc định `0.0.0.0`** nếu bạn muốn các thiết bị khác kết nối
- **Chọn `127.0.0.1`** nếu chỉ test trên máy của bạn

### Bước 4: Cấu hình Max Clients (Số lượng client tối đa)

**Max Clients** giới hạn số lượng thiết bị có thể kết nối cùng một lúc.

- **Mặc định**: 20 clients
- **Tối đa**: 20 clients
- **Tối thiểu khuyến nghị**: 1

**Cách cấu hình:**

1. Tìm mục **"Max Clients"** ở Sidebar
2. Nhập số lượng (ví dụ: 5, 10, 20)

> **Ví dụ**: Nếu bạn chỉ cần kiểm thử 3 thiết bị, đặt Max Clients = 3. Thiết bị thứ 4 sẽ bị từ chối kết nối.

### Cấu hình ví dụ

Dưới đây là một ví dụ cấu hình đơn giản:

```
┌─────────────────────────────────────┐
│  Cấu hình TCP Server                │
├─────────────────────────────────────┤
│  Port: 5000                         │
│  Bind Address: 0.0.0.0              │
│  Max Clients: 10                    │
│                                     │
│  [Khởi động Server]  [Dừng Server] │
└─────────────────────────────────────┘
```

---

## Khởi động Server

Sau khi cấu hình, bạn có thể khởi động server.

### Bước 1: Kiểm tra cấu hình

Trước khi khởi động, hãy kiểm tra lại:
- ✓ Port: Hợp lệ (1024-65535) và không bị chiếm
- ✓ Bind Address: Chọn đúng (0.0.0.0 hoặc 127.0.0.1)
- ✓ Max Clients: Đủ lớn cho nhu cầu của bạn

### Bước 2: Nhấp nút "Khởi động Server"

1. Tìm nút **"Khởi động Server"** (Start Server) ở Sidebar
2. Nhấp vào nó
3. Nếu thành công:
   - Header sẽ hiển thị **"Server đang chạy"** (màu xanh)
   - Nút thay đổi thành **"Dừng Server"** (Stop Server)
   - Terminal sẽ hiển thị: `Server listening on 0.0.0.0:5000` (hoặc port bạn đã cấu hình)

### Bước 3: Nếu gặp lỗi

**Lỗi: "Port already in use"**
- Đổi port sang số khác (ví dụ: 5001, 5002)
- Hoặc đóng ứng dụng khác đang dùng port

**Lỗi: "Permission denied"**
- Port < 1024 cần quyền admin
- Chọn port >= 1024
- Hoặc chạy TermiPro dưới quyền admin

> **Lưu ý**: Sau khi khởi động server, bạn có thể ngay lập tức thấy clients kết nối nếu có. Bước tiếp theo là chờ các clients kết nối.

---

## Quản lý Client kết nối

Khi server đang chạy, các thiết bị (clients) có thể kết nối. TermiPro sẽ hiển thị tất cả các kết nối.

### Xem danh sách Clients

**Vị trí**: Phía dưới nút "Khởi động Server" ở Sidebar, có một phần **"Clients kết nối"** (Connected Clients)

**Thông tin hiển thị cho mỗi client:**

| Thông tin | Ý nghĩa |
|-----------|---------|
| **Client ID** | Mã định danh (ví dụ: client-1, client-2) |
| **Địa chỉ IP:Port** | Nơi client kết nối từ (ví dụ: 192.168.1.100:54321) |
| **Thời gian kết nối** | Khi nào client kết nối |
| **Trạng thái** | "Đã kết nối" (Connected) |

### Ví dụ danh sách clients

```
Clients kết nối (3):
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. client-1
   IP: 192.168.1.100:54321
   Kết nối lúc: 14:30:25

2. client-2
   IP: 192.168.1.101:54322
   Kết nối lúc: 14:30:45

3. client-3
   IP: 192.168.1.102:54323
   Kết nối lúc: 14:31:00
```

### Ngắt kết nối một Client

Nếu bạn muốn **đặc biệt** ngắt kết nối một client mà không dừng server:

**Cách làm:**

1. Tìm client trong danh sách **"Clients kết nối"** ở Sidebar
2. Nhấp vào client đó
3. Nhấp nút **"Ngắt kết nối"** (Disconnect) hoặc biểu tượng X

**Kết quả:**
- Client bị ngắt kết nối
- Các client khác vẫn kết nối bình thường
- Terminal sẽ hiển thị: `Client [client-1] disconnected`

### Ngắt kết nối tất cả Clients

Nếu bạn muốn ngắt **toàn bộ clients** nhưng vẫn giữ server chạy:

1. Nhấp nút **"Ngắt kết nối tất cả"** (Disconnect All) nếu có
2. Hoặc dừng server rồi khởi động lại (xem phần "Dừng Server")

### Dừng Server

Để dừng server hoàn toàn:

1. Nhấp nút **"Dừng Server"** (Stop Server) ở Sidebar
2. Tất cả clients sẽ bị ngắt kết nối
3. Header sẽ hiển thị **"Server đã dừng"** (màu xám)
4. Nút thay đổi lại thành **"Khởi động Server"**

---

## Chế độ Echo

**Echo** là một tính năng tự động: khi client gửi dữ liệu, server sẽ **gửi lại** dữ liệu đó cho client.

### Khi nào sử dụng Echo?

| Tình huống | Dùng Echo? |
|-----------|-----------|
| **Kiểm thử kết nối** | ✓ Có - xác nhận dữ liệu được gửi |
| **Gỡ lỗi giao tiếp** | ✓ Có - kiểm tra dữ liệu có bị thay đổi |
| **Ứng dụng thực tế** | ✗ Không - server sẽ xử lý dữ liệu, không lặp lại |

### Ví dụ Echo

```
1. Client gửi: "Hello"
   ↓
2. Server nhận: "Hello"
   ↓ (Nếu Echo bật)
3. Server gửi về: "Echo: Hello"
   ↓
4. Client nhận: "Echo: Hello"
```

### Bật/Tắt Echo

**Cách làm:**

1. Tìm mục **"Chế độ Echo"** ở Sidebar
2. Nhấp vào **"Bật"** (Enable) hoặc **"Tắt"** (Disable)
3. Hoặc nhấp vào nút toggle (Bật/Tắt)

**Trạng thái hiển thị:**
- **Bật** (Enable): Badge xanh, Echo hoạt động
- **Tắt** (Disable): Badge xám, Echo tắt

### Quan sát Echo trong Terminal

Khi Echo bật:

```
[14:30:25] client-1 (RX): Hello
[14:30:25] Server (TX): Echo: Hello
[14:30:26] client-1 (RX): Test
[14:30:26] Server (TX): Echo: Test
```

---

## Gửi dữ liệu đến Clients

TermiPro cho phép bạn gửi dữ liệu một cách linh hoạt.

### 3 chế độ gửi dữ liệu

| Chế độ | Nơi nhận | Khi nào dùng |
|-------|---------|-------------|
| **Gửi đến tất cả** | Broadcast (toàn bộ clients) | Lệnh chung cho mọi thiết bị |
| **Gửi đến client cụ thể** | Một client duy nhất | Lệnh riêng cho thiết bị đó |
| **Gửi tự động** | Tất cả/hoặc client cụ thể | Gửi dữ liệu định kỳ |

### Gửi Text (Văn bản)

**Cách gửi đến tất cả clients:**

1. Ở Footer (chân trang), tìm ô nhập liệu
2. Gõ dữ liệu (ví dụ: `RESET` hoặc `STATUS?`)
3. Chọn **"Text Mode"** (nếu chưa chọn)
4. Nhấp nút **"Gửi đến tất cả"** (Broadcast)

**Kết quả:**
- Tất cả clients sẽ nhận: `RESET`
- Terminal sẽ hiển thị: `[14:35:00] Server (TX): RESET` (Broadcast)

**Ví dụ:**

```
Input: STATUS?
Mode: Text
Nhấp: Gửi đến tất cả

Tất cả clients nhận: "STATUS?"
```

### Gửi Hex (Dữ liệu nhị phân)

**Khi sử dụng Hex:**
- Bạn muốn gửi dữ liệu nhị phân (bytes)
- Dữ liệu không phải là văn bản thường

**Định dạng Hex đúng:**
```
48 65 6C 6C 6F
↓  ↓  ↓  ↓  ↓
H  e  l  l  o
```

**Cách gửi Hex:**

1. Ở Footer, tìm ô nhập liệu
2. Nhập dữ liệu hex (cách nhau bằng khoảng trắng): `48 65 6C 6C 6F`
3. Chọn **"Hex Mode"**
4. Nhấp **"Gửi đến tất cả"**

**Kết quả:**
- Tất cả clients nhận: byte `0x48 0x65 0x6C 0x6C 0x6F`
- Terminal hiển thị: `[14:35:10] Server (TX): 48 65 6C 6C 6F` (Hex)

### Gửi đến Client cụ thể

Nếu bạn chỉ muốn gửi lệnh đến **một thiết bị**:

**Cách làm:**

1. Ở Sidebar, tìm **"Clients kết nối"**
2. Chọn client mà bạn muốn gửi (nhấp vào nó)
3. Ở Footer, nhập dữ liệu
4. Nhấp nút **"Gửi"** (Send) hoặc **"Gửi đến [client-1]"**

**Kết quả:**
- Chỉ client được chọn nhận dữ liệu
- Các client khác không nhận

### Ví dụ thực tế: Gửi lệnh khác nhau

```
Ứng dụng: Điều khiển 3 robot khác nhau

1. Gửi "MOVE_FORWARD" đến robot-1
   Cách: Chọn client-1 → Nhập "MOVE_FORWARD" → Gửi

2. Gửi "TURN_LEFT" đến robot-2
   Cách: Chọn client-2 → Nhập "TURN_LEFT" → Gửi

3. Gửi "STATUS?" đến tất cả
   Cách: Chọn "Broadcast" → Nhập "STATUS?" → Gửi
```

### Gửi tự động (Auto Send)

Tương tự như Serial Connection, bạn có thể gửi dữ liệu định kỳ:

**Cách cấu hình:**

1. Tìm mục **"Gửi tự động"** (Auto Send) ở Sidebar
2. Nhập dữ liệu muốn gửi lặp lại
3. Cấu hình **Interval** (khoảng thời gian, ví dụ: 1000ms = 1 giây)
4. Chọn **gửi đến tất cả** hoặc **client cụ thể**
5. Nhấp **"Start"**

**Ví dụ:**

```
Dữ liệu: PING
Interval: 5000ms (5 giây)
Gửi đến: Tất cả

Kết quả: Mỗi 5 giây, tất cả clients nhận "PING"
```

---

## Giám sát Terminal

Terminal hiển thị **tất cả** giao tiếp (TX/RX) giữa server và clients.

### Thành phần Terminal

```
┌─────────────────────────────────────────────────────────┐
│  Terminal - TCP Server                                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ [14:30:25] client-1 (RX): Hello                        │
│ [14:30:25] Server (TX): Echo: Hello                    │
│ [14:30:26] client-2 (RX): STATUS?                      │
│ [14:30:26] Server (TX): OK                             │
│ [14:30:27] Server (TX): Broadcast message              │
│                                                         │
│ TX: 3  |  RX: 2                                        │
├─────────────────────────────────────────────────────────┤
│ [Text Mode] [Hex Mode] [Clear]                         │
└─────────────────────────────────────────────────────────┘
```

### Hiểu các loại tin nhắn

| Loại | Biểu tượng | Ý nghĩa |
|------|-----------|---------|
| **RX** | Xanh (💚) | Client gửi dữ liệu về server |
| **TX** | Vàng (💛) | Server gửi dữ liệu cho client |
| **Status** | Xám | Thông báo (kết nối, ngắt kết nối, lỗi) |

### Ví dụ đọc Terminal

```
[14:30:00] client-1 connected from 192.168.1.100:54321   ← Client kết nối
[14:30:05] client-1 (RX): HELLO                          ← Client gửi
[14:30:05] Server (TX): Echo: HELLO                      ← Server gửi lại
[14:30:10] client-2 connected from 192.168.1.101:54322   ← Client 2 kết nối
[14:30:15] Server (TX): Broadcast STATUS CHECK           ← Gửi đến tất cả
[14:30:15] client-1 (RX): ACK                            ← Client 1 phản hồi
[14:30:16] client-2 (RX): ACK                            ← Client 2 phản hồi
[14:30:20] client-1 disconnected                         ← Client 1 mất kết nối
```

### Thống kê TX/RX

Ở trên Terminal, bạn sẽ thấy:

```
TX: 15  |  RX: 8
```

**Ý nghĩa:**
- **TX**: Số gói tin server **gửi** đi
- **RX**: Số gói tin server **nhận** từ clients

### Chuyển đổi Text/Hex

**Nút "Text Mode"** (mặc định):
- Dữ liệu hiển thị dạng văn bản
- Ví dụ: `Hello World`

**Nút "Hex Mode"**:
- Dữ liệu hiển thị dạng Hex
- Ví dụ: `48 65 6C 6C 6F 20 57 6F 72 6C 64`

**Khi nào chuyển sang Hex Mode?**
- Khi nhận dữ liệu nhị phân (ảnh, dữ liệu cảm biến phức tạp)
- Để debug dữ liệu không phải UTF-8

### Xóa Terminal

Nếu Terminal quá nhiều dữ liệu cũ:

1. Nhấp nút **"Clear"**
2. Terminal sẽ được xóa sạch
3. Tiếp tục nhận dữ liệu mới

---

## Mẹo sử dụng

### Mẹo 1: Kiểm thử TCP Server trên máy tính của bạn

**Cách tạo client test đơn giản:**

**Trên Windows (cmd hoặc PowerShell):**
```bash
# Kết nối đến server
telnet localhost 5000

# Gõ dữ liệu và bấm Enter
Hello
Test
```

**Trên macOS/Linux (terminal):**
```bash
# Sử dụng nc (netcat)
nc localhost 5000

# Hoặc telnet
telnet localhost 5000

# Gõ dữ liệu
Hello
Test
```

**Kết quả:**
- TermiPro sẽ hiển thị client kết nối
- Dữ liệu bạn gõ sẽ xuất hiện trên Terminal
- Nếu Echo bật, bạn sẽ nhận lại "Echo: Hello"

### Mẹo 2: Kiểm thử nhiều clients cùng lúc

**Mở 2 Terminal/CMD:**

```
Terminal 1:                    Terminal 2:
nc localhost 5000             nc localhost 5000
Hello                          Test
DATA1                          DATA2
```

**Kết quả:**
- TermiPro sẽ hiển thị 2 clients
- Mỗi client được quản lý riêng

### Mẹo 3: Ghi chép cấu hình Server

Tương tự như Serial Connection, ghi lại cấu hình của các server bạn thường dùng:

| Tên Server | Port | Bind Address | Max Clients | Ghi chú |
|-----------|------|--------------|------------|---------|
| Robot Test | 5000 | 0.0.0.0 | 5 | 3 robot + 2 control |
| Sensor Monitor | 5001 | 127.0.0.1 | 10 | Test trên máy local |

### Mẹo 4: Sử dụng Echo để debug

1. Bật **Echo**
2. Gửi dữ liệu từ client
3. Kiểm tra dữ liệu "Echo" trả về
4. Nếu Echo giống dữ liệu gửi → Kết nối ổn
5. Nếu khác → Có vấn đề kết nối hoặc dữ liệu

### Mẹo 5: Phân biệt giữa "Gửi đến tất cả" và "Gửi đến client cụ thể"

**Gửi đến tất cả (Broadcast):**
- Nhấp vào phần trống (không chọn client)
- Tất cả clients hiện tại sẽ nhận

**Gửi đến client cụ thể:**
- Nhấp vào client trong danh sách
- Chỉ client đó nhận
- Nếu client kết nối lại, bạn phải chọn lại

### Mẹo 6: Giám sát hiệu suất

Nếu server có nhiều clients:

- **TX cao, RX thấp**: Server gửi nhiều, clients gửi ít
- **TX thấp, RX cao**: Clients gửi nhiều, server gửi ít
- **Cân bằng**: Giao tiếp hai chiều bình thường

---

## Xử lý sự cố

### Vấn đề 1: "Port already in use"

**Nguyên nhân**: Port bạn chọn đang được sử dụng bởi ứng dụng khác

**Giải pháp:**

1. **Đổi port:**
   - Thay đổi port từ 5000 → 5001, 5002, v.v.
   - Nhấp "Khởi động Server" lại

2. **Kiểm tra ứng dụng khác:**
   - Đóng các ứng dụng khác dùng port đó
   - Khởi động lại TermiPro

3. **Khởi động lại máy tính** (nếu vẫn không được)

### Vấn đề 2: Clients không thể kết nối

**Nguyên nhân có thể:**
- Bind address sai (chọn 127.0.0.1 thay vì 0.0.0.0)
- Tường lửa (Firewall) chặn port
- Client kết nối sai port hoặc địa chỉ IP

**Giải pháp:**

1. **Kiểm tra Bind Address:**
   - Chắc chắn chọn **0.0.0.0** (nếu clients từ máy khác)
   - Chọn **127.0.0.1** (nếu chỉ test trên máy local)

2. **Kiểm tra Tường lửa:**
   - Cho phép TermiPro qua tường lửa
   - Hoặc tạm tắt tường lửa để test

3. **Kiểm tra Client:**
   - Chắc chắn client dùng đúng địa chỉ IP và port
   - Ví dụ: `nc 192.168.1.100 5000` (điều chỉnh IP phù hợp)

4. **Test với localhost trước:**
   - Khởi động server với Bind Address = 0.0.0.0
   - Mở Terminal khác trên máy tính: `nc localhost 5000`
   - Nếu thành công → Server ổn, vấn đề ở client

### Vấn đề 3: Dữ liệu bị sai lệch hoặc hỏng

**Nguyên nhân có thể:**
- Client gửi dữ liệu không đúng định dạng
- Kết nối mạng không ổn định

**Giải pháp:**

1. **Bật Echo để debug:**
   - Bật chế độ Echo
   - Gửi dữ liệu từ client
   - Kiểm tra "Echo" trả về

2. **Chuyển sang Hex Mode:**
   - Xem dữ liệu ở dạng hex
   - Kiểm tra byte-by-byte

3. **Kiểm tra client:**
   - Đảm bảo client gửi dữ liệu đúng định dạng

### Vấn đề 4: Server "lag" hoặc chậm khi có quá nhiều dữ liệu

**Nguyên nhân:** Terminal có quá nhiều tin nhắn cũ

**Giải pháp:**

1. **Xóa Terminal:**
   - Nhấp nút **"Clear"** để xóa dữ liệu cũ

2. **Giảm số lượng clients:**
   - Ngắt kết nối các clients không cần
   - Giảm Max Clients

3. **Tắt Echo** (nếu bật):
   - Echo sẽ tạo thêm dữ liệu trên Terminal
   - Tắt nó để giảm tải

### Vấn đề 5: Client đột ngột mất kết nối

**Nguyên nhân:**
- Client tự ngắt kết nối
- Kết nối mạng không ổn định
- Client bị sự cố

**Giải pháp:**

1. **Kiểm tra Terminal:**
   - Xem có thông báo "disconnected" không

2. **Thử kết nối lại:**
   - Khởi động lại client
   - Server sẽ nhận kết nối mới

3. **Kiểm tra mạng:**
   - Chắc chắn kết nối mạng ổn định
   - Không có packet loss

### Vấn đề 6: "Permission denied" khi khởi động server

**Nguyên nhân:** Chạy ứng dụng không đủ quyền

**Giải pháp:**

1. **Chọn port >= 1024:**
   - Port < 1024 cần quyền admin
   - Chọn port từ 1024 trở lên

2. **Chạy TermiPro với quyền Admin:**
   - **Windows**: Chuột phải → "Run as Administrator"
   - **macOS**: Terminal → `sudo open /Applications/TermiPro.app`
   - **Linux**: `sudo ./TermiPro.AppImage`

---

## Tóm tắt quy trình sử dụng TCP Server

```
1. Cấu hình:
   ├─ Chọn Port (ví dụ: 5000)
   ├─ Chọn Bind Address (0.0.0.0 hoặc 127.0.0.1)
   └─ Cấu hình Max Clients

2. Khởi động:
   └─ Nhấp "Khởi động Server"

3. Chờ Clients kết nối:
   ├─ Xem danh sách Clients ở Sidebar
   └─ Giám sát Terminal

4. Gửi dữ liệu:
   ├─ Gửi đến tất cả: Broadcast
   └─ Gửi đến client cụ thể: Chọn client

5. Tắt Server:
   └─ Nhấp "Dừng Server"
```

---

## Các lệnh phổ biến dùng với TCP Server

### Kiểm thử kết nối

```bash
# Trên Windows
telnet localhost 5000

# Trên macOS/Linux
nc localhost 5000
# hoặc
telnet localhost 5000
```

### Kiểm thử từ máy khác trên mạng

```bash
# Trên máy khác, thay localhost bằng IP máy chủ
nc 192.168.1.100 5000
# hoặc
telnet 192.168.1.100 5000
```

### Tắt client

```bash
# Trên Windows (trong cmd)
Ctrl + ]
quit

# Trên macOS/Linux
Ctrl + C
```

---

## Hỏi đáp nhanh (FAQ)

### Q: TCP Server khác Serial Connection gì?

**A:**
- **Serial Connection**: Kết nối đến 1 cụm thiết bị vật lý (COM/TTY port)
- **TCP Server**: Là máy chủ mạng, nhận nhiều kết nối từ phần mềm/thiết bị khác

### Q: Tôi có thể chạy nhiều TCP Server cùng lúc không?

**A:** Có, nhưng mỗi server phải dùng port khác (ví dụ: 5000, 5001, 5002)

### Q: Port nào an toàn để sử dụng?

**A:** Port từ 5000-65535 thường an toàn. Tránh:
- Port < 1024 (cần admin)
- Port đang sử dụng bởi ứng dụng khác

### Q: Echo có ảnh hưởng đến hiệu suất không?

**A:** Rất ít. Nó chỉ tạo thêm một bản sao dữ liệu gửi lại. Có thể tắt nếu không cần.

### Q: Làm cách nào để gửi dữ liệu nhị phân (binary)?

**A:** Sử dụng **Hex Mode**:
- Nhập dữ liệu dạng hex (ví dụ: `00 FF AA BB`)
- Chọn "Hex Mode"
- Nhấp Gửi

### Q: Tôi có thể save dữ liệu nhận được không?

**A:** Hiện tại, bạn có thể:
- Copy từ Terminal
- Chụp ảnh (Screenshot)
- Ghi chép thủ công

---

## Bước tiếp theo

Sau khi nắm vững TCP Server:

1. **Thử nhiều clients:** Tạo 3-4 clients test để quen với quản lý
2. **Thử Echo Mode:** Debug kết nối bằng Echo
3. **Kết hợp với ứng dụng:** Viết một ứng dụng nhỏ kết nối tới TCP Server
4. **Tìm hiểu Modbus**: Nếu làm việc với thiết bị công nghiệp

---

**Chúc bạn sử dụng TCP Server của TermiPro vui vẻ!**

*Cập nhật lần cuối: 28/12/2025*
