# TermiPro - Professional Serial Monitor

## Tổng quan

**TermiPro** là ứng dụng desktop chuyên nghiệp để giao tiếp với cổng serial, phục vụ lĩnh vực embedded systems. Được xây dựng với công nghệ hiện đại, giao diện đẹp mắt và dễ sử dụng.

- **Frontend**: Vue.js 3 + Vite
- **Backend**: Rust
- **Framework**: Tauri 2

## Cấu trúc dự án

```
termipro/
├── src/                    # Frontend Vue.js
│   ├── App.vue            # Component chính (~1830 lines)
│   └── main.js            # Entry point
├── src-tauri/             # Backend Rust
│   ├── src/
│   │   ├── lib.rs         # Tauri commands & serial logic
│   │   └── main.rs        # Entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Cấu hình Tauri
├── index.html
├── package.json
└── vite.config.js
```

## Tính năng

### 1. Quản lý kết nối Serial
- **Liệt kê cổng**: Chỉ hiển thị USB ports (`/dev/tty*`), loại trừ `usbmodem`
- **Cấu hình đầy đủ**:
  - Baud Rate: 300 - 921.600
  - Data Bits: 5, 6, 7, 8
  - Stop Bits: 1, 1.5, 2
  - Parity: None, Odd, Even
- **Auto refresh**: Tự động cập nhật danh sách port khi click dropdown

### 2. Gửi dữ liệu
- **Text mode**: Gửi chuỗi văn bản thông thường
- **Hex mode**: Gửi dữ liệu dạng hex (VD: `48 65 6C 6C 6F`)
- **Giữ nội dung**: Input không bị xóa sau khi gửi
- **Nút clear**: Icon X để xóa nội dung input

### 3. Auto Send
- **Interval**: Cấu hình tần suất gửi (50ms - 60.000ms)
- **Start/Stop**: Bắt đầu/dừng gửi tự động
- **Counter**: Hiển thị số lần đã gửi

### 4. Terminal hiển thị
- **Real-time**: Nhận dữ liệu qua Tauri Events
- **TX/RX badges**: Phân biệt dữ liệu gửi (vàng) và nhận (xanh)
- **Thống kê riêng**: Hiển thị số lượng TX và RX riêng biệt
- **Toggle view**: Chuyển đổi hiển thị Text/Hex
- **Auto scroll**: Tự động cuộn xuống cuối
- **Compact design**: Tối ưu hiển thị nhiều tin nhắn

## Backend Rust - Tauri Commands

| Command | Mô tả |
|---------|-------|
| `list_serial_ports()` | Liệt kê các cổng USB serial |
| `open_port(config)` | Mở kết nối với cấu hình |
| `close_port(port_name)` | Đóng kết nối |
| `send_data(port_name, data, is_hex)` | Gửi dữ liệu |
| `is_port_open(port_name)` | Kiểm tra trạng thái |

### Event
- `serial-data`: Stream dữ liệu nhận được từ serial về frontend

## Dependencies

### Rust (Cargo.toml)
- `tauri` v2
- `serialport` v4.3
- `tokio` v1 (sync, time)
- `parking_lot` v0.12
- `serde` v1
- `serde_json` v1

### Frontend (package.json)
- `vue` ^3.5.13
- `@tauri-apps/api` ^2
- `@fontsource/plus-jakarta-sans` - Font chính (bundled offline)
- `@fontsource/jetbrains-mono` - Font terminal (bundled offline)
- `vite` ^6.0.3

## Giao diện

### Theme
- **Light mode** với màu xanh dương (sky blue) làm accent
- **Font**: Plus Jakarta Sans (UI), JetBrains Mono (terminal/code)
- **Offline**: Tất cả fonts được bundle sẵn, không cần internet

### Layout
- **Header**: Logo, tên app, trạng thái kết nối
- **Sidebar (320px)**: Chọn port, cấu hình, auto send, display options
- **Main**: Terminal với thống kê TX/RX
- **Footer**: Input gửi tin nhắn với 2 nút (Gửi, Auto)

### Custom Dropdowns
- Dropdown chuyên nghiệp thay thế native select
- Icon cho mỗi config item
- Animation slide khi mở
- Checkmark cho option đang chọn

## Chạy ứng dụng

```bash
# Development
npm run tauri dev

# Build production
npm run tauri build
```

## Cửa sổ

- **Kích thước mặc định**: 1200 x 850
- **Kích thước tối thiểu**: 900 x 600
- **Title**: TermiPro - Professional Serial Monitor
