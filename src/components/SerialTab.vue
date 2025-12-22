<script setup>
import { ref, computed, nextTick, watch, inject, onMounted, onUnmounted } from 'vue';
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
  connectedPorts: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(['connect', 'disconnect', 'refreshPorts']);

const t = inject('t');

// Constants
const MAX_TERMINAL_ENTRIES = 500;

// Local refs for dropdowns
const openDropdown = ref(null);
const terminalRef = ref(null);
const scrollPending = ref(false);

// Config options
const baudRateOptions = [9600, 19200, 57600, 115200, 460800, 921600];
const dataBitsOptions = [5, 6, 7, 8];
const stopBitsOptions = ['1', '1.5', '2'];
const parityOptions = ['none', 'odd', 'even'];
const lineEndingOptions = ['None', 'CR', 'LF', 'CRLF'];

// Helper to add terminal entry with limit
function addTerminalEntry(entry) {
  // Trim old entries if exceeding limit
  if (props.tabState.terminalData.length >= MAX_TERMINAL_ENTRIES) {
    const removed = props.tabState.terminalData.shift();
    if (removed.type === 'tx') props.tabState.txCount--;
    else props.tabState.rxCount--;
  }

  props.tabState.terminalData.push(entry);
  if (entry.type === 'tx') {
    props.tabState.txCount++;
    props.tabState.totalTxCount++;
  } else {
    props.tabState.rxCount++;
    props.tabState.totalRxCount++;
  }

  throttledScrollToBottom();
}

// Format baud rate với dấu chấm
function formatBaudRate(rate) {
  return rate.toString().replace(/\B(?=(\d{3})+(?!\d))/g, '.');
}

// Line ending helpers
function getLineEndingBytes() {
  switch (props.tabState.lineEnding) {
    case 'CR': return [0x0D];
    case 'LF': return [0x0A];
    case 'CRLF': return [0x0D, 0x0A];
    default: return [];
  }
}

function getLineEndingStr() {
  switch (props.tabState.lineEnding) {
    case 'CR': return '\r';
    case 'LF': return '\n';
    case 'CRLF': return '\r\n';
    default: return '';
  }
}

// TextDecoder for UTF-8 (supports Vietnamese)
const textDecoder = new TextDecoder('utf-8', { fatal: false });

// Format data for display
function formatDataLine(entry) {
  if (props.tabState.displayMode === 'hex') {
    return entry.data.map(b => b.toString(16).padStart(2, '0').toUpperCase()).join(' ');
  }
  // Decode UTF-8 bytes to string (supports Vietnamese and other Unicode)
  const uint8Array = new Uint8Array(entry.data);
  let text = textDecoder.decode(uint8Array);
  // Remove carriage return for cleaner display
  text = text.replace(/\r/g, '');
  return text;
}

// Dropdown handlers
function toggleDropdown(name) {
  if (props.tabState.isConnected && name !== 'port') return;
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
  }
  openDropdown.value = null;
}

function closeDropdowns() {
  openDropdown.value = null;
}

// Check if port is available (not connected in other tabs)
function isPortAvailable(portName) {
  // If this tab is connected to this port, it's available for this tab
  if (props.tabState.selectedPort === portName && props.tabState.isConnected) {
    return true;
  }
  // Otherwise, check if it's connected elsewhere
  return !props.connectedPorts.includes(portName);
}

// Connection handlers
async function toggleConnection() {
  if (props.tabState.isConnected) {
    emit('disconnect', props.tabId);
  } else {
    emit('connect', props.tabId);
  }
}

// Send message
async function sendMessage() {
  if (!props.tabState.inputMessage || !props.tabState.isConnected) return;

  try {
    const dataToSend = props.tabState.sendAsHex
      ? props.tabState.inputMessage
      : (props.tabState.inputMessage + getLineEndingStr());

    await invoke('send_data', {
      portName: props.tabState.selectedPort,
      data: dataToSend,
      isHex: props.tabState.sendAsHex,
      byteDelayUs: props.tabState.byteDelay > 0 ? props.tabState.byteDelay : null,
    });

    // Add to terminal as TX
    const timestamp = new Date().toLocaleTimeString();
    let dataBytes;
    if (props.tabState.sendAsHex) {
      const hexStr = props.tabState.inputMessage.replace(/\s/g, '').replace(/0x/gi, '');
      dataBytes = [];
      for (let i = 0; i < hexStr.length; i += 2) {
        dataBytes.push(parseInt(hexStr.substr(i, 2), 16));
      }
    } else {
      dataBytes = Array.from(new TextEncoder().encode(props.tabState.inputMessage));
      dataBytes.push(...getLineEndingBytes());
    }

    addTerminalEntry({
      type: 'tx',
      data: dataBytes,
      timestamp,
    });
  } catch (error) {
    console.error('Error sending data:', error);
    alert('Error: ' + error);
  }
}

function handleKeyDown(event) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    sendMessage();
  }
}

// Auto send with backpressure
async function doAutoSend() {
  if (!props.tabState.autoSendCurrentMessage || !props.tabState.isConnected) return;

  // Backpressure: skip if previous send still in progress
  if (props.tabState.autoSendInProgress) return;

  props.tabState.autoSendInProgress = true;

  try {
    const dataToSend = props.tabState.sendAsHex
      ? props.tabState.autoSendCurrentMessage
      : (props.tabState.autoSendCurrentMessage + getLineEndingStr());

    await invoke('send_data', {
      portName: props.tabState.selectedPort,
      data: dataToSend,
      isHex: props.tabState.sendAsHex,
      byteDelayUs: props.tabState.byteDelay > 0 ? props.tabState.byteDelay : null,
    });

    const timestamp = new Date().toLocaleTimeString();
    let dataBytes;
    if (props.tabState.sendAsHex) {
      const hexStr = props.tabState.autoSendCurrentMessage.replace(/\s/g, '').replace(/0x/gi, '');
      dataBytes = [];
      for (let i = 0; i < hexStr.length; i += 2) {
        dataBytes.push(parseInt(hexStr.substr(i, 2), 16));
      }
    } else {
      dataBytes = Array.from(new TextEncoder().encode(props.tabState.autoSendCurrentMessage));
      dataBytes.push(...getLineEndingBytes());
    }

    addTerminalEntry({
      type: 'tx',
      data: dataBytes,
      timestamp,
    });

    props.tabState.autoSendCount++;
  } catch (error) {
    console.error('Auto send error:', error);
    stopAutoSend();
  } finally {
    props.tabState.autoSendInProgress = false;
  }
}

function startAutoSend() {
  if (!props.tabState.inputMessage) {
    alert(t.value.pleaseEnterMessage);
    return;
  }
  if (!props.tabState.isConnected) {
    alert(t.value.pleaseConnectFirst);
    return;
  }

  props.tabState.autoSendCurrentMessage = props.tabState.inputMessage;
  props.tabState.autoSendCount = 0;
  props.tabState.autoSendEnabled = true;

  doAutoSend();

  props.tabState.autoSendTimer = setInterval(() => {
    doAutoSend();
  }, props.tabState.autoSendInterval);
}

function stopAutoSend() {
  props.tabState.autoSendEnabled = false;
  if (props.tabState.autoSendTimer) {
    clearInterval(props.tabState.autoSendTimer);
    props.tabState.autoSendTimer = null;
  }
}

// Terminal helpers
function clearTerminal() {
  props.tabState.terminalData.splice(0, props.tabState.terminalData.length);
  props.tabState.txCount = 0;
  props.tabState.rxCount = 0;
  props.tabState.totalTxCount = 0;
  props.tabState.totalRxCount = 0;
}

function scrollToBottom() {
  if (props.tabState.autoScroll && terminalRef.value) {
    terminalRef.value.scrollTop = terminalRef.value.scrollHeight;
  }
}

// Throttled scroll - only scroll once per animation frame
function throttledScrollToBottom() {
  if (!scrollPending.value && props.tabState.autoScroll) {
    scrollPending.value = true;
    requestAnimationFrame(() => {
      scrollToBottom();
      scrollPending.value = false;
    });
  }
}

// Watch for terminal data changes (RX from App.vue) and auto-scroll
watch(
  () => props.tabState.terminalData.length,
  () => {
    throttledScrollToBottom();
  }
);

// Lifecycle
onMounted(() => {
  document.addEventListener('click', closeDropdowns);
});

onUnmounted(() => {
  document.removeEventListener('click', closeDropdowns);
  stopAutoSend();
});
</script>

<template>
  <div class="serial-tab">
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
          <div class="dropdown-item port-dropdown" :class="{ disabled: tabState.isConnected, open: openDropdown === 'port' }">
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
                <span v-if="!tabState.selectedPort" class="placeholder">{{ t.selectPort }}</span>
                <div v-else class="selected-port-info">
                  <span class="selected-port">{{ ports.find(p => p.name === tabState.selectedPort)?.port_type || tabState.selectedPort }}</span>
                  <span class="selected-device" v-if="ports.find(p => p.name === tabState.selectedPort)?.product || ports.find(p => p.name === tabState.selectedPort)?.manufacturer">
                    {{ ports.find(p => p.name === tabState.selectedPort)?.product || ports.find(p => p.name === tabState.selectedPort)?.manufacturer }}
                  </span>
                </div>
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
                :class="{
                  selected: tabState.selectedPort === port.name,
                  disabled: !isPortAvailable(port.name)
                }"
                @click="isPortAvailable(port.name) && selectOption('port', port.name)"
              >
                <div class="port-details">
                  <span class="port-name">{{ port.port_type }}</span>
                  <span class="port-manufacturer" v-if="port.manufacturer || port.product">
                    {{ port.product || port.manufacturer }}
                  </span>
                  <span v-if="!isPortAvailable(port.name)" class="port-in-use">(In use)</span>
                </div>
                <svg v-if="tabState.selectedPort === port.name" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
          <div class="dropdown-item" :class="{ disabled: tabState.isConnected, open: openDropdown === 'baudRate' }">
            <div class="dropdown-trigger" @click="toggleDropdown('baudRate')">
              <div class="dropdown-label">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                </svg>
                <span>{{ t.baud }}</span>
              </div>
              <div class="dropdown-value">
                <span>{{ formatBaudRate(tabState.baudRate) }}</span>
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
                :class="{ selected: tabState.baudRate === rate }"
                @click="selectOption('baudRate', rate)"
              >
                {{ formatBaudRate(rate) }}
                <svg v-if="tabState.baudRate === rate" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20,6 9,17 4,12"/>
                </svg>
              </div>
            </div>
          </div>

          <!-- Data Bits -->
          <div class="dropdown-item" :class="{ disabled: tabState.isConnected, open: openDropdown === 'dataBits' }">
            <div class="dropdown-trigger" @click="toggleDropdown('dataBits')">
              <div class="dropdown-label">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="3" y="3" width="18" height="18" rx="2"/>
                  <path d="M9 9h6v6H9z"/>
                </svg>
                <span>{{ t.data }}</span>
              </div>
              <div class="dropdown-value">
                <span>{{ tabState.dataBits }} bits</span>
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
                :class="{ selected: tabState.dataBits === bits }"
                @click="selectOption('dataBits', bits)"
              >
                {{ bits }} bits
                <svg v-if="tabState.dataBits === bits" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20,6 9,17 4,12"/>
                </svg>
              </div>
            </div>
          </div>

          <!-- Stop Bits -->
          <div class="dropdown-item" :class="{ disabled: tabState.isConnected, open: openDropdown === 'stopBits' }">
            <div class="dropdown-trigger" @click="toggleDropdown('stopBits')">
              <div class="dropdown-label">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="6" y="4" width="4" height="16"/>
                  <rect x="14" y="4" width="4" height="16"/>
                </svg>
                <span>{{ t.stop }}</span>
              </div>
              <div class="dropdown-value">
                <span>{{ tabState.stopBits }}</span>
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
                :class="{ selected: tabState.stopBits === bits }"
                @click="selectOption('stopBits', bits)"
              >
                {{ bits }}
                <svg v-if="tabState.stopBits === bits" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20,6 9,17 4,12"/>
                </svg>
              </div>
            </div>
          </div>

          <!-- Parity -->
          <div class="dropdown-item" :class="{ disabled: tabState.isConnected, open: openDropdown === 'parity' }">
            <div class="dropdown-trigger" @click="toggleDropdown('parity')">
              <div class="dropdown-label">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                  <polyline points="22,4 12,14.01 9,11.01"/>
                </svg>
                <span>{{ t.parity }}</span>
              </div>
              <div class="dropdown-value">
                <span>{{ tabState.parity.charAt(0).toUpperCase() + tabState.parity.slice(1) }}</span>
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
                :class="{ selected: tabState.parity === p }"
                @click="selectOption('parity', p)"
              >
                {{ p.charAt(0).toUpperCase() + p.slice(1) }}
                <svg v-if="tabState.parity === p" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20,6 9,17 4,12"/>
                </svg>
              </div>
            </div>
          </div>
        </div>
      </div>

      <button class="btn-connect" :class="{ connected: tabState.isConnected }" @click="toggleConnection" :disabled="!tabState.selectedPort">
        <svg v-if="!tabState.isConnected" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M5 12h14M12 5l7 7-7 7"/>
        </svg>
        <svg v-else width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
        <span>{{ tabState.isConnected ? t.disconnect : t.connect }}</span>
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
          <button :class="{ active: tabState.displayMode === 'text' }" @click="tabState.displayMode = 'text'">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="4,7 4,4 20,4 20,7"/>
              <line x1="9" y1="20" x2="15" y2="20"/>
              <line x1="12" y1="4" x2="12" y2="20"/>
            </svg>
            Text
          </button>
          <button :class="{ active: tabState.displayMode === 'hex' }" @click="tabState.displayMode = 'hex'">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M4 17l6-6-6-6M12 19h8"/>
            </svg>
            Hex
          </button>
        </div>

        <label class="toggle-switch">
          <input type="checkbox" v-model="tabState.autoScroll" />
          <span class="toggle-slider"></span>
          <span class="toggle-label">{{ t.autoScroll }}</span>
        </label>

        <div class="line-ending-row">
          <span class="line-ending-label">Line Ending</span>
          <div class="line-ending-toggle">
            <button
              v-for="opt in lineEndingOptions"
              :key="opt"
              :class="{ active: tabState.lineEnding === opt }"
              @click="tabState.lineEnding = opt"
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
          <span v-if="tabState.autoSendEnabled" class="auto-send-badge running">ON</span>
        </div>

        <div class="auto-send-config">
          <div class="config-row-inline">
            <label>{{ t.frequency }}</label>
            <div class="interval-group">
              <input
                type="number"
                v-model.number="tabState.autoSendInterval"
                min="50"
                max="60000"
                :disabled="tabState.autoSendEnabled"
                class="interval-input"
              />
              <span class="interval-unit">ms</span>
            </div>
          </div>
          <div class="config-row-inline">
            <label>Byte Delay</label>
            <div class="interval-group">
              <input
                type="number"
                v-model.number="tabState.byteDelay"
                min="0"
                max="10000"
                step="100"
                :disabled="tabState.autoSendEnabled"
                class="interval-input"
                title="Inter-byte delay for slow devices (0 = disabled)"
              />
              <span class="interval-unit">µs</span>
            </div>
          </div>
          <div class="auto-send-info" v-if="tabState.autoSendEnabled">
            <span class="send-count">{{ t.sent }}: {{ tabState.autoSendCount }} {{ t.times }}</span>
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
        <div class="terminal-stats" v-if="tabState.totalTxCount > 0 || tabState.totalRxCount > 0">
          <span class="stat-item tx">
            <span class="stat-arrow">↑</span>
            TX {{ tabState.totalTxCount }}
          </span>
          <span class="stat-item rx">
            <span class="stat-arrow">↓</span>
            RX {{ tabState.totalRxCount }}
          </span>
        </div>
        <button class="btn-clear" @click="clearTerminal">
          <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3,6 5,6 21,6"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
          </svg>
          {{ t.clear }}
        </button>
      </div>

      <div class="terminal" ref="terminalRef">
        <div
          v-for="(entry, index) in tabState.terminalData"
          :key="index"
          class="terminal-line"
          :class="entry.type"
        >
          <span class="timestamp">[{{ entry.timestamp }}]</span>
          <span class="direction-badge" :class="entry.type">{{ entry.type === 'tx' ? 'TX' : 'RX' }}</span>
          <span class="data">{{ formatDataLine(entry) }}</span>
        </div>
        <div v-if="tabState.terminalData.length === 0" class="terminal-empty">
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
            <input type="checkbox" v-model="tabState.sendAsHex" :disabled="tabState.autoSendEnabled" />
            <span class="toggle-slider"></span>
            <span class="toggle-label">Hex</span>
          </label>
          <div class="input-wrapper">
            <input
              type="text"
              v-model="tabState.inputMessage"
              :placeholder="tabState.sendAsHex ? t.hexExample : t.enterMessage"
              :disabled="!tabState.isConnected || tabState.autoSendEnabled"
              @keydown="handleKeyDown"
            />
            <button
              v-if="tabState.inputMessage"
              class="btn-clear-input"
              @click="tabState.inputMessage = ''"
              :title="t.clearContent"
            >
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
          <button class="btn-send" :disabled="!tabState.isConnected || !tabState.inputMessage || tabState.autoSendEnabled" @click="sendMessage">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="22" y1="2" x2="11" y2="13"/>
              <polygon points="22,2 15,22 11,13 2,9 22,2"/>
            </svg>
            {{ t.send }}
          </button>
          <button
            v-if="!tabState.autoSendEnabled"
            class="btn-auto-send"
            :disabled="!tabState.isConnected || !tabState.inputMessage"
            @click="startAutoSend"
            :title="t.auto"
          >
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
            <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
              <rect x="6" y="4" width="4" height="16"/>
              <rect x="14" y="4" width="4" height="16"/>
            </svg>
            {{ t.stop }}
          </button>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.serial-tab {
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

/* Port dropdown */
.port-dropdown-wrapper {
  margin-bottom: 0;
}

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

.dropdown-option.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.dropdown-option.disabled:hover {
  background: transparent;
}

.dropdown-option svg {
  color: var(--accent-primary);
}

/* Port specific */
.port-dropdown .dropdown-trigger {
  padding: 10px 12px;
}

.port-dropdown .dropdown-label {
  flex: 1;
  gap: 10px;
}

.port-dropdown .placeholder {
  color: var(--text-tertiary);
  font-size: 0.8rem;
}

.port-dropdown .selected-port-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.port-dropdown .selected-port {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.port-dropdown .selected-device {
  font-size: 0.65rem;
  color: var(--accent-primary);
  font-weight: 500;
  opacity: 0.9;
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

.port-manufacturer {
  font-size: 0.7rem;
  color: var(--accent-primary);
  font-weight: 500;
}

.port-in-use {
  font-size: 0.6rem;
  color: var(--warning);
  font-weight: 600;
}

/* Config grid */
.config-grid-pro {
  display: flex;
  flex-direction: column;
  gap: 6px;
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

.btn-connect:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgb(99 102 241 / 0.4);
}

.btn-connect:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}

.btn-connect.connected {
  background: linear-gradient(135deg, var(--danger), #f87171);
  box-shadow: 0 4px 14px rgb(239 68 68 / 0.35);
}

/* Display Card */
.display-card {
  margin-top: auto;
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

.toggle-switch.compact {
  gap: 4px;
}

.toggle-switch.compact .toggle-slider {
  width: 26px;
  height: 14px;
}

.toggle-switch.compact .toggle-slider::before {
  width: 10px;
  height: 10px;
}

.toggle-switch.compact input:checked + .toggle-slider::before {
  transform: translateX(12px);
}

.toggle-switch.compact .toggle-label {
  font-size: 0.65rem;
}

/* Line ending */
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

.line-ending-toggle button.active {
  background: var(--bg-secondary);
  color: var(--accent-primary);
  box-shadow: var(--shadow-sm);
}

/* Auto Send Card */
.auto-send-card {
  border: 1px solid var(--border-color);
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
  padding: 6px 12px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.terminal-title {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-primary);
}

.terminal-title svg {
  width: 12px;
  height: 12px;
  color: var(--accent-primary);
}

.terminal-stats {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-left: auto;
  margin-right: 8px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.6rem;
  font-weight: 600;
  font-family: var(--font-mono);
}

.stat-item.tx {
  background: var(--warning);
  color: white;
}

.stat-item.rx {
  background: var(--success);
  color: white;
}

.stat-arrow {
  font-size: 0.55rem;
  font-weight: 700;
}

.btn-clear {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  font-size: 0.7rem;
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

/* Send Container */
.send-container {
  padding: 8px 12px;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.send-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
}

.input-wrapper {
  flex: 1;
  position: relative;
}

.input-wrapper input {
  width: 100%;
  padding: 8px 32px 8px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 0.8rem;
  font-family: var(--font-mono);
  transition: all 0.2s ease;
}

.input-wrapper input:focus {
  outline: none;
  border-color: var(--border-focus);
  background: var(--bg-secondary);
  box-shadow: 0 0 0 2px var(--accent-light);
}

.input-wrapper input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.input-wrapper input::placeholder {
  color: var(--text-tertiary);
}

.btn-clear-input {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
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

.btn-send {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 8px 14px;
  font-size: 0.75rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  border: none;
  border-radius: var(--radius-md);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgb(99 102 241 / 0.3);
}

.btn-send:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgb(99 102 241 / 0.4);
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
  gap: 4px;
  padding: 8px 12px;
  font-size: 0.75rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: var(--success);
  border: none;
  border-radius: var(--radius-md);
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
  gap: 4px;
  padding: 8px 12px;
  font-size: 0.75rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: var(--danger);
  border: none;
  border-radius: var(--radius-md);
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
</style>
