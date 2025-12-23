import { ref, reactive, computed } from 'vue';

const MAX_TABS = 8;
let tabIdCounter = 0;

// Connection types
export const CONNECTION_TYPES = {
  SERIAL: 'serial',
  TCP_CLIENT: 'tcp_client',
  TCP_SERVER: 'tcp_server',
  MODBUS: 'modbus',
};

// Singleton store instance
let storeInstance = null;

export function useTabStore() {
  if (storeInstance) {
    return storeInstance;
  }

  const tabs = reactive(new Map());
  const activeTabId = ref(null);
  const tabOrder = ref([]);

  // Computed properties
  const activeTab = computed(() => tabs.get(activeTabId.value));
  const tabCount = computed(() => tabs.size);
  const canAddTab = computed(() => tabs.size < MAX_TABS);

  // Base tab state (shared across all connection types)
  function createBaseTabState(id, connectionType) {
    return {
      id,
      connectionType,
      isConnected: false,

      // Terminal data
      terminalData: [],
      txCount: 0,
      rxCount: 0,
      totalTxCount: 0,
      totalRxCount: 0,
      inputMessage: '',

      // Auto-send state
      autoSendEnabled: false,
      autoSendInterval: 1000,
      autoSendCount: 0,
      autoSendCurrentMessage: '',
      autoSendTimer: null,
      autoSendInProgress: false,

      // Display settings
      displayMode: 'text',
      sendAsHex: false,
      autoScroll: true,
      lineEnding: 'CR',
    };
  }

  // Serial tab state
  function createSerialTabState(id) {
    return reactive({
      ...createBaseTabState(id, CONNECTION_TYPES.SERIAL),
      // Serial-specific
      selectedPort: '',
      baudRate: 115200,
      dataBits: 8,
      stopBits: '1',
      parity: 'none',
      dtr: false,
      rts: false,
      byteDelay: 0,
    });
  }

  // TCP Client tab state
  function createTcpClientTabState(id) {
    return reactive({
      ...createBaseTabState(id, CONNECTION_TYPES.TCP_CLIENT),
      // TCP Client-specific
      host: 'localhost',
      port: 8080,
      connectionId: id,
      // Connection status tracking
      connectionStatus: 'idle', // idle, connected, reconnecting, retrying, write_failed, disconnected, error
      statusMessage: null,
      isReconnecting: false,
    });
  }

  // TCP Server tab state
  function createTcpServerTabState(id) {
    return reactive({
      ...createBaseTabState(id, CONNECTION_TYPES.TCP_SERVER),
      // TCP Server-specific
      listenPort: 5000,
      bindAddress: '0.0.0.0',
      serverId: id,
      maxClients: 20,
      connectedClients: [],
      selectedClientId: null, // null = send to all
      statusMessage: null,
      echoEnabled: false, // Echo received data back to client
    });
  }

  // Modbus Master tab state
  function createModbusTabState(id) {
    return reactive({
      id,
      connectionType: CONNECTION_TYPES.MODBUS,
      connectionId: id,
      isConnected: false,

      // Mode: 'rtu' or 'tcp'
      mode: 'rtu',

      // RTU Config (Serial)
      selectedPort: '',
      baudRate: 9600,
      dataBits: 8,
      stopBits: '1',
      parity: 'even', // Modbus default is even parity
      slaveId: 1,

      // TCP Config
      host: 'localhost',
      port: 502, // Standard Modbus TCP port
      unitId: 1,

      // Common Config
      responseTimeout: 1000,

      // Current Request
      functionCode: 3, // Read Holding Registers
      startAddress: 0,
      quantity: 10,
      writeValues: [],
      coilValues: [],

      // Data Display
      dataFormat: 'unsigned', // unsigned, signed, hex, float32, binary
      registerData: [], // [{address, value, rawHex}]
      coilData: [], // [{address, value}]

      // Transaction Log
      transactionLog: [], // [{id, type, timestamp, functionCode, requestFrame, responseFrame, success, responseTime, data}]
      maxLogEntries: 100,

      // Polling
      pollingEnabled: false,
      pollingInterval: 1000,
      pollRequests: [], // [{functionCode, startAddress, quantity}]

      // Status
      connectionStatus: 'idle', // idle, connected, disconnected, error
      statusMessage: null,
      lastResponseTime: null,
    });
  }

  // Create a new tab with specified connection type
  function createTab(connectionType = CONNECTION_TYPES.SERIAL) {
    if (!canAddTab.value) return null;

    const id = `tab-${++tabIdCounter}`;
    let tabState;

    switch (connectionType) {
      case CONNECTION_TYPES.TCP_CLIENT:
        tabState = createTcpClientTabState(id);
        break;
      case CONNECTION_TYPES.TCP_SERVER:
        tabState = createTcpServerTabState(id);
        break;
      case CONNECTION_TYPES.MODBUS:
        tabState = createModbusTabState(id);
        break;
      default:
        tabState = createSerialTabState(id);
    }

    tabs.set(id, tabState);
    tabOrder.value.push(id);

    // Always set new tab as active
    activeTabId.value = id;

    return id;
  }

  // Close a tab
  function closeTab(tabId) {
    const tab = tabs.get(tabId);
    if (!tab) return;

    // Clear auto-send timer if running
    if (tab.autoSendTimer) {
      clearInterval(tab.autoSendTimer);
      tab.autoSendTimer = null;
    }

    // Remove from tabs
    tabs.delete(tabId);

    // Remove from order
    const orderIndex = tabOrder.value.indexOf(tabId);
    if (orderIndex > -1) {
      tabOrder.value.splice(orderIndex, 1);
    }

    // Update active tab if needed
    if (activeTabId.value === tabId) {
      if (tabOrder.value.length > 0) {
        // Activate the previous tab, or the first one
        const newIndex = Math.max(0, orderIndex - 1);
        activeTabId.value = tabOrder.value[newIndex] || tabOrder.value[0];
      } else {
        activeTabId.value = null;
      }
    }
  }

  // Set active tab
  function setActiveTab(tabId) {
    if (tabs.has(tabId)) {
      activeTabId.value = tabId;
    }
  }

  // Get tab by port name (for serial event routing)
  function getTabByPortName(portName) {
    for (const [, tab] of tabs) {
      if (tab.connectionType === CONNECTION_TYPES.SERIAL &&
          tab.selectedPort === portName && tab.isConnected) {
        return tab;
      }
    }
    return null;
  }

  // Get tab by connection ID (for TCP/Modbus event routing)
  function getTabByConnectionId(connectionId) {
    for (const [, tab] of tabs) {
      if (tab.connectionType === CONNECTION_TYPES.TCP_CLIENT &&
          tab.connectionId === connectionId) {
        return tab;
      }
      if (tab.connectionType === CONNECTION_TYPES.TCP_SERVER &&
          tab.serverId === connectionId) {
        return tab;
      }
      if (tab.connectionType === CONNECTION_TYPES.MODBUS &&
          tab.connectionId === connectionId) {
        return tab;
      }
    }
    return null;
  }

  // Get Modbus tab by connection ID prefix
  function getModbusTabByConnectionId(connectionId) {
    for (const [, tab] of tabs) {
      if (tab.connectionType === CONNECTION_TYPES.MODBUS) {
        // Match by exact connectionId or by stored modbusConnectionId
        if (tab.connectionId === connectionId || tab.modbusConnectionId === connectionId) {
          return tab;
        }
      }
    }
    return null;
  }

  // Check if port is already connected in any tab
  function isPortConnected(portName) {
    for (const [, tab] of tabs) {
      if (tab.connectionType === CONNECTION_TYPES.SERIAL &&
          tab.selectedPort === portName && tab.isConnected) {
        return true;
      }
    }
    return false;
  }

  // Get all connected ports (serial only)
  function getConnectedPorts() {
    const connected = [];
    for (const [, tab] of tabs) {
      if (tab.connectionType === CONNECTION_TYPES.SERIAL &&
          tab.isConnected && tab.selectedPort) {
        connected.push(tab.selectedPort);
      }
    }
    return connected;
  }

  storeInstance = {
    // State
    tabs,
    activeTabId,
    tabOrder,

    // Computed
    activeTab,
    tabCount,
    canAddTab,

    // Methods
    createTab,
    closeTab,
    setActiveTab,
    getTabByPortName,
    getTabByConnectionId,
    getModbusTabByConnectionId,
    isPortConnected,
    getConnectedPorts,

    // Constants
    MAX_TABS,
    CONNECTION_TYPES,
  };

  return storeInstance;
}
