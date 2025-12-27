# TermiPro - Professional Serial Monitor

<p align="center">
  <img src="src-tauri/icons/icon.png" alt="TermiPro Logo" width="128" height="128">
</p>

<p align="center">
  <strong>Ứng dụng Serial Monitor chuyên nghiệp cho embedded systems</strong><br>
  <strong>Professional Serial Monitor for embedded systems</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri" alt="Tauri">
  <img src="https://img.shields.io/badge/Vue.js-3.5-green?logo=vue.js" alt="Vue.js">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Platform-macOS%20|%20Windows%20|%20Linux-lightgrey" alt="Platform">
</p>

<p align="center">
  <img src="https://img.shields.io/github/downloads/vienkmt/TermiPro/total?label=Downloads" />
  <img src="https://img.shields.io/github/contributors/vienkmt/TermiPro" />
  <img src="https://img.shields.io/github/last-commit/vienkmt/TermiPro" />
</p>

---

## Tiếng Việt

### Giới thiệu

**TermiPro** là ứng dụng desktop chuyên nghiệp để giao tiếp với cổng serial, phục vụ cho lập trình viên embedded systems, IoT developers và những người làm việc với vi điều khiển. Được xây dựng với công nghệ hiện đại, giao diện đẹp mắt và trải nghiệm người dùng mượt mà.

### Tính năng chính

#### Quản lý kết nối Serial
- Tự động phát hiện và liệt kê các cổng USB serial
- Hỗ trợ đầy đủ cấu hình:
  - **Baud Rate**: 300 - 921.600
  - **Data Bits**: 5, 6, 7, 8
  - **Stop Bits**: 1, 1.5, 2
  - **Parity**: None, Odd, Even
- Hiển thị trạng thái kết nối real-time

#### Gửi dữ liệu linh hoạt
- **Text Mode**: Gửi chuỗi văn bản thông thường
- **Hex Mode**: Gửi dữ liệu dạng hex (VD: `48 65 6C 6C 6F`)
- Giữ nội dung input sau khi gửi để tiện chỉnh sửa
- Nút xóa nhanh nội dung input

#### Auto Send - Gửi tự động
- Cấu hình tần suất gửi từ 50ms đến 60.000ms
- Bắt đầu/dừng gửi tự động với một click
- Hiển thị bộ đếm số lần đã gửi

#### Terminal chuyên nghiệp
- Hiển thị dữ liệu real-time
- Phân biệt rõ ràng dữ liệu TX (gửi) và RX (nhận) với màu sắc khác nhau
- Thống kê số lượng TX/RX riêng biệt
- Chuyển đổi hiển thị Text/Hex
- Tự động cuộn xuống tin nhắn mới
- Thiết kế compact, tối ưu hiển thị

#### Giao diện hiện đại
- Theme sáng với accent màu xanh sky blue
- Font Plus Jakarta Sans cho giao diện, JetBrains Mono cho terminal
- Dropdown tùy chỉnh với animation mượt mà
- Hoạt động offline hoàn toàn, không cần internet

### Yêu cầu hệ thống

- **macOS**: 10.15 (Catalina) trở lên
- **Windows**: Windows 10 trở lên
- **Linux**: Ubuntu 18.04 hoặc tương đương

### Cài đặt

Tải phiên bản mới nhất từ [Releases](../../releases).

### Phát triển

```bash
# Clone repository
git clone https://github.com/vienkmt/TermiPro.git
cd TermiPro

# Cài đặt dependencies
npm install

# Chạy development mode
npm run tauri dev

# Build production
npm run tauri build
```

---

## English

### Introduction

**TermiPro** is a professional desktop application for serial port communication, designed for embedded systems programmers, IoT developers, and anyone working with microcontrollers. Built with modern technology, featuring a beautiful interface and smooth user experience.

### Key Features

#### Serial Connection Management
- Automatic detection and listing of USB serial ports
- Full configuration support:
  - **Baud Rate**: 300 - 921,600
  - **Data Bits**: 5, 6, 7, 8
  - **Stop Bits**: 1, 1.5, 2
  - **Parity**: None, Odd, Even
- Real-time connection status display

#### Flexible Data Transmission
- **Text Mode**: Send regular text strings
- **Hex Mode**: Send hex data (e.g., `48 65 6C 6C 6F`)
- Input content preserved after sending for easy editing
- Quick clear button for input field

#### Auto Send
- Configure send interval from 50ms to 60,000ms
- Start/stop automatic sending with one click
- Display counter for number of sends

#### Professional Terminal
- Real-time data display
- Clear distinction between TX (sent) and RX (received) data with different colors
- Separate TX/RX statistics
- Toggle between Text/Hex display
- Auto-scroll to new messages
- Compact design optimized for viewing

#### Modern Interface
- Light theme with sky blue accent color
- Plus Jakarta Sans font for UI, JetBrains Mono for terminal
- Custom dropdowns with smooth animations
- Fully offline operation, no internet required

### System Requirements

- **macOS**: 10.15 (Catalina) or later
- **Windows**: Windows 10 or later
- **Linux**: Ubuntu 18.04 or equivalent

### Installation

Download the latest version from [Releases](../../releases).

### Development

```bash
# Clone repository
git clone https://github.com/vienkmt/TermiPro.git
cd TermiPro

# Install dependencies
npm install

# Run development mode
npm run tauri dev

# Build production
npm run tauri build
```

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Frontend | Vue.js 3 + Vite |
| Backend | Rust |
| Framework | Tauri 2 |
| Fonts | Plus Jakarta Sans, JetBrains Mono |

## License

MIT License - see [LICENSE](LICENSE) for details.

---

<p align="center">
  Made with ❤️ for the embedded community
</p>
