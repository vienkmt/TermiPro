<script setup>
import { ref, computed, inject, watch, nextTick, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  tabId: {
    type: String,
    required: true,
  },
  tabState: {
    type: Object,
    required: true,
  },
  ports: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(['connect', 'disconnect', 'refreshPorts']);

const t = inject('t');

// Local refs
const openDropdown = ref(null);
const requestLogRef = ref(null);
const isLoading = ref(false);
const editingCell = ref(null);
const editValue = ref('');

// Auto-increment state (for registers)
const autoIncrementAddresses = ref(new Set()); // Set of addresses being auto-incremented
const autoIncrementTimers = ref(new Map()); // Map of address -> timer ID
const autoIncrementInterval = 500; // ms between increments

// Auto-toggle state (for coils/discrete inputs)
const autoToggleAddresses = ref(new Set()); // Set of addresses being auto-toggled
const autoToggleTimers = ref(new Map()); // Map of address -> timer ID
const autoToggleInterval = 1000; // ms between toggles

// Constants
const MAX_LOG_ENTRIES = 100;
const MAX_VISIBLE_ENTRIES = 50; // Limit visible entries for performance

// Config options
const baudRateOptions = [9600, 19200, 38400, 57600, 115200];
const dataBitsOptions = [7, 8];
const stopBitsOptions = ['1', '2'];
const parityOptions = ['none', 'odd', 'even'];

const dataFormats = [
  { value: 'unsigned', label: 'Unsigned' },
  { value: 'signed', label: 'Signed' },
  { value: 'hex', label: 'Hex' },
  { value: 'binary', label: 'Binary' },
];

const dataTabs = [
  { value: 'coils', label: 'Coils', fc: '01/05/15' },
  { value: 'discrete_inputs', label: 'Discrete Inputs', fc: '02' },
  { value: 'holding_registers', label: 'Holding Registers', fc: '03/06/16' },
  { value: 'input_registers', label: 'Input Registers', fc: '04' },
];

// Computed
const currentDataTab = computed(() => props.tabState.activeDataTab);
const isCoilData = computed(() => ['coils', 'discrete_inputs'].includes(currentDataTab.value));
const isEditable = computed(() => ['coils', 'holding_registers'].includes(currentDataTab.value));

const displayData = computed(() => {
  const start = props.tabState.viewStartAddress;
  const qty = props.tabState.viewQuantity;

  let sourceData = [];
  switch (currentDataTab.value) {
    case 'coils':
      sourceData = props.tabState.coilsData;
      break;
    case 'discrete_inputs':
      sourceData = props.tabState.discreteInputsData;
      break;
    case 'holding_registers':
      sourceData = props.tabState.holdingRegistersData;
      break;
    case 'input_registers':
      sourceData = props.tabState.inputRegistersData;
      break;
  }

  return sourceData.slice(start, start + qty);
});

// Displayed log entries - limit and reverse for performance (newest at bottom)
const displayedLogEntries = computed(() => {
  const log = props.tabState.requestLog || [];
  // Take last MAX_VISIBLE_ENTRIES and reverse so newest is at bottom
  return log.slice(0, MAX_VISIBLE_ENTRIES).reverse();
});

// Helpers
function formatBaudRate(rate) {
  return rate.toString().replace(/\B(?=(\d{3})+(?!\d))/g, '.');
}

function formatHex(bytes) {
  if (!bytes || bytes.length === 0) return '';
  return bytes.map(b => b.toString(16).padStart(2, '0').toUpperCase()).join(' ');
}

function formatValue(value, format) {
  if (value === undefined || value === null) return '-';
  switch (format) {
    case 'unsigned':
      return value.toString();
    case 'signed':
      return value > 32767 ? (value - 65536).toString() : value.toString();
    case 'hex':
      return '0x' + value.toString(16).toUpperCase().padStart(4, '0');
    case 'binary':
      return value.toString(2).padStart(16, '0');
    default:
      return value.toString();
  }
}

function formatAddress(addr) {
  return addr.toString().padStart(5, '0');
}

function getFcName(fc) {
  const names = {
    1: 'Read Coils',
    2: 'Read DI',
    3: 'Read HR',
    4: 'Read IR',
    5: 'Write Coil',
    6: 'Write Reg',
    15: 'Write Coils',
    16: 'Write Regs',
  };
  return names[fc] || `FC${fc}`;
}

// Dropdown handlers
function toggleDropdown(name) {
  if (props.tabState.isConnected && !['dataFormat', 'activeDataTab'].includes(name)) return;
  if (name === 'port' && openDropdown.value !== 'port') {
    emit('refreshPorts');
  }
  openDropdown.value = openDropdown.value === name ? null : name;
}

function selectOption(name, value) {
  switch (name) {
    case 'port': props.tabState.selectedPort = value; break;
    case 'baudRate': props.tabState.baudRate = value; break;
    case 'dataBits': props.tabState.dataBits = value; break;
    case 'stopBits': props.tabState.stopBits = value; break;
    case 'parity': props.tabState.parity = value; break;
    case 'dataFormat': props.tabState.dataFormat = value; break;
  }
  openDropdown.value = null;
}

function closeDropdowns() {
  openDropdown.value = null;
}

// Connection handlers
async function toggleConnection() {
  if (props.tabState.isConnected) {
    // Stop all auto operations
    stopAllAutoIncrements();
    stopAllAutoToggles();

    // Reset data to defaults
    props.tabState.coilsData = [];
    props.tabState.discreteInputsData = [];
    props.tabState.holdingRegistersData = [];
    props.tabState.inputRegistersData = [];
    props.tabState.requestLog = [];
    props.tabState.requestCount = 0;

    emit('disconnect', props.tabId);
  } else {
    // Generate initial data based on viewStartAddress and viewQuantity
    initializeData();
    emit('connect', props.tabId);
  }
}

// Initialize data arrays with default values
function initializeData() {
  const start = props.tabState.viewStartAddress || 0;
  const qty = props.tabState.viewQuantity || 100;
  const end = start + qty;

  // Initialize coils (false by default)
  props.tabState.coilsData = new Array(end).fill(false);

  // Initialize discrete inputs (false by default)
  props.tabState.discreteInputsData = new Array(end).fill(false);

  // Initialize holding registers (0 by default)
  props.tabState.holdingRegistersData = new Array(end).fill(0);

  // Initialize input registers (0 by default)
  props.tabState.inputRegistersData = new Array(end).fill(0);
}

// Data editing
async function toggleCoil(address) {
  if (!props.tabState.isConnected || !isEditable.value) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    const currentValue = props.tabState.coilsData[address] || false;
    const newValue = !currentValue;

    await invoke('modbus_slave_set_coil', {
      connectionId,
      address,
      value: newValue,
    });

    // Update local state
    props.tabState.coilsData[address] = newValue;
  } catch (error) {
    console.error('Set coil error:', error);
  }
}

async function toggleDiscreteInput(address) {
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    const currentValue = props.tabState.discreteInputsData[address] || false;
    const newValue = !currentValue;

    await invoke('modbus_slave_set_discrete_input', {
      connectionId,
      address,
      value: newValue,
    });

    // Update local state
    props.tabState.discreteInputsData[address] = newValue;
  } catch (error) {
    console.error('Set discrete input error:', error);
  }
}

// Auto toggle for coils/discrete inputs
function isAutoToggling(address, dataType) {
  const key = getToggleKey(address, dataType);
  return autoToggleAddresses.value.has(key);
}

function getToggleKey(address, dataType) {
  return `${dataType}-${address}`;
}

function toggleAutoToggle(address, dataType) {
  if (!props.tabState.isConnected) return;

  const key = getToggleKey(address, dataType);
  if (autoToggleAddresses.value.has(key)) {
    stopAutoToggle(address, dataType);
  } else {
    startAutoToggle(address, dataType);
  }
}

function startAutoToggle(address, dataType) {
  const key = getToggleKey(address, dataType);
  autoToggleAddresses.value.add(key);
  autoToggleAddresses.value = new Set(autoToggleAddresses.value);

  const timer = setInterval(async () => {
    if (!props.tabState.isConnected) {
      stopAutoToggle(address, dataType);
      return;
    }

    try {
      const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;

      if (dataType === 'coils') {
        const currentValue = props.tabState.coilsData[address] || false;
        const newValue = !currentValue;
        await invoke('modbus_slave_set_coil', {
          connectionId,
          address,
          value: newValue,
        });
        props.tabState.coilsData[address] = newValue;
      } else if (dataType === 'discrete_inputs') {
        const currentValue = props.tabState.discreteInputsData[address] || false;
        const newValue = !currentValue;
        await invoke('modbus_slave_set_discrete_input', {
          connectionId,
          address,
          value: newValue,
        });
        props.tabState.discreteInputsData[address] = newValue;
      }
    } catch (error) {
      console.error('Auto toggle error:', error);
      stopAutoToggle(address, dataType);
    }
  }, autoToggleInterval);

  autoToggleTimers.value.set(key, timer);
}

function stopAutoToggle(address, dataType) {
  const key = getToggleKey(address, dataType);
  const timer = autoToggleTimers.value.get(key);
  if (timer) {
    clearInterval(timer);
    autoToggleTimers.value.delete(key);
  }
  autoToggleAddresses.value.delete(key);
  autoToggleAddresses.value = new Set(autoToggleAddresses.value);
}

function stopAllAutoToggles() {
  for (const [, timer] of autoToggleTimers.value) {
    clearInterval(timer);
  }
  autoToggleTimers.value.clear();
  autoToggleAddresses.value.clear();
}

function startEdit(address, currentValue) {
  if (!props.tabState.isConnected) return;
  // Stop auto-increment if running for this address
  if (isAutoIncrementing(address, currentDataTab.value)) {
    stopAutoIncrement(address, currentDataTab.value);
  }
  editingCell.value = address;
  editValue.value = currentValue.toString();
}

async function saveEdit(address) {
  // Prevent double-save (Enter + blur)
  if (editingCell.value === null) return;
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    const value = parseInt(editValue.value);
    // If invalid input, cancel edit instead of saving 0
    if (isNaN(value)) {
      cancelEdit();
      return;
    }
    const clampedValue = Math.max(0, Math.min(65535, value));

    if (currentDataTab.value === 'holding_registers') {
      await invoke('modbus_slave_set_register', {
        connectionId,
        address,
        value: clampedValue,
      });
      props.tabState.holdingRegistersData[address] = clampedValue;
    } else if (currentDataTab.value === 'input_registers') {
      await invoke('modbus_slave_set_input_register', {
        connectionId,
        address,
        value: clampedValue,
      });
      props.tabState.inputRegistersData[address] = clampedValue;
    }
  } catch (error) {
    console.error('Set register error:', error);
  }

  editingCell.value = null;
  editValue.value = '';
}

function cancelEdit() {
  editingCell.value = null;
  editValue.value = '';
}

// Auto increment register value (single increment)
async function incrementRegister(address) {
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    let currentValue, newValue;

    if (currentDataTab.value === 'holding_registers') {
      currentValue = props.tabState.holdingRegistersData[address] || 0;
      newValue = (currentValue + 1) % 65536; // Wrap around at 65535
      await invoke('modbus_slave_set_register', {
        connectionId,
        address,
        value: newValue,
      });
      props.tabState.holdingRegistersData[address] = newValue;
    } else if (currentDataTab.value === 'input_registers') {
      currentValue = props.tabState.inputRegistersData[address] || 0;
      newValue = (currentValue + 1) % 65536;
      await invoke('modbus_slave_set_input_register', {
        connectionId,
        address,
        value: newValue,
      });
      props.tabState.inputRegistersData[address] = newValue;
    }
  } catch (error) {
    console.error('Increment register error:', error);
  }
}

// Get unique key for auto-increment
function getIncrementKey(address, dataType) {
  return `${dataType}-${address}`;
}

// Check if address is auto-incrementing
function isAutoIncrementing(address, dataType) {
  const key = getIncrementKey(address, dataType);
  return autoIncrementAddresses.value.has(key);
}

// Toggle auto-increment for an address
function toggleAutoIncrement(address, dataType) {
  if (!props.tabState.isConnected) return;

  const key = getIncrementKey(address, dataType);
  if (autoIncrementAddresses.value.has(key)) {
    // Stop auto-increment
    stopAutoIncrement(address, dataType);
  } else {
    // Start auto-increment
    startAutoIncrement(address, dataType);
  }
}

// Start auto-increment for an address
function startAutoIncrement(address, dataType) {
  const key = getIncrementKey(address, dataType);
  autoIncrementAddresses.value.add(key);
  // Force reactivity update
  autoIncrementAddresses.value = new Set(autoIncrementAddresses.value);

  const timer = setInterval(async () => {
    if (!props.tabState.isConnected) {
      stopAutoIncrement(address, dataType);
      return;
    }

    try {
      const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
      let currentValue, newValue;

      if (dataType === 'holding_registers') {
        currentValue = props.tabState.holdingRegistersData[address] || 0;
        newValue = (currentValue + 1) % 65536;
        await invoke('modbus_slave_set_register', {
          connectionId,
          address,
          value: newValue,
        });
        props.tabState.holdingRegistersData[address] = newValue;
      } else if (dataType === 'input_registers') {
        currentValue = props.tabState.inputRegistersData[address] || 0;
        newValue = (currentValue + 1) % 65536;
        await invoke('modbus_slave_set_input_register', {
          connectionId,
          address,
          value: newValue,
        });
        props.tabState.inputRegistersData[address] = newValue;
      }
    } catch (error) {
      console.error('Auto increment error:', error);
      stopAutoIncrement(address, dataType);
    }
  }, autoIncrementInterval);

  autoIncrementTimers.value.set(key, timer);
}

// Stop auto-increment for an address
function stopAutoIncrement(address, dataType) {
  const key = getIncrementKey(address, dataType);
  const timer = autoIncrementTimers.value.get(key);
  if (timer) {
    clearInterval(timer);
    autoIncrementTimers.value.delete(key);
  }
  autoIncrementAddresses.value.delete(key);
  // Force reactivity update
  autoIncrementAddresses.value = new Set(autoIncrementAddresses.value);
}

// Stop all auto-increments
function stopAllAutoIncrements() {
  for (const [address, timer] of autoIncrementTimers.value) {
    clearInterval(timer);
  }
  autoIncrementTimers.value.clear();
  autoIncrementAddresses.value.clear();
}

// Response delay
async function updateResponseDelay() {
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    await invoke('modbus_slave_set_delay', {
      connectionId,
      delayConfig: {
        global_delay_ms: props.tabState.responseDelay,
      },
    });
  } catch (error) {
    console.error('Set delay error:', error);
  }
}

// Statistics
async function refreshStats() {
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    const stats = await invoke('modbus_slave_get_stats', { connectionId });
    props.tabState.statistics = stats;
  } catch (error) {
    console.error('Get stats error:', error);
  }
}

async function resetStats() {
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    await invoke('modbus_slave_reset_stats', { connectionId });
    props.tabState.statistics = null;
  } catch (error) {
    console.error('Reset stats error:', error);
  }
}

// Log management
function clearLog() {
  props.tabState.requestLog = [];
}

// Data management
async function loadData() {
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    await invoke('modbus_slave_load_data', { connectionId });
    await refreshData();
  } catch (error) {
    console.error('Load data error:', error);
  }
}

async function saveData() {
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;
    await invoke('modbus_slave_save_data', { connectionId });
  } catch (error) {
    console.error('Save data error:', error);
  }
}

async function refreshData() {
  if (!props.tabState.isConnected) return;

  try {
    const connectionId = props.tabState.slaveConnectionId || props.tabState.connectionId;

    // Get data for current tab
    const result = await invoke('modbus_slave_get_data', {
      connectionId,
      dataType: currentDataTab.value,
      startAddress: 0,
      quantity: 10000,
    });

    switch (currentDataTab.value) {
      case 'coils':
        props.tabState.coilsData = result;
        break;
      case 'discrete_inputs':
        props.tabState.discreteInputsData = result;
        break;
      case 'holding_registers':
        props.tabState.holdingRegistersData = result;
        break;
      case 'input_registers':
        props.tabState.inputRegistersData = result;
        break;
    }
  } catch (error) {
    console.error('Refresh data error:', error);
  }
}

// Watch for data tab changes
watch(() => props.tabState.activeDataTab, () => {
  if (props.tabState.isConnected) {
    refreshData();
  }
});

// Auto scroll log when new entries are added
watch(
  () => props.tabState.requestLog?.length,
  () => {
    nextTick(() => {
      if (requestLogRef.value) {
        requestLogRef.value.scrollTop = requestLogRef.value.scrollHeight;
      }
    });
  },
  { flush: 'post' }
);

// Initialize data arrays
onMounted(() => {
  if (!props.tabState.coilsData.length) {
    props.tabState.coilsData = new Array(10000).fill(false);
  }
  if (!props.tabState.discreteInputsData.length) {
    props.tabState.discreteInputsData = new Array(10000).fill(false);
  }
  if (!props.tabState.holdingRegistersData.length) {
    props.tabState.holdingRegistersData = new Array(10000).fill(0);
  }
  if (!props.tabState.inputRegistersData.length) {
    props.tabState.inputRegistersData = new Array(10000).fill(0);
  }
});

// Cleanup on unmount
onUnmounted(() => {
  stopAllAutoIncrements();
});
</script>

<template>
  <div class="modbus-slave-tab" @click="closeDropdowns">
    <!-- Sidebar -->
    <aside class="sidebar">
      <!-- Mode Selector -->
      <div class="mode-selector">
        <button
          :class="{ active: tabState.mode === 'rtu' }"
          @click.stop="tabState.mode = 'rtu'"
          :disabled="tabState.isConnected"
        >
          RTU
        </button>
        <button
          :class="{ active: tabState.mode === 'tcp' }"
          @click.stop="tabState.mode = 'tcp'"
          :disabled="tabState.isConnected"
        >
          TCP/IP
        </button>
      </div>

      <!-- RTU Config -->
      <div v-if="tabState.mode === 'rtu'" class="config-card">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/>
          </svg>
          <span>RTU Configuration</span>
        </div>

        <!-- Port Selection -->
        <div class="dropdown-item" :class="{ open: openDropdown === 'port', disabled: tabState.isConnected }" @click.stop="toggleDropdown('port')">
          <div class="dropdown-trigger">
            <div class="dropdown-label">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="2" y="4" width="20" height="16" rx="2"/>
                <path d="M6 8h.01M10 8h.01M14 8h.01"/>
              </svg>
              <span>{{ tabState.selectedPort || 'Select Port' }}</span>
            </div>
            <svg class="chevron" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 9l6 6 6-6"/>
            </svg>
          </div>
          <div v-if="openDropdown === 'port'" class="dropdown-menu" @click.stop>
            <div v-if="ports.length === 0" class="dropdown-empty">No ports available</div>
            <div
              v-for="port in ports"
              :key="port.name"
              class="dropdown-option"
              :class="{ selected: tabState.selectedPort === port.name }"
              @click="selectOption('port', port.name)"
            >
              <span>{{ port.port_type }}</span>
              <span v-if="port.product" class="port-product">{{ port.product }}</span>
            </div>
          </div>
        </div>

        <!-- Baud Rate -->
        <div class="dropdown-item" :class="{ open: openDropdown === 'baudRate', disabled: tabState.isConnected }" @click.stop="toggleDropdown('baudRate')">
          <div class="dropdown-trigger">
            <div class="dropdown-label">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
              </svg>
              <span>{{ formatBaudRate(tabState.baudRate) }} baud</span>
            </div>
            <svg class="chevron" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 9l6 6 6-6"/>
            </svg>
          </div>
          <div v-if="openDropdown === 'baudRate'" class="dropdown-menu" @click.stop>
            <div
              v-for="rate in baudRateOptions"
              :key="rate"
              class="dropdown-option"
              :class="{ selected: tabState.baudRate === rate }"
              @click="selectOption('baudRate', rate)"
            >
              {{ formatBaudRate(rate) }}
            </div>
          </div>
        </div>

        <!-- Data Bits, Parity, Stop Bits -->
        <div class="config-row">
          <div class="dropdown-item mini" :class="{ open: openDropdown === 'dataBits', disabled: tabState.isConnected }" @click.stop="toggleDropdown('dataBits')">
            <div class="dropdown-trigger">
              <span>{{ tabState.dataBits }} bit</span>
              <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M6 9l6 6 6-6"/>
              </svg>
            </div>
            <div v-if="openDropdown === 'dataBits'" class="dropdown-menu" @click.stop>
              <div
                v-for="bits in dataBitsOptions"
                :key="bits"
                class="dropdown-option"
                :class="{ selected: tabState.dataBits === bits }"
                @click="selectOption('dataBits', bits)"
              >
                {{ bits }}
              </div>
            </div>
          </div>

          <div class="dropdown-item mini" :class="{ open: openDropdown === 'parity', disabled: tabState.isConnected }" @click.stop="toggleDropdown('parity')">
            <div class="dropdown-trigger">
              <span>{{ tabState.parity }}</span>
              <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M6 9l6 6 6-6"/>
              </svg>
            </div>
            <div v-if="openDropdown === 'parity'" class="dropdown-menu" @click.stop>
              <div
                v-for="p in parityOptions"
                :key="p"
                class="dropdown-option"
                :class="{ selected: tabState.parity === p }"
                @click="selectOption('parity', p)"
              >
                {{ p }}
              </div>
            </div>
          </div>

          <div class="dropdown-item mini" :class="{ open: openDropdown === 'stopBits', disabled: tabState.isConnected }" @click.stop="toggleDropdown('stopBits')">
            <div class="dropdown-trigger">
              <span>{{ tabState.stopBits }} stop</span>
              <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M6 9l6 6 6-6"/>
              </svg>
            </div>
            <div v-if="openDropdown === 'stopBits'" class="dropdown-menu" @click.stop>
              <div
                v-for="sb in stopBitsOptions"
                :key="sb"
                class="dropdown-option"
                :class="{ selected: tabState.stopBits === sb }"
                @click="selectOption('stopBits', sb)"
              >
                {{ sb }}
              </div>
            </div>
          </div>
        </div>

        <!-- Slave ID -->
        <div class="field-row" style="margin-top: 8px;">
          <label>Slave ID</label>
          <input
            type="number"
            v-model.number="tabState.slaveId"
            :disabled="tabState.isConnected"
            min="1"
            max="247"
          />
        </div>
      </div>

      <!-- TCP Config -->
      <div v-else class="config-card">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 0 1-9 9m9-9a9 9 0 0 0-9-9m9 9H3m9 9a9 9 0 0 1-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 0 1 9-9"/>
          </svg>
          <span>TCP Configuration</span>
        </div>

        <div class="field-row">
          <label>Bind Address</label>
          <input
            type="text"
            v-model="tabState.bindAddress"
            :disabled="tabState.isConnected"
            placeholder="0.0.0.0"
            autocomplete="off"
            data-form-type="other"
            spellcheck="false"
          />
        </div>

        <div class="field-row">
          <label>Port</label>
          <input
            type="number"
            v-model.number="tabState.listenPort"
            :disabled="tabState.isConnected"
            min="1"
            max="65535"
            placeholder="502"
          />
        </div>

        <div class="field-row">
          <label>Unit ID</label>
          <input
            type="number"
            v-model.number="tabState.unitId"
            :disabled="tabState.isConnected"
            min="1"
            max="247"
          />
        </div>
      </div>

      <!-- Connect Button -->
      <button
        class="btn-connect"
        :class="{ connected: tabState.isConnected }"
        @click="toggleConnection"
        :disabled="tabState.mode === 'rtu' && !tabState.selectedPort"
      >
        <svg v-if="!tabState.isConnected" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polygon points="5 3 19 12 5 21 5 3"/>
        </svg>
        <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="6" y="4" width="4" height="16"/>
          <rect x="14" y="4" width="4" height="16"/>
        </svg>
        <span>{{ tabState.isConnected ? 'Stop Slave' : 'Start Slave' }}</span>
      </button>

      <!-- Status Message -->
      <div v-if="tabState.statusMessage" class="status-message" :class="tabState.connectionStatus">
        {{ tabState.statusMessage }}
      </div>

      <!-- Connected Clients (TCP mode) -->
      <div v-if="tabState.mode === 'tcp' && tabState.isConnected" class="config-card">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
            <circle cx="9" cy="7" r="4"/>
            <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
            <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
          </svg>
          <span>Connected Clients</span>
          <span class="badge">{{ tabState.connectedClients?.length || 0 }}</span>
        </div>

        <div v-if="tabState.connectedClients?.length > 0" class="clients-list">
          <div v-for="client in tabState.connectedClients" :key="client.id" class="client-item">
            <span class="client-addr">{{ client.address }}</span>
            <span class="client-time">{{ client.connected_at }}</span>
          </div>
        </div>
        <div v-else class="empty-clients">
          No clients connected
        </div>
      </div>

      <!-- Response Delay -->
      <div class="config-card" :class="{ disabled: !tabState.isConnected }">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <polyline points="12 6 12 12 16 14"/>
          </svg>
          <span>Response Delay</span>
        </div>

        <div class="field-row">
          <label>Delay (ms)</label>
          <input
            type="number"
            v-model.number="tabState.responseDelay"
            @change="updateResponseDelay"
            min="0"
            max="10000"
            step="10"
          />
        </div>
      </div>

      <!-- Statistics -->
      <div class="config-card" :class="{ disabled: !tabState.isConnected }">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="20" x2="18" y2="10"/>
            <line x1="12" y1="20" x2="12" y2="4"/>
            <line x1="6" y1="20" x2="6" y2="14"/>
          </svg>
          <span>Statistics</span>
        </div>

        <div class="stats-grid">
          <div class="stat-item">
            <span class="stat-label">Requests</span>
            <span class="stat-value">{{ tabState.requestCount || 0 }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Last Request</span>
            <span class="stat-value">{{ tabState.lastRequestTime || '-' }}</span>
          </div>
        </div>

        <div class="stats-actions">
          <button class="btn-stats" @click="refreshStats">Refresh</button>
          <button class="btn-stats danger" @click="resetStats">Reset</button>
        </div>
      </div>

      <!-- Data Persistence -->
      <div class="config-card" :class="{ disabled: !tabState.isConnected }">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
            <polyline points="17 21 17 13 7 13 7 21"/>
            <polyline points="7 3 7 8 15 8"/>
          </svg>
          <span>Data Persistence</span>
        </div>

        <div class="persistence-buttons">
          <button class="btn-persistence" @click="saveData">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
            </svg>
            Save
          </button>
          <button class="btn-persistence" @click="loadData">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 15v4c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2v-4"/>
              <polyline points="17 8 12 3 7 8"/>
              <line x1="12" y1="3" x2="12" y2="15"/>
            </svg>
            Load
          </button>
        </div>
      </div>
    </aside>

    <!-- Main Area -->
    <main class="main-area">
      <!-- Content wrapper: Data + Log side by side -->
      <div class="content-wrapper">
        <!-- Data Section -->
        <div class="data-section">
        <!-- Data Tabs -->
        <div class="data-tabs">
          <button
            v-for="tab in dataTabs"
            :key="tab.value"
            class="data-tab"
            :class="{ active: tabState.activeDataTab === tab.value }"
            @click="tabState.activeDataTab = tab.value"
          >
            <span class="tab-label">{{ tab.label }}</span>
            <span class="tab-fc">{{ tab.fc }}</span>
          </button>
        </div>

        <!-- Data Header -->
        <div class="section-header">
          <div class="view-controls">
            <label>Start:</label>
            <input
              type="number"
              v-model.number="tabState.viewStartAddress"
              min="0"
              max="9999"
              class="addr-input"
            />
            <label>Qty:</label>
            <input
              type="number"
              v-model.number="tabState.viewQuantity"
              min="1"
              max="1000"
              class="qty-input"
            />
          </div>

          <div class="format-selector" v-if="!isCoilData">
            <span>Format:</span>
            <div class="dropdown-item mini" :class="{ open: openDropdown === 'dataFormat' }" @click.stop="toggleDropdown('dataFormat')">
              <div class="dropdown-trigger">
                <span>{{ tabState.dataFormat }}</span>
                <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 9l6 6 6-6"/>
                </svg>
              </div>
              <div v-if="openDropdown === 'dataFormat'" class="dropdown-menu" @click.stop>
                <div
                  v-for="fmt in dataFormats"
                  :key="fmt.value"
                  class="dropdown-option"
                  :class="{ selected: tabState.dataFormat === fmt.value }"
                  @click="selectOption('dataFormat', fmt.value)"
                >
                  {{ fmt.label }}
                </div>
              </div>
            </div>
          </div>

          <button class="btn-refresh" @click="refreshData" title="Refresh data">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="23 4 23 10 17 10"/>
              <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
            </svg>
          </button>
        </div>

        <!-- Data Table -->
        <div class="data-table-container">
          <!-- Coil/Discrete Input Table -->
          <table v-if="isCoilData" class="data-table coil-table">
            <thead>
              <tr>
                <th>Address</th>
                <th>Value</th>
                <th v-if="isEditable || currentDataTab === 'discrete_inputs'">Action</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(value, index) in displayData" :key="tabState.viewStartAddress + index">
                <td class="addr">{{ formatAddress(tabState.viewStartAddress + index) }}</td>
                <td class="value" :class="{ 'coil-on': value, 'coil-off': !value }">
                  {{ value ? 'ON' : 'OFF' }}
                </td>
                <td v-if="isEditable || currentDataTab === 'discrete_inputs'" class="action">
                  <button
                    class="btn-auto"
                    :class="{ active: isAutoToggling(tabState.viewStartAddress + index, currentDataTab) }"
                    @click="toggleAutoToggle(tabState.viewStartAddress + index, currentDataTab)"
                    :disabled="!tabState.isConnected"
                    :title="isAutoToggling(tabState.viewStartAddress + index, currentDataTab) ? 'Stop' : 'Auto'"
                  >
                    <svg v-if="!isAutoToggling(tabState.viewStartAddress + index, currentDataTab)" width="10" height="10" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                      <polygon points="5 3 19 12 5 21 5 3"/>
                    </svg>
                    <svg v-else width="10" height="10" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                      <rect x="6" y="4" width="4" height="16"/>
                      <rect x="14" y="4" width="4" height="16"/>
                    </svg>
                  </button>
                  <button
                    class="btn-switch"
                    :class="{ on: value }"
                    @click="currentDataTab === 'coils' ? toggleCoil(tabState.viewStartAddress + index) : toggleDiscreteInput(tabState.viewStartAddress + index)"
                    :disabled="!tabState.isConnected"
                    title="Toggle"
                  >
                    <span class="switch-track">
                      <span class="switch-thumb"></span>
                    </span>
                  </button>
                </td>
              </tr>
            </tbody>
          </table>

          <!-- Register Table -->
          <table v-else class="data-table register-table">
            <thead>
              <tr>
                <th>Address</th>
                <th>Value</th>
                <th>Hex</th>
                <th v-if="isEditable || currentDataTab === 'input_registers'">Action</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(value, index) in displayData" :key="tabState.viewStartAddress + index">
                <td class="addr">{{ formatAddress(tabState.viewStartAddress + index) }}</td>
                <td class="value">
                  <template v-if="editingCell === tabState.viewStartAddress + index">
                    <input
                      type="number"
                      v-model="editValue"
                      class="edit-input"
                      @keyup.enter="saveEdit(tabState.viewStartAddress + index)"
                      @keyup.escape="cancelEdit"
                      @blur="saveEdit(tabState.viewStartAddress + index)"
                      autofocus
                    />
                  </template>
                  <template v-else>
                    <span class="value-display">
                      {{ formatValue(value, tabState.dataFormat) }}
                      <button
                        v-if="tabState.isConnected"
                        class="btn-edit-inline"
                        @click.stop="startEdit(tabState.viewStartAddress + index, value || 0)"
                        title="Edit value"
                      >
                        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                        </svg>
                      </button>
                    </span>
                  </template>
                </td>
                <td class="hex">0x{{ (value || 0).toString(16).toUpperCase().padStart(4, '0') }}</td>
                <td v-if="isEditable || currentDataTab === 'input_registers'" class="action">
                  <button
                    class="btn-auto"
                    :class="{ active: isAutoIncrementing(tabState.viewStartAddress + index, currentDataTab) }"
                    @click="toggleAutoIncrement(tabState.viewStartAddress + index, currentDataTab)"
                    :disabled="!tabState.isConnected"
                    :title="isAutoIncrementing(tabState.viewStartAddress + index, currentDataTab) ? 'Stop' : 'Auto'"
                  >
                    <svg v-if="!isAutoIncrementing(tabState.viewStartAddress + index, currentDataTab)" width="10" height="10" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                      <polygon points="5 3 19 12 5 21 5 3"/>
                    </svg>
                    <svg v-else width="10" height="10" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                      <rect x="6" y="4" width="4" height="16"/>
                      <rect x="14" y="4" width="4" height="16"/>
                    </svg>
                  </button>
                  <button
                    class="btn-increment"
                    @click="incrementRegister(tabState.viewStartAddress + index)"
                    :disabled="!tabState.isConnected"
                    title="Increment +1"
                  >
                    <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                      <rect x="10" y="4" width="4" height="16"/>
                      <rect x="4" y="10" width="16" height="4"/>
                    </svg>
                  </button>
                </td>
              </tr>
            </tbody>
          </table>

          <div v-if="displayData.length === 0" class="empty-data">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
            </svg>
            <p>No data to display</p>
          </div>
        </div>
        </div><!-- End data-section -->

        <!-- Request Log -->
        <div class="log-section">
        <div class="section-header">
          <h3>Request Log</h3>
          <span class="log-count">{{ tabState.requestCount || 0 }} entries</span>
          <button class="btn-clear-log" @click="clearLog" title="Clear log">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
          </button>
        </div>

        <div class="log-container" ref="requestLogRef">
          <div
            v-for="entry in displayedLogEntries"
            :key="entry.id"
            class="log-entry"
            :class="entry.success ? 'success' : 'error'"
          >
            <div class="log-header">
              <span class="log-time">{{ entry.timestamp }}</span>
              <span class="log-fc">FC{{ entry.function_code?.toString().padStart(2, '0') }}</span>
              <span class="log-addr">{{ entry.start_address }}-{{ entry.start_address + (entry.quantity || 1) - 1 }}</span>
              <span class="log-status" :class="entry.success ? 'success' : 'error'">
                {{ entry.success ? 'OK' : 'ERR' }}
              </span>
              <span v-if="entry.response_time_ms" class="log-duration">{{ entry.response_time_ms }}ms</span>
            </div>
            <div class="log-frames">
              <div class="log-frame rx">
                <span class="frame-label">RX:</span>
                <span class="frame-data">{{ formatHex(entry.request_frame) }}</span>
              </div>
              <div class="log-frame tx">
                <span class="frame-label">TX:</span>
                <span class="frame-data">{{ formatHex(entry.response_frame) }}</span>
              </div>
            </div>
            <div v-if="entry.error_message" class="log-error">
              {{ entry.error_message }}
            </div>
          </div>

          <div v-if="displayedLogEntries.length === 0" class="empty-log">
            No requests received yet. Start the slave to begin listening.
          </div>
        </div>
      </div>
      </div><!-- End content-wrapper -->
    </main>
  </div>
</template>

<style scoped>
.modbus-slave-tab {
  display: flex;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.sidebar {
  width: 280px;
  min-width: 280px;
  padding: 8px;
  overflow-y: auto;
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.main-area {
  flex: 1;
  height: 100%;
  min-width: 0;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 8px;
}

.content-wrapper {
  flex: 1;
  height: 100%;
  display: flex;
  gap: 8px;
  overflow: hidden;
  min-height: 0;
}

/* Mode Selector */
.mode-selector {
  display: flex;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  padding: 2px;
  gap: 2px;
}

.mode-selector button {
  flex: 1;
  padding: 5px 12px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-secondary);
  font-weight: 500;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.mode-selector button.active {
  background: var(--accent-primary);
  color: white;
}

.mode-selector button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Config Card */
.config-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  padding: 8px;
}

.config-card.disabled {
  opacity: 0.6;
  pointer-events: none;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 8px;
  font-weight: 600;
  font-size: 11px;
  color: var(--text-primary);
}

.card-header svg {
  color: var(--accent-primary);
  width: 14px;
  height: 14px;
}

.card-header .badge {
  margin-left: auto;
  background: var(--accent-primary);
  color: white;
  padding: 1px 6px;
  border-radius: 8px;
  font-size: 10px;
}

/* Field Row */
.field-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 6px;
}

.field-row label {
  min-width: 70px;
  font-size: 11px;
  color: var(--text-secondary);
}

.field-row input {
  flex: 1;
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  font-size: 12px;
  background: var(--bg-primary);
}

.field-row input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.field-row-inline {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 6px;
}

.field-row-3cols {
  display: flex;
  gap: 8px;
  margin-bottom: 6px;
}

.field-col {
  display: flex;
  flex-direction: column;
  gap: 3px;
  margin-bottom: 10px;
}

.field-col .field-input {
  width: 100%;
  text-align: right;
}

.field-label-float {
  font-size: 10px;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.field-input {
  padding: 4px 8px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 12px;
  font-family: var(--font-mono);
}

.field-input.host-input {
  flex: 2;
  min-width: 0;
}

.field-input.port-input {
  flex: 1;
  min-width: 0;
  text-align: center;
}

.field-label-inline {
  flex: 2;
  font-size: 11px;
  color: var(--text-secondary);
  text-align: right;
  padding-right: 8px;
}

.field-input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.field-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.port-separator {
  font-size: 12px;
  font-weight: 700;
  color: var(--text-secondary);
  flex-shrink: 0;
}

/* Config Row */
.config-row {
  display: flex;
  gap: 4px;
  margin-top: 6px;
}

/* Dropdown */
.dropdown-item {
  position: relative;
  margin-bottom: 6px;
}

.dropdown-item.mini {
  margin-bottom: 0;
  flex: 1;
}

.dropdown-item.disabled {
  opacity: 0.6;
  pointer-events: none;
}

.dropdown-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 5px 8px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s;
}

.dropdown-item.mini .dropdown-trigger {
  padding: 4px 6px;
  font-size: 11px;
}

.dropdown-trigger:hover {
  border-color: var(--accent-primary);
}

.dropdown-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
}

.dropdown-label svg {
  width: 12px;
  height: 12px;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  box-shadow: var(--shadow-lg);
  z-index: 100;
  max-height: 180px;
  overflow-y: auto;
  margin-top: 2px;
}

.dropdown-option {
  padding: 5px 8px;
  cursor: pointer;
  font-size: 12px;
  transition: background 0.1s;
}

.dropdown-option:hover {
  background: var(--bg-tertiary);
}

.dropdown-option.selected {
  background: var(--accent-primary);
  color: white;
}

.dropdown-empty {
  padding: 8px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 11px;
}

.port-product {
  font-size: 10px;
  color: var(--text-tertiary);
  margin-left: 6px;
}

.chevron {
  transition: transform 0.2s;
  width: 10px;
  height: 10px;
}

.dropdown-item.open .chevron {
  transform: rotate(180deg);
}

/* Connect Button */
.btn-connect {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  width: 100%;
  padding: 8px;
  border: none;
  border-radius: var(--radius-sm);
  background: linear-gradient(135deg, var(--accent-primary), #0284c7);
  color: white;
  font-weight: 600;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-connect svg {
  width: 14px;
  height: 14px;
}

.btn-connect:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 3px 10px rgba(14, 165, 233, 0.3);
}

.btn-connect.connected {
  background: linear-gradient(135deg, var(--danger), #dc2626);
}

.btn-connect:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Status Message */
.status-message {
  padding: 5px 8px;
  border-radius: var(--radius-sm);
  font-size: 11px;
  text-align: center;
  font-family: var(--font-mono);
}

.status-message.error {
  background: rgba(239, 68, 68, 0.1);
  color: var(--danger);
}

.status-message.connected {
  background: rgba(16, 185, 129, 0.08);
  color: #059669;
  font-weight: 500;
}

/* Clients List */
.clients-list {
  max-height: 100px;
  overflow-y: auto;
}

.client-item {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
  font-size: 11px;
  border-bottom: 1px solid var(--border-color);
}

.client-item:last-child {
  border-bottom: none;
}

.client-addr {
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.client-time {
  color: var(--text-tertiary);
}

.empty-clients {
  padding: 8px;
  text-align: center;
  color: var(--text-tertiary);
  font-size: 11px;
}

/* Stats */
.stats-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 6px;
  margin-bottom: 6px;
}

.stat-item {
  background: var(--bg-primary);
  padding: 6px 8px;
  border-radius: var(--radius-sm);
  text-align: center;
}

.stat-label {
  display: block;
  font-size: 10px;
  color: var(--text-tertiary);
  margin-bottom: 2px;
}

.stat-value {
  display: block;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.stats-actions {
  display: flex;
  gap: 6px;
}

.btn-stats {
  flex: 1;
  padding: 5px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  background: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 11px;
  cursor: pointer;
}

.btn-stats:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.btn-stats.danger:hover {
  border-color: var(--danger);
  color: var(--danger);
}

/* Persistence */
.persistence-buttons {
  display: flex;
  gap: 6px;
}

.btn-persistence {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 5px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  background: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 11px;
  cursor: pointer;
}

.btn-persistence:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

/* Data Section */
.data-section {
  flex: 0 0 auto;
  width: 42%;
  height: 100%;
  min-width: 340px;
  max-width: 450px;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

/* Data Tabs */
.data-tabs {
  display: flex;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-color);
  padding: 5px 5px 0;
  gap: 3px;
}

.data-tab {
  flex: 1;
  padding: 6px 8px;
  border: none;
  border-radius: var(--radius-sm) var(--radius-sm) 0 0;
  background: transparent;
  color: var(--text-primary);
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.data-tab:hover {
  background: var(--bg-hover);
}

.data-tab.active {
  background: var(--bg-secondary);
  color: var(--accent-primary);
  font-weight: 600;
}

.tab-label {
  font-weight: 600;
  font-size: 10px;
}

.tab-fc {
  font-size: 9px;
  font-weight: 500;
  color: var(--text-secondary);
}

.data-tab.active .tab-fc {
  color: var(--accent-primary);
  opacity: 0.8;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-tertiary);
}

.section-header h3 {
  font-size: 12px;
  font-weight: 600;
  margin: 0;
}

.log-section .section-header {
  padding: 4px 8px;
}

.log-section .section-header h3 {
  font-size: 10px;
}

.view-controls {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--text-secondary);
}

.addr-input, .qty-input {
  width: 60px;
  padding: 3px 6px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  font-size: 11px;
  background: var(--bg-primary);
}

.format-selector {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-left: auto;
  font-size: 11px;
  color: var(--text-secondary);
}

.format-selector .dropdown-item.mini {
  min-width: 80px;
}

.format-selector .dropdown-item.mini .dropdown-trigger {
  justify-content: space-between;
}

.btn-refresh {
  padding: 3px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-refresh:hover {
  background: var(--bg-tertiary);
  color: var(--accent-primary);
}

.data-table-container {
  flex: 1;
  overflow: auto;
  padding: 4px;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 11px;
  font-family: var(--font-mono);
}

.data-table th, .data-table td {
  padding: 4px 8px;
  text-align: left;
  border-bottom: 1px solid var(--border-color);
}

.data-table th {
  background: var(--bg-tertiary);
  font-weight: 600;
  color: var(--text-secondary);
  position: sticky;
  top: 0;
  font-size: 10px;
}

.data-table .addr {
  color: var(--text-tertiary);
  width: 25%;
}

.data-table .value {
  font-weight: 500;
  width: 25%;
}

.value-display {
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-edit-inline {
  padding: 1px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s;
  display: inline-flex;
  align-items: center;
}

tr:hover .btn-edit-inline {
  opacity: 1;
}

.btn-edit-inline:hover {
  color: var(--accent-primary);
}

.data-table .hex {
  color: var(--accent-primary);
  width: 25%;
}

.data-table .action {
  width: 25%;
  vertical-align: middle;
}

.data-table .action button {
  vertical-align: middle;
}

.coil-on {
  color: var(--success) !important;
}

.coil-off {
  color: var(--text-tertiary) !important;
}

.btn-switch {
  width: 32px;
  height: 18px;
  padding: 0;
  border: none;
  background: transparent;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin-left: 6px;
  vertical-align: middle;
}

.btn-switch:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.switch-track {
  width: 32px;
  height: 18px;
  background: var(--border-color);
  border-radius: 9px;
  position: relative;
  transition: background 0.2s;
}

.btn-switch.on .switch-track {
  background: var(--success);
}

.switch-thumb {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 12px;
  height: 12px;
  background: white;
  border-radius: 50%;
  transition: left 0.2s;
  box-shadow: 0 1px 2px rgba(0,0,0,0.2);
}

.btn-switch.on .switch-thumb {
  left: 17px;
}

.btn-switch:hover:not(:disabled) .switch-track {
  background: var(--text-tertiary);
}

.btn-switch.on:hover:not(:disabled) .switch-track {
  background: #059669;
}

.btn-auto {
  width: 22px;
  height: 18px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  background: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
  margin-right: 6px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.btn-auto:hover:not(:disabled) {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.btn-auto.active {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: white;
  animation: pulse 1s infinite;
}

.btn-auto:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.7; }
}

.btn-edit, .btn-save, .btn-cancel {
  padding: 2px 6px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  background: var(--bg-primary);
  font-size: 10px;
  cursor: pointer;
  margin-right: 2px;
}

.btn-edit {
  color: var(--text-secondary);
}

.btn-edit:hover:not(:disabled) {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.btn-edit:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-increment {
  width: 22px;
  height: 18px;
  border: 1px solid var(--success);
  border-radius: var(--radius-sm);
  background: var(--success);
  color: white;
  cursor: pointer;
  margin-right: 2px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.btn-increment:hover:not(:disabled) {
  background: #059669;
  border-color: #059669;
}

.btn-increment:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-save {
  background: var(--success);
  border-color: var(--success);
  color: white;
}

.btn-cancel {
  color: var(--text-secondary);
}

.btn-cancel:hover {
  border-color: var(--danger);
  color: var(--danger);
}

.edit-input {
  width: 80px;
  padding: 2px 4px;
  border: 1px solid var(--accent-primary);
  border-radius: var(--radius-sm);
  font-size: 11px;
  font-family: var(--font-mono);
}

.empty-data {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-tertiary);
  gap: 8px;
}

.empty-data svg {
  width: 36px;
  height: 36px;
}

.empty-data p {
  font-size: 11px;
}

/* Log Section */
.log-section {
  flex: 1;
  height: 100%;
  display: flex;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.log-count {
  font-size: 9px;
  color: var(--text-tertiary);
  margin-left: auto;
}

.btn-clear-log {
  padding: 2px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-clear-log svg {
  width: 10px;
  height: 10px;
}

.btn-clear-log:hover {
  background: var(--bg-tertiary);
  color: var(--danger);
}

.log-container {
  flex: 1;
  overflow-y: auto;
  padding: 3px;
}

.log-entry {
  padding: 3px 5px;
  margin-bottom: 3px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
  border-left: 2px solid var(--success);
}

.log-entry.error {
  border-left-color: var(--danger);
}

.log-header {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-bottom: 2px;
}

.log-time {
  font-size: 9px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

.log-fc {
  font-size: 9px;
  font-weight: 600;
  padding: 0px 3px;
  background: var(--accent-primary);
  color: white;
  border-radius: 2px;
}

.log-addr {
  font-size: 9px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.log-status {
  font-size: 8px;
  font-weight: 600;
  padding: 0px 3px;
  border-radius: 2px;
}

.log-status.success {
  background: rgba(16, 185, 129, 0.2);
  color: var(--success);
}

.log-status.error {
  background: rgba(239, 68, 68, 0.2);
  color: var(--danger);
}

.log-duration {
  font-size: 8px;
  color: var(--text-tertiary);
  margin-left: auto;
}

.log-frames {
  font-family: var(--font-mono);
  font-size: 9px;
}

.log-frame {
  display: flex;
  gap: 4px;
  margin-bottom: 0px;
}

.frame-label {
  min-width: 18px;
  font-weight: 600;
}

.log-frame.rx .frame-label {
  color: var(--success);
}

.log-frame.tx .frame-label {
  color: var(--warning);
}

.frame-data {
  color: var(--text-secondary);
  word-break: break-all;
  word-spacing: -2px;
  letter-spacing: -0.3px;
}

.log-error {
  margin-top: 2px;
  padding: 2px 4px;
  background: rgba(239, 68, 68, 0.1);
  border-radius: var(--radius-sm);
  color: var(--danger);
  font-size: 9px;
}

.empty-log {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-tertiary);
  font-size: 10px;
}
</style>
