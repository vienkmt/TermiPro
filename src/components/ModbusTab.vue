<script setup>
import { ref, computed, inject, watch, nextTick } from 'vue';
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
const transactionLogRef = ref(null);
const isLoading = ref(false);

// Constants
const MAX_LOG_ENTRIES = 100;
const MAX_VISIBLE_ENTRIES = 50; // Limit visible entries for performance

// Config options
const baudRateOptions = [9600, 19200, 38400, 57600, 115200];
const dataBitsOptions = [7, 8];
const stopBitsOptions = ['1', '2'];
const parityOptions = ['none', 'odd', 'even'];

const functionCodes = [
  { value: 1, label: 'FC01 - Read Coils', type: 'read', isCoil: true },
  { value: 2, label: 'FC02 - Read Discrete Inputs', type: 'read', isCoil: true },
  { value: 3, label: 'FC03 - Read Holding Registers', type: 'read', isCoil: false },
  { value: 4, label: 'FC04 - Read Input Registers', type: 'read', isCoil: false },
  { value: 5, label: 'FC05 - Write Single Coil', type: 'write-single', isCoil: true },
  { value: 6, label: 'FC06 - Write Single Register', type: 'write-single', isCoil: false },
  { value: 15, label: 'FC15 - Write Multiple Coils', type: 'write-multiple', isCoil: true },
  { value: 16, label: 'FC16 - Write Multiple Registers', type: 'write-multiple', isCoil: false },
];

const dataFormats = [
  { value: 'unsigned', label: 'Unsigned' },
  { value: 'signed', label: 'Signed' },
  { value: 'hex', label: 'Hex' },
  { value: 'float32', label: 'Float32' },
  { value: 'binary', label: 'Binary' },
];

// Computed
const currentFunctionCode = computed(() => {
  return functionCodes.find(fc => fc.value === props.tabState.functionCode) || functionCodes[2];
});

const isReadOperation = computed(() => currentFunctionCode.value?.type === 'read');
const isWriteOperation = computed(() => currentFunctionCode.value?.type?.startsWith('write'));
const isSingleWrite = computed(() => currentFunctionCode.value?.type === 'write-single');
const isCoilOperation = computed(() => currentFunctionCode.value?.isCoil);

// Displayed log entries - limit and reverse for performance (newest at bottom)
const displayedLogEntries = computed(() => {
  const log = props.tabState.transactionLog || [];
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
    case 'float32':
      return value.toString(); // Will be handled specially for 2-register floats
    default:
      return value.toString();
  }
}

function formatAddress(addr) {
  return addr.toString().padStart(5, '0');
}

// Dropdown handlers
function toggleDropdown(name) {
  if (props.tabState.isConnected && !['functionCode', 'dataFormat'].includes(name)) return;
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
    case 'functionCode': props.tabState.functionCode = value; break;
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
    emit('disconnect', props.tabId);
  } else {
    emit('connect', props.tabId);
  }
}

// Send Modbus request
async function sendRequest() {
  if (!props.tabState.isConnected || isLoading.value) return;

  isLoading.value = true;

  try {
    // Build request
    const request = {
      connection_id: props.tabState.modbusConnectionId || props.tabState.connectionId,
      function_code: props.tabState.functionCode,
      start_address: props.tabState.startAddress,
      quantity: isReadOperation.value ? props.tabState.quantity : (isSingleWrite.value ? 1 : props.tabState.writeValues.length),
      values: isCoilOperation.value ? null : (isWriteOperation.value ? props.tabState.writeValues : null),
      coil_values: isCoilOperation.value && isWriteOperation.value ? props.tabState.coilValues : null,
    };

    const response = await invoke('modbus_request', { request });

    // Add to transaction log
    addToLog({
      type: response.success ? 'success' : 'error',
      timestamp: new Date().toLocaleTimeString(),
      functionCode: response.function_code,
      requestFrame: response.request_frame,
      responseFrame: response.response_frame,
      success: response.success,
      responseTime: response.response_time_ms,
      data: response.data,
      coils: response.coils,
      errorMessage: response.error_message,
    });

    // Update data display
    if (response.success) {
      if (response.data) {
        props.tabState.registerData = response.data.map((value, index) => ({
          address: props.tabState.startAddress + index,
          value: value,
          rawHex: value.toString(16).toUpperCase().padStart(4, '0'),
        }));
      }
      if (response.coils) {
        props.tabState.coilData = response.coils.slice(0, props.tabState.quantity).map((value, index) => ({
          address: props.tabState.startAddress + index,
          value: value,
        }));
      }
      props.tabState.lastResponseTime = response.response_time_ms;
    }

  } catch (error) {
    console.error('Modbus request error:', error);
    addToLog({
      type: 'error',
      timestamp: new Date().toLocaleTimeString(),
      functionCode: props.tabState.functionCode,
      requestFrame: [],
      responseFrame: [],
      success: false,
      responseTime: 0,
      errorMessage: error.toString(),
    });
  } finally {
    isLoading.value = false;
  }
}

function addToLog(entry) {
  const logEntry = {
    id: Date.now(),
    ...entry,
  };

  props.tabState.transactionLog.unshift(logEntry);

  // Trim old entries
  if (props.tabState.transactionLog.length > MAX_LOG_ENTRIES) {
    props.tabState.transactionLog.pop();
  }
}

function clearLog() {
  props.tabState.transactionLog = [];
}

function clearData() {
  props.tabState.registerData = [];
  props.tabState.coilData = [];
}

// Polling
async function startPolling() {
  if (!props.tabState.isConnected || props.tabState.pollingEnabled) return;

  try {
    const config = {
      connection_id: props.tabState.modbusConnectionId || props.tabState.connectionId,
      requests: [{
        function_code: props.tabState.functionCode,
        start_address: props.tabState.startAddress,
        quantity: props.tabState.quantity,
      }],
      interval_ms: props.tabState.pollingInterval,
    };

    await invoke('modbus_start_polling', { config });
    props.tabState.pollingEnabled = true;
  } catch (error) {
    console.error('Start polling error:', error);
    alert('Error: ' + error);
  }
}

async function stopPolling() {
  if (!props.tabState.pollingEnabled) return;

  try {
    await invoke('modbus_stop_polling', {
      connectionId: props.tabState.modbusConnectionId || props.tabState.connectionId,
    });
    props.tabState.pollingEnabled = false;
  } catch (error) {
    console.error('Stop polling error:', error);
  }
}

// Initialize write values when FC changes
watch(() => props.tabState.functionCode, (newFc) => {
  const fc = functionCodes.find(f => f.value === newFc);
  if (fc?.type === 'write-single') {
    props.tabState.writeValues = [0];
    props.tabState.coilValues = [false];
  } else if (fc?.type === 'write-multiple') {
    props.tabState.writeValues = props.tabState.writeValues.length > 0 ? props.tabState.writeValues : [0];
    props.tabState.coilValues = props.tabState.coilValues.length > 0 ? props.tabState.coilValues : [false];
  }
});

// Auto scroll log when new entries are added
watch(() => props.tabState.transactionLog?.length, () => {
  nextTick(() => {
    if (transactionLogRef.value) {
      transactionLogRef.value.scrollTop = transactionLogRef.value.scrollHeight;
    }
  });
});

// Add/remove write values
function addWriteValue() {
  if (isCoilOperation.value) {
    props.tabState.coilValues.push(false);
  } else {
    props.tabState.writeValues.push(0);
  }
}

function removeWriteValue(index) {
  if (isCoilOperation.value) {
    props.tabState.coilValues.splice(index, 1);
  } else {
    props.tabState.writeValues.splice(index, 1);
  }
}
</script>

<template>
  <div class="modbus-tab" @click="closeDropdowns">
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

        <!-- Data Bits, Parity in one row -->
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
          <label>Host</label>
          <input
            type="text"
            v-model="tabState.host"
            :disabled="tabState.isConnected"
            placeholder="localhost"
          />
        </div>

        <div class="field-row">
          <label>Port</label>
          <input
            type="number"
            v-model.number="tabState.port"
            :disabled="tabState.isConnected"
            min="1"
            max="65535"
          />
        </div>
      </div>

      <!-- Modbus Settings -->
      <div class="config-card">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
          <span>Modbus Settings</span>
        </div>

        <div class="field-row" v-if="tabState.mode === 'rtu'">
          <label>Slave ID</label>
          <input
            type="number"
            v-model.number="tabState.slaveId"
            :disabled="tabState.isConnected"
            min="1"
            max="247"
          />
        </div>
        <div class="field-row" v-else>
          <label>Unit ID</label>
          <input
            type="number"
            v-model.number="tabState.unitId"
            :disabled="tabState.isConnected"
            min="1"
            max="247"
          />
        </div>

        <div class="field-row">
          <label>Timeout (ms)</label>
          <input
            type="number"
            v-model.number="tabState.responseTimeout"
            :disabled="tabState.isConnected"
            min="100"
            max="10000"
            step="100"
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
          <path d="M5 12h14M12 5l7 7-7 7"/>
        </svg>
        <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M18.36 6.64a9 9 0 1 1-12.73 0"/>
          <line x1="12" y1="2" x2="12" y2="12"/>
        </svg>
        <span>{{ tabState.isConnected ? 'Disconnect' : 'Connect' }}</span>
      </button>

      <!-- Status Message -->
      <div v-if="tabState.statusMessage" class="status-message" :class="tabState.connectionStatus">
        {{ tabState.statusMessage }}
      </div>

      <!-- Request Builder -->
      <div class="config-card" :class="{ disabled: !tabState.isConnected }">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
            <polyline points="22 4 12 14.01 9 11.01"/>
          </svg>
          <span>Request</span>
        </div>

        <!-- Function Code -->
        <div class="dropdown-item" :class="{ open: openDropdown === 'functionCode' }" @click.stop="toggleDropdown('functionCode')">
          <div class="dropdown-trigger">
            <div class="dropdown-label">
              <span>{{ currentFunctionCode.label }}</span>
            </div>
            <svg class="chevron" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 9l6 6 6-6"/>
            </svg>
          </div>
          <div v-if="openDropdown === 'functionCode'" class="dropdown-menu" @click.stop>
            <div
              v-for="fc in functionCodes"
              :key="fc.value"
              class="dropdown-option"
              :class="{ selected: tabState.functionCode === fc.value }"
              @click="selectOption('functionCode', fc.value)"
            >
              {{ fc.label }}
            </div>
          </div>
        </div>

        <div class="field-row">
          <label>Start Address</label>
          <input
            type="number"
            v-model.number="tabState.startAddress"
            min="0"
            max="65535"
          />
        </div>

        <div v-if="isReadOperation" class="field-row">
          <label>Quantity</label>
          <input
            type="number"
            v-model.number="tabState.quantity"
            min="1"
            max="125"
          />
        </div>

        <!-- Write Values (for write operations) -->
        <div v-if="isWriteOperation" class="write-values">
          <label>{{ isCoilOperation ? 'Coil Values' : 'Register Values' }}</label>

          <div v-if="isCoilOperation" class="coil-values">
            <div v-for="(val, idx) in tabState.coilValues" :key="idx" class="coil-item">
              <span class="coil-index">{{ tabState.startAddress + idx }}:</span>
              <label class="toggle-switch compact">
                <input type="checkbox" v-model="tabState.coilValues[idx]" />
                <span class="toggle-slider"></span>
              </label>
              <button v-if="!isSingleWrite && tabState.coilValues.length > 1" class="btn-remove" @click="removeWriteValue(idx)">×</button>
            </div>
          </div>

          <div v-else class="register-values">
            <div v-for="(val, idx) in tabState.writeValues" :key="idx" class="register-item">
              <span class="register-index">{{ tabState.startAddress + idx }}:</span>
              <input type="number" v-model.number="tabState.writeValues[idx]" min="0" max="65535" />
              <button v-if="!isSingleWrite && tabState.writeValues.length > 1" class="btn-remove" @click="removeWriteValue(idx)">×</button>
            </div>
          </div>

          <button v-if="!isSingleWrite" class="btn-add-value" @click="addWriteValue">
            + Add {{ isCoilOperation ? 'Coil' : 'Register' }}
          </button>
        </div>

        <button
          class="btn-send"
          @click="sendRequest"
          :disabled="!tabState.isConnected || isLoading"
        >
          <svg v-if="!isLoading" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="22" y1="2" x2="11" y2="13"/>
            <polygon points="22 2 15 22 11 13 2 9 22 2"/>
          </svg>
          <span v-if="isLoading" class="spinner"></span>
          <span>{{ isReadOperation ? 'Read' : 'Write' }}</span>
        </button>
      </div>

      <!-- Polling -->
      <div class="config-card" :class="{ disabled: !tabState.isConnected }">
        <div class="card-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="23 4 23 10 17 10"/>
            <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
          </svg>
          <span>Polling</span>
          <span v-if="tabState.pollingEnabled" class="badge polling">Active</span>
        </div>

        <div class="field-row">
          <label>Interval (ms)</label>
          <input
            type="number"
            v-model.number="tabState.pollingInterval"
            :disabled="tabState.pollingEnabled"
            min="100"
            max="60000"
            step="100"
          />
        </div>

        <div class="polling-buttons">
          <button
            v-if="!tabState.pollingEnabled"
            class="btn-start-poll"
            @click="startPolling"
            :disabled="!tabState.isConnected || !isReadOperation"
          >
            Start
          </button>
          <button
            v-else
            class="btn-stop-poll"
            @click="stopPolling"
          >
            Stop
          </button>
        </div>
      </div>
    </aside>

    <!-- Main Area -->
    <main class="main-area">
      <!-- Content wrapper: Data + Log side by side -->
      <div class="content-wrapper">
      <!-- Data Display -->
      <div class="data-section">
        <div class="section-header">
          <h3>{{ isCoilOperation ? 'Coils' : 'Registers' }}</h3>
          <div class="format-selector">
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
          <button class="btn-clear-data" @click="clearData" title="Clear data">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
          </button>
        </div>

        <div class="data-table-container">
          <!-- Register Table -->
          <table v-if="tabState.registerData.length > 0" class="data-table">
            <thead>
              <tr>
                <th>Address</th>
                <th>Value</th>
                <th>Hex</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="reg in tabState.registerData" :key="reg.address">
                <td class="addr">{{ formatAddress(reg.address) }}</td>
                <td class="value">{{ formatValue(reg.value, tabState.dataFormat) }}</td>
                <td class="hex">0x{{ reg.rawHex }}</td>
              </tr>
            </tbody>
          </table>

          <!-- Coil Table -->
          <table v-else-if="tabState.coilData.length > 0" class="data-table coil-table">
            <thead>
              <tr>
                <th>Address</th>
                <th>Value</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="coil in tabState.coilData" :key="coil.address">
                <td class="addr">{{ formatAddress(coil.address) }}</td>
                <td class="value" :class="{ 'coil-on': coil.value, 'coil-off': !coil.value }">
                  {{ coil.value ? 'ON' : 'OFF' }}
                </td>
              </tr>
            </tbody>
          </table>

          <div v-else class="empty-data">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
              <line x1="16" y1="13" x2="8" y2="13"/>
              <line x1="16" y1="17" x2="8" y2="17"/>
              <polyline points="10 9 9 9 8 9"/>
            </svg>
            <p>No data. Send a read request to view registers.</p>
          </div>
        </div>

        <!-- Response Time -->
        <div v-if="tabState.lastResponseTime !== null" class="response-time">
          Last response: {{ tabState.lastResponseTime }}ms
        </div>
      </div>

      <!-- Transaction Log -->
      <div class="log-section">
        <div class="section-header">
          <h3>Transaction Log</h3>
          <span class="log-count">{{ tabState.transactionLog.length }} entries</span>
          <button class="btn-clear-log" @click="clearLog" title="Clear log">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
          </button>
        </div>

        <div class="log-container" ref="transactionLogRef">
          <div
            v-for="entry in displayedLogEntries"
            :key="entry.id"
            class="log-entry"
            :class="entry.type"
          >
            <div class="log-header">
              <span class="log-time">{{ entry.timestamp }}</span>
              <span class="log-fc">FC{{ entry.functionCode.toString().padStart(2, '0') }}</span>
              <span class="log-status" :class="entry.success ? 'success' : 'error'">
                {{ entry.success ? 'OK' : 'ERR' }}
              </span>
              <span v-if="entry.responseTime > 0" class="log-duration">{{ entry.responseTime }}ms</span>
            </div>
            <div class="log-frames">
              <div class="log-frame tx">
                <span class="frame-label">TX:</span>
                <span class="frame-data">{{ formatHex(entry.requestFrame) }}</span>
              </div>
              <div class="log-frame rx">
                <span class="frame-label">RX:</span>
                <span class="frame-data">{{ formatHex(entry.responseFrame) }}</span>
              </div>
            </div>
            <div v-if="entry.errorMessage" class="log-error">
              {{ entry.errorMessage }}
            </div>
          </div>

          <div v-if="displayedLogEntries.length === 0" class="empty-log">
            No transactions yet
          </div>
        </div>
      </div>
      </div><!-- End content-wrapper -->
    </main>
  </div>
</template>

<style scoped>
.modbus-tab {
  display: flex;
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
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 8px;
}

.content-wrapper {
  flex: 1;
  display: flex;
  gap: 8px;
  overflow: hidden;
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
}

.status-message.error {
  background: rgba(239, 68, 68, 0.1);
  color: var(--danger);
}

.status-message.connected {
  background: rgba(16, 185, 129, 0.1);
  color: var(--success);
}

/* Send Button */
.btn-send {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 5px;
  width: 100%;
  padding: 7px;
  border: none;
  border-radius: var(--radius-sm);
  background: var(--accent-primary);
  color: white;
  font-weight: 500;
  font-size: 12px;
  cursor: pointer;
  margin-top: 6px;
}

.btn-send:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.spinner {
  width: 12px;
  height: 12px;
  border: 2px solid white;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* Write Values */
.write-values {
  margin-top: 6px;
}

.write-values > label {
  display: block;
  font-size: 11px;
  color: var(--text-secondary);
  margin-bottom: 5px;
}

.register-item, .coil-item {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 4px;
}

.register-index, .coil-index {
  min-width: 42px;
  font-size: 10px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

.register-item input {
  flex: 1;
  padding: 3px 6px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  font-size: 11px;
}

.btn-remove {
  width: 16px;
  height: 16px;
  border: none;
  border-radius: 50%;
  background: var(--danger);
  color: white;
  cursor: pointer;
  font-size: 12px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-add-value {
  width: 100%;
  padding: 4px;
  border: 1px dashed var(--border-color);
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-secondary);
  font-size: 11px;
  cursor: pointer;
  margin-top: 3px;
}

.btn-add-value:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

/* Toggle Switch */
.toggle-switch {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
}

.toggle-switch.compact {
  gap: 3px;
}

.toggle-switch input {
  display: none;
}

.toggle-slider {
  width: 28px;
  height: 16px;
  background: var(--bg-tertiary);
  border-radius: 8px;
  position: relative;
  transition: background 0.2s;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  width: 12px;
  height: 12px;
  background: white;
  border-radius: 50%;
  top: 2px;
  left: 2px;
  transition: transform 0.2s;
}

.toggle-switch input:checked + .toggle-slider {
  background: var(--accent-primary);
}

.toggle-switch input:checked + .toggle-slider::before {
  transform: translateX(12px);
}

/* Polling */
.badge {
  font-size: 9px;
  padding: 1px 5px;
  border-radius: 8px;
  margin-left: auto;
}

.badge.polling {
  background: var(--success);
  color: white;
}

.polling-buttons {
  display: flex;
  gap: 6px;
  margin-top: 6px;
}

.btn-start-poll, .btn-stop-poll {
  flex: 1;
  padding: 5px;
  border: none;
  border-radius: var(--radius-sm);
  font-size: 11px;
  font-weight: 500;
  cursor: pointer;
}

.btn-start-poll {
  background: var(--success);
  color: white;
}

.btn-stop-poll {
  background: var(--danger);
  color: white;
}

.btn-start-poll:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Data Section */
.data-section {
  flex: 0 0 auto;
  width: 42%;
  min-width: 340px;
  max-width: 450px;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  overflow: hidden;
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

.format-selector {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-left: auto;
  font-size: 11px;
  color: var(--text-secondary);
}

.btn-clear-data, .btn-clear-log {
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

.btn-clear-data svg {
  width: 12px;
  height: 12px;
}

.btn-clear-log svg {
  width: 10px;
  height: 10px;
}

.btn-clear-data:hover, .btn-clear-log:hover {
  background: var(--bg-tertiary);
  color: var(--danger);
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
}

.data-table .value {
  font-weight: 500;
}

.data-table .hex {
  color: var(--accent-primary);
}

.coil-on {
  color: var(--success) !important;
}

.coil-off {
  color: var(--text-tertiary) !important;
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

.response-time {
  padding: 4px 10px;
  font-size: 10px;
  color: var(--text-secondary);
  border-top: 1px solid var(--border-color);
}

/* Log Section */
.log-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
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

.log-frame.tx .frame-label {
  color: var(--warning);
}

.log-frame.rx .frame-label {
  color: var(--success);
}

.frame-data {
  color: var(--text-secondary);
  word-break: break-all;
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
