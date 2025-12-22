<script setup>
import { ref, computed, provide, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getVersion } from "@tauri-apps/api/app";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import TabBar from "./components/TabBar.vue";
import SerialTab from "./components/SerialTab.vue";
import ConfirmDialog from "./components/ConfirmDialog.vue";
import UpdateModal from "./components/UpdateModal.vue";
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
    deviceDisconnected: "Thiết bị đã bị ngắt kết nối",
    // Tabs
    newTab: "Tab mới",
    closeTab: "Đóng tab",
    closeTabConfirm: "Đóng kết nối?",
    closeTabWarning: "Cổng serial đang kết nối. Bạn có muốn ngắt kết nối và đóng tab?",
    cancel: "Hủy",
    maxTabsReached: "Đã đạt giới hạn tab",
    // Signal Help
    signalHelp: "Trợ giúp tín hiệu",
    signalHelpTitle: "Tín hiệu điều khiển",
    dtrMeaning: "Máy tính đã sẵn sàng",
    rtsMeaning: "Tôi muốn gửi dữ liệu",
    dtrDesc: "Tín hiệu từ máy tính → thiết bị. Khi bật, báo cho thiết bị biết máy tính đang lắng nghe.",
    rtsDesc: "Tín hiệu từ máy tính → thiết bị. Một số module dùng để điều khiển nguồn hoặc chế độ hoạt động.",
    whenToUse: "Khi nào cần bật",
    dtrCase1Label: "Arduino - reset khi kết nối",
    dtrCase2Label: "Arduino - không reset",
    dtrCase3Label: "ESP32/ESP8266 - chế độ flash",
    rtsCase1Label: "ESP32/ESP8266 - chế độ flash",
    rtsCase2Label: "Một số module RS485",
    rtsCase3Label: "Đọc dữ liệu bình thường",
    signalSummary: "90% trường hợp: Để cả 2 OFF (mặc định)",
    signalNote: "Chỉ bật khi thiết bị không hoạt động hoặc tài liệu yêu cầu.",
    // Update
    updateAvailable: "Cập nhật mới",
    updateTitle: "Phiên bản mới",
    updateNow: "Cập nhật ngay",
    updateLater: "Để sau",
    downloading: "Đang tải xuống...",
    installing: "Đang cài đặt...",
    updateReady: "Sẵn sàng cập nhật",
    updateFailed: "Cập nhật thất bại",
    changelog: "Thay đổi",
    currentVersion: "Phiên bản hiện tại",
    newVersion: "Phiên bản mới",
    checkForUpdates: "Kiểm tra cập nhật",
    checking: "Đang kiểm tra...",
    noUpdateAvailable: "Bạn đang dùng phiên bản mới nhất",
    updateError: "Lỗi kiểm tra cập nhật",
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
    deviceDisconnected: "Device has been disconnected",
    // Tabs
    newTab: "New Tab",
    closeTab: "Close tab",
    closeTabConfirm: "Close connection?",
    closeTabWarning: "Serial port is connected. Disconnect and close tab?",
    cancel: "Cancel",
    maxTabsReached: "Maximum tabs reached",
    // Signal Help
    signalHelp: "Signal help",
    signalHelpTitle: "Control Signals",
    dtrMeaning: "Computer is ready",
    rtsMeaning: "I want to send data",
    dtrDesc: "Signal from computer → device. When enabled, tells the device the computer is listening.",
    rtsDesc: "Signal from computer → device. Some modules use it for power control or operating mode.",
    whenToUse: "When to enable",
    dtrCase1Label: "Arduino - reset on connect",
    dtrCase2Label: "Arduino - no reset",
    dtrCase3Label: "ESP32/ESP8266 - flash mode",
    rtsCase1Label: "ESP32/ESP8266 - flash mode",
    rtsCase2Label: "Some RS485 modules",
    rtsCase3Label: "Normal data reading",
    signalSummary: "90% of cases: Keep both OFF (default)",
    signalNote: "Only enable when device doesn't work or documentation requires it.",
    // Update
    updateAvailable: "Update available",
    updateTitle: "New Version",
    updateNow: "Update Now",
    updateLater: "Later",
    downloading: "Downloading...",
    installing: "Installing...",
    updateReady: "Ready to update",
    updateFailed: "Update failed",
    changelog: "Changelog",
    currentVersion: "Current version",
    newVersion: "New version",
    checkForUpdates: "Check for updates",
    checking: "Checking...",
    noUpdateAvailable: "You're using the latest version",
    updateError: "Update check failed",
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

// Update state (Windows only)
const updateAvailable = ref(false);
const updateInfo = ref(null);
const showUpdateModal = ref(false);
const updateProgress = ref(0);
const updateStatus = ref('idle'); // idle, checking, downloading, installing, ready, error, noUpdate
const currentVersion = ref('');
const updateError = ref('');
const isWindowsPlatform = ref(false);
let updateCheckInterval = null;

// Serial event listeners
let unlistenSerial = null;
let unlistenDisconnect = null;

// Batching for terminal updates (performance optimization)
const pendingRxData = new Map(); // port_name -> [{data, timestamp}]
let updateScheduled = false;

function flushPendingData() {
  for (const [portName, dataList] of pendingRxData) {
    const tab = getTabByPortName(portName);
    if (tab && dataList.length > 0) {
      // Merge all pending data into entries
      for (const item of dataList) {
        // Limit terminal entries
        if (tab.terminalData.length >= MAX_TERMINAL_ENTRIES) {
          const removed = tab.terminalData.shift();
          if (removed.type === 'tx') tab.txCount--;
          else tab.rxCount--;
        }

        tab.terminalData.push({
          type: "rx",
          data: item.data,
          timestamp: item.timestamp,
        });
        tab.rxCount++;
        tab.totalRxCount++;
      }
    }
  }
  pendingRxData.clear();
  updateScheduled = false;
}

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
      dtr: tab.dtr,
      rts: tab.rts,
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

async function requestCloseTab(tabId) {
  const tab = tabs.get(tabId);
  if (!tab) return;

  // Auto disconnect if connected
  if (tab.isConnected) {
    try {
      await invoke("close_port", { portName: tab.selectedPort });
      tab.isConnected = false;
    } catch (error) {
      console.error("Error closing port:", error);
    }
  }

  closeTab(tabId);
  // Create new tab if all tabs are closed
  if (tabs.size === 0) {
    createTab();
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

// Handle device disconnection (unplugged)
function handleDeviceDisconnected(portName, reason) {
  const tab = getTabByPortName(portName);
  if (tab) {
    // Update tab state
    tab.isConnected = false;

    // Stop auto send if running
    if (tab.autoSendEnabled) {
      tab.autoSendEnabled = false;
      if (tab.autoSendTimer) {
        clearInterval(tab.autoSendTimer);
        tab.autoSendTimer = null;
      }
    }

    // Show notification
    console.warn(`Device disconnected: ${portName} - ${reason}`);
    alert(`${t.value.deviceDisconnected}: ${portName}`);
  }
}

// Check if running on Windows
function isWindows() {
  return navigator.userAgent.includes('Windows');
}

// Check for updates (Windows only)
async function checkForUpdates(showModal = false) {
  if (!isWindows()) return;

  if (showModal) {
    updateStatus.value = 'checking';
    updateError.value = '';
    showUpdateModal.value = true;
  }

  try {
    const update = await check();

    if (update) {
      updateAvailable.value = true;
      updateInfo.value = {
        version: update.version,
        body: update.body, // Changelog from GitHub release notes
        date: update.date,
      };
      if (showModal) {
        updateStatus.value = 'idle';
      }
    } else {
      updateAvailable.value = false;
      updateInfo.value = null;
      if (showModal) {
        updateStatus.value = 'noUpdate';
      }
    }
  } catch (error) {
    console.error('Update check failed:', error);
    updateError.value = error.message || String(error);
    if (showModal) {
      updateStatus.value = 'error';
    }
  }
}

// Handle update button click
function handleUpdateClick() {
  checkForUpdates(true);
}

// Perform the update
async function performUpdate() {
  if (!updateInfo.value) return;

  updateStatus.value = 'downloading';

  try {
    const update = await check();

    if (update) {
      let downloaded = 0;
      let contentLength = 0;

      // Download with progress
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength || 0;
            updateProgress.value = 0;
            break;
          case 'Progress':
            downloaded += event.data.chunkLength;
            if (contentLength > 0) {
              updateProgress.value = Math.round((downloaded / contentLength) * 100);
            }
            break;
          case 'Finished':
            updateProgress.value = 100;
            updateStatus.value = 'installing';
            break;
        }
      });

      updateStatus.value = 'ready';

      // Relaunch after short delay
      setTimeout(async () => {
        await relaunch();
      }, 1000);
    }
  } catch (error) {
    console.error('Update failed:', error);
    updateStatus.value = 'error';
  }
}

// Cancel update modal
function cancelUpdate() {
  showUpdateModal.value = false;
  updateStatus.value = 'idle';
  updateProgress.value = 0;
}

// Lifecycle
onMounted(async () => {
  // Set window title with version
  try {
    const version = await getVersion();
    currentVersion.value = version;
    await getCurrentWindow().setTitle(`TermiPro by vienkmt - v${version}`);
  } catch (e) {
    console.warn('Could not set window title:', e);
  }

  // Create initial tab
  createTab();

  // Refresh ports
  await refreshPorts();

  // Check platform
  isWindowsPlatform.value = isWindows();

  // Check for updates on startup (Windows only)
  await checkForUpdates();

  // Check for updates every 30 minutes
  updateCheckInterval = setInterval(() => checkForUpdates(false), 30 * 60 * 1000);

  // Global serial data listener - routes data to correct tab with batching
  unlistenSerial = await listen("serial-data", (event) => {
    const { port_name, data, timestamp } = event.payload;

    // Add to pending batch
    if (!pendingRxData.has(port_name)) {
      pendingRxData.set(port_name, []);
    }
    pendingRxData.get(port_name).push({
      data: data,
      timestamp: new Date(timestamp).toLocaleTimeString(),
    });

    // Schedule update if not already scheduled
    if (!updateScheduled) {
      updateScheduled = true;
      requestAnimationFrame(flushPendingData);
    }
  });

  // Listen for device disconnection events
  unlistenDisconnect = await listen("serial-disconnected", (event) => {
    const { port_name, reason } = event.payload;
    handleDeviceDisconnected(port_name, reason);
  });
});

onUnmounted(async () => {
  // Cleanup event listeners
  if (unlistenSerial) {
    unlistenSerial();
  }
  if (unlistenDisconnect) {
    unlistenDisconnect();
  }

  // Clear update check interval
  if (updateCheckInterval) {
    clearInterval(updateCheckInterval);
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
        <!-- Update Button (Windows only) -->
        <button
          v-if="isWindowsPlatform"
          class="update-btn"
          :class="{ 'has-update': updateAvailable }"
          @click="handleUpdateClick"
          :title="updateAvailable ? t.updateAvailable : t.checkForUpdates"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
          <span v-if="updateAvailable" class="update-badge-dot">1</span>
        </button>

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

    <!-- Update Modal -->
    <UpdateModal
      :visible="showUpdateModal"
      :update-info="updateInfo"
      :current-version="currentVersion"
      :status="updateStatus"
      :progress="updateProgress"
      :error-message="updateError"
      @confirm="performUpdate"
      @cancel="cancelUpdate"
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
  padding: 5px 12px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  border-radius: var(--radius-sm);
  color: white;
}

.logo svg {
  width: 12px;
  height: 12px;
}

.header h1 {
  font-size: 0.8rem;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.02em;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 0.6rem;
  font-weight: 600;
  transition: all 0.3s ease;
}

.status-badge.connected {
  background: var(--success);
  color: white;
}

.status-badge.disconnected {
  background: var(--bg-tertiary);
  color: var(--text-tertiary);
}

.status-dot {
  width: 5px;
  height: 5px;
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
  gap: 4px;
  padding: 3px 6px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
  margin-left: 8px;
}

.lang-switch:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}

.lang-flag {
  flex-shrink: 0;
  border-radius: 2px;
  box-shadow: 0 0 0 1px rgba(0,0,0,0.1);
  width: 16px;
  height: 11px;
}

.lang-code {
  font-size: 0.6rem;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: 0.05em;
}

/* Update Button */
.update-btn {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  padding: 0;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  margin-right: 12px;
}

.update-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
  transform: scale(1.05);
}

.update-btn.has-update {
  border: 2px solid var(--warning);
  color: var(--warning);
  animation: blink-border 1.5s ease-in-out infinite;
}

.update-btn.has-update:hover {
  background: var(--warning-light);
}

@keyframes blink-border {
  0%, 100% {
    border-color: var(--warning);
    box-shadow: 0 0 0 0 rgba(245, 158, 11, 0);
  }
  50% {
    border-color: var(--warning);
    box-shadow: 0 0 8px 3px rgba(245, 158, 11, 0.4);
  }
}

.update-badge-dot {
  position: absolute;
  top: -4px;
  right: -4px;
  min-width: 16px;
  height: 16px;
  padding: 0 4px;
  background: var(--danger);
  border-radius: 50%;
  font-size: 0.6rem;
  font-weight: 700;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
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
