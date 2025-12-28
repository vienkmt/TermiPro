# MQTT Client - Hướng dẫn sử dụng

Chào mừng bạn đến với tính năng **MQTT Client** của TermiPro. Hướng dẫn này sẽ giúp bạn nhanh chóng hiểu và sử dụng MQTT để kết nối với các hệ thống IoT, nhà thông minh, và các ứng dụng yêu cầu giao tiếp không dây.

## Mục lục

1. [Giới thiệu MQTT](#giới-thiệu-mqtt)
2. [Kết nối đến Broker](#kết-nối-đến-broker)
3. [Giao thức kết nối](#giao-thức-kết-nối)
4. [Last Will (LWT) - Di chúc máy](#last-will-lwt---di-chúc-máy)
5. [Đăng ký nhận tin (Subscribe)](#đăng-ký-nhận-tin-subscribe)
6. [Gửi tin nhắn (Publish)](#gửi-tin-nhắn-publish)
7. [Giám sát terminal](#giám-sát-terminal)
8. [Gửi tin tự động](#gửi-tin-tự-động)
9. [Mẹo sử dụng thực tế](#mẹo-sử-dụng-thực-tế)

---

## Giới thiệu MQTT

### MQTT là gì?

**MQTT** (Message Queuing Telemetry Transport) là một giao thức giao tiếp nhẹ được thiết kế cho các hệ thống **IoT (Internet of Things)** và các ứng dụng yêu cầu truyền dữ liệu hiệu quả qua mạng Internet.

#### Tại sao sử dụng MQTT?

- **Nhẹ và hiệu quả**: Sử dụng ít dung lượng dữ liệu, phù hợp cho các thiết bị có kết nối mạng yếu
- **Đáng tin cậy**: Hỗ trợ các cấp độ bảo đảm giao hàng khác nhau
- **Mở rộng được**: Dễ dàng kết nối hàng ngàn thiết bị cùng một lúc
- **Phổ biến**: Được sử dụng rộng rãi trong nhà thông minh, công nghiệp 4.0, theo dõi sức khỏe

### Các khái niệm cơ bản

#### 1. Broker (Trung tâm truyền tin)

**Broker** là một máy chủ MQTT mà tất cả các thiết bị kết nối đến. Nó nhận tin từ một thiết bị và phân phát cho những thiết bị khác.

- Ví dụ: **test.mosquitto.org**, **broker.hivemq.com** (công cộng miễn phí để thử nghiệm)

#### 2. Client (Ứng dụng/Thiết bị)

**Client** là bất kỳ ứng dụng nào kết nối đến Broker. TermiPro khi kết nối MQTT cũng trở thành một Client.

#### 3. Topic (Chủ đề)

**Topic** là một "kênh" hoặc "chủ đề" mà dữ liệu được gửi đến. Mỗi topic có một tên duy nhất.

Ví dụ:
- `home/living_room/temperature` - Nhiệt độ phòng khách
- `office/door/status` - Trạng thái cửa văn phòng
- `car/gps/location` - Vị trí của xe
- `plant/moisture/level` - Độ ẩm đất

#### 4. Message (Tin nhắn)

**Message** là dữ liệu được gửi qua một Topic. Nó có thể là:
- Văn bản thường: `"26.5"` (nhiệt độ)
- JSON: `{"temp": 26.5, "humidity": 60}`
- Số nhị phân: Dữ liệu cảm biến

### Mô hình Publish-Subscribe

MQTT hoạt động theo mô hình **Publish-Subscribe**:

```
┌──────────────┐        ┌────────────┐        ┌──────────────┐
│ Sensor nhệt  │        │   MQTT     │        │ Ứng dụng điều │
│   độ         │        │   Broker   │        │   khiển      │
└──────────────┘        └────────────┘        └──────────────┘
       │                      │                      │
       └──→ Publish          │        Subscribe ←────┘
           (Gửi dữ liệu)     │        (Nhận dữ liệu)
           home/temp         │
                             │
```

---

## Kết nối đến Broker

### Các bước kết nối cơ bản

#### Bước 1: Mở tab MQTT

1. Trong TermiPro, nhấp nút **"+"** để thêm tab mới
2. Chọn **"MQTT"** từ danh sách

#### Bước 2: Nhập thông tin Broker

Bạn sẽ thấy một biểu mẫu với các trường sau:

##### a) **Host (Địa chỉ Broker)**

Nhập địa chỉ IP hoặc tên miền của Broker.

Ví dụ:
- `test.mosquitto.org` - Broker công cộng (miễn phí, dùng để thử)
- `broker.hivemq.com` - Broker public HiveMQ
- `192.168.1.100` - Broker cục bộ (local)
- `mqtt.mycompany.com` - Broker riêng của công ty

##### b) **Port (Cổng kết nối)**

Nhập số cổng. Cổng mặc định tùy thuộc vào giao thức:

| Giao thức | Cổng mặc định |
|-----------|---------------|
| TCP | 1883 |
| TLS | 8883 |
| WebSocket (WS) | 8080 |
| WebSocket Secure (WSS) | 443 |

**Lưu ý:** Nếu Broker của bạn sử dụng cổng khác, hãy kiểm tra tài liệu của nhà cung cấp.

##### c) **Client ID (Mã định danh Client)**

Mỗi Client cần có một ID duy nhất để Broker nhận dạng. Ứng dụng sẽ tự tạo một ID nếu bạn không nhập.

Bạn có thể:
- Để trống để tự động tạo
- Nhập một tên dễ nhớ như: `sensor_01`, `home_app`, `phone_control`

##### d) **Username & Password (Tên đăng nhập & Mật khẩu)**

Nếu Broker yêu cầu xác thực:

1. Nhập **Username**
2. Nhập **Password**

Nếu không có xác thực, để trống.

**Mẹo:** Hầu hết các Broker công cộng (test.mosquitto.org, broker.hivemq.com) không yêu cầu xác thực.

##### e) **Keep Alive (Kiểm tra kết nối)**

Thời gian (giây) để kiểm tra xem kết nối có còn sống không. Mặc định là **60 giây**.

Để ý:
- Số nhỏ hơn = kiểm tra thường xuyên hơn, tốn điện năng hơn
- Số lớn hơn = kiểm tra ít hơn, nếu mất kết nối sẽ mất lâu hơn để phát hiện

**Khuyến nghị:** Giữ mặc định **60** nếu không chắc.

#### Bước 3: Kết nối

1. Nhấp nút **"Kết nối"** (Connect)
2. Đợi 1-2 giây để kết nối
3. Nếu thành công, bạn sẽ thấy:
   - Trạng thái hiển thị **"Đã kết nối"** (xanh)
   - Nút chuyển thành **"Ngắt kết nối"** (Disconnect)

---

## Giao thức kết nối

TermiPro hỗ trợ **4 giao thức** để kết nối đến Broker:

### 1. TCP (Không mã hóa)

- **Cổng**: 1883
- **Tốc độ**: Nhanh nhất
- **Bảo mật**: Không, dữ liệu gửi qua mạng không được mã hóa
- **Sử dụng khi**: Mạng nội bộ an toàn (trong nhà, công ty riêng tư)

**Ví dụ:**
```
Host: 192.168.1.100
Port: 1883
Protocol: TCP
```

### 2. TLS (Mã hóa)

- **Cổng**: 8883
- **Tốc độ**: Hơi chậm hơn TCP
- **Bảo mật**: Có, dữ liệu được mã hóa end-to-end
- **Sử dụng khi**: Kết nối qua Internet công cộng hoặc dữ liệu nhạy cảm

**Ví dụ:**
```
Host: broker.hivemq.com
Port: 8883
Protocol: TLS
```

### 3. WebSocket (WS)

- **Cổng**: 8080 (hoặc khác)
- **Bảo mật**: Không mã hóa
- **Đặc điểm**: Có thể sử dụng qua proxy web hoặc firewall
- **Sử dụng khi**: Kết nối từ trình duyệt web hoặc phía sau firewall hạn chế

**Ví dụ:**
```
Host: broker.hivemq.com
Port: 8000
Protocol: WebSocket (WS)
```

### 4. WebSocket Secure (WSS)

- **Cổng**: 443
- **Bảo mật**: Có, mã hóa + phía sau proxy web
- **Đặc điểm**: An toàn nhất, hoạt động qua hầu hết firewall
- **Sử dụng khi**: Kết nối công cộng an toàn qua web

**Ví dụ:**
```
Host: broker.hivemq.com
Port: 8884
Protocol: WebSocket Secure (WSS)
```

### Cách chọn giao thức?

| Tình huống | Giao thức |
|-----------|-----------|
| Mạng Wi-Fi nội bộ nhà | TCP |
| Kết nối từ Internet | TLS hoặc WSS |
| Qua proxy web/firewall | WS hoặc WSS |
| Để bảo mật tối đa | TLS hoặc WSS |

---

## Last Will (LWT) - Di chúc máy

### Last Will là gì?

**Last Will** (di chúc máy) là tin nhắn tự động được gửi đi khi Client ngắt kết nối không mong muốn (ví dụ: mất điện, sự cố mạng, lỗi ứng dụng).

### Khi nào sử dụng?

Ví dụ thực tế:

**Ứng dụng 1: Giám sát thiết bị**
- Client gửi: `home/sensor/status` = `"online"`
- Nếu Client mất kết nối, Last Will sẽ tự gửi: `"offline"`
- Ứng dụng khác sẽ biết cảm biến không hoạt động

**Ứng dụng 2: Thông báo máy chủ sự cố**
- Client gửi: `alert/server/power`
- Nếu mất điện, Last Will sẽ tự gửi: `"Server lost power"`
- Đội quản trị sẽ nhận được cảnh báo

### Cách cấu hình Last Will

1. Trong phần kết nối, tìm mục **"Last Will & Testament (LWT)"**
2. Nhập các thông tin:

#### a) **Topic (Chủ đề)**
Topic mà Last Will sẽ gửi đến. Ví dụ: `home/sensor/status`

#### b) **Message (Tin nhắn)**
Nội dung sẽ gửi. Ví dụ: `offline` hoặc `device_disconnected`

#### c) **QoS**
Mức độ bảo đảm giao hàng (xem phần "QoS" dưới):
- `0` - Không đảm bảo (nhanh)
- `1` - Đảm bảo giao (phổ biến)
- `2` - Đảm bảo chính xác (chậm)

**Khuyến nghị:** Chọn `1` cho hầu hết các trường hợp.

#### d) **Retain**
Nếu **bật**:
- Broker sẽ lưu tin nhắn cuối cùng này
- Khi một Client mới subscribe, nó sẽ nhận tin nhắn này ngay lập tức

Nếu **tắt**:
- Tin chỉ gửi nếu có Client đang subscribe

**Khuyến nghị:** Bật nếu bạn muốn các thiết bị mới biết trạng thái cũ nhất.

### Ví dụ cấu hình Last Will

```
Topic: home/devices/phone/status
Message: offline
QoS: 1
Retain: Yes (bật)
```

Khi ứng dụng TermiPro mất kết nối, Broker sẽ tự động gửi:
- **Topic:** `home/devices/phone/status`
- **Message:** `offline`
- Các ứng dụng subscribe sẽ nhận được tin này

---

## Đăng ký nhận tin (Subscribe)

### Subscribe là gì?

**Subscribe** nghĩa là "đăng ký" để nhận tin từ một Topic. Khi có dữ liệu mới trên Topic đó, Client sẽ nhận được ngay.

### Các bước đăng ký

#### Bước 1: Nhập Topic

1. Tìm ô **"Topic"** trong phần Subscribe
2. Nhập tên Topic muốn nhận tin
3. Ví dụ: `home/living_room/temperature`

#### Bước 2: Chọn QoS

**QoS (Quality of Service)** là mức độ bảo đảm giao hàng tin nhắn:

##### QoS 0: **At Most Once** (Tối đa một lần)

- **Mô tả**: Gửi tin nhắn 1 lần, không đảm bảo
- **Tốc độ**: Nhanh nhất
- **Độ tin cậy**: Thấp nhất - có thể mất tin
- **Sử dụng**: Dữ liệu theo thời gian thực không quan trọng (ví dụ: cảm biến nhiệt độ)

**Ví dụ:**
```
Topic: home/temp
QoS: 0
(Nếu kết nối bị sập khi gửi, tin có thể bị mất)
```

##### QoS 1: **At Least Once** (Ít nhất một lần)

- **Mô tả**: Gửi lại tin nếu chưa nhận được xác nhận
- **Tốc độ**: Trung bình
- **Độ tin cậy**: Trung bình - đảm bảo nhận ít nhất 1 lần (nhưng có thể nhận duplicate)
- **Sử dụng**: Hầu hết các ứng dụng thông thường

**Ví dụ:**
```
Topic: home/light/switch
QoS: 1
(Đảm bảo lệnh bật/tắt đèn được gửi)
```

##### QoS 2: **Exactly Once** (Chính xác một lần)

- **Mô tả**: Đảm bảo giao hàng chính xác một lần, không bị mất hoặc duplicate
- **Tốc độ**: Chậm nhất
- **Độ tin cậy**: Cao nhất
- **Sử dụng**: Giao dịch tài chính, điều khiển quan trọng

**Ví dụ:**
```
Topic: bank/transfer/confirmation
QoS: 2
(Đảm bảo ghi nhận chính xác 1 lần)
```

### Bảng so sánh QoS

| QoS | Tốc độ | Độ tin cậy | Sử dụng khi |
|-----|--------|-----------|-----------|
| 0 | Nhanh nhất | Thấp | Dữ liệu theo thời gian thực không quan trọng |
| 1 | Trung bình | Trung bình | Hầu hết các ứng dụng thông thường |
| 2 | Chậm nhất | Cao nhất | Giao dịch quan trọng, lệnh điều khiển |

**Khuyến nghị:** Sử dụng **QoS 1** cho hầu hết trường hợp.

#### Bước 3: Nhấp Subscribe

1. Tìm nút **"Subscribe"** (hoặc biểu tượng dấu cộng)
2. Nhấp để đăng ký

#### Bước 4: Xem dữ liệu

Sau khi subscribe thành công:
- Dữ liệu sẽ hiển thị trên **Terminal** (phía bên phải)
- Mỗi tin nhận được sẽ có nhãn **RX** (xanh)
- Bạn sẽ thấy Topic, nội dung, và thời gian

### Đăng ký nhiều Topic

Bạn có thể đăng ký nhiều Topic cùng lúc:

1. Nhập Topic thứ nhất, chọn QoS, nhấp Subscribe
2. Nhập Topic thứ hai, chọn QoS, nhấp Subscribe
3. Tiếp tục...

**Ví dụ:**
```
1. home/living_room/temperature (QoS 0)
2. home/living_room/humidity (QoS 0)
3. home/lights/status (QoS 1)
```

### Sử dụng Wildcard (Ký tự đại diện)

MQTT cho phép sử dụng **ký tự đại diện** để subscribe nhiều Topic cùng lúc:

#### `+` - Thay thế một cấp

```
home/+/temperature
```

Sẽ subscribe:
- `home/living_room/temperature` ✓
- `home/kitchen/temperature` ✓
- `home/bedroom/temperature` ✓

Nhưng **KHÔNG**:
- `home/temperature` (thiếu cấp giữa)
- `home/living_room/sensor/temperature` (quá nhiều cấp)

#### `#` - Thay thế bất kỳ số cấp nào

```
home/#
```

Sẽ subscribe:
- `home/temperature` ✓
- `home/light/status` ✓
- `home/sensor/temp/value` ✓

**Lưu ý:** `#` phải ở cuối. Ví dụ: `home/#` đúng, `home/#/status` sai.

---

## Gửi tin nhắn (Publish)

### Publish là gì?

**Publish** nghĩa là "gửi" một tin nhắn đến một Topic. Tất cả các Client subscribe Topic đó sẽ nhận được tin nhắn này.

### Các bước gửi tin

#### Bước 1: Nhập Topic

1. Tìm ô **"Topic"** trong phần Publish
2. Nhập tên Topic muốn gửi đến
3. Ví dụ: `home/lights/living_room`

#### Bước 2: Nhập Payload (Nội dung tin)

Nhập nội dung muốn gửi. Có 4 định dạng:

##### 1. **Text (Văn bản)**

Gửi dữ liệu dạng văn bản thường.

**Ví dụ:**
```
Payload: on
Topic: home/lights/status
→ Gửi: on (tin nhắn tắt hay bật đèn)

Payload: 26.5
Topic: home/temp
→ Gửi: 26.5 (nhiệt độ)
```

**Khi sử dụng:**
- Lệnh bật/tắt: `on`, `off`, `true`, `false`
- Giá trị số: `26.5`, `100`, `0`
- Tin nhắn: `hello`, `alarm triggered`

##### 2. **JSON (Đối tượng dữ liệu)**

Gửi dữ liệu có cấu trúc (nhiều trường).

**Ví dụ:**
```
Payload: {"temp": 26.5, "humidity": 60, "location": "living_room"}
Topic: home/sensor/data
→ Gửi: Đối tượng JSON với 3 trường

Payload: {"status": "ok", "battery": 85}
Topic: device/status
→ Gửi: Trạng thái thiết bị
```

**Khi sử dụng:**
- Gửi nhiều giá trị cùng lúc
- Dữ liệu có cấu trúc phức tạp
- Ứng dụng nhận JSON tự động phân tích

**Lưu ý:** JSON phải đúng định dạng, có dấu ngoặc kép `""`

##### 3. **Hex (Thập lục phân)**

Gửi dữ liệu dạng nhị phân được mã hóa hex.

**Ví dụ:**
```
Payload: 48 65 6C 6C 6F
→ Gửi: H e l l o (5 byte)

Payload: 01 02 03 FF
→ Gửi: 4 byte dữ liệu nhị phân
```

**Khi sử dụng:**
- Gửi lệnh nhị phân tùy chỉ
- Giao thức cảm biến yêu cầu byte cụ thể
- Dữ liệu không phải là văn bản

**Cách nhập:** Các byte được cách nhau bằng **khoảng trắng**

##### 4. **Base64 (Mã hóa Base64)**

Gửi dữ liệu đã mã hóa Base64.

**Ví dụ:**
```
Payload: SGVsbG8gV29ybGQ=
→ Gửi: "Hello World" (mã hóa Base64)
```

**Khi sử dụng:**
- Gửi tệp nhị phân nhỏ (ảnh, âm thanh)
- Dữ liệu cần mã hóa
- Giao thức yêu cầu Base64

#### Bước 3: Chọn QoS

Chọn mức độ bảo đảm giao hàng (như phần Subscribe):
- **QoS 0** - Nhanh, không đảm bảo
- **QoS 1** - Đảm bảo giao ít nhất 1 lần
- **QoS 2** - Chính xác 1 lần

**Khuyến nghị:** **QoS 1** cho hầu hết trường hợp.

#### Bước 4: Chọn Retain (Lưu giữ)

**Retain** (giữ lại) - Broker sẽ lưu tin nhắn cuối cùng.

**Nếu bật:**
- Broker sẽ lưu tin này
- Khi Client mới subscribe Topic, sẽ nhận tin này ngay lập tức
- **Sử dụng khi:** Trạng thái không thay đổi thường xuyên (đèn bật/tắt, cửa mở/đóng)

**Nếu tắt:**
- Tin chỉ gửi cho Client đang subscribe hiện tại
- Khi Client mới subscribe, sẽ không nhận tin cũ
- **Sử dụng khi:** Sự kiện một lần (cảnh báo, thông báo)

#### Bước 5: Nhấp Publish

1. Tìm nút **"Publish"** (gửi)
2. Nhấp để gửi tin nhắn
3. Tin nhắn sẽ hiển thị trên Terminal với nhãn **TX** (vàng)

### Ví dụ gửi tin thực tế

#### Ví dụ 1: Bật đèn

```
Topic: home/lights/living_room
Payload: on
Format: Text
QoS: 1
Retain: Yes
```

Tất cả Client subscribe `home/lights/living_room` sẽ nhận và bật đèn.

#### Ví dụ 2: Gửi dữ liệu cảm biến

```
Topic: home/sensors/temperature
Payload: {"temp": 26.5, "unit": "C", "timestamp": 1640000000}
Format: JSON
QoS: 0
Retain: No
```

Các ứng dụng giám sát sẽ nhận dữ liệu nhiệt độ có cấu trúc rõ ràng.

#### Ví dụ 3: Gửi lệnh nhị phân

```
Topic: device/sensor/config
Payload: 01 04 00 00 00 0A
Format: Hex
QoS: 1
Retain: No
```

Gửi lệnh cấu hình Modbus hoặc giao thức nhị phân khác.

---

## Giám sát Terminal

### Terminal là gì?

**Terminal** là khu vực hiển thị tất cả các tin nhắn đã gửi (TX) và nhận (RX).

### Bố cục Terminal

```
┌─────────────────────────────────────────────────────┐
│ Tin nhắn RX (nhận)                                  │
│ [RX] home/temp: 26.5                                │
│                                                     │
│ Tin nhắn TX (gửi)                                   │
│ [TX] home/lights: on                                │
│                                                     │
│ Tin nhắn RX khác                                    │
│ [RX] home/humidity: 60                              │
└─────────────────────────────────────────────────────┘
```

### Các thông tin hiển thị

Mỗi tin nhắn sẽ hiển thị:

- **Nhãn (TX/RX)**:
  - **TX** (vàng) = Tin bạn gửi
  - **RX** (xanh) = Tin bạn nhận
- **Topic**: Chủ đề
- **Payload**: Nội dung
- **Thời gian**: Khi tin được gửi/nhận
- **QoS & Retain**: Các thuộc tính của tin

### Lọc tin nhắn

Bạn có thể lọc để chỉ xem:

1. **Tất cả (All)** - Hiển thị TX và RX
2. **Chỉ TX** - Hiển thị các tin bạn gửi
3. **Chỉ RX** - Hiển thị các tin bạn nhận

**Cách sử dụng:**
- Tìm dropdown **"Filter"** hoặc nút lọc ở trên Terminal
- Chọn loại tin muốn xem

### Chuyển đổi định dạng hiển thị

Bạn có thể xem dữ liệu ở các định dạng khác nhau:

1. **Text** - Văn bản thường
2. **Hex** - Thập lục phân (để xem byte)
3. **JSON** - Định dạng JSON (tự động sắp xếp)
4. **Base64** - Mã hóa Base64

**Cách sử dụng:**
- Tìm nút hoặc dropdown **"Display Format"**
- Chọn định dạng muốn xem
- Terminal sẽ tự động chuyển đổi hiển thị

### Xóa Terminal

Nếu Terminal quá đông dữ liệu cũ:

1. Nhấp nút **"Clear"**
2. Tất cả dữ liệu cũ sẽ bị xóa
3. Terminal sẽ trống, sẵn sàng nhận dữ liệu mới

### Thống kê TX/RX

Ở phía trên cùng Terminal, bạn sẽ thấy:

```
TX: 25 | RX: 48
```

- **TX: 25** = Bạn đã gửi 25 tin
- **RX: 48** = Bạn đã nhận 48 tin

Giúp bạn theo dõi lượng giao tiếp.

---

## Gửi tin tự động

### Auto Publish là gì?

**Auto Publish** (gửi tự động) cho phép bạn gửi cùng một tin nhắn lặp lại với một khoảng thời gian nhất định.

### Khi nào sử dụng?

**Ví dụ 1: Kiểm tra sức khỏe (Heartbeat)**
- Gửi `heartbeat` mỗi 10 giây để cho Broker biết thiết bị vẫn hoạt động
- Nếu không gửi trong 30 giây, Broker sẽ biết thiết bị offline

**Ví dụ 2: Dữ liệu theo định kỳ**
- Gửi dữ liệu cảm biến mỗi 1 phút
- Ví dụ: `{"temp": 25, "humidity": 60}`

**Ví dụ 3: Lệnh điều khiển lặp**
- Gửi lệnh bật máy bơm nước mỗi 5 phút

### Các bước cấu hình

#### Bước 1: Chuẩn bị tin nhắn

1. Nhập **Topic** và **Payload** như bình thường
2. Chọn **Format** (Text, JSON, Hex, Base64)
3. Chọn **QoS** và **Retain**

#### Bước 2: Cấu hình Interval (Khoảng thời gian)

1. Tìm ô **"Interval"** hoặc **"Frequency"**
2. Nhập số **mili-giây (ms)** hoặc **giây (s)** giữa các lần gửi

**Ví dụ:**
- `1000` = 1 giây
- `5000` = 5 giây
- `60000` = 1 phút

#### Bước 3: Nhấp Start

1. Tìm nút **"Start Auto"** hoặc **"Auto Start"**
2. Nhấp để bắt đầu
3. Nút sẽ thay đổi thành **"Stop Auto"**
4. Bộ đếm sẽ hiển thị số lần đã gửi: `Count: 1`, `Count: 2`, ...

#### Bước 4: Dừng lại

1. Nhấp nút **"Stop Auto"**
2. Việc gửi tự động sẽ dừng

### Ví dụ cấu hình

#### Ví dụ 1: Heartbeat

```
Topic: device/heartbeat
Payload: {"status": "alive", "timestamp": null}
Format: JSON
QoS: 0
Interval: 10000 (10 giây)
```

Mỗi 10 giây, TermiPro sẽ gửi heartbeat để cho Broker biết nó vẫn kết nối.

#### Ví dụ 2: Dữ liệu cảm biến

```
Topic: home/sensors/data
Payload: {"temp": 25.0, "humidity": 55}
Format: JSON
QoS: 1
Interval: 60000 (1 phút)
```

Mỗi phút, TermiPro sẽ gửi dữ liệu cảm biến.

#### Ví dụ 3: Kiểm tra trạng thái

```
Topic: home/check
Payload: ping
Format: Text
QoS: 0
Interval: 30000 (30 giây)
```

Mỗi 30 giây, gửi "ping" để các ứng dụng biết TermiPro còn sống.

---

## Mẹo sử dụng thực tế

### Mẹo 1: Sử dụng Broker công cộng để thử nghiệm

Để học MQTT mà không cần cài đặt Broker riêng:

**HiveMQ Public Broker**
```
Host: broker.hivemq.com
Port: 1883 (TCP) hoặc 8883 (TLS)
Protocol: TCP hoặc TLS
Username/Password: Không cần
```

**Test Mosquitto**
```
Host: test.mosquitto.org
Port: 1883 (TCP)
Protocol: TCP
Username/Password: Không cần
```

**Lợi ích:**
- Miễn phí
- Không cần cài đặt
- Dùng để học và thử nghiệm
- Không bảo đảm độ tin cậy cho ứng dụng thực tế

### Mẹo 2: Sử dụng cấu trúc Topic rõ ràng

Tạo Topic theo mô hình phân cấp dễ hiểu:

```
[lĩnh vực]/[vị trí]/[thiết bị]/[chức năng]
```

**Ví dụ cho nhà thông minh:**
```
home/living_room/light/status      (Trạng thái đèn phòng khách)
home/living_room/light/brightness  (Độ sáng đèn)
home/bedroom/temp/value            (Nhiệt độ phòng ngủ)
home/bedroom/window/open           (Cửa sổ mở/đóng)
home/kitchen/air_purifier/mode     (Chế độ máy lọc không khí)
```

**Lợi ích:**
- Dễ tìm kiếm
- Dễ quản lý khi có nhiều thiết bị
- Có thể subscribe bằng wildcard: `home/+/temp/+`

### Mẹo 3: Sử dụng JSON để gửi dữ liệu có cấu trúc

Thay vì gửi từng Topic riêng:
```
Topic: home/sensor/temp
Payload: 25.5

Topic: home/sensor/humidity
Payload: 60

Topic: home/sensor/pressure
Payload: 1013
```

Gửi một Topic với dữ liệu JSON:
```
Topic: home/sensor/data
Payload: {"temp": 25.5, "humidity": 60, "pressure": 1013}
```

**Lợi ích:**
- Giảm số Topic
- Dữ liệu có cấu trúc rõ ràng
- Dễ xử lý trên ứng dụng nhận

### Mẹo 4: Sử dụng Retain cho trạng thái, không dùng cho sự kiện

**Sử dụng Retain:**
```
Topic: home/lights/status
Payload: on
Retain: Yes ✓
(Trạng thái đèn, ở lại trong Broker)
```

**Không sử dụng Retain:**
```
Topic: home/alert/motion
Payload: {"motion_detected": true}
Retain: No ✓
(Sự kiện phát hiện chuyển động, không cần lưu)
```

### Mẹo 5: Sử dụng QoS thích hợp

| Loại dữ liệu | QoS | Lý do |
|---|---|---|
| Sự kiện quan trọng (lệnh bật/tắt) | 1 | Đảm bảo giao hàng |
| Cảm biến, đọc giá trị | 0 | Nhanh, dữ liệu mới sẽ sớm đến |
| Giao dịch, thanh toán | 2 | Chính xác, không được bỏ hoặc lặp |

### Mẹo 6: Giám sát kết nối với Last Will

Luôn cấu hình Last Will cho các ứng dụng quan trọng:

```
Topic: app/status
Message: offline
QoS: 1
Retain: Yes
```

Khi ứng dụng mất kết nối, các ứng dụng khác sẽ biết ngay.

### Mẹo 7: Sử dụng TLS hoặc WSS cho kết nối công cộng

**Không nên dùng TCP không mã hóa** khi:
- Kết nối qua Internet
- Dữ liệu nhạy cảm (mật khẩu, thông tin cá nhân)
- Truy cập từ mạng công cộng

**Sử dụng TLS hoặc WSS** để:
- Mã hóa dữ liệu
- Xác thực Broker
- Bảo vệ thông tin

### Mẹo 8: Tổ chức Subscribe theo dự án

Nếu quản lý nhiều hệ thống:

**Hệ thống 1: Nhà thông minh**
```
Topic: home/#
Topic: home/lights/+
Topic: home/sensors/+
```

**Hệ thống 2: Ô tô**
```
Topic: car/gps/#
Topic: car/engine/+
Topic: car/alarm/+
```

**Hệ thống 3: Công nghiệp**
```
Topic: factory/equipment/+
Topic: factory/sensors/+
Topic: factory/alerts/+
```

Mỗi tab MQTT riêng cho một hệ thống.

---

## Khắc phục sự cố

### Không thể kết nối đến Broker

**Nguyên nhân có thể:**
1. Sai Host hoặc Port
2. Broker không hoạt động
3. Mạng bị cắt
4. Tường lửa chặn

**Giải pháp:**
1. Kiểm tra lại Host và Port trong tài liệu Broker
2. Thử kết nối với Broker công cộng (test.mosquitto.org)
3. Kiểm tra kết nối Internet
4. Nếu dùng TLS, thử TCP; nếu dùng WSS, thử WS

### Không nhận dữ liệu dù đã Subscribe

**Nguyên nhân có thể:**
1. Topic sai
2. Chưa có Client nào publish dữ liệu
3. Subscribe QoS không phù hợp

**Giải pháp:**
1. Kiểm tra lại tên Topic (chữ hoa/thường, ký tự đặc biệt)
2. Thử publish một tin từ chính TermiPro để test
3. Sử dụng MQTT Explorer để xem tất cả Topic đang có

### Publish thành công nhưng không thấy dữ liệu

**Nguyên nhân có thể:**
1. Chưa có Client nào subscribe
2. Terminal được lọc (Filter = TX chỉ)
3. Dữ liệu quá nhiều, bị giới hạn hiển thị

**Giải pháp:**
1. Mở filter xem tất cả (All)
2. Kiểm tra thống kê TX/RX
3. Xóa Terminal và thử lại (Clear)

### Last Will không hoạt động

**Nguyên nhân có thể:**
1. Topic hoặc Message trống
2. Broker không hỗ trợ LWT
3. Ngắt kết nối bình thường (không phải sự cố)

**Giải pháp:**
1. Kiểm tra lại Topic và Message có đầy đủ không
2. Đảm bảo đã bật tính năng LWT
3. Test bằng cách kill process TermiPro (không phải Close graceful)

### Kết nối bị mất đột ngột

**Nguyên nhân có thể:**
1. Mạng không ổn định
2. Keep Alive quá lớn
3. Broker bị overload

**Giải pháp:**
1. Giảm Keep Alive (ví dụ từ 60 xuống 30)
2. Kiểm tra kết nối Internet
3. Thử Broker khác

---

## Các ứng dụng MQTT phổ biến

Dưới đây là một số ứng dụng thực tế bạn có thể triển khai:

### 1. Nhà thông minh (Smart Home)

```
Topic: home/lights/living_room/switch
Payload: on/off

Topic: home/temperature/bedroom
Payload: {"temp": 26.5, "unit": "C"}

Topic: home/door/status
Payload: open/closed

Topic: home/motion/detected
Payload: true/false
```

### 2. Hệ thống ô tô (Connected Car)

```
Topic: car/location/gps
Payload: {"lat": 10.123, "lng": 106.456}

Topic: car/engine/status
Payload: running/idle/off

Topic: car/fuel/level
Payload: 75 (%)

Topic: car/alarm
Payload: motion_detected
```

### 3. Công nghiệp 4.0 (Industry 4.0)

```
Topic: factory/equipment/001/status
Payload: running/error/idle

Topic: factory/sensors/temperature
Payload: 350 (độ C)

Topic: factory/production/count
Payload: 1542 (sản phẩm)

Topic: factory/maintenance/alert
Payload: {"equipment_id": "001", "issue": "overheating"}
```

### 4. Giám sát sức khỏe (Health Monitoring)

```
Topic: health/user/heart_rate
Payload: {"bpm": 72, "timestamp": 1640000000}

Topic: health/user/steps
Payload: {"count": 5000, "unit": "steps"}

Topic: health/alert/abnormal
Payload: {"alert": "high_blood_pressure", "value": 160}
```

### 5. Nông nghiệp (Agriculture)

```
Topic: farm/field/soil_moisture
Payload: {"value": 45, "unit": "%"}

Topic: farm/weather/rainfall
Payload: {"mm": 2.5}

Topic: farm/irrigation/pump
Payload: on/off

Topic: farm/crop/status
Payload: {"health": "good", "stage": "flowering"}
```

---

## Tham khảo thêm

Để học thêm về MQTT:

- **MQTT.org**: Trang chính thức của MQTT
- **HiveMQ Blog**: Các bài hướng dẫn chi tiết
- **MQTT Explorer**: Ứng dụng để khám phá các Topic trên Broker
- **Mosquitto**: Broker MQTT mã nguồn mở để cài đặt cục bộ

---

## Liên hệ hỗ trợ

Nếu bạn gặp vấn đề:

1. Kiểm tra hướng dẫn này
2. Tìm kiếm trên diễn đàn MQTT
3. Liên hệ với nhóm phát triển TermiPro

---

**Chúc bạn sử dụng MQTT với TermiPro vui vẻ!**

*Cập nhật lần cuối: 28/12/2025*
