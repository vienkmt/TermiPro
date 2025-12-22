# New Tab Modal - Documentation

## Overview
Modal cho phép user chọn loại kết nối khi tạo tab mới: Serial, TCP Client, hoặc TCP Server.

## Files

### Frontend
- `src/components/NewTabModal.vue` - Modal component
- `src/stores/tabStore.js` - Tab state management với CONNECTION_TYPES
- `src/App.vue` - Integration và event handlers

## NewTabModal.vue

### Props
```javascript
{
  visible: Boolean  // Hiển thị/ẩn modal
}
```

### Events
```javascript
emit('select', connectionType)  // User chọn loại kết nối
emit('cancel')                  // User đóng modal
```

### Template Structure
```
Dialog Overlay (click outside to cancel)
└── Dialog Content
    ├── Header (title + close button)
    └── Type Cards
        ├── Serial Card (green icon)
        ├── TCP Client Card (blue icon)
        └── TCP Server Card (purple icon)
```

### Styling
- Cards có hover effect với transform và box-shadow
- Icons với gradient backgrounds:
  - Serial: `#10b981 → #059669` (green)
  - TCP Client: `#0ea5e9 → #0284c7` (blue)
  - TCP Server: `#8b5cf6 → #7c3aed` (purple)

## tabStore.js

### CONNECTION_TYPES
```javascript
export const CONNECTION_TYPES = {
  SERIAL: 'serial',
  TCP_CLIENT: 'tcp_client',
  TCP_SERVER: 'tcp_server',
};
```

### Tab State Factories
```javascript
createBaseTabState(id, connectionType)     // Shared state
createSerialTabState(id)                   // Serial-specific
createTcpClientTabState(id)                // TCP Client-specific
createTcpServerTabState(id)                // TCP Server-specific
```

### createTab(connectionType)
```javascript
function createTab(connectionType = CONNECTION_TYPES.SERIAL) {
  // Switch để tạo đúng loại tab state
  // Insert vào tabs Map
  // Set active tab
  // Return tab ID
}
```

## App.vue Integration

### State
```javascript
const showNewTabModal = ref(false);
```

### Functions
```javascript
function handleAddTab() {
  showNewTabModal.value = true;
}

function handleNewTabSelect(connectionType) {
  createTab(connectionType);
  showNewTabModal.value = false;
}

function cancelNewTabModal() {
  showNewTabModal.value = false;
}
```

### Template
```vue
<!-- Tab Bar triggers modal -->
<TabBar @add-tab="handleAddTab" />

<!-- Modal -->
<NewTabModal
  :visible="showNewTabModal"
  @select="handleNewTabSelect"
  @cancel="cancelNewTabModal"
/>
```

## Tab Rendering (v-for + v-show)

```vue
<!-- Render ALL tabs, show only active -->
<template v-for="[tabId, tab] in tabs" :key="tabId">
  <SerialTab
    v-if="tab.connectionType === 'serial'"
    v-show="tabId === activeTabId"
    ...
  />
  <TcpClientTab
    v-else-if="tab.connectionType === 'tcp_client'"
    v-show="tabId === activeTabId"
    ...
  />
  <TcpServerTab
    v-else-if="tab.connectionType === 'tcp_server'"
    v-show="tabId === activeTabId"
    ...
  />
</template>
```

**Lý do dùng v-show thay v-if:**
- Components không bị unmount khi switch tab
- Auto send timers tiếp tục chạy
- Scroll position được giữ nguyên

## TabBar.vue Updates

### Connection Type Icons
```vue
<!-- Serial Icon -->
<svg v-if="getConnectionType(tabId) === 'serial'" ...>

<!-- TCP Client Icon -->
<svg v-else-if="getConnectionType(tabId) === 'tcp_client'" ...>

<!-- TCP Server Icon -->
<svg v-else-if="getConnectionType(tabId) === 'tcp_server'" ...>
```

### Display Names
```javascript
function getTabDisplayName(tabId) {
  // Serial: port name hoặc "New Tab"
  // TCP Client: "host:port" hoặc "TCP Client"
  // TCP Server: ":port" hoặc "TCP Server"
}
```

### Icon Colors (CSS)
```css
.tab.type-serial.active .tab-icon { color: #10b981; }
.tab.type-tcp-client.active .tab-icon { color: #0ea5e9; }
.tab.type-tcp-server.active .tab-icon { color: #8b5cf6; }
```

## i18n Translations

### Vietnamese
```javascript
selectConnectionType: "Chọn loại kết nối",
connectionTypeSerial: "Serial Port",
connectionTypeTcpClient: "TCP Client",
connectionTypeTcpServer: "TCP Server",
serialDesc: "Kết nối với thiết bị qua cổng serial",
tcpClientDesc: "Kết nối đến server TCP từ xa",
tcpServerDesc: "Lắng nghe kết nối TCP đến",
```

### English
```javascript
selectConnectionType: "Select Connection Type",
connectionTypeSerial: "Serial Port",
connectionTypeTcpClient: "TCP Client",
connectionTypeTcpServer: "TCP Server",
serialDesc: "Connect to device via serial port",
tcpClientDesc: "Connect to remote TCP server",
tcpServerDesc: "Listen for incoming TCP connections",
```
