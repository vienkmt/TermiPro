import { ref, reactive, computed } from 'vue';

const MAX_TABS = 8;
let tabIdCounter = 0;

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

  // Create default tab state
  function createTabState(id) {
    return reactive({
      id,
      // Connection state
      selectedPort: '',
      isConnected: false,

      // Terminal data
      terminalData: [],
      inputMessage: '',

      // Serial configuration
      baudRate: 115200,
      dataBits: 8,
      stopBits: '1',
      parity: 'none',

      // Auto-send state
      autoSendEnabled: false,
      autoSendInterval: 1000,
      autoSendCount: 0,
      autoSendCurrentMessage: '',
      autoSendTimer: null,

      // Display settings
      displayMode: 'text',
      sendAsHex: false,
      autoScroll: true,
      lineEnding: 'CR',
    });
  }

  // Create a new tab
  function createTab() {
    if (!canAddTab.value) return null;

    const id = `tab-${++tabIdCounter}`;
    const tabState = createTabState(id);

    tabs.set(id, tabState);
    tabOrder.value.push(id);

    // Set as active if it's the first tab
    if (tabs.size === 1) {
      activeTabId.value = id;
    }

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

  // Get tab by port name (for event routing)
  function getTabByPortName(portName) {
    for (const [, tab] of tabs) {
      if (tab.selectedPort === portName && tab.isConnected) {
        return tab;
      }
    }
    return null;
  }

  // Check if port is already connected in any tab
  function isPortConnected(portName) {
    for (const [, tab] of tabs) {
      if (tab.selectedPort === portName && tab.isConnected) {
        return true;
      }
    }
    return false;
  }

  // Get all connected ports
  function getConnectedPorts() {
    const connected = [];
    for (const [, tab] of tabs) {
      if (tab.isConnected && tab.selectedPort) {
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
    isPortConnected,
    getConnectedPorts,

    // Constants
    MAX_TABS,
  };

  return storeInstance;
}
