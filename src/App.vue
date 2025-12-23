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
import TcpClientTab from "./components/TcpClientTab.vue";
import TcpServerTab from "./components/TcpServerTab.vue";
import ModbusTab from "./components/ModbusTab.vue";
import ConfirmDialog from "./components/ConfirmDialog.vue";
import UpdateModal from "./components/UpdateModal.vue";
import NewTabModal from "./components/NewTabModal.vue";
import { useTabStore, CONNECTION_TYPES } from "./stores/tabStore";

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
    // TCP
    tcpClient: "TCP Client",
    tcpServer: "TCP Server",
    host: "Host",
    port: "Port",
    listenPort: "Cổng lắng nghe",
    bindAddress: "Địa chỉ bind",
    maxClients: "Tối đa client",
    startServer: "Khởi động",
    stopServer: "Dừng Server",
    connectedClients: "Client đã kết nối",
    noClients: "Chưa có client kết nối",
    sendTo: "Gửi đến",
    sendToAll: "Tất cả client",
    selectConnectionType: "Chọn loại kết nối",
    connectionTypeSerial: "Serial Port",
    connectionTypeTcpClient: "TCP Client",
    connectionTypeTcpServer: "TCP Server",
    connectionTypeModbus: "Modbus Master",
    serialDesc: "Kết nối với thiết bị qua cổng serial",
    tcpClientDesc: "Kết nối đến server TCP từ xa",
    tcpServerDesc: "Lắng nghe kết nối TCP đến",
    modbusDesc: "Giao tiếp Modbus RTU/TCP với thiết bị công nghiệp",
    // Modbus
    mode: "Chế độ",
    rtu: "RTU",
    tcp: "TCP",
    slaveId: "Slave ID",
    unitId: "Unit ID",
    responseTimeout: "Timeout (ms)",
    functionCode: "Function Code",
    startAddress: "Địa chỉ bắt đầu",
    quantity: "Số lượng",
    value: "Giá trị",
    readCoils: "Đọc Coils",
    readDiscreteInputs: "Đọc Discrete Inputs",
    readHoldingRegisters: "Đọc Holding Registers",
    readInputRegisters: "Đọc Input Registers",
    writeSingleCoil: "Ghi Single Coil",
    writeSingleRegister: "Ghi Single Register",
    writeMultipleCoils: "Ghi Multiple Coils",
    writeMultipleRegisters: "Ghi Multiple Registers",
    dataFormat: "Định dạng",
    unsigned: "Unsigned",
    signed: "Signed",
    hex: "Hex",
    float32: "Float32",
    binary: "Binary",
    polling: "Polling",
    pollingInterval: "Chu kỳ (ms)",
    addRequest: "Thêm request",
    startPolling: "Bắt đầu",
    stopPolling: "Dừng",
    transactionLog: "Nhật ký giao dịch",
    address: "Địa chỉ",
    rawValue: "Raw",
    formattedValue: "Giá trị",
    request: "Yêu cầu",
    response: "Phản hồi",
    success: "Thành công",
    error: "Lỗi",
    sendRequest: "Gửi",
    noRegisterData: "Chưa có dữ liệu register",
    executeRequestFirst: "Gửi yêu cầu để xem dữ liệu",
    coilOn: "BẬT",
    coilOff: "TẮT",
    writeValues: "Giá trị ghi",
    coilValue: "Giá trị Coil",
    startServerToBegin: "Khởi động server để bắt đầu",
    pleaseStartFirst: "Vui lòng khởi động server trước!",
    reconnecting: "Đang kết nối lại...",
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
    // TCP
    tcpClient: "TCP Client",
    tcpServer: "TCP Server",
    host: "Host",
    port: "Port",
    listenPort: "Listen Port",
    bindAddress: "Bind Address",
    maxClients: "Max Clients",
    startServer: "Start",
    stopServer: "Stop Server",
    connectedClients: "Connected Clients",
    noClients: "No clients connected",
    sendTo: "Send to",
    sendToAll: "All clients",
    selectConnectionType: "Select Connection Type",
    connectionTypeSerial: "Serial Port",
    connectionTypeTcpClient: "TCP Client",
    connectionTypeTcpServer: "TCP Server",
    connectionTypeModbus: "Modbus Master",
    serialDesc: "Connect to device via serial port",
    tcpClientDesc: "Connect to remote TCP server",
    tcpServerDesc: "Listen for incoming TCP connections",
    modbusDesc: "Modbus RTU/TCP communication with industrial devices",
    // Modbus
    mode: "Mode",
    rtu: "RTU",
    tcp: "TCP",
    slaveId: "Slave ID",
    unitId: "Unit ID",
    responseTimeout: "Timeout (ms)",
    functionCode: "Function Code",
    startAddress: "Start Address",
    quantity: "Quantity",
    value: "Value",
    readCoils: "Read Coils",
    readDiscreteInputs: "Read Discrete Inputs",
    readHoldingRegisters: "Read Holding Registers",
    readInputRegisters: "Read Input Registers",
    writeSingleCoil: "Write Single Coil",
    writeSingleRegister: "Write Single Register",
    writeMultipleCoils: "Write Multiple Coils",
    writeMultipleRegisters: "Write Multiple Registers",
    dataFormat: "Format",
    unsigned: "Unsigned",
    signed: "Signed",
    hex: "Hex",
    float32: "Float32",
    binary: "Binary",
    polling: "Polling",
    pollingInterval: "Interval (ms)",
    addRequest: "Add Request",
    startPolling: "Start",
    stopPolling: "Stop",
    transactionLog: "Transaction Log",
    address: "Address",
    rawValue: "Raw",
    formattedValue: "Value",
    request: "Request",
    response: "Response",
    success: "Success",
    error: "Error",
    sendRequest: "Send",
    noRegisterData: "No register data",
    executeRequestFirst: "Send a request to view data",
    coilOn: "ON",
    coilOff: "OFF",
    writeValues: "Write Values",
    coilValue: "Coil Value",
    startServerToBegin: "Start server to begin",
    pleaseStartFirst: "Please start server first!",
    reconnecting: "Reconnecting...",
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
const { tabs, activeTabId, tabOrder, activeTab, canAddTab, createTab, closeTab, setActiveTab, getTabByPortName, getTabByConnectionId, getConnectedPorts } = tabStore;

// New tab modal state
const showNewTabModal = ref(false);

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

// TCP event listeners
let unlistenTcpData = null;
let unlistenTcpClientStatus = null;
let unlistenTcpServerStatus = null;
let unlistenTcpServerClientEvent = null;

// Modbus event listeners
let unlistenModbusStatus = null;
let unlistenModbusResponse = null;
let unlistenModbusPollData = null;

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

  // Auto disconnect if connected based on connection type
  if (tab.isConnected) {
    try {
      if (tab.connectionType === CONNECTION_TYPES.SERIAL) {
        await invoke("close_port", { portName: tab.selectedPort });
      } else if (tab.connectionType === CONNECTION_TYPES.TCP_CLIENT) {
        await invoke("tcp_client_disconnect", { connectionId: tab.connectionId });
      } else if (tab.connectionType === CONNECTION_TYPES.TCP_SERVER) {
        await invoke("tcp_server_stop", { serverId: tab.serverId });
      } else if (tab.connectionType === CONNECTION_TYPES.MODBUS) {
        // Stop polling first if running
        if (tab.pollingEnabled) {
          await invoke("modbus_stop_polling", {
            connectionId: tab.modbusConnectionId || tab.connectionId,
          });
        }
        await invoke("modbus_disconnect", {
          connectionId: tab.modbusConnectionId || tab.connectionId,
        });
      }
      tab.isConnected = false;
    } catch (error) {
      console.error("Error closing connection:", error);
    }
  }

  closeTab(tabId);
  // Create new tab if all tabs are closed
  if (tabs.size === 0) {
    showNewTabModal.value = true;
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
  showNewTabModal.value = true;
}

function handleNewTabSelect(connectionType) {
  createTab(connectionType);
  showNewTabModal.value = false;
}

function cancelNewTabModal() {
  showNewTabModal.value = false;
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

// ===================== TCP HANDLERS =====================

// TCP Client connect
async function handleTcpClientConnect(tabId) {
  const tab = tabs.get(tabId);
  if (!tab) return;

  try {
    const config = {
      host: tab.host,
      port: tab.port,
      connection_id: tab.connectionId,
    };

    await invoke("tcp_client_connect", { config });
    // Status will be updated via event listener
  } catch (error) {
    console.error("TCP Client connection error:", error);
    alert("Error: " + error);
  }
}

// TCP Client disconnect
async function handleTcpClientDisconnect(tabId) {
  const tab = tabs.get(tabId);
  if (!tab) return;

  try {
    await invoke("tcp_client_disconnect", { connectionId: tab.connectionId });
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
    console.error("TCP Client disconnect error:", error);
    alert("Error: " + error);
  }
}

// TCP Server start
async function handleTcpServerStart(tabId) {
  console.log("[TCP Server] handleTcpServerStart called with tabId:", tabId);
  const tab = tabs.get(tabId);
  if (!tab) {
    console.error("[TCP Server] Tab not found:", tabId);
    return;
  }

  try {
    const config = {
      port: tab.listenPort,
      bind_address: tab.bindAddress,
      server_id: tab.serverId,
      max_clients: tab.maxClients,
    };
    console.log("[TCP Server] Starting with config:", config);

    const result = await invoke("tcp_server_start", { config });
    console.log("[TCP Server] invoke result:", result);
    // Status will be updated via event listener
  } catch (error) {
    console.error("[TCP Server] Start error:", error);
    alert("Error: " + error);
  }
}

// TCP Server stop
async function handleTcpServerStop(tabId) {
  const tab = tabs.get(tabId);
  if (!tab) return;

  try {
    await invoke("tcp_server_stop", { serverId: tab.serverId });
    tab.isConnected = false;
    tab.connectedClients = [];

    // Stop auto send if running
    if (tab.autoSendEnabled) {
      tab.autoSendEnabled = false;
      if (tab.autoSendTimer) {
        clearInterval(tab.autoSendTimer);
        tab.autoSendTimer = null;
      }
    }
  } catch (error) {
    console.error("TCP Server stop error:", error);
    alert("Error: " + error);
  }
}

// ===================== MODBUS HANDLERS =====================

// Modbus connect (RTU or TCP)
async function handleModbusConnect(tabId) {
  const tab = tabs.get(tabId);
  if (!tab) return;

  try {
    let connectionId;

    if (tab.mode === 'rtu') {
      // RTU mode - use serial port
      if (!tab.selectedPort) {
        alert(t.value.pleaseSelectPort);
        return;
      }

      const config = {
        port_name: tab.selectedPort,
        baud_rate: tab.baudRate,
        data_bits: tab.dataBits,
        stop_bits: tab.stopBits,
        parity: tab.parity,
        slave_id: tab.slaveId,
        response_timeout_ms: tab.responseTimeout,
      };

      connectionId = await invoke("modbus_rtu_connect", { config });
    } else {
      // TCP mode
      const config = {
        host: tab.host,
        port: tab.port,
        unit_id: tab.unitId,
        response_timeout_ms: tab.responseTimeout,
      };

      connectionId = await invoke("modbus_tcp_connect", { config });
    }

    // Store the actual connection ID returned from backend
    tab.modbusConnectionId = connectionId;
    tab.isConnected = true;
    tab.connectionStatus = 'connected';
  } catch (error) {
    console.error("Modbus connection error:", error);
    tab.connectionStatus = 'error';
    tab.statusMessage = String(error);
    alert("Error: " + error);
  }
}

// Modbus disconnect
async function handleModbusDisconnect(tabId) {
  const tab = tabs.get(tabId);
  if (!tab) return;

  try {
    // Stop polling if running
    if (tab.pollingEnabled) {
      await invoke("modbus_stop_polling", {
        connectionId: tab.modbusConnectionId || tab.connectionId,
      });
      tab.pollingEnabled = false;
    }

    await invoke("modbus_disconnect", {
      connectionId: tab.modbusConnectionId || tab.connectionId,
    });
    tab.isConnected = false;
    tab.connectionStatus = 'idle';
    tab.modbusConnectionId = null;
  } catch (error) {
    console.error("Modbus disconnect error:", error);
    alert("Error: " + error);
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
// Keyboard shortcut handler
let isClosingTab = false;
function handleKeyboardShortcuts(event) {
  const isMac = navigator.userAgent.toUpperCase().indexOf('MAC') >= 0;
  const modifier = isMac ? event.metaKey : event.ctrlKey;

  if (modifier && event.key.toLowerCase() === 't') {
    event.preventDefault();
    event.stopPropagation();
    event.stopImmediatePropagation();
    if (canAddTab.value && !showNewTabModal.value) {
      showNewTabModal.value = true;
    }
    return false;
  } else if (modifier && event.key.toLowerCase() === 'w') {
    event.preventDefault();
    event.stopPropagation();
    event.stopImmediatePropagation();
    // Prevent multiple close calls
    if (activeTabId.value && !isClosingTab && !showConfirmDialog.value) {
      isClosingTab = true;
      console.log('[Shortcut] Closing tab:', activeTabId.value);
      requestCloseTab(activeTabId.value);
      setTimeout(() => { isClosingTab = false; }, 300);
    }
    return false;
  }
}

onMounted(async () => {
  // Add keyboard shortcuts (capture phase to intercept before other handlers)
  document.addEventListener('keydown', handleKeyboardShortcuts, true);

  // Set window title with version
  try {
    const version = await getVersion();
    currentVersion.value = version;
    await getCurrentWindow().setTitle(`TermiPro v${version} - by vienkmt`);
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

  // ===================== TCP EVENT LISTENERS =====================

  // TCP data listener - routes data to correct tab
  unlistenTcpData = await listen("tcp-data", (event) => {
    const { connection_id, client_id, data, timestamp } = event.payload;
    const tab = getTabByConnectionId(connection_id);
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
        clientId: client_id || null,
      });
      tab.rxCount++;
      tab.totalRxCount++;
    }
  });

  // TCP Client status listener
  unlistenTcpClientStatus = await listen("tcp-client-status", (event) => {
    const { connection_id, status, message } = event.payload;
    const tab = getTabByConnectionId(connection_id);
    if (tab && tab.connectionType === CONNECTION_TYPES.TCP_CLIENT) {
      // Update connection status
      tab.connectionStatus = status;
      tab.statusMessage = message || null;

      if (status === "connected") {
        tab.isConnected = true;
        tab.isReconnecting = false;
      } else if (status === "reconnecting" || status === "retrying") {
        // Đang thử kết nối lại hoặc retry gửi data
        tab.isReconnecting = true;
        // Vẫn giữ isConnected = true trong lúc reconnecting để không break UI
        console.warn(`TCP Client ${status}:`, message);
      } else if (status === "write_failed") {
        // Gửi thất bại nhưng chưa disconnect hẳn
        tab.isReconnecting = true;
        console.error("TCP Client write failed:", message);
      } else if (status === "disconnected" || status === "error") {
        tab.isConnected = false;
        tab.isReconnecting = false;
        // Stop auto send if running
        if (tab.autoSendEnabled) {
          tab.autoSendEnabled = false;
          if (tab.autoSendTimer) {
            clearInterval(tab.autoSendTimer);
            tab.autoSendTimer = null;
          }
        }
        if (message) {
          console.error(`TCP Client ${status}:`, message);
        }
      }
    }
  });

  // TCP Server status listener
  unlistenTcpServerStatus = await listen("tcp-server-status", (event) => {
    console.log("[TCP Server] Status event received:", event.payload);
    const { connection_id, status, message } = event.payload;
    const tab = getTabByConnectionId(connection_id);
    console.log("[TCP Server] Found tab for connection_id:", connection_id, "tab:", tab?.id);
    if (tab && tab.connectionType === CONNECTION_TYPES.TCP_SERVER) {
      if (status === "started") {
        console.log("[TCP Server] Setting isConnected = true for tab:", tab.id);
        tab.isConnected = true;
        tab.statusMessage = message || null;
        // Sync echo state to Rust backend
        if (tab.echoEnabled) {
          invoke("tcp_server_set_echo", {
            serverId: tab.serverId,
            enabled: tab.echoEnabled,
          }).catch(err => console.error("Error syncing echo state:", err));
        }
      } else if (status === "stopped") {
        tab.isConnected = false;
        tab.connectedClients = [];
        tab.statusMessage = null;
        // Stop auto send if running
        if (tab.autoSendEnabled) {
          tab.autoSendEnabled = false;
          if (tab.autoSendTimer) {
            clearInterval(tab.autoSendTimer);
            tab.autoSendTimer = null;
          }
        }
      } else if (status === "error") {
        tab.isConnected = false;
        tab.connectedClients = [];
        // Set error message to show in UI
        tab.statusMessage = message || "Lỗi không xác định";
        // Stop auto send if running
        if (tab.autoSendEnabled) {
          tab.autoSendEnabled = false;
          if (tab.autoSendTimer) {
            clearInterval(tab.autoSendTimer);
            tab.autoSendTimer = null;
          }
        }
        if (message) {
          console.error("[TCP Server] Error:", message);
        }
      }
    } else {
      console.warn("[TCP Server] Tab not found or wrong type for connection_id:", connection_id);
    }
  });

  // TCP Server client events listener
  unlistenTcpServerClientEvent = await listen("tcp-server-client-event", (event) => {
    const { server_id, client_id, remote_addr, event_type } = event.payload;
    const tab = getTabByConnectionId(server_id);
    if (tab && tab.connectionType === CONNECTION_TYPES.TCP_SERVER) {
      if (event_type === "connected") {
        tab.connectedClients.push({
          clientId: client_id,
          remoteAddr: remote_addr,
          connectedAt: Date.now(),
        });
      } else if (event_type === "disconnected") {
        tab.connectedClients = tab.connectedClients.filter(c => c.clientId !== client_id);
        // Reset selectedClientId if the selected client disconnected
        if (tab.selectedClientId === client_id) {
          tab.selectedClientId = null;
        }
      }
    }
  });

  // ===================== MODBUS EVENT LISTENERS =====================

  // Modbus status listener
  unlistenModbusStatus = await listen("modbus-status", (event) => {
    const { connection_id, status, message } = event.payload;
    const tab = tabStore.getModbusTabByConnectionId(connection_id);
    if (tab) {
      tab.connectionStatus = status;
      tab.statusMessage = message || null;

      if (status === "connected") {
        tab.isConnected = true;
      } else if (status === "disconnected" || status === "error") {
        tab.isConnected = false;
        // Stop polling if running
        tab.pollingEnabled = false;
      }
    }
  });

  // Modbus response listener (for single requests)
  unlistenModbusResponse = await listen("modbus-response", (event) => {
    const { connection_id, transaction_id, slave_id, function_code, success, data, coils, error_code, request_frame, response_frame, response_time_ms, timestamp } = event.payload;
    const tab = tabStore.getModbusTabByConnectionId(connection_id);
    if (tab) {
      // Add to transaction log
      const logEntry = {
        id: transaction_id,
        timestamp: new Date(timestamp).toLocaleTimeString(),
        functionCode: function_code,
        slaveId: slave_id,
        success: success,
        requestFrame: request_frame,
        responseFrame: response_frame,
        responseTime: response_time_ms,
        errorCode: error_code,
      };

      tab.transactionLog.unshift(logEntry);
      // Limit log entries
      if (tab.transactionLog.length > tab.maxLogEntries) {
        tab.transactionLog.pop();
      }

      // Update last response time
      tab.lastResponseTime = response_time_ms;

      // Update data if successful
      if (success) {
        if (data && data.length > 0) {
          // Register data (FC03, FC04)
          tab.registerData = data.map((value, index) => ({
            address: tab.startAddress + index,
            value: value,
            rawHex: value.toString(16).toUpperCase().padStart(4, '0'),
          }));
        }
        if (coils && coils.length > 0) {
          // Coil data (FC01, FC02)
          tab.coilData = coils.map((value, index) => ({
            address: tab.startAddress + index,
            value: value,
          }));
        }
      }
    }
  });

  // Modbus poll data listener
  unlistenModbusPollData = await listen("modbus-poll-data", (event) => {
    const { connection_id, function_code, start_address, data, coils, timestamp } = event.payload;
    const tab = tabStore.getModbusTabByConnectionId(connection_id);
    if (tab) {
      // Update data based on function code
      if (data && data.length > 0) {
        tab.registerData = data.map((value, index) => ({
          address: start_address + index,
          value: value,
          rawHex: value.toString(16).toUpperCase().padStart(4, '0'),
        }));
      }
      if (coils && coils.length > 0) {
        tab.coilData = coils.map((value, index) => ({
          address: start_address + index,
          value: value,
        }));
      }
    }
  });
});

onUnmounted(async () => {
  // Cleanup keyboard shortcuts
  document.removeEventListener('keydown', handleKeyboardShortcuts, true);

  // Cleanup serial event listeners
  if (unlistenSerial) {
    unlistenSerial();
  }
  if (unlistenDisconnect) {
    unlistenDisconnect();
  }

  // Cleanup TCP event listeners
  if (unlistenTcpData) {
    unlistenTcpData();
  }
  if (unlistenTcpClientStatus) {
    unlistenTcpClientStatus();
  }
  if (unlistenTcpServerStatus) {
    unlistenTcpServerStatus();
  }
  if (unlistenTcpServerClientEvent) {
    unlistenTcpServerClientEvent();
  }

  // Cleanup Modbus event listeners
  if (unlistenModbusStatus) {
    unlistenModbusStatus();
  }
  if (unlistenModbusResponse) {
    unlistenModbusResponse();
  }
  if (unlistenModbusPollData) {
    unlistenModbusPollData();
  }

  // Clear update check interval
  if (updateCheckInterval) {
    clearInterval(updateCheckInterval);
  }

  // Close all connections based on connection type
  for (const [, tab] of tabs) {
    if (tab.isConnected) {
      try {
        if (tab.connectionType === CONNECTION_TYPES.SERIAL) {
          await invoke("close_port", { portName: tab.selectedPort });
        } else if (tab.connectionType === CONNECTION_TYPES.TCP_CLIENT) {
          await invoke("tcp_client_disconnect", { connectionId: tab.connectionId });
        } else if (tab.connectionType === CONNECTION_TYPES.TCP_SERVER) {
          await invoke("tcp_server_stop", { serverId: tab.serverId });
        } else if (tab.connectionType === CONNECTION_TYPES.MODBUS) {
          // Stop polling first if running
          if (tab.pollingEnabled) {
            await invoke("modbus_stop_polling", {
              connectionId: tab.modbusConnectionId || tab.connectionId,
            });
          }
          await invoke("modbus_disconnect", {
            connectionId: tab.modbusConnectionId || tab.connectionId,
          });
        }
      } catch (error) {
        console.error("Error closing connection:", error);
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
        <!-- Update Button (Windows only) -->
        <button
          v-if="isWindowsPlatform"
          class="update-btn"
          :class="{ 'has-update': updateAvailable }"
          @click="handleUpdateClick"
          :title="updateAvailable ? t.updateAvailable : t.checkForUpdates"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
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

    <!-- Main Content - Tabs (render all tabs, show only active) -->
    <div class="main-content">
      <template v-for="[tabId, tab] in tabs" :key="tabId">
        <!-- Serial Tab -->
        <SerialTab
          v-if="tab.connectionType === 'serial'"
          v-show="tabId === activeTabId"
          :tab-id="tabId"
          :tab-state="tab"
          :ports="ports"
          :connected-ports="connectedPorts"
          @connect="handleConnect"
          @disconnect="handleDisconnect"
          @refresh-ports="refreshPorts"
        />

        <!-- TCP Client Tab -->
        <TcpClientTab
          v-else-if="tab.connectionType === 'tcp_client'"
          v-show="tabId === activeTabId"
          :tab-id="tabId"
          :tab-state="tab"
          @connect="handleTcpClientConnect"
          @disconnect="handleTcpClientDisconnect"
        />

        <!-- TCP Server Tab -->
        <TcpServerTab
          v-else-if="tab.connectionType === 'tcp_server'"
          v-show="tabId === activeTabId"
          :tab-id="tabId"
          :tab-state="tab"
          @start="handleTcpServerStart"
          @stop="handleTcpServerStop"
        />

        <!-- Modbus Tab -->
        <ModbusTab
          v-else-if="tab.connectionType === 'modbus'"
          v-show="tabId === activeTabId"
          :tab-id="tabId"
          :tab-state="tab"
          :ports="ports"
          @connect="handleModbusConnect"
          @disconnect="handleModbusDisconnect"
          @refresh-ports="refreshPorts"
        />
      </template>
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

    <!-- New Tab Modal -->
    <NewTabModal
      :visible="showNewTabModal"
      @select="handleNewTabSelect"
      @cancel="cancelNewTabModal"
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
  width: 28px;
  height: 28px;
  padding: 0;
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all 0.2s ease;
  margin-left: 8px;
}

.update-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.update-btn.has-update {
  color: var(--accent-primary);
  animation: pulse-update 2s ease-in-out infinite;
}

.update-btn.has-update::after {
  content: '';
  position: absolute;
  top: 2px;
  right: 2px;
  width: 8px;
  height: 8px;
  background: var(--warning);
  border-radius: 50%;
  animation: ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
}

.update-btn.has-update:hover {
  color: var(--accent-secondary);
}

@keyframes pulse-update {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
}

@keyframes ping {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  75%, 100% {
    transform: scale(2);
    opacity: 0;
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
