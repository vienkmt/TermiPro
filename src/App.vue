<script setup>
import { ref, onMounted, onUnmounted, computed, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

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
  }
};

// Language state
const currentLang = ref("vi");
const t = computed(() => translations[currentLang.value]);

function toggleLanguage() {
  currentLang.value = currentLang.value === "vi" ? "en" : "vi";
}

// State
const ports = ref([]);
const selectedPort = ref("");
const isConnected = ref(false);
const terminalData = ref([]);
const inputMessage = ref("");
const displayMode = ref("text"); // 'text' or 'hex'
const sendAsHex = ref(false);
const autoScroll = ref(true);
const lineEnding = ref("CR"); // None, CR, LF, CRLF
const lineEndingOptions = ["None", "CR", "LF", "CRLF"];

// Line ending bytes
const getLineEndingBytes = () => {
  switch (lineEnding.value) {
    case "CR": return [0x0D];
    case "LF": return [0x0A];
    case "CRLF": return [0x0D, 0x0A];
    default: return [];
  }
};

const getLineEndingStr = () => {
  switch (lineEnding.value) {
    case "CR": return "\r";
    case "LF": return "\n";
    case "CRLF": return "\r\n";
    default: return "";
  }
};

// Serial config
const baudRate = ref(115200);
const dataBits = ref(8);
const stopBits = ref("1");
const parity = ref("none");

// Auto send config
const autoSendEnabled = ref(false);
const autoSendInterval = ref(1000);
let autoSendTimer = null;
const autoSendCount = ref(0);
const autoSendCurrentMessage = ref(""); // Lưu message khi bắt đầu auto send

// Config options
const baudRateOptions = [9600, 19200, 57600, 115200, 460800, 921600];
const dataBitsOptions = [5, 6, 7, 8];
const stopBitsOptions = ["1", "1.5", "2"];
const parityOptions = ["none", "odd", "even"];

// Dropdown state
const openDropdown = ref(null); // 'baudRate', 'dataBits', 'stopBits', 'parity', null

function toggleDropdown(name) {
  if (isConnected.value) return;
  // Refresh ports khi mở dropdown port
  if (name === 'port' && openDropdown.value !== 'port') {
    refreshPorts();
  }
  openDropdown.value = openDropdown.value === name ? null : name;
}

function selectOption(name, value) {
  switch(name) {
    case 'port': selectedPort.value = value; break;
    case 'baudRate': baudRate.value = value; break;
    case 'dataBits': dataBits.value = value; break;
    case 'stopBits': stopBits.value = value; break;
    case 'parity': parity.value = value; break;
  }
  openDropdown.value = null;
}

function closeDropdowns() {
  openDropdown.value = null;
}

// Format baud rate với dấu chấm
function formatBaudRate(rate) {
  return rate.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ".");
}

// Terminal ref
const terminalRef = ref(null);

// Unlisten function
let unlistenSerial = null;

// Computed
const connectionStatus = computed(() => {
  return isConnected.value ? t.value.connected : t.value.disconnected;
});

const connectionStatusClass = computed(() => {
  return isConnected.value ? "connected" : "disconnected";
});

// Đếm RX/TX
const rxCount = computed(() => terminalData.value.filter(e => e.type === 'rx').length);
const txCount = computed(() => terminalData.value.filter(e => e.type === 'tx').length);

// Format data for display
const formatByte = (byte) => {
  if (displayMode.value === "hex") {
    return byte.toString(16).padStart(2, "0").toUpperCase();
  }
  // Text mode
  if (byte >= 32 && byte <= 126) {
    return String.fromCharCode(byte);
  } else if (byte === 10) {
    return "\n";
  } else if (byte === 13) {
    return "";
  }
  return ".";
};

const formatDataLine = (entry) => {
  if (displayMode.value === "hex") {
    return entry.data.map((b) => b.toString(16).padStart(2, "0").toUpperCase()).join(" ");
  }
  return entry.data.map((b) => formatByte(b)).join("");
};

// Functions
async function refreshPorts() {
  try {
    const result = await invoke("list_serial_ports");
    ports.value = result;
    
    // Clear selectedPort nếu thiết bị đã bị tháo
    if (selectedPort.value && !result.find(p => p.name === selectedPort.value)) {
      selectedPort.value = "";
    }
  } catch (error) {
    console.error("Lỗi lấy danh sách port:", error);
  }
}

async function toggleConnection() {
  if (isConnected.value) {
    await disconnect();
  } else {
    await connect();
  }
}

async function connect() {
  if (!selectedPort.value) {
    alert(t.value.pleaseSelectPort);
    return;
  }

  try {
    const config = {
      port_name: selectedPort.value,
      baud_rate: baudRate.value,
      data_bits: dataBits.value,
      stop_bits: stopBits.value,
      parity: parity.value,
    };

    await invoke("open_port", { config });
    isConnected.value = true;

    // Listen for serial data
    unlistenSerial = await listen("serial-data", (event) => {
      const timestamp = new Date(event.payload.timestamp).toLocaleTimeString();
      terminalData.value.push({
        type: "rx",
        data: event.payload.data,
        timestamp,
      });

      // Auto scroll
      if (autoScroll.value) {
        nextTick(() => {
          if (terminalRef.value) {
            terminalRef.value.scrollTop = terminalRef.value.scrollHeight;
          }
        });
      }
    });
  } catch (error) {
    console.error("Lỗi kết nối:", error);
    alert("Lỗi: " + error);
  }
}

async function disconnect() {
  try {
    await invoke("close_port", { portName: selectedPort.value });
    isConnected.value = false;

    if (unlistenSerial) {
      unlistenSerial();
      unlistenSerial = null;
    }
  } catch (error) {
    console.error("Lỗi ngắt kết nối:", error);
    alert("Lỗi: " + error);
  }
}

async function sendMessage() {
  if (!inputMessage.value || !isConnected.value) return;

  try {
    // Thêm line ending vào data
    const dataToSend = sendAsHex.value ? inputMessage.value : (inputMessage.value + getLineEndingStr());
    
    await invoke("send_data", {
      portName: selectedPort.value,
      data: dataToSend,
      isHex: sendAsHex.value,
    });

    // Add to terminal as TX
    const timestamp = new Date().toLocaleTimeString();
    let dataBytes;
    if (sendAsHex.value) {
      const hexStr = inputMessage.value.replace(/\s/g, "").replace(/0x/gi, "");
      dataBytes = [];
      for (let i = 0; i < hexStr.length; i += 2) {
        dataBytes.push(parseInt(hexStr.substr(i, 2), 16));
      }
    } else {
      dataBytes = Array.from(new TextEncoder().encode(inputMessage.value));
      // Thêm line ending bytes
      dataBytes.push(...getLineEndingBytes());
    }

    terminalData.value.push({
      type: "tx",
      data: dataBytes,
      timestamp,
    });

    // Giữ nguyên nội dung input, không clear

    // Auto scroll
    if (autoScroll.value) {
      nextTick(() => {
        if (terminalRef.value) {
          terminalRef.value.scrollTop = terminalRef.value.scrollHeight;
        }
      });
    }
  } catch (error) {
    console.error("Lỗi gửi dữ liệu:", error);
    alert("Lỗi: " + error);
  }
}

function clearTerminal() {
  terminalData.value = [];
}

function handleKeyDown(event) {
  if (event.key === "Enter" && !event.shiftKey) {
    event.preventDefault();
    sendMessage();
  }
}

// Auto Send Functions
async function doAutoSend() {
  if (!autoSendCurrentMessage.value || !isConnected.value) return;

  try {
    // Thêm line ending vào data
    const dataToSend = sendAsHex.value ? autoSendCurrentMessage.value : (autoSendCurrentMessage.value + getLineEndingStr());
    
    await invoke("send_data", {
      portName: selectedPort.value,
      data: dataToSend,
      isHex: sendAsHex.value,
    });

    // Add to terminal as TX
    const timestamp = new Date().toLocaleTimeString();
    let dataBytes;
    if (sendAsHex.value) {
      const hexStr = autoSendCurrentMessage.value.replace(/\s/g, "").replace(/0x/gi, "");
      dataBytes = [];
      for (let i = 0; i < hexStr.length; i += 2) {
        dataBytes.push(parseInt(hexStr.substr(i, 2), 16));
      }
    } else {
      dataBytes = Array.from(new TextEncoder().encode(autoSendCurrentMessage.value));
      // Thêm line ending bytes
      dataBytes.push(...getLineEndingBytes());
    }

    terminalData.value.push({
      type: "tx",
      data: dataBytes,
      timestamp,
    });

    autoSendCount.value++;

    // Auto scroll
    if (autoScroll.value) {
      nextTick(() => {
        if (terminalRef.value) {
          terminalRef.value.scrollTop = terminalRef.value.scrollHeight;
        }
      });
    }
  } catch (error) {
    console.error("Auto send error:", error);
    stopAutoSend();
  }
}

function startAutoSend() {
  if (!inputMessage.value) {
    alert(t.value.pleaseEnterMessage);
    return;
  }
  if (!isConnected.value) {
    alert(t.value.pleaseConnectFirst);
    return;
  }
  
  // Lưu message hiện tại để gửi
  autoSendCurrentMessage.value = inputMessage.value;
  autoSendCount.value = 0;
  autoSendEnabled.value = true;
  
  // Gửi ngay lập tức lần đầu
  doAutoSend();
  
  // Thiết lập interval
  autoSendTimer = setInterval(() => {
    doAutoSend();
  }, autoSendInterval.value);
}

function stopAutoSend() {
  autoSendEnabled.value = false;
  if (autoSendTimer) {
    clearInterval(autoSendTimer);
    autoSendTimer = null;
  }
}

// Lifecycle
onMounted(() => {
  refreshPorts();
  // Click outside to close dropdowns
  document.addEventListener('click', closeDropdowns);
});

onUnmounted(() => {
  document.removeEventListener('click', closeDropdowns);
  stopAutoSend();
  if (unlistenSerial) {
    unlistenSerial();
  }
  if (isConnected.value) {
    disconnect();
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

    <div class="main-content">
      <!-- Sidebar -->
      <aside class="sidebar">
        <div class="config-card">
          <div class="card-header">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3"/>
              <path d="M12 1v6m0 6v10M4.22 4.22l4.24 4.24m7.08 7.08l4.24 4.24M1 12h6m6 0h10M4.22 19.78l4.24-4.24m7.08-7.08l4.24-4.24"/>
            </svg>
            <span>{{ t.serialPort }}</span>
          </div>
          <div class="port-dropdown-wrapper" @click.stop>
            <div class="dropdown-item port-dropdown" :class="{ disabled: isConnected, open: openDropdown === 'port' }">
              <div class="dropdown-trigger" @click="toggleDropdown('port')">
                <div class="dropdown-label">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="4" y="4" width="16" height="16" rx="2"/>
                    <rect x="9" y="9" width="6" height="6"/>
                    <line x1="9" y1="1" x2="9" y2="4"/>
                    <line x1="15" y1="1" x2="15" y2="4"/>
                    <line x1="9" y1="20" x2="9" y2="23"/>
                    <line x1="15" y1="20" x2="15" y2="23"/>
                    <line x1="20" y1="9" x2="23" y2="9"/>
                    <line x1="20" y1="15" x2="23" y2="15"/>
                    <line x1="1" y1="9" x2="4" y2="9"/>
                    <line x1="1" y1="15" x2="4" y2="15"/>
                  </svg>
                  <span v-if="!selectedPort" class="placeholder">{{ t.selectPort }}</span>
                  <span v-else class="selected-port">{{ ports.find(p => p.name === selectedPort)?.port_type || selectedPort }}</span>
                </div>
                <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="6,9 12,15 18,9"/>
                </svg>
              </div>
              <div class="dropdown-menu port-menu" v-if="openDropdown === 'port'">
                <div v-if="ports.length === 0" class="dropdown-empty">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"/>
                    <line x1="12" y1="8" x2="12" y2="12"/>
                    <line x1="12" y1="16" x2="12.01" y2="16"/>
                  </svg>
                  <span>{{ t.noDeviceFound }}</span>
                </div>
                <div 
                  v-for="port in ports" 
                  :key="port.name" 
                  class="dropdown-option port-option"
                  :class="{ selected: selectedPort === port.name }"
                  @click="selectOption('port', port.name)"
                >
                  <div class="port-details">
                    <span class="port-name">{{ port.port_type }}</span>
                    <span class="port-path">{{ port.name }}</span>
                  </div>
                  <svg v-if="selectedPort === port.name" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20,6 9,17 4,12"/>
                  </svg>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="config-card" @click.stop>
          <div class="card-header">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 20V10M18 20V4M6 20v-4"/>
            </svg>
            <span>{{ t.configuration }}</span>
          </div>

          <div class="config-grid-pro">
            <!-- Baud Rate -->
            <div class="dropdown-item" :class="{ disabled: isConnected, open: openDropdown === 'baudRate' }">
              <div class="dropdown-trigger" @click="toggleDropdown('baudRate')">
                <div class="dropdown-label">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                  </svg>
                  <span>{{ t.baud }}</span>
                </div>
                <div class="dropdown-value">
                  <span>{{ formatBaudRate(baudRate) }}</span>
                  <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="6,9 12,15 18,9"/>
                  </svg>
                </div>
              </div>
              <div class="dropdown-menu" v-if="openDropdown === 'baudRate'">
                <div 
                  v-for="rate in baudRateOptions" 
                  :key="rate" 
                  class="dropdown-option"
                  :class="{ selected: baudRate === rate }"
                  @click="selectOption('baudRate', rate)"
                >
                  {{ formatBaudRate(rate) }}
                  <svg v-if="baudRate === rate" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20,6 9,17 4,12"/>
                  </svg>
                </div>
              </div>
            </div>

            <!-- Data Bits -->
            <div class="dropdown-item" :class="{ disabled: isConnected, open: openDropdown === 'dataBits' }">
              <div class="dropdown-trigger" @click="toggleDropdown('dataBits')">
                <div class="dropdown-label">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="3" y="3" width="18" height="18" rx="2"/>
                    <path d="M9 9h6v6H9z"/>
                  </svg>
                  <span>{{ t.data }}</span>
                </div>
                <div class="dropdown-value">
                  <span>{{ dataBits }} bits</span>
                  <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="6,9 12,15 18,9"/>
                  </svg>
                </div>
              </div>
              <div class="dropdown-menu" v-if="openDropdown === 'dataBits'">
                <div 
                  v-for="bits in dataBitsOptions" 
                  :key="bits" 
                  class="dropdown-option"
                  :class="{ selected: dataBits === bits }"
                  @click="selectOption('dataBits', bits)"
                >
                  {{ bits }} bits
                  <svg v-if="dataBits === bits" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20,6 9,17 4,12"/>
                  </svg>
                </div>
              </div>
            </div>

            <!-- Stop Bits -->
            <div class="dropdown-item" :class="{ disabled: isConnected, open: openDropdown === 'stopBits' }">
              <div class="dropdown-trigger" @click="toggleDropdown('stopBits')">
                <div class="dropdown-label">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="6" y="4" width="4" height="16"/>
                    <rect x="14" y="4" width="4" height="16"/>
                  </svg>
                  <span>{{ t.stop }}</span>
                </div>
                <div class="dropdown-value">
                  <span>{{ stopBits }}</span>
                  <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="6,9 12,15 18,9"/>
                  </svg>
                </div>
              </div>
              <div class="dropdown-menu" v-if="openDropdown === 'stopBits'">
                <div 
                  v-for="bits in stopBitsOptions" 
                  :key="bits" 
                  class="dropdown-option"
                  :class="{ selected: stopBits === bits }"
                  @click="selectOption('stopBits', bits)"
                >
                  {{ bits }}
                  <svg v-if="stopBits === bits" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20,6 9,17 4,12"/>
                  </svg>
                </div>
              </div>
            </div>

            <!-- Parity -->
            <div class="dropdown-item" :class="{ disabled: isConnected, open: openDropdown === 'parity' }">
              <div class="dropdown-trigger" @click="toggleDropdown('parity')">
                <div class="dropdown-label">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                    <polyline points="22,4 12,14.01 9,11.01"/>
                  </svg>
                  <span>{{ t.parity }}</span>
                </div>
                <div class="dropdown-value">
                  <span>{{ parity.charAt(0).toUpperCase() + parity.slice(1) }}</span>
                  <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="6,9 12,15 18,9"/>
                  </svg>
                </div>
              </div>
              <div class="dropdown-menu" v-if="openDropdown === 'parity'">
                <div 
                  v-for="p in parityOptions" 
                  :key="p" 
                  class="dropdown-option"
                  :class="{ selected: parity === p }"
                  @click="selectOption('parity', p)"
                >
                  {{ p.charAt(0).toUpperCase() + p.slice(1) }}
                  <svg v-if="parity === p" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20,6 9,17 4,12"/>
                  </svg>
                </div>
              </div>
            </div>
          </div>
        </div>

        <button class="btn-connect" :class="{ connected: isConnected }" @click="toggleConnection">
          <svg v-if="!isConnected" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
          <svg v-else width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
          <span>{{ isConnected ? t.disconnect : t.connect }}</span>
        </button>

        <div class="config-card display-card">
          <div class="card-header">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
              <line x1="8" y1="21" x2="16" y2="21"/>
              <line x1="12" y1="17" x2="12" y2="21"/>
            </svg>
            <span>{{ t.display }}</span>
          </div>
          
          <div class="display-toggle">
            <button :class="{ active: displayMode === 'text' }" @click="displayMode = 'text'">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="4,7 4,4 20,4 20,7"/>
                <line x1="9" y1="20" x2="15" y2="20"/>
                <line x1="12" y1="4" x2="12" y2="20"/>
              </svg>
              Text
            </button>
            <button :class="{ active: displayMode === 'hex' }" @click="displayMode = 'hex'">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M4 17l6-6-6-6M12 19h8"/>
              </svg>
              Hex
            </button>
          </div>

          <label class="toggle-switch">
            <input type="checkbox" v-model="autoScroll" />
            <span class="toggle-slider"></span>
            <span class="toggle-label">{{ t.autoScroll }}</span>
          </label>

          <div class="line-ending-row">
            <span class="line-ending-label">Line Ending</span>
            <div class="line-ending-toggle">
              <button 
                v-for="opt in lineEndingOptions" 
                :key="opt"
                :class="{ active: lineEnding === opt }"
                @click="lineEnding = opt"
              >
                {{ opt }}
              </button>
            </div>
          </div>
        </div>

        <!-- Auto Send Config -->
        <div class="config-card auto-send-card">
          <div class="card-header">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <polyline points="12,6 12,12 16,14"/>
            </svg>
            <span>{{ t.autoSendSettings }}</span>
            <span v-if="autoSendEnabled" class="auto-send-badge running">ON</span>
          </div>

          <div class="auto-send-config">
            <div class="config-row-inline">
              <label>{{ t.frequency }}</label>
              <div class="interval-group">
                <input 
                  type="number" 
                  v-model.number="autoSendInterval" 
                  min="50" 
                  max="60000"
                  :disabled="autoSendEnabled"
                  class="interval-input"
                />
                <span class="interval-unit">ms</span>
              </div>
            </div>
            <div class="auto-send-info" v-if="autoSendEnabled">
              <span class="send-count">{{ t.sent }}: {{ autoSendCount }} {{ t.times }}</span>
            </div>
          </div>
        </div>
      </aside>

      <!-- Terminal -->
      <main class="terminal-container">
        <div class="terminal-header">
          <div class="terminal-title">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="4,17 10,11 4,5"/>
              <line x1="12" y1="19" x2="20" y2="19"/>
            </svg>
            <span>{{ t.terminal }}</span>
          </div>
          <div class="terminal-stats" v-if="terminalData.length > 0">
            <span class="stat-item tx">
              <span class="stat-arrow">↑</span>
              TX {{ txCount }}
            </span>
            <span class="stat-item rx">
              <span class="stat-arrow">↓</span>
              RX {{ rxCount }}
            </span>
          </div>
          <button class="btn-clear" @click="clearTerminal">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3,6 5,6 21,6"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
            {{ t.clear }}
          </button>
        </div>

        <div class="terminal" ref="terminalRef">
          <div
            v-for="(entry, index) in terminalData"
            :key="index"
            class="terminal-line"
            :class="entry.type"
          >
            <span class="timestamp">[{{ entry.timestamp }}]</span>
            <span class="direction-badge" :class="entry.type">{{ entry.type === "tx" ? "TX" : "RX" }}</span>
            <span class="data">{{ formatDataLine(entry) }}</span>
          </div>
          <div v-if="terminalData.length === 0" class="terminal-empty">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
              <line x1="8" y1="21" x2="16" y2="21"/>
              <line x1="12" y1="17" x2="12" y2="21"/>
            </svg>
            <p>{{ t.noData }}</p>
            <span>{{ t.connectToStart }}</span>
          </div>
        </div>

        <div class="send-container">
          <div class="send-wrapper">
            <label class="toggle-switch compact">
              <input type="checkbox" v-model="sendAsHex" :disabled="autoSendEnabled" />
              <span class="toggle-slider"></span>
              <span class="toggle-label">Hex</span>
            </label>
            <div class="input-wrapper">
              <input
                type="text"
                v-model="inputMessage"
                :placeholder="sendAsHex ? t.hexExample : t.enterMessage"
                :disabled="!isConnected || autoSendEnabled"
                @keydown="handleKeyDown"
              />
              <button 
                v-if="inputMessage" 
                class="btn-clear-input" 
                @click="inputMessage = ''"
                :title="t.clearContent"
              >
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="18" y1="6" x2="6" y2="18"/>
                  <line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </button>
            </div>
            <button class="btn-send" :disabled="!isConnected || !inputMessage || autoSendEnabled" @click="sendMessage">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="22" y1="2" x2="11" y2="13"/>
                <polygon points="22,2 15,22 11,13 2,9 22,2"/>
              </svg>
              {{ t.send }}
            </button>
            <button 
              v-if="!autoSendEnabled"
              class="btn-auto-send" 
              :disabled="!isConnected || !inputMessage" 
              @click="startAutoSend"
              :title="t.auto"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polygon points="5,3 19,12 5,21" fill="currentColor"/>
              </svg>
              {{ t.auto }}
            </button>
            <button 
              v-else
              class="btn-auto-stop" 
              @click="stopAutoSend"
              :title="t.stop"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <rect x="6" y="4" width="4" height="16"/>
                <rect x="14" y="4" width="4" height="16"/>
              </svg>
              {{ t.stop }}
            </button>
          </div>
        </div>
      </main>
    </div>
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

/* Sidebar */
.sidebar {
  width: 320px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  overflow-y: auto;
}

.config-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: 12px;
  box-shadow: var(--shadow-sm);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  color: var(--text-secondary);
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.card-header svg {
  color: var(--accent-primary);
  width: 14px;
  height: 14px;
}

/* Custom Select */
.port-selector {
  display: flex;
  gap: 6px;
  margin-bottom: 6px;
}

.port-selector .custom-select {
  flex: 1;
}

.custom-select {
  position: relative;
  width: 100%;
}

.custom-select select {
  width: 100%;
  padding: 8px 28px 8px 10px;
  background: var(--bg-tertiary);
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 0.75rem;
  font-family: var(--font-sans);
  font-weight: 500;
  cursor: pointer;
  appearance: none;
  -webkit-appearance: none;
  transition: all 0.2s ease;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.custom-select select:hover:not(:disabled) {
  background: var(--bg-hover);
}

.custom-select select:focus {
  outline: none;
  border-color: var(--border-focus);
  background: var(--bg-secondary);
  box-shadow: 0 0 0 4px var(--accent-light);
}

.custom-select.disabled select {
  opacity: 0.6;
  cursor: not-allowed;
}

.select-arrow {
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  pointer-events: none;
  color: var(--text-secondary);
  transition: transform 0.2s ease;
}

.custom-select select:focus + .select-arrow {
  transform: translateY(-50%) rotate(180deg);
}

.port-info {
  padding: 8px 12px;
  background: var(--accent-light);
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
  color: var(--accent-primary);
  font-weight: 500;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  min-width: 32px;
  height: 32px;
  background: var(--bg-tertiary);
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-icon svg {
  width: 14px;
  height: 14px;
}

.btn-icon:hover:not(:disabled) {
  background: var(--accent-light);
  color: var(--accent-primary);
}

.btn-icon:active:not(:disabled) {
  transform: scale(0.95);
}

.btn-icon:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Config Grid Pro */
.config-grid-pro {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

/* Custom Dropdown */
.dropdown-item {
  position: relative;
}

.dropdown-item.disabled {
  opacity: 0.6;
  pointer-events: none;
}

.dropdown-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
}

.dropdown-trigger:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}

.dropdown-item.open .dropdown-trigger {
  background: var(--bg-secondary);
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px var(--accent-light);
}

.dropdown-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.7rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.dropdown-label svg {
  color: var(--accent-primary);
}

.dropdown-value {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.dropdown-value .chevron {
  transition: transform 0.2s ease;
  color: var(--text-secondary);
}

.dropdown-item.open .dropdown-value .chevron {
  transform: rotate(180deg);
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  box-shadow: var(--shadow-lg);
  z-index: 100;
  max-height: 200px;
  overflow-y: auto;
  animation: dropdownSlide 0.15s ease;
}

@keyframes dropdownSlide {
  from {
    opacity: 0;
    transform: translateY(-8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.dropdown-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  font-size: 0.75rem;
  font-family: var(--font-mono);
  color: var(--text-primary);
  cursor: pointer;
  transition: background 0.15s ease;
}

.dropdown-option:hover {
  background: var(--bg-tertiary);
}

.dropdown-option.selected {
  background: var(--accent-light);
  color: var(--accent-primary);
  font-weight: 600;
}

.dropdown-option svg {
  color: var(--accent-primary);
}

/* Port Dropdown Specific */
.port-dropdown-wrapper {
  margin-bottom: 0;
}

.port-dropdown .dropdown-trigger {
  padding: 10px 12px;
}

.port-dropdown .dropdown-label {
  flex: 1;
  gap: 10px;
}

.port-dropdown .dropdown-label svg {
  flex-shrink: 0;
}

.port-dropdown .placeholder {
  color: var(--text-tertiary);
  font-size: 0.8rem;
}

.port-dropdown .selected-port {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.port-menu {
  max-height: 250px;
}

.dropdown-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 20px;
  color: var(--text-tertiary);
  font-size: 0.75rem;
}

.dropdown-empty svg {
  opacity: 0.5;
}

.port-option {
  padding: 10px 12px;
}

.port-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.port-name {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.port-path {
  font-size: 0.65rem;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

/* Connect Button */
.btn-connect {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 10px 16px;
  font-size: 0.8rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  border: none;
  border-radius: var(--radius-lg);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 14px rgb(99 102 241 / 0.35);
}

.btn-connect:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgb(99 102 241 / 0.4);
}

.btn-connect:active {
  transform: translateY(0);
}

.btn-connect.connected {
  background: linear-gradient(135deg, var(--danger), #f87171);
  box-shadow: 0 4px 14px rgb(239 68 68 / 0.35);
}

.btn-connect.connected:hover {
  box-shadow: 0 6px 20px rgb(239 68 68 / 0.4);
}

/* Display Card */
.display-card {
  margin-top: auto;
}

.line-ending-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--border-color);
}

.line-ending-label {
  font-size: 0.7rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.line-ending-toggle {
  display: flex;
  gap: 2px;
  padding: 2px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
}

.line-ending-toggle button {
  padding: 4px 8px;
  font-size: 0.65rem;
  font-weight: 600;
  font-family: var(--font-mono);
  background: transparent;
  border: none;
  border-radius: 3px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.line-ending-toggle button:hover {
  color: var(--text-primary);
}

.line-ending-toggle button.active {
  background: var(--bg-secondary);
  color: var(--accent-primary);
  box-shadow: var(--shadow-sm);
}

/* Auto Send Card */
.auto-send-card {
  border: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.auto-send-card .card-header {
  position: relative;
}

.auto-send-badge {
  position: absolute;
  right: 0;
  padding: 2px 8px;
  background: var(--success);
  color: white;
  border-radius: 10px;
  font-size: 0.6rem;
  font-weight: 600;
  animation: pulse-badge 1.5s infinite;
}

@keyframes pulse-badge {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

.auto-send-config {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.config-row-inline {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.config-row-inline label {
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--text-secondary);
}

.interval-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.interval-input {
  width: 80px;
  padding: 6px 8px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 0.75rem;
  font-family: var(--font-mono);
  text-align: right;
}

.interval-input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.interval-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.interval-unit {
  font-size: 0.7rem;
  color: var(--text-secondary);
}

.auto-send-info {
  padding: 6px 10px;
  background: var(--success-light);
  border-radius: var(--radius-sm);
  text-align: center;
}

.send-count {
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--success);
}

.display-toggle {
  display: flex;
  gap: 4px;
  padding: 3px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  margin-bottom: 10px;
}

.display-toggle button {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 6px;
  font-size: 0.7rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.display-toggle button svg {
  width: 12px;
  height: 12px;
}

.display-toggle button:hover {
  color: var(--text-primary);
}

.display-toggle button.active {
  background: var(--bg-secondary);
  color: var(--accent-primary);
  box-shadow: var(--shadow-sm);
}

/* Toggle Switch */
.toggle-switch {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.toggle-switch input {
  display: none;
}

.toggle-slider {
  position: relative;
  width: 36px;
  height: 20px;
  background: var(--bg-tertiary);
  border-radius: 50px;
  transition: all 0.3s ease;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background: white;
  border-radius: 50%;
  box-shadow: var(--shadow-sm);
  transition: all 0.3s ease;
}

.toggle-switch input:checked + .toggle-slider {
  background: var(--accent-primary);
}

.toggle-switch input:checked + .toggle-slider::before {
  transform: translateX(16px);
}

.toggle-label {
  font-size: 0.7rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.toggle-switch.compact .toggle-slider {
  width: 32px;
  height: 18px;
}

.toggle-switch.compact .toggle-slider::before {
  width: 14px;
  height: 14px;
}

.toggle-switch.compact input:checked + .toggle-slider::before {
  transform: translateX(14px);
}

/* Terminal Container */
.terminal-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  overflow: hidden;
}

.terminal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 20px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.terminal-title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
}

.terminal-title svg {
  color: var(--accent-primary);
}

.terminal-stats {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: auto;
  margin-right: 12px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 0.65rem;
  font-weight: 600;
  font-family: var(--font-mono);
}

.stat-item.tx {
  background: var(--warning-light);
  color: var(--warning);
}

.stat-item.rx {
  background: var(--success-light);
  color: var(--success);
}

.stat-arrow {
  font-size: 0.7rem;
  font-weight: 700;
}

.btn-clear {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  font-size: 0.8rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: var(--bg-tertiary);
  border: none;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-clear:hover {
  background: var(--danger-light);
  color: var(--danger);
}

/* Terminal */
.terminal {
  flex: 1;
  padding: 8px 12px;
  overflow-y: auto;
  font-family: var(--font-mono);
  font-size: 0.75rem;
  line-height: 1.4;
  background: var(--bg-primary);
}

.terminal-line {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 10px;
  margin-bottom: 2px;
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  border-left: 2px solid transparent;
}

.terminal-line.tx {
  border-left-color: var(--warning);
}

.terminal-line.rx {
  border-left-color: var(--success);
}

.terminal-line .timestamp {
  color: var(--text-tertiary);
  font-size: 0.65rem;
  min-width: 65px;
  flex-shrink: 0;
}

.direction-badge {
  padding: 1px 6px;
  border-radius: 3px;
  font-size: 0.6rem;
  font-weight: 700;
  min-width: 26px;
  text-align: center;
}

.direction-badge.tx {
  background: var(--warning-light);
  color: var(--warning);
}

.direction-badge.rx {
  background: var(--success-light);
  color: var(--success);
}

.terminal-line .data {
  color: var(--text-primary);
  word-break: break-all;
  white-space: pre-wrap;
}

.terminal-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-tertiary);
  text-align: center;
  gap: 12px;
}

.terminal-empty svg {
  opacity: 0.4;
}

.terminal-empty p {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-secondary);
}

.terminal-empty span {
  font-size: 0.85rem;
}

/* Send Container */
.send-container {
  padding: 16px 20px;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.send-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

.input-wrapper {
  flex: 1;
  position: relative;
}

.input-wrapper input {
  width: 100%;
  padding: 14px 40px 14px 18px;
  background: var(--bg-tertiary);
  border: 2px solid transparent;
  border-radius: var(--radius-lg);
  color: var(--text-primary);
  font-size: 0.9rem;
  font-family: var(--font-mono);
  transition: all 0.2s ease;
}

.btn-clear-input {
  position: absolute;
  right: 12px;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: var(--bg-hover);
  border: none;
  border-radius: 50%;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-clear-input:hover {
  background: var(--danger-light);
  color: var(--danger);
}

.input-wrapper input:focus {
  outline: none;
  border-color: var(--border-focus);
  background: var(--bg-secondary);
  box-shadow: 0 0 0 4px var(--accent-light);
}

.input-wrapper input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.input-wrapper input::placeholder {
  color: var(--text-tertiary);
}

.btn-send {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 24px;
  font-size: 0.9rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  border: none;
  border-radius: var(--radius-lg);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 4px 14px rgb(99 102 241 / 0.35);
}

.btn-send:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgb(99 102 241 / 0.4);
}

.btn-send:active:not(:disabled) {
  transform: translateY(0);
}

.btn-send:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.btn-auto-send {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 14px 20px;
  font-size: 0.85rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: var(--success);
  border: none;
  border-radius: var(--radius-lg);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-auto-send:hover:not(:disabled) {
  background: #059669;
  transform: translateY(-2px);
}

.btn-auto-send:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-auto-stop {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 14px 20px;
  font-size: 0.85rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: var(--danger);
  border: none;
  border-radius: var(--radius-lg);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  animation: pulse-btn 1.5s infinite;
}

@keyframes pulse-btn {
  0%, 100% { box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.4); }
  50% { box-shadow: 0 0 0 8px rgba(239, 68, 68, 0); }
}

.btn-auto-stop:hover {
  background: #dc2626;
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
