<script setup>
import { ref, watch, inject, onMounted, onUnmounted, computed } from 'vue';
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
});

const emit = defineEmits(['start', 'stop']);

const t = inject('t');

// Constants
const MAX_TERMINAL_ENTRIES = 500;

// Local refs
const terminalRef = ref(null);
const scrollPending = ref(false);
const openDropdown = ref(null);

// Line ending options
const lineEndingOptions = ['None', 'CR', 'LF', 'CRLF'];
const bindAddressOptions = ['0.0.0.0', '127.0.0.1'];

// Computed
const clientCount = computed(() => props.tabState.connectedClients?.length || 0);
const selectedClientLabel = computed(() => {
  if (!props.tabState.selectedClientId) return t.sendToAll;
  const client = props.tabState.connectedClients?.find(c => c.clientId === props.tabState.selectedClientId);
  return client ? client.remoteAddr : t.sendToAll;
});

// Helper to add terminal entry with limit
function addTerminalEntry(entry) {
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

// Line ending helpers
function getLineEndingStr() {
  switch (props.tabState.lineEnding) {
    case 'CR': return '\r';
    case 'LF': return '\n';
    case 'CRLF': return '\r\n';
    default: return '';
  }
}

function getLineEndingBytes() {
  switch (props.tabState.lineEnding) {
    case 'CR': return [0x0D];
    case 'LF': return [0x0A];
    case 'CRLF': return [0x0D, 0x0A];
    default: return [];
  }
}

// TextDecoder for UTF-8
const textDecoder = new TextDecoder('utf-8', { fatal: false });

// Format data for display
function formatDataLine(entry) {
  if (props.tabState.displayMode === 'hex') {
    return entry.data.map(b => b.toString(16).padStart(2, '0').toUpperCase()).join(' ');
  }
  const uint8Array = new Uint8Array(entry.data);
  let text = textDecoder.decode(uint8Array);
  text = text.replace(/\r/g, '');
  return text;
}

// Dropdown handlers
function toggleDropdown(name) {
  if (props.tabState.isConnected && (name === 'bindAddress')) return;
  openDropdown.value = openDropdown.value === name ? null : name;
}

function selectOption(name, value) {
  switch (name) {
    case 'bindAddress':
      props.tabState.bindAddress = value;
      break;
    case 'client':
      props.tabState.selectedClientId = value;
      break;
  }
  openDropdown.value = null;
}

function closeDropdowns() {
  openDropdown.value = null;
}

// Server handlers
async function toggleServer() {
  console.log("[TcpServerTab] toggleServer called, isConnected:", props.tabState.isConnected, "tabId:", props.tabId);
  if (props.tabState.isConnected) {
    console.log("[TcpServerTab] Emitting 'stop' event");
    emit('stop', props.tabId);
  } else {
    console.log("[TcpServerTab] Emitting 'start' event");
    emit('start', props.tabId);
  }
}

// Disconnect a client
async function disconnectClient(clientId) {
  try {
    await invoke('tcp_server_disconnect_client', {
      serverId: props.tabState.serverId,
      clientId,
    });
  } catch (error) {
    console.error('Error disconnecting client:', error);
  }
}

// Send message
async function sendMessage() {
  if (!props.tabState.inputMessage || !props.tabState.isConnected) return;

  try {
    const dataToSend = props.tabState.sendAsHex
      ? props.tabState.inputMessage
      : (props.tabState.inputMessage + getLineEndingStr());

    await invoke('tcp_server_send', {
      serverId: props.tabState.serverId,
      clientId: props.tabState.selectedClientId || null,
      data: dataToSend,
      isHex: props.tabState.sendAsHex,
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

    const targetLabel = props.tabState.selectedClientId
      ? props.tabState.connectedClients?.find(c => c.clientId === props.tabState.selectedClientId)?.remoteAddr || 'client'
      : 'all';

    addTerminalEntry({
      type: 'tx',
      data: dataBytes,
      timestamp,
      target: targetLabel,
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

  if (props.tabState.autoSendInProgress) return;

  props.tabState.autoSendInProgress = true;

  try {
    const dataToSend = props.tabState.sendAsHex
      ? props.tabState.autoSendCurrentMessage
      : (props.tabState.autoSendCurrentMessage + getLineEndingStr());

    await invoke('tcp_server_send', {
      serverId: props.tabState.serverId,
      clientId: props.tabState.selectedClientId || null,
      data: dataToSend,
      isHex: props.tabState.sendAsHex,
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
    alert(t.value?.pleaseEnterMessage || 'Please enter a message');
    return;
  }
  if (!props.tabState.isConnected) {
    alert(t.value?.pleaseStartFirst || 'Please start server first');
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

function throttledScrollToBottom() {
  if (!scrollPending.value && props.tabState.autoScroll) {
    scrollPending.value = true;
    requestAnimationFrame(() => {
      scrollToBottom();
      scrollPending.value = false;
    });
  }
}

// Watch for terminal data changes
watch(
  () => props.tabState.terminalData.length,
  () => {
    throttledScrollToBottom();
  }
);

// Watch for echo toggle changes - sync with Rust backend
watch(
  () => props.tabState.echoEnabled,
  async (enabled) => {
    if (props.tabState.isConnected) {
      try {
        await invoke('tcp_server_set_echo', {
          serverId: props.tabState.serverId,
          enabled,
        });
      } catch (error) {
        console.error('Error setting echo:', error);
      }
    }
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

// Expose addTerminalEntry for parent component
defineExpose({ addTerminalEntry });
</script>

<template>
  <div class="tcp-server-tab">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="config-card">
        <div class="card-header">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="3" width="20" height="6" rx="1"/>
            <rect x="2" y="15" width="20" height="6" rx="1"/>
            <circle cx="6" cy="6" r="1" fill="currentColor"/>
            <circle cx="6" cy="18" r="1" fill="currentColor"/>
          </svg>
          <span>{{ t.tcpServer }}</span>
        </div>

        <div class="config-fields">
          <!-- Bind Address + Port on same row -->
          <div class="field-row-inline" @click.stop>
            <div class="dropdown-item mini bind-dropdown" :class="{ disabled: tabState.isConnected, open: openDropdown === 'bindAddress' }">
              <div class="dropdown-trigger" @click="toggleDropdown('bindAddress')">
                <span class="dropdown-value-text">{{ tabState.bindAddress }}</span>
                <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="6,9 12,15 18,9"/>
                </svg>
              </div>
              <div class="dropdown-menu" v-if="openDropdown === 'bindAddress'">
                <div
                  v-for="addr in bindAddressOptions"
                  :key="addr"
                  class="dropdown-option"
                  :class="{ selected: tabState.bindAddress === addr }"
                  @click="selectOption('bindAddress', addr)"
                >
                  {{ addr }}
                  <svg v-if="tabState.bindAddress === addr" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20,6 9,17 4,12"/>
                  </svg>
                </div>
              </div>
            </div>
            <span class="port-separator">:</span>
            <input
              type="number"
              v-model.number="tabState.listenPort"
              :disabled="tabState.isConnected"
              min="1"
              max="65535"
              placeholder="5000"
              class="field-input port-input-inline"
            />
          </div>
        </div>
      </div>

      <div class="server-controls">
        <button class="btn-connect" :class="{ connected: tabState.isConnected }" @click="toggleServer" :disabled="!tabState.listenPort">
          <svg v-if="!tabState.isConnected" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="5,3 19,12 5,21" fill="currentColor"/>
          </svg>
          <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="6" y="4" width="4" height="16" fill="currentColor"/>
            <rect x="14" y="4" width="4" height="16" fill="currentColor"/>
          </svg>
          <span>{{ tabState.isConnected ? t.stopServer : t.startServer }}</span>
          <span v-if="tabState.isConnected" class="listening-info">:{{ tabState.listenPort }}</span>
        </button>
        <label class="toggle-switch mini" title="Echo - Gửi lại dữ liệu nhận được">
          <input type="checkbox" v-model="tabState.echoEnabled" />
          <span class="toggle-slider"></span>
          <span class="toggle-label">Echo</span>
        </label>
      </div>

      <!-- Error Message -->
      <div v-if="tabState.statusMessage && !tabState.isConnected" class="status-message error">
        {{ tabState.statusMessage }}
      </div>

      <!-- Connected Clients -->
      <div class="config-card clients-card" v-if="tabState.isConnected">
        <div class="card-header">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
            <circle cx="9" cy="7" r="4"/>
            <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
            <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
          </svg>
          <span>{{ t.connectedClients }}</span>
          <span class="client-count-badge">{{ clientCount }}</span>
        </div>

        <div class="clients-list" v-if="clientCount > 0">
          <div
            v-for="client in tabState.connectedClients"
            :key="client.clientId"
            class="client-item"
          >
            <span class="client-addr">{{ client.remoteAddr }}</span>
            <span class="client-id">{{ client.clientId }}</span>
            <button class="btn-disconnect-client" @click="disconnectClient(client.clientId)" :title="t.disconnect">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
        </div>
        <div class="no-clients" v-else>
          <span>{{ t.noClients }}</span>
        </div>

        <!-- Send target selector -->
        <div class="send-target" v-if="clientCount > 0" @click.stop>
          <label>{{ t.sendTo }}</label>
          <div class="dropdown-item mini" :class="{ open: openDropdown === 'client' }">
            <div class="dropdown-trigger" @click="toggleDropdown('client')">
              <span class="dropdown-value-text">{{ selectedClientLabel }}</span>
              <svg class="chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="6,9 12,15 18,9"/>
              </svg>
            </div>
            <div class="dropdown-menu" v-if="openDropdown === 'client'">
              <div
                class="dropdown-option"
                :class="{ selected: !tabState.selectedClientId }"
                @click="selectOption('client', null)"
              >
                {{ t.sendToAll }}
                <svg v-if="!tabState.selectedClientId" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20,6 9,17 4,12"/>
                </svg>
              </div>
              <div
                v-for="client in tabState.connectedClients"
                :key="client.clientId"
                class="dropdown-option"
                :class="{ selected: tabState.selectedClientId === client.clientId }"
                @click="selectOption('client', client.clientId)"
              >
                {{ client.remoteAddr }}
                <svg v-if="tabState.selectedClientId === client.clientId" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20,6 9,17 4,12"/>
                </svg>
              </div>
            </div>
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
            <span class="stat-arrow">&#8593;</span>
            TX {{ tabState.totalTxCount }}
          </span>
          <span class="stat-item rx">
            <span class="stat-arrow">&#8595;</span>
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
          <span v-if="entry.clientId" class="client-badge">{{ entry.clientId }}</span>
          <span class="data">{{ formatDataLine(entry) }}</span>
        </div>
        <div v-if="tabState.terminalData.length === 0" class="terminal-empty">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <rect x="2" y="3" width="20" height="6" rx="1"/>
            <rect x="2" y="15" width="20" height="6" rx="1"/>
            <circle cx="6" cy="6" r="1" fill="currentColor"/>
            <circle cx="6" cy="18" r="1" fill="currentColor"/>
          </svg>
          <p>{{ t.noData }}</p>
          <span>{{ t.startServerToBegin }}</span>
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
              :disabled="!tabState.isConnected || tabState.autoSendEnabled || clientCount === 0"
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
          <button class="btn-send" :disabled="!tabState.isConnected || !tabState.inputMessage || tabState.autoSendEnabled || clientCount === 0" @click="sendMessage">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="22" y1="2" x2="11" y2="13"/>
              <polygon points="22,2 15,22 11,13 2,9 22,2"/>
            </svg>
            {{ t.send }}
          </button>
          <button
            v-if="!tabState.autoSendEnabled"
            class="btn-auto-send"
            :disabled="!tabState.isConnected || !tabState.inputMessage || clientCount === 0"
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
.tcp-server-tab {
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
  color: var(--text-primary);
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.card-header svg {
  color: #8b5cf6;
  width: 14px;
  height: 14px;
}

/* Config fields */
.config-fields {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.field-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.field-row label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-secondary);
}

.field-row-inline {
  display: flex;
  align-items: center;
  gap: 4px;
}

.bind-dropdown {
  flex: 2;
  min-width: 0;
}

.port-separator {
  font-size: 1rem;
  font-weight: 700;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.port-input-inline {
  flex: 1;
  min-width: 0;
  padding: 6px 4px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 0.7rem;
  font-family: var(--font-mono);
  text-align: center;
}

.port-input-inline:focus {
  outline: none;
  border-color: #8b5cf6;
  box-shadow: 0 0 0 2px rgba(139, 92, 246, 0.2);
}

.port-input-inline:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.field-input {
  flex: 1;
  padding: 8px 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 0.8rem;
  font-family: var(--font-mono);
  transition: all 0.2s ease;
}

.field-input:focus {
  outline: none;
  border-color: #8b5cf6;
  box-shadow: 0 0 0 2px rgba(139, 92, 246, 0.2);
}

.field-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.port-input {
  max-width: 100px;
  text-align: right;
}

/* Mini Dropdown */
.dropdown-item.mini {
  position: relative;
}

.dropdown-item.mini:not(.bind-dropdown) {
  flex: 1;
}

.dropdown-item.mini .dropdown-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
}

.dropdown-item.mini .dropdown-trigger:hover {
  border-color: #8b5cf6;
}

.dropdown-item.mini.open .dropdown-trigger {
  border-color: #8b5cf6;
  box-shadow: 0 0 0 2px rgba(139, 92, 246, 0.2);
}

.dropdown-item.mini.disabled {
  opacity: 0.6;
  pointer-events: none;
}

.dropdown-value-text {
  font-size: 0.75rem;
  font-family: var(--font-mono);
  color: var(--text-primary);
}

.chevron {
  transition: transform 0.2s ease;
  color: var(--text-secondary);
}

.dropdown-item.mini.open .chevron {
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
  background: rgba(139, 92, 246, 0.1);
  color: #8b5cf6;
  font-weight: 600;
}

.dropdown-option svg {
  color: #8b5cf6;
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
  background: linear-gradient(135deg, #8b5cf6, #7c3aed);
  border: none;
  border-radius: var(--radius-lg);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 14px rgb(139 92 246 / 0.35);
}

.btn-connect:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgb(139 92 246 / 0.4);
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

.server-controls {
  display: flex;
  align-items: center;
  gap: 10px;
}

.server-controls .btn-connect {
  flex: 1;
}

.server-controls .toggle-switch.mini {
  flex-shrink: 0;
  padding: 8px 10px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
}

.listening-info {
  font-size: 0.7rem;
  font-weight: 600;
  font-family: var(--font-mono);
  opacity: 0.9;
  background: rgba(255,255,255,0.2);
  padding: 2px 6px;
  border-radius: 4px;
}

/* Error Message */
.status-message.error {
  padding: 8px 12px;
  border-radius: var(--radius-md);
  font-size: 0.7rem;
  font-weight: 500;
  text-align: center;
  word-break: break-word;
  background: var(--danger-light);
  color: #dc2626;
  border: 1px solid var(--danger);
}

/* Clients Card */
.clients-card {
  border-color: #8b5cf6;
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.clients-card .card-header {
  flex-shrink: 0;
}

.client-count-badge {
  margin-left: auto;
  padding: 2px 8px;
  background: #8b5cf6;
  color: white;
  border-radius: 10px;
  font-size: 0.6rem;
  font-weight: 700;
}

.clients-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
  min-height: 0;
  overflow-y: auto;
}

.client-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 3px 8px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  border-left: 2px solid var(--success);
}

.client-addr {
  font-size: 0.7rem;
  font-weight: 600;
  font-family: var(--font-mono);
  color: var(--text-primary);
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.client-id {
  font-size: 0.6rem;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.btn-disconnect-client {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-disconnect-client:hover {
  background: var(--danger-light);
  color: var(--danger);
}

.no-clients {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 16px;
  text-align: center;
  color: var(--text-tertiary);
  font-size: 0.75rem;
}

.send-target {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--border-color);
}

.send-target label {
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--text-secondary);
}

/* Display Card */
.display-card {
  margin-top: auto;
}

.display-card .card-header svg {
  color: var(--accent-primary);
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

/* Display + Echo Row */
.display-echo-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.display-toggle.compact {
  flex: 1;
  margin-bottom: 0;
}

.toggle-switch.mini {
  flex-shrink: 0;
}

.toggle-switch.mini .toggle-label {
  font-size: 0.7rem;
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

.auto-send-card .card-header svg {
  color: var(--accent-primary);
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
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 6px 8px;
  background: #ea580c;
  border-radius: var(--radius-sm);
}

.send-count {
  font-size: 0.65rem;
  font-weight: 700;
  color: white;
  font-family: var(--font-mono);
  line-height: 1;
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
  color: #8b5cf6;
}

.terminal-stats {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-left: auto;
  margin-right: 6px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 1px;
  padding: 1px 4px;
  border-radius: 2px;
  font-size: 0.55rem;
  font-weight: 600;
  font-family: var(--font-mono);
}

.stat-item.tx {
  background: #ea580c;
  color: white;
}

.stat-item.rx {
  background: #8b5cf6;
  color: white;
}

.stat-arrow {
  font-size: 0.5rem;
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
  gap: 6px;
  padding: 3px 8px;
  margin-bottom: 4px;
  background: var(--bg-secondary);
  border-radius: 2px;
  border-left: 2px solid transparent;
}

.terminal-line.tx {
  border-left-color: #ea580c;
}

.terminal-line.rx {
  border-left-color: #8b5cf6;
}

.terminal-line .timestamp {
  color: var(--text-tertiary);
  font-size: 0.6rem;
  min-width: 58px;
  flex-shrink: 0;
}

.direction-badge {
  padding: 0px 4px;
  border-radius: 2px;
  font-size: 0.5rem;
  font-weight: 700;
  min-width: 20px;
  text-align: center;
}

.direction-badge.tx {
  background: #ea580c;
  color: white;
}

.direction-badge.rx {
  background: #8b5cf6;
  color: white;
}

.client-badge {
  padding: 0px 4px;
  border-radius: 2px;
  font-size: 0.5rem;
  font-weight: 600;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
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
  color: #8b5cf6;
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
  box-shadow: 0 0 0 2px rgba(139, 92, 246, 0.2);
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
  background: linear-gradient(135deg, #8b5cf6, #7c3aed);
  border: none;
  border-radius: var(--radius-md);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgb(139 92 246 / 0.3);
}

.btn-send:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgb(139 92 246 / 0.4);
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
