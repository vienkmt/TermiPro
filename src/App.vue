<script setup>
import { ref, computed, provide, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import TabBar from "./components/TabBar.vue";
import SerialTab from "./components/SerialTab.vue";
import ConfirmDialog from "./components/ConfirmDialog.vue";
import { useTabStore } from "./stores/tabStore";

// i18n Translations
const translations = {
  vi: {
    // Status
    connected: "Đã kết nối",
    disconnected: "Chưa kết nối",
    // Sidebar
    serialPort: "Cổng Serial",
    selectPort: "Chọn cổng...",
    noDeviceFound: "Không tìm thấy thiết bị",
    configuration: "Cấu hình",
    baud: "Baud",
    data: "Data",
    stop: "Stop",
    parity: "Parity",
    connect: "Kết nối",
    disconnect: "Ngắt kết nối",
    display: "Hiển thị",
    autoScroll: "Auto Scroll",
    autoSendSettings: "Cài đặt Auto Send",
    frequency: "Tần suất",
    sent: "Đã gửi",
    times: "lần",
    // Terminal
    terminal: "Terminal",
    clear: "Xóa",
    noData: "Chưa có dữ liệu",
    connectToStart: "Kết nối cổng serial để bắt đầu",
    // Send
    enterMessage: "Nhập tin nhắn...",
    hexExample: "VD: 48 65 6C 6C 6F",
    send: "Gửi",
    auto: "Auto",
    stop: "Stop",
    clearContent: "Xóa nội dung",
    // Alerts
    pleaseSelectPort: "Vui lòng chọn cổng serial!",
    pleaseEnterMessage: "Vui lòng nhập tin nhắn để gửi tự động!",
    pleaseConnectFirst: "Vui lòng kết nối trước!",
    // Tabs
    newTab: "Tab mới",
    closeTab: "Đóng tab",
    closeTabConfirm: "Đóng kết nối?",
    closeTabWarning: "Cổng serial đang kết nối. Bạn có muốn ngắt kết nối và đóng tab?",
    cancel: "Hủy",
    maxTabsReached: "Đã đạt giới hạn tab",
  },
  en: {
    // Status
    connected: "Connected",
    disconnected: "Disconnected",
    // Sidebar
    serialPort: "Serial Port",
    selectPort: "Select port...",
    noDeviceFound: "No device found",
    configuration: "Configuration",
    baud: "Baud",
    data: "Data",
    stop: "Stop",
    parity: "Parity",
    connect: "Connect",
    disconnect: "Disconnect",
    display: "Display",
    autoScroll: "Auto Scroll",
    autoSendSettings: "Auto Send Settings",
    frequency: "Frequency",
    sent: "Sent",
    times: "times",
    // Terminal
    terminal: "Terminal",
    clear: "Clear",
    noData: "No data",
    connectToStart: "Connect to serial port to start",
    // Send
    enterMessage: "Enter message...",
    hexExample: "Ex: 48 65 6C 6C 6F",
    send: "Send",
    auto: "Auto",
    stop: "Stop",
    clearContent: "Clear content",
    // Alerts
    pleaseSelectPort: "Please select a serial port!",
    pleaseEnterMessage: "Please enter a message to auto send!",
    pleaseConnectFirst: "Please connect first!",
    // Tabs
    newTab: "New Tab",
    closeTab: "Close tab",
    closeTabConfirm: "Close connection?",
    closeTabWarning: "Serial port is connected. Disconnect and close tab?",
    cancel: "Cancel",
    maxTabsReached: "Maximum tabs reached",
  }
};

// Language state
const currentLang = ref("vi");
const t = computed(() => translations[currentLang.value]);

function toggleLanguage() {
  currentLang.value = currentLang.value === "vi" ? "en" : "vi";
}

// Provide i18n to child components
provide('t', t);
provide('currentLang', currentLang);
provide('toggleLanguage', toggleLanguage);

// Tab store
const tabStore = useTabStore();
const { tabs, activeTabId, tabOrder, activeTab, canAddTab, createTab, closeTab, setActiveTab, getTabByPortName, getConnectedPorts } = tabStore;

// Constants
const MAX_TERMINAL_ENTRIES = 500;

// Available ports
const ports = ref([]);

// Confirmation dialog state
const showConfirmDialog = ref(false);
const pendingCloseTabId = ref(null);

// Serial event listener
let unlistenSerial = null;

// Computed
const connectionStatus = computed(() => {
  if (!activeTab.value) return t.value.disconnected;
  return activeTab.value.isConnected ? t.value.connected : t.value.disconnected;
});

const connectionStatusClass = computed(() => {
  if (!activeTab.value) return "disconnected";
  return activeTab.value.isConnected ? "connected" : "disconnected";
});

const connectedPorts = computed(() => getConnectedPorts());

// Functions
async function refreshPorts() {
  try {
    const result = await invoke("list_serial_ports");
    ports.value = result;
  } catch (error) {
    console.error("Error listing ports:", error);
  }
}

async function handleConnect(tabId) {
  const tab = tabs.get(tabId);
  if (!tab || !tab.selectedPort) {
    alert(t.value.pleaseSelectPort);
    return;
  }

  try {
    const config = {
      port_name: tab.selectedPort,
      baud_rate: tab.baudRate,
      data_bits: tab.dataBits,
      stop_bits: tab.stopBits,
      parity: tab.parity,
    };

    await invoke("open_port", { config });
    tab.isConnected = true;
  } catch (error) {
    console.error("Connection error:", error);
    alert("Error: " + error);
  }
}

async function handleDisconnect(tabId) {
  const tab = tabs.get(tabId);
  if (!tab) return;

  try {
    await invoke("close_port", { portName: tab.selectedPort });
    tab.isConnected = false;

    // Stop auto send if running
    if (tab.autoSendEnabled) {
      tab.autoSendEnabled = false;
      if (tab.autoSendTimer) {
        clearInterval(tab.autoSendTimer);
        tab.autoSendTimer = null;
      }
    }
  } catch (error) {
    console.error("Disconnect error:", error);
    alert("Error: " + error);
  }
}

function requestCloseTab(tabId) {
  const tab = tabs.get(tabId);
  if (!tab) return;

  if (tab.isConnected) {
    pendingCloseTabId.value = tabId;
    showConfirmDialog.value = true;
  } else {
    closeTab(tabId);
    // Create new tab if all tabs are closed
    if (tabs.size === 0) {
      createTab();
    }
  }
}

async function confirmCloseTab() {
  const tabId = pendingCloseTabId.value;
  const tab = tabs.get(tabId);

  if (tab && tab.isConnected) {
    try {
      await invoke("close_port", { portName: tab.selectedPort });
    } catch (error) {
      console.error("Error closing port:", error);
    }
  }

  closeTab(tabId);
  showConfirmDialog.value = false;
  pendingCloseTabId.value = null;

  // Create new tab if all tabs are closed
  if (tabs.size === 0) {
    createTab();
  }
}

function cancelCloseTab() {
  showConfirmDialog.value = false;
  pendingCloseTabId.value = null;
}

function handleAddTab() {
  createTab();
}

// Lifecycle
onMounted(async () => {
  // Create initial tab
  createTab();

  // Refresh ports
  await refreshPorts();

  // Global serial data listener - routes data to correct tab
  unlistenSerial = await listen("serial-data", (event) => {
    const { port_name, data, timestamp } = event.payload;

    // Find the tab that owns this port
    const tab = getTabByPortName(port_name);
    if (tab) {
      // Limit terminal entries
      if (tab.terminalData.length >= MAX_TERMINAL_ENTRIES) {
        const removed = tab.terminalData.shift();
        if (removed.type === 'tx') tab.txCount--;
        else tab.rxCount--;
      }

      tab.terminalData.push({
        type: "rx",
        data: data,
        timestamp: new Date(timestamp).toLocaleTimeString(),
      });
      tab.rxCount++;
    }
  });
});

onUnmounted(async () => {
  // Cleanup event listener
  if (unlistenSerial) {
    unlistenSerial();
  }

  // Close all connected ports
  for (const [, tab] of tabs) {
    if (tab.isConnected) {
      try {
        await invoke("close_port", { portName: tab.selectedPort });
      } catch (error) {
        console.error("Error closing port:", error);
      }
    }
    // Clear auto send timers
    if (tab.autoSendTimer) {
      clearInterval(tab.autoSendTimer);
    }
  }
});
</script>

<template>
  <div class="app-container">
    <!-- Header -->
    <header class="header">
      <div class="header-left">
        <div class="logo">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <h1>TermiPro</h1>
        <button class="lang-switch" @click="toggleLanguage" :title="currentLang === 'vi' ? 'Switch to English' : 'Chuyển sang tiếng Việt'">
          <!-- Vietnam Flag -->
          <svg v-if="currentLang === 'vi'" class="lang-flag" width="20" height="14" viewBox="0 0 30 20">
            <rect width="30" height="20" fill="#da251d"/>
            <polygon points="15,4 11.5,14 19.5,7.5 10.5,7.5 18.5,14" fill="#ffff00"/>
          </svg>
          <!-- UK Flag -->
          <svg v-else class="lang-flag" width="20" height="14" viewBox="0 0 60 40">
            <rect width="60" height="40" fill="#012169"/>
            <path d="M0,0 L60,40 M60,0 L0,40" stroke="#fff" stroke-width="6"/>
            <path d="M0,0 L60,40 M60,0 L0,40" stroke="#C8102E" stroke-width="4" clip-path="url(#clip)"/>
            <path d="M30,0 V40 M0,20 H60" stroke="#fff" stroke-width="10"/>
            <path d="M30,0 V40 M0,20 H60" stroke="#C8102E" stroke-width="6"/>
          </svg>
          <span class="lang-code">{{ currentLang.toUpperCase() }}</span>
        </button>
      </div>
      <div class="header-right">
        <div class="status-badge" :class="connectionStatusClass">
          <span class="status-dot"></span>
          <span class="status-text">{{ connectionStatus }}</span>
        </div>
      </div>
    </header>

    <!-- Tab Bar -->
    <TabBar
      :tabs="tabs"
      :tab-order="tabOrder"
      :active-tab-id="activeTabId"
      :can-add-tab="canAddTab"
      @select-tab="setActiveTab"
      @close-tab="requestCloseTab"
      @add-tab="handleAddTab"
    />

    <!-- Main Content - Tabs -->
    <div class="main-content">
      <KeepAlive :max="8">
        <SerialTab
          v-if="activeTab"
          :key="activeTabId"
          :tab-id="activeTabId"
          :tab-state="activeTab"
          :ports="ports"
          :connected-ports="connectedPorts"
          @connect="handleConnect"
          @disconnect="handleDisconnect"
          @refresh-ports="refreshPorts"
        />
      </KeepAlive>
    </div>

    <!-- Confirm Dialog -->
    <ConfirmDialog
      :visible="showConfirmDialog"
      @confirm="confirmCloseTab"
      @cancel="cancelCloseTab"
    />
  </div>
</template>

<style>
/* CSS Variables - Light Theme */
:root {
  --bg-primary: #f8fafc;
  --bg-secondary: #ffffff;
  --bg-tertiary: #f1f5f9;
  --bg-hover: #e2e8f0;
  --accent-primary: #0ea5e9;
  --accent-secondary: #38bdf8;
  --accent-light: #e0f2fe;
  --success: #10b981;
  --success-light: #d1fae5;
  --danger: #ef4444;
  --danger-light: #fee2e2;
  --warning: #f59e0b;
  --warning-light: #fef3c7;
  --text-primary: #1e293b;
  --text-secondary: #64748b;
  --text-tertiary: #94a3b8;
  --border-color: #e2e8f0;
  --border-focus: #0ea5e9;
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
  --shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
  --font-sans: 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', 'SF Mono', Consolas, monospace;
  --radius-sm: 4px;
  --radius-md: 6px;
  --radius-lg: 8px;
  --radius-xl: 10px;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: var(--font-sans);
  background-color: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
  -webkit-font-smoothing: antialiased;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

/* Header */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 16px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  box-shadow: var(--shadow-sm);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  border-radius: var(--radius-sm);
  color: white;
}

.logo svg {
  width: 18px;
  height: 18px;
}

.header h1 {
  font-size: 1rem;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.02em;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 50px;
  font-size: 0.7rem;
  font-weight: 600;
  transition: all 0.3s ease;
}

.status-badge.connected {
  background: var(--success-light);
  color: var(--success);
}

.status-badge.disconnected {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
}

.status-badge.connected .status-dot {
  animation: pulse-dot 2s infinite;
}

@keyframes pulse-dot {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.2); opacity: 0.7; }
}

/* Language Switch */
.lang-switch {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
  margin-left: 16px;
}

.lang-switch:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}

.lang-flag {
  flex-shrink: 0;
  border-radius: 2px;
  box-shadow: 0 0 0 1px rgba(0,0,0,0.1);
}

.lang-code {
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: 0.05em;
}

/* Main Content */
.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

/* Scrollbar */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 10px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-tertiary);
}
</style>
