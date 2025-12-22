# Auto-Update Feature Implementation Plan

## Tổng quan
Triển khai tính năng auto-update cho TermiPro trên Windows, sử dụng GitHub Releases làm nguồn cập nhật.

## Yêu cầu
- **Platform**: Windows only
- **Nút update**: Chỉ hiển thị khi có bản mới, viền blink animation màu cam
- **GitHub repo**: `vienkmt/TermiPro`
- **Tần suất check**: Khi khởi động + mỗi 30 phút
- **Flow**: Click nút → Modal changelog → Confirm → Tải & cài đặt

---

## Các file đã thay đổi

| File | Thay đổi | Status |
|------|----------|--------|
| `src-tauri/Cargo.toml` | Thêm `tauri-plugin-updater`, `tauri-plugin-process` | ✅ Done |
| `src-tauri/tauri.conf.json` | Cấu hình updater plugin | ✅ Done |
| `src-tauri/src/lib.rs` | Khởi tạo updater plugin | ✅ Done |
| `package.json` | Thêm `@tauri-apps/plugin-updater`, `@tauri-apps/plugin-process` | ✅ Done |
| `src/App.vue` | Thêm nút update, logic check, translations | ✅ Done |
| `src/components/UpdateModal.vue` | **MỚI** - Modal changelog | ✅ Done |
| `.github/workflows/build.yml` | Thêm signing, release job | ✅ Done |

---

## Bước còn lại: Tạo Signing Key

### 1. Cài đặt Tauri CLI (nếu chưa có)
```bash
cargo install tauri-cli
```

### 2. Tạo signing key
```bash
cargo tauri signer generate -w ~/.tauri/termipro.key
```

Lệnh này sẽ:
- Tạo private key tại `~/.tauri/termipro.key`
- In ra **public key** trên terminal

### 3. Cập nhật public key vào tauri.conf.json
Thay `REPLACE_WITH_YOUR_PUBLIC_KEY` bằng public key vừa tạo:
```json
"plugins": {
  "updater": {
    "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6...(public key của bạn)...",
    ...
  }
}
```

### 4. Thêm GitHub Secrets
Vào repo GitHub → Settings → Secrets and variables → Actions → New repository secret:

- **`TAURI_SIGNING_PRIVATE_KEY`**: Copy toàn bộ nội dung file `~/.tauri/termipro.key`
- **`TAURI_SIGNING_PRIVATE_KEY_PASSWORD`**: Password bạn đã nhập khi tạo key (để trống nếu không có)

---

## Quy trình Release

1. Update version trong `src-tauri/tauri.conf.json`:
   ```json
   "version": "1.0.6"
   ```

2. Commit và push:
   ```bash
   git add .
   git commit -m "Bump version to 1.0.6"
   git push
   ```

3. Tạo và push tag:
   ```bash
   git tag v1.0.6
   git push --tags
   ```

4. GitHub Actions sẽ tự động:
   - Build Windows installer (.exe)
   - Sign installer với private key
   - Tạo `latest.json` manifest
   - Publish GitHub Release

---

## Test thử

1. Sau khi release v1.0.6, mở ứng dụng v1.0.5
2. Nút update sẽ xuất hiện và nhấp nháy
3. Click vào nút → Modal changelog hiện ra
4. Click "Cập nhật ngay" → Tải và cài đặt tự động

---

## Trạng thái triển khai

- [x] Update Cargo.toml + lib.rs
- [x] Update tauri.conf.json (cần thêm public key)
- [x] Install npm package
- [x] Tạo UpdateModal.vue
- [x] Update App.vue (translations, state, button, CSS)
- [x] Update GitHub Actions workflow
- [ ] **Generate signing keys** ← Cần thực hiện thủ công
- [ ] **Add GitHub secrets** ← Cần thực hiện thủ công
- [ ] Test với bản release thử
