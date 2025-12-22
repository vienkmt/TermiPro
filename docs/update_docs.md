# Quy trình Auto-Update - TermiPro

## Tổng quan

TermiPro sử dụng `tauri-plugin-updater` để tự động cập nhật ứng dụng trên Windows. Dữ liệu update được lưu trữ trên GitHub Releases.

---

## Kiến trúc

```
GitHub Releases
       │
       ├── latest.json (manifest chứa version, signature, URL)
       ├── TermiPro_x.x.x_x64-setup.exe
       └── TermiPro_x.x.x_x64-setup.exe.sig (chữ ký)

       ↓ (App check mỗi 30 phút)

TermiPro App (Windows)
       │
       ├── So sánh version hiện tại với latest.json
       ├── Nếu có bản mới → Hiện nút update (nhấp nháy)
       └── User confirm → Download → Install → Restart
```

---

## Quy trình Release phiên bản mới

### Bước 1: Cập nhật version

Sửa file `src-tauri/tauri.conf.json`:
```json
{
  "version": "1.0.6"  // Tăng version
}
```

### Bước 2: Commit và push

```bash
git add .
git commit -m "release: v1.0.6"
git push
```

### Bước 3: Tạo tag và push

```bash
git tag v1.0.6
git push --tags
```

### Bước 4: GitHub Actions tự động

Khi push tag `v*`, workflow sẽ tự động:

1. **Build** Windows installer (.exe)
2. **Sign** installer với private key
3. **Tạo** `latest.json` manifest
4. **Publish** GitHub Release

---

## Flow phía người dùng

```
┌─────────────────────────────────────────────────────────────┐
│                    APP KHỞI ĐỘNG                            │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Check: github.com/vienkmt/TermiPro/releases/.../latest.json│
└─────────────────────────────────────────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼                           ▼
┌─────────────────────┐     ┌─────────────────────────────────┐
│  Không có bản mới   │     │  Có bản mới (v1.0.6 > v1.0.5)   │
│  → Không làm gì     │     │  → Hiện nút update (nhấp nháy)  │
└─────────────────────┘     └─────────────────────────────────┘
                                          │
                                          ▼
                            ┌─────────────────────────────────┐
                            │  User click nút update          │
                            └─────────────────────────────────┘
                                          │
                                          ▼
                            ┌─────────────────────────────────┐
                            │  Modal hiện changelog           │
                            │  ┌───────────────────────────┐  │
                            │  │ v1.0.5 ──→ v1.0.6        │  │
                            │  │                           │  │
                            │  │ Changelog:                │  │
                            │  │ - Feature A               │  │
                            │  │ - Bug fix B               │  │
                            │  │                           │  │
                            │  │ [Để sau] [Cập nhật ngay]  │  │
                            │  └───────────────────────────┘  │
                            └─────────────────────────────────┘
                                          │
                                          ▼
                            ┌─────────────────────────────────┐
                            │  User click "Cập nhật ngay"     │
                            └─────────────────────────────────┘
                                          │
                                          ▼
                            ┌─────────────────────────────────┐
                            │  Download .exe (hiện progress)  │
                            └─────────────────────────────────┘
                                          │
                                          ▼
                            ┌─────────────────────────────────┐
                            │  Install & Restart app          │
                            └─────────────────────────────────┘
                                          │
                                          ▼
                            ┌─────────────────────────────────┐
                            │  App chạy phiên bản mới! 🎉     │
                            └─────────────────────────────────┘
```

---

## Tần suất check update

| Thời điểm | Hành động |
|-----------|-----------|
| Khi mở app | Check ngay lập tức |
| Sau đó | Check mỗi 30 phút |

---

## Cấu trúc file latest.json

```json
{
  "version": "1.0.6",
  "notes": "See release notes at https://github.com/vienkmt/TermiPro/releases/tag/v1.0.6",
  "pub_date": "2024-01-15T10:30:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6...",
      "url": "https://github.com/vienkmt/TermiPro/releases/download/v1.0.6/TermiPro_1.0.6_x64-setup.exe"
    }
  }
}
```

---

## GitHub Secrets cần thiết

| Secret | Mô tả |
|--------|-------|
| `TAURI_SIGNING_PRIVATE_KEY` | Nội dung file private key (`~/.tauri/termipro.key`) |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Password của key (để trống nếu không có) |

---

## Các file liên quan

| File | Chức năng |
|------|-----------|
| `src-tauri/tauri.conf.json` | Cấu hình updater (pubkey, endpoints) |
| `src-tauri/Cargo.toml` | Dependencies (`tauri-plugin-updater`) |
| `src-tauri/src/lib.rs` | Khởi tạo updater plugin |
| `src/App.vue` | Logic check update, UI nút update |
| `src/components/UpdateModal.vue` | Modal hiển thị changelog |
| `.github/workflows/build.yml` | CI/CD build và release |

---

## Troubleshooting

### 1. Nút update không xuất hiện
- Kiểm tra `latest.json` trên GitHub Releases
- Đảm bảo version trong `latest.json` > version hiện tại
- Kiểm tra console log cho errors

### 2. Download thất bại
- Kiểm tra signature trong `latest.json` khớp với pubkey
- Đảm bảo URL download đúng

### 3. Build thất bại trên GitHub Actions
- Kiểm tra secrets đã được thêm đúng
- Xem logs của workflow

---

## Lưu ý bảo mật

- **KHÔNG** commit private key vào repo
- Private key chỉ lưu trong GitHub Secrets
- Public key có thể public (đã có trong `tauri.conf.json`)

---

## Mở rộng macOS (TODO)

Hiện tại chỉ hỗ trợ Windows. Để thêm macOS:

1. Uncomment phần `build-macos` trong `.github/workflows/build.yml`
2. Thêm macOS vào `platforms` trong `latest.json`
3. Cấu hình code signing cho macOS (Apple Developer account)
