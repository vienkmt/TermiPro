<script setup>
import { ref, watch, inject, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  tabId: { type: String, required: true },
  tabState: { type: Object, required: true },
});

const emit = defineEmits(['connect', 'disconnect']);
const t = inject('t');

// Constants
const MAX_TERMINAL_ENTRIES = 500;
const MAX_VISIBLE_MESSAGES = 100;
let messageIdCounter = 0;
const TOPIC_COLORS = [
  '#ec4899', '#0ea5e9', '#10b981', '#f59e0b', '#ef4444',
  '#8b5cf6', '#14b8a6', '#f97316',
];

// Local refs
const terminalRef = ref(null);
const showConnectionPanel = ref(true);
const openDropdown = ref(null);
const messageFilter = ref('all'); // 'all', 'rx', 'tx'

// Dropdown options
const protocolOptions = [
  { value: 'tcp', label: 'TCP (1883)' },
  { value: 'tls', label: 'TLS (8883)' },
  { value: 'ws', label: 'WebSocket' },
  { value: 'wss', label: 'WS Secure' },
];
const qosOptions = [
  { value: 0, shortDesc: 'At most once', description: 'Fire and forget - no guarantee of delivery' },
  { value: 1, shortDesc: 'At least once', description: 'Guaranteed delivery, may have duplicates' },
  { value: 2, shortDesc: 'Exactly once', description: 'Guaranteed exactly one delivery' },
];
const qosDescriptions = {
  0: 'QoS 0: At most once (fire & forget)',
  1: 'QoS 1: At least once (guaranteed)',
  2: 'QoS 2: Exactly once (highest reliability)',
};
const formatOptions = [
  { value: 'text', label: 'Plaintext' },
  { value: 'json', label: 'JSON' },
  { value: 'hex', label: 'Hex' },
  { value: 'base64', label: 'Base64' },
];

// Dropdown handlers
function toggleDropdown(name, event) {
  if (event) event.stopPropagation();
  openDropdown.value = openDropdown.value === name ? null : name;
}

function selectOption(name, value) {
  switch (name) {
    case 'protocol': props.tabState.protocol = value; break;
    case 'subscribeQos': props.tabState.newSubscribeQos = value; break;
    case 'publishQos': props.tabState.publishQos = value; break;
    case 'publishFormat': props.tabState.publishFormat = value; break;
    case 'displayFormat': props.tabState.displayMode = value; break;
  }
  openDropdown.value = null;
}

function closeDropdowns() {
  openDropdown.value = null;
}

// TextDecoder for UTF-8
const textDecoder = new TextDecoder('utf-8', { fatal: false });

// Get color for topic
function getTopicColor(topic) {
  const sub = props.tabState.subscriptions.find(s => s.topic === topic);
  if (sub) return sub.color;
  let hash = 0;
  for (let i = 0; i < topic.length; i++) {
    hash = topic.charCodeAt(i) + ((hash << 5) - hash);
  }
  return TOPIC_COLORS[Math.abs(hash) % TOPIC_COLORS.length];
}

// Filtered messages based on filter
const filteredMessages = computed(() => {
  if (messageFilter.value === 'all') return props.tabState.terminalData;
  return props.tabState.terminalData.filter(m => m.type === messageFilter.value);
});

// Visible messages for performance - only render last N messages
const visibleMessages = computed(() => {
  const messages = filteredMessages.value;
  if (messages.length <= MAX_VISIBLE_MESSAGES) return messages;
  return messages.slice(-MAX_VISIBLE_MESSAGES);
});

// Convert payload to bytes
function payloadToBytes(payload) {
  if (Array.isArray(payload)) {
    return new Uint8Array(payload);
  }
  return new TextEncoder().encode(payload);
}

// Prepare payload for publishing based on format
function preparePayload(rawPayload, format) {
  let payload = rawPayload;
  let isHex = false;
  let warning = null;

  switch (format) {
    case 'hex':
      isHex = true;
      break;
    case 'base64':
      try {
        const decoded = atob(payload.replace(/\s/g, ''));
        payload = Array.from(decoded, c => c.charCodeAt(0).toString(16).padStart(2, '0')).join(' ');
        isHex = true;
      } catch (e) {
        warning = `Invalid Base64: ${e.message}`;
      }
      break;
    case 'json':
      try {
        JSON.parse(payload);
      } catch (e) {
        warning = `Invalid JSON: ${e.message}`;
      }
      break;
  }

  return { payload, isHex, warning };
}

// Check if entry has format warning (e.g., invalid JSON when display mode is JSON)
function hasFormatWarning(entry) {
  if (entry.isWarning) return false; // Already a warning entry
  if (props.tabState.displayMode !== 'json') return false;

  const payload = entry.payload;
  const bytes = payloadToBytes(payload);
  const text = textDecoder.decode(bytes);

  try {
    JSON.parse(text);
    return false;
  } catch {
    return true;
  }
}

// Format payload for display
function formatPayload(entry) {
  const payload = entry.payload;
  const bytes = payloadToBytes(payload);
  const mode = props.tabState.displayMode;

  switch (mode) {
    case 'hex':
      return Array.from(bytes)
        .map(b => b.toString(16).padStart(2, '0').toUpperCase())
        .join(' ');

    case 'base64':
      return btoa(String.fromCharCode(...bytes));

    case 'json': {
      const text = textDecoder.decode(bytes);
      try {
        const obj = JSON.parse(text);
        return JSON.stringify(obj, null, 2);
      } catch {
        // Just show raw text - warning styling indicates invalid format
        return text;
      }
    }

    case 'text':
    default:
      return textDecoder.decode(bytes);
  }
}

// Helper to add terminal entry with limit
function addTerminalEntry(entry) {
  if (props.tabState.terminalData.length >= MAX_TERMINAL_ENTRIES) {
    const removed = props.tabState.terminalData.shift();
    if (removed.type === 'tx') props.tabState.txCount--;
    else props.tabState.rxCount--;
  }

  // Add unique ID for Vue key binding
  entry.id = ++messageIdCounter;
  props.tabState.terminalData.push(entry);
  if (entry.type === 'tx') props.tabState.txCount++;
  else props.tabState.rxCount++;

  throttledScrollToBottom();
}

// Smooth scroll management
let scrollAnimationId = null;

function smoothScrollToBottom() {
  if (!terminalRef.value || !props.tabState.autoScroll) return;

  const element = terminalRef.value;
  const target = element.scrollHeight - element.clientHeight;
  const distance = target - element.scrollTop;

  // Already at bottom or very close
  if (distance <= 2) return;

  // Cancel ongoing animation
  if (scrollAnimationId) {
    cancelAnimationFrame(scrollAnimationId);
    scrollAnimationId = null;
  }

  // Longer duration for smoother feel (250-400ms)
  const duration = Math.min(400, Math.max(250, distance * 0.8));
  const startPos = element.scrollTop;
  const startTime = performance.now();

  function animate() {
    const elapsed = performance.now() - startTime;
    const progress = Math.min(elapsed / duration, 1);

    // easeOutQuart - smooth and gentle deceleration
    const eased = 1 - Math.pow(1 - progress, 4);

    element.scrollTop = startPos + distance * eased;

    if (progress < 1) {
      scrollAnimationId = requestAnimationFrame(animate);
    } else {
      scrollAnimationId = null;
    }
  }

  scrollAnimationId = requestAnimationFrame(animate);
}

function throttledScrollToBottom() {
  // Use next frame to batch updates
  if (scrollAnimationId) return; // Animation already running
  requestAnimationFrame(smoothScrollToBottom);
}

// Connection
async function toggleConnection() {
  if (props.tabState.isConnected) {
    emit('disconnect', props.tabId);
  } else {
    emit('connect', props.tabId);
  }
}

// Subscribe
async function addSubscription() {
  const topic = props.tabState.newSubscribeTopic.trim();
  if (!topic || !props.tabState.isConnected) return;

  try {
    await invoke('mqtt_subscribe', {
      connectionId: props.tabState.connectionId,
      topic,
      qos: props.tabState.newSubscribeQos,
    });

    const color = TOPIC_COLORS[props.tabState.subscriptions.length % TOPIC_COLORS.length];
    props.tabState.subscriptions.push({
      topic,
      qos: props.tabState.newSubscribeQos,
      color,
    });
    props.tabState.newSubscribeTopic = '';
  } catch (error) {
    console.error('Subscribe error:', error);
    alert('Subscribe error: ' + error);
  }
}

async function removeSubscription(index) {
  const sub = props.tabState.subscriptions[index];
  if (!sub || !props.tabState.isConnected) return;

  try {
    await invoke('mqtt_unsubscribe', {
      connectionId: props.tabState.connectionId,
      topic: sub.topic,
    });
    props.tabState.subscriptions.splice(index, 1);
  } catch (error) {
    console.error('Unsubscribe error:', error);
  }
}

// Publish
async function publishMessage() {
  const topic = props.tabState.publishTopic.trim();
  if (!topic || !props.tabState.isConnected) return;

  const { payload, isHex, warning } = preparePayload(props.tabState.publishPayload, props.tabState.publishFormat);

  try {
    await invoke('mqtt_publish', {
      connectionId: props.tabState.connectionId,
      topic,
      payload,
      qos: props.tabState.publishQos,
      retain: props.tabState.publishRetain,
      isHex,
    });

    // Add TX entry
    addTerminalEntry({
      type: 'tx',
      topic,
      payload: props.tabState.publishPayload,
      qos: props.tabState.publishQos,
      retain: props.tabState.publishRetain,
      timestamp: new Date().toLocaleTimeString(),
    });
  } catch (error) {
    console.error('Publish error:', error);
    addTerminalEntry({
      type: 'tx',
      topic,
      payload: props.tabState.publishPayload,
      qos: props.tabState.publishQos,
      retain: props.tabState.publishRetain,
      timestamp: new Date().toLocaleTimeString(),
      warning: `Publish failed: ${error}`,
    });
  }
}

function handleKeyDown(event) {
  if (event.key === 'Enter' && event.ctrlKey) {
    event.preventDefault();
    publishMessage();
  }
}

// Auto-publish
function toggleAutoPublish() {
  if (props.tabState.autoPublishEnabled) {
    stopAutoPublish();
  } else {
    startAutoPublish();
  }
}

function startAutoPublish() {
  if (!props.tabState.isConnected || !props.tabState.publishTopic) return;

  props.tabState.autoPublishEnabled = true;
  props.tabState.autoPublishCount = 0;

  props.tabState.autoPublishTimer = setInterval(async () => {
    if (!props.tabState.isConnected) {
      stopAutoPublish();
      return;
    }

    try {
      const { payload, isHex } = preparePayload(props.tabState.publishPayload, props.tabState.publishFormat);

      await invoke('mqtt_publish', {
        connectionId: props.tabState.connectionId,
        topic: props.tabState.publishTopic,
        payload,
        qos: props.tabState.publishQos,
        retain: props.tabState.publishRetain,
        isHex,
      });

      // Add TX entry
      addTerminalEntry({
        type: 'tx',
        topic: props.tabState.publishTopic,
        payload: props.tabState.publishPayload,
        qos: props.tabState.publishQos,
        retain: props.tabState.publishRetain,
        timestamp: new Date().toLocaleTimeString(),
      });

      props.tabState.autoPublishCount++;
    } catch (error) {
      console.error('Auto-publish error:', error);
      stopAutoPublish();
    }
  }, props.tabState.autoPublishInterval);
}

function stopAutoPublish() {
  if (props.tabState.autoPublishTimer) {
    clearInterval(props.tabState.autoPublishTimer);
    props.tabState.autoPublishTimer = null;
  }
  props.tabState.autoPublishEnabled = false;
}

// Clear terminal
function clearTerminal() {
  props.tabState.terminalData = [];
  props.tabState.txCount = 0;
  props.tabState.rxCount = 0;
  props.tabState.autoPublishCount = 0;
}

// Protocol change - auto update port
watch(() => props.tabState.protocol, (newProtocol) => {
  if (props.tabState.isConnected) return;
  switch (newProtocol) {
    case 'tcp': props.tabState.brokerPort = 1883; break;
    case 'tls': props.tabState.brokerPort = 8883; break;
    case 'ws': props.tabState.brokerPort = 8080; break;
    case 'wss': props.tabState.brokerPort = 443; break;
  }
});

// Auto-collapse connection panel on successful connect
watch(() => props.tabState.isConnected, (isConnected) => {
  if (isConnected) {
    showConnectionPanel.value = false;
  }
});

// Auto-scroll when new messages arrive
watch(
  () => filteredMessages.value.length,
  () => {
    if (props.tabState.autoScroll) {
      throttledScrollToBottom();
    }
  }
);

// Cleanup on unmount
onUnmounted(() => {
  stopAutoPublish();
  if (scrollAnimationId) cancelAnimationFrame(scrollAnimationId);
});

// Expose for parent to call
defineExpose({ addTerminalEntry });
</script>

<template>
  <div class="mqtt-tab" @click="closeDropdowns">
    <!-- Left Sidebar - Subscriptions -->
    <div class="subscriptions-sidebar">
      <!-- Connection Header -->
      <div class="sidebar-header">
        <div class="connection-info" @click="showConnectionPanel = !showConnectionPanel">
          <div class="connection-status" :class="tabState.isConnected ? 'connected' : 'disconnected'"></div>
          <div class="connection-details">
            <span class="connection-name" v-if="tabState.name?.trim()">{{ tabState.name.trim() }}</span>
            <span class="broker-name">{{ tabState.brokerHost || t.mqttNotConnected }}</span>
          </div>
          <svg class="toggle-icon" :class="{ rotated: showConnectionPanel }" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M6 9l6 6 6-6"/>
          </svg>
        </div>
      </div>

      <!-- Connection Panel (collapsible) -->
      <form v-if="showConnectionPanel" class="connection-panel" autocomplete="off" @submit.prevent>
        <div class="form-group">
          <label>{{ t.mqttName }} <span class="required">*</span></label>
          <input v-model="tabState.name" :disabled="tabState.isConnected" :placeholder="t.mqttNamePlaceholder" autocomplete="off" data-form-type="other" spellcheck="false" />
        </div>
        <div class="form-group">
          <label>{{ t.mqttHost }}</label>
          <input v-model="tabState.brokerHost" :disabled="tabState.isConnected" :placeholder="t.mqttHostPlaceholder" autocomplete="off" data-form-type="other" spellcheck="false" />
        </div>
        <div class="form-row">
          <div class="form-group half">
            <label>{{ t.mqttPort }}</label>
            <input type="number" v-model.number="tabState.brokerPort" :disabled="tabState.isConnected" autocomplete="off" />
          </div>
          <div class="form-group half">
            <label>{{ t.mqttProtocol }}</label>
            <div class="mini-dropdown" :class="{ open: openDropdown === 'protocol', disabled: tabState.isConnected }" @click.stop="!tabState.isConnected && toggleDropdown('protocol', $event)">
              <div class="mini-dropdown-trigger">
                <span>{{ protocolOptions.find(p => p.value === tabState.protocol)?.label.split(' ')[0] || 'TCP' }}</span>
                <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
              </div>
              <div v-if="openDropdown === 'protocol'" class="mini-dropdown-menu" @click.stop>
                <div v-for="opt in protocolOptions" :key="opt.value" class="mini-dropdown-option" :class="{ selected: tabState.protocol === opt.value }" @click="selectOption('protocol', opt.value)">
                  {{ opt.label }}
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="form-group">
          <label>{{ t.mqttClientId }}</label>
          <input v-model="tabState.clientId" :disabled="tabState.isConnected" autocomplete="off" data-form-type="other" spellcheck="false" />
        </div>
        <div class="form-row">
          <div class="form-group half">
            <label>{{ t.mqttUsername }}</label>
            <input v-model="tabState.username" :disabled="tabState.isConnected" :placeholder="t.mqttOptional" autocomplete="off" data-form-type="other" spellcheck="false" />
          </div>
          <div class="form-group half">
            <label>{{ t.mqttPassword }}</label>
            <input type="password" v-model="tabState.password" :disabled="tabState.isConnected" :placeholder="t.mqttOptional" autocomplete="new-password" data-form-type="other" />
          </div>
        </div>
        <button type="button" class="connect-btn" :class="{ connected: tabState.isConnected }" :disabled="!tabState.isConnected && !tabState.name?.trim()" @click="toggleConnection">
          {{ tabState.isConnected ? t.mqttDisconnect : t.mqttConnect }}
        </button>
        <div v-if="tabState.statusMessage" class="status-msg" :class="tabState.connectionStatus">
          {{ tabState.statusMessage }}
        </div>
      </form>

      <!-- New Subscription -->
      <div class="new-subscription">
        <div class="subscribe-input-row">
          <input
            v-model="tabState.newSubscribeTopic"
            :placeholder="t.mqttTopicPlaceholder"
            :disabled="!tabState.isConnected"
            @keydown.enter="addSubscription"
            autocomplete="off"
            data-form-type="other"
            spellcheck="false"
          />
          <div class="qos-selector" :class="{ disabled: !tabState.isConnected }" @click.stop="tabState.isConnected && toggleDropdown('subscribeQos', $event)" :title="qosDescriptions[tabState.newSubscribeQos]">
            <span class="qos-value">QoS {{ tabState.newSubscribeQos }}</span>
            <div v-if="openDropdown === 'subscribeQos'" class="qos-dropdown" @click.stop>
              <div v-for="opt in qosOptions" :key="opt.value" class="qos-option" :class="{ selected: tabState.newSubscribeQos === opt.value }" @click="selectOption('subscribeQos', opt.value)" :title="opt.description">
                <span class="qos-label">Q{{ opt.value }}</span>
                <span class="qos-separator">-</span>
                <span class="qos-full">QoS {{ opt.value }}</span>
                <span class="qos-desc">{{ opt.shortDesc }}</span>
              </div>
            </div>
          </div>
          <button class="add-sub-btn" @click="addSubscription" :disabled="!tabState.isConnected || !tabState.newSubscribeTopic" :title="t.mqttSubscribe">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
              <line x1="12" y1="5" x2="12" y2="19"/>
              <line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- Subscriptions List -->
      <div class="subscriptions-list">
        <div v-for="(sub, idx) in tabState.subscriptions" :key="idx" class="subscription-item">
          <span class="color-bar" :style="{ background: sub.color }"></span>
          <div class="sub-content">
            <span class="sub-topic">{{ sub.topic }}</span>
            <span class="sub-qos">QoS {{ sub.qos }}</span>
          </div>
          <button class="remove-sub-btn" @click="removeSubscription(idx)">×</button>
        </div>
        <div v-if="tabState.subscriptions.length === 0" class="empty-subscriptions">
          {{ t.mqttNoSubscriptions }}
        </div>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="main-content">
      <!-- Top Toolbar -->
      <div class="top-toolbar">
        <div class="left-section">
          <!-- Display Format Dropdown -->
          <div class="toolbar-dropdown" :class="{ open: openDropdown === 'displayFormat' }" @click.stop="toggleDropdown('displayFormat', $event)">
            <span>{{ formatOptions.find(f => f.value === tabState.displayMode)?.label || 'Plaintext' }}</span>
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
            <div v-if="openDropdown === 'displayFormat'" class="toolbar-dropdown-menu" @click.stop>
              <div v-for="opt in formatOptions" :key="opt.value" class="toolbar-dropdown-option" :class="{ selected: tabState.displayMode === opt.value }" @click="selectOption('displayFormat', opt.value)">
                {{ opt.label }}
              </div>
            </div>
          </div>
        </div>

        <div class="center-section">
          <!-- Message Filter Tabs -->
          <div class="filter-tabs">
            <button :class="{ active: messageFilter === 'all' }" @click="messageFilter = 'all'">{{ t.mqttAll }}</button>
            <button :class="{ active: messageFilter === 'rx' }" @click="messageFilter = 'rx'">{{ t.mqttReceived }}</button>
            <button :class="{ active: messageFilter === 'tx' }" @click="messageFilter = 'tx'">{{ t.mqttPublished }}</button>
          </div>
        </div>

        <div class="right-section">
          <div class="stats-compact">
            <span class="stat-item rx">↓{{ tabState.rxCount }}</span>
            <span class="stat-item tx">↑{{ tabState.txCount }}</span>
          </div>
          <button class="clear-btn" @click="clearTerminal">{{ t.clear }}</button>
        </div>
      </div>

      <!-- Messages Area -->
      <div class="messages-area" ref="terminalRef">
        <div v-for="(entry, idx) in visibleMessages" :key="entry.id || idx" class="message-bubble" :class="[entry.type, { warning: entry.warning || hasFormatWarning(entry) }]">
          <div class="message-header">
            <span class="message-type-badge" :class="[entry.type, { warning: entry.warning || hasFormatWarning(entry) }]">{{ entry.type === 'tx' ? t.mqttPublished : t.mqttReceived }}{{ (entry.warning || hasFormatWarning(entry)) ? ' ⚠' : '' }}</span>
            <span class="message-topic" :style="{ color: entry.color || getTopicColor(entry.topic) }">{{ entry.topic }}</span>
            <span class="message-qos">QoS {{ entry.qos }}</span>
            <span v-if="entry.retain" class="message-retain">{{ t.mqttRetain }}</span>
            <span class="message-time">{{ entry.timestamp }}</span>
            <span v-if="hasFormatWarning(entry)" class="format-error-badge">{{ t.mqttInvalidJson }}</span>
          </div>
          <div v-if="entry.warning" class="message-warning">⚠ {{ entry.warning }}</div>
          <pre class="message-payload">{{ formatPayload(entry) }}</pre>
        </div>
        <div v-if="visibleMessages.length === 0" class="empty-messages">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
          </svg>
          <p>{{ t.mqttNoMessages }}</p>
          <span>{{ t.mqttNoMessagesHint }}</span>
        </div>
      </div>

      <!-- Bottom Publish Bar -->
      <div class="publish-bar">
        <!-- Combined Topic + Options Row -->
        <div class="publish-header">
          <input v-model="tabState.publishTopic" :placeholder="t.mqttTopicPath" :disabled="!tabState.isConnected" class="topic-input" autocomplete="off" data-form-type="other" spellcheck="false" />

          <!-- Format Dropdown -->
          <div class="toolbar-dropdown compact" :class="{ open: openDropdown === 'publishFormat' }" @click.stop="toggleDropdown('publishFormat', $event)">
            <span>{{ formatOptions.find(f => f.value === tabState.publishFormat)?.label || 'Text' }}</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
            <div v-if="openDropdown === 'publishFormat'" class="toolbar-dropdown-menu" @click.stop>
              <div v-for="opt in formatOptions" :key="opt.value" class="toolbar-dropdown-option" :class="{ selected: tabState.publishFormat === opt.value }" @click="selectOption('publishFormat', opt.value)">
                {{ opt.label }}
              </div>
            </div>
          </div>

          <!-- QoS Dropdown -->
          <div class="toolbar-dropdown compact" :class="{ open: openDropdown === 'publishQos' }" @click.stop="toggleDropdown('publishQos', $event)" :title="qosDescriptions[tabState.publishQos]">
            <span>QoS {{ tabState.publishQos }}</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
            <div v-if="openDropdown === 'publishQos'" class="toolbar-dropdown-menu wide" @click.stop>
              <div v-for="opt in qosOptions" :key="opt.value" class="toolbar-dropdown-option" :class="{ selected: tabState.publishQos === opt.value }" @click="selectOption('publishQos', opt.value)" :title="opt.description">
                <span class="qos-num">Q{{ opt.value }}</span>
                <span class="qos-text">{{ opt.shortDesc }}</span>
              </div>
            </div>
          </div>

          <!-- Retain Toggle -->
          <label class="retain-toggle" :class="{ active: tabState.publishRetain }" :title="t.mqttRetain">
            <input type="checkbox" v-model="tabState.publishRetain" />
            <span>{{ t.mqttRetain }}</span>
          </label>

          <!-- Auto-publish interval -->
          <div class="auto-interval">
            <input type="number" v-model.number="tabState.autoPublishInterval" min="100" max="300000" step="100" title="Auto-publish interval (ms)" />
            <span>ms</span>
          </div>
        </div>

        <!-- Message Input with Actions -->
        <div class="message-input-wrapper">
          <textarea
            v-model="tabState.publishPayload"
            :placeholder="tabState.publishFormat === 'hex' ? '48 65 6C 6C 6F' : tabState.publishFormat === 'base64' ? 'SGVsbG8gV29ybGQ=' : '{ msg: hello }'"
            :disabled="!tabState.isConnected"
            @keydown="handleKeyDown"
            autocomplete="off"
            spellcheck="false"
            class="message-textarea"
          ></textarea>
          <div class="publish-actions">
            <button class="auto-btn" :class="{ active: tabState.autoPublishEnabled }" @click="toggleAutoPublish" :disabled="!tabState.isConnected || !tabState.publishTopic" :title="tabState.autoPublishEnabled ? t.mqttStop : t.mqttStart">
              <template v-if="tabState.autoPublishEnabled">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="6" width="12" height="12" rx="2"/></svg>
                <span>{{ t.mqttStop }} ({{ tabState.autoPublishCount }})</span>
              </template>
              <template v-else>
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/></svg>
                <span>{{ t.auto }}</span>
              </template>
            </button>
            <button class="send-btn" @click="publishMessage" :disabled="!tabState.isConnected || !tabState.publishTopic" :title="t.mqttPublish + ' (Ctrl+Enter)'">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 2L11 13"/>
                <path d="M22 2L15 22l-4-9-9-4z"/>
              </svg>
              <span>{{ t.mqttSend }}</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mqtt-tab {
  display: flex;
  width: 100%;
  height: 100%;
  background: #f8fafc;
  font-family: 'Plus Jakarta Sans', sans-serif;
}

/* ===== Left Sidebar ===== */
.subscriptions-sidebar {
  width: 360px;
  min-width: 360px;
  background: white;
  border-right: 1px solid #e2e8f0;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 12px 16px;
  border-bottom: 1px solid #e2e8f0;
}

.connection-info {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  padding: 8px 12px;
  border-radius: 8px;
  background: #f8fafc;
  transition: background 0.15s;
}

.connection-info:hover {
  background: #f1f5f9;
}

.connection-status {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.connection-status.connected {
  background: #10b981;
  box-shadow: 0 0 8px rgba(16, 185, 129, 0.5);
}

.connection-status.disconnected {
  background: #94a3b8;
}

.connection-details {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.connection-name {
  font-size: 0.85rem;
  font-weight: 600;
  color: #0ea5e9;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.broker-name {
  font-size: 0.75rem;
  font-weight: 500;
  color: #64748b;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.toggle-icon {
  color: #94a3b8;
  transition: transform 0.2s;
}

.toggle-icon.rotated {
  transform: rotate(180deg);
}

/* Connection Panel */
.connection-panel {
  padding: 12px 16px;
  border-bottom: 1px solid #e2e8f0;
  background: #fafbfc;
}

.form-group {
  margin-bottom: 10px;
}

.form-group label {
  display: block;
  font-size: 0.7rem;
  font-weight: 500;
  color: #64748b;
  margin-bottom: 4px;
}

.form-group label .required {
  color: #ef4444;
  margin-left: 2px;
}

.form-group input {
  width: 100%;
  padding: 8px 10px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 0.8rem;
  background: white;
  transition: all 0.15s;
}

.form-group input:focus {
  outline: none;
  border-color: #0ea5e9;
}

.form-group input:disabled {
  background: #f1f5f9;
  color: #94a3b8;
}

.form-row {
  display: flex;
  gap: 10px;
}

.form-group.half {
  flex: 1;
}

.connect-btn {
  width: 100%;
  padding: 10px;
  background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
}

.connect-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(14, 165, 233, 0.3);
}

.connect-btn.connected {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
}

.connect-btn:disabled {
  background: #cbd5e1;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.status-msg {
  margin-top: 8px;
  padding: 6px 10px;
  border-radius: 6px;
  font-size: 0.75rem;
  text-align: center;
}

.status-msg.connecting {
  background: #fef3c7;
  color: #92400e;
}

.status-msg.connected {
  background: #d1fae5;
  color: #065f46;
}

.status-msg.error {
  background: #fee2e2;
  color: #991b1b;
}

/* Mini Dropdown */
.mini-dropdown {
  position: relative;
}

.mini-dropdown.disabled {
  opacity: 0.6;
  pointer-events: none;
}

.mini-dropdown-trigger {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px 10px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 0.8rem;
  cursor: pointer;
  transition: border-color 0.15s;
}

.mini-dropdown-trigger:hover {
  border-color: #0ea5e9;
}

.mini-dropdown-trigger.compact {
  padding: 6px 8px;
  font-size: 0.75rem;
}

.mini-dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  min-width: 100px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  z-index: 100;
  margin-top: 4px;
  overflow: hidden;
}

.mini-dropdown-option {
  padding: 8px 12px;
  font-size: 0.8rem;
  cursor: pointer;
  transition: background 0.1s;
}

.mini-dropdown-option:hover {
  background: #f1f5f9;
}

.mini-dropdown-option.selected {
  background: #0ea5e9;
  color: white;
}

/* New Subscription */
.new-subscription {
  padding: 10px 16px;
  border-bottom: 1px solid #e2e8f0;
}

.subscribe-input-row {
  display: flex;
  gap: 6px;
  align-items: center;
}

.subscribe-input-row input {
  flex: 1;
  min-width: 0;
  padding: 5px 8px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 0.7rem;
  font-family: 'JetBrains Mono', monospace;
  height: 30px;
}

.subscribe-input-row input:focus {
  outline: none;
  border-color: #0ea5e9;
}

.subscribe-input-row input:disabled {
  background: #f1f5f9;
  color: #94a3b8;
}

/* QoS Selector */
.qos-selector {
  position: relative;
  flex-shrink: 0;
  height: 30px;
  padding: 0 10px;
  background: #f1f5f9;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.15s;
}

.qos-selector:hover:not(.disabled) {
  border-color: #0ea5e9;
  background: #f8fafc;
}

.qos-selector.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.qos-value {
  font-size: 0.7rem;
  font-weight: 600;
  color: #475569;
  white-space: nowrap;
}

.qos-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  min-width: 220px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0,0,0,0.15);
  z-index: 100;
  margin-top: 4px;
  overflow: hidden;
}

.qos-option {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 12px;
  cursor: pointer;
  transition: background 0.1s;
}

.qos-option:hover {
  background: #f1f5f9;
}

.qos-option.selected {
  background: #0ea5e9;
}

.qos-option.selected .qos-label,
.qos-option.selected .qos-separator,
.qos-option.selected .qos-full,
.qos-option.selected .qos-desc {
  color: white;
}

.qos-label {
  font-size: 0.8rem;
  font-weight: 700;
  color: #0ea5e9;
  font-family: 'JetBrains Mono', monospace;
  flex-shrink: 0;
}

.qos-separator {
  font-size: 0.75rem;
  color: #94a3b8;
}

.qos-full {
  font-size: 0.75rem;
  font-weight: 500;
  color: #334155;
  flex-shrink: 0;
}

.qos-desc {
  font-size: 0.7rem;
  color: #64748b;
  white-space: nowrap;
  margin-left: auto;
}

.add-sub-btn {
  flex-shrink: 0;
  width: 30px;
  height: 30px;
  padding: 0;
  background: #0ea5e9;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.add-sub-btn:hover:not(:disabled) {
  background: #0284c7;
  transform: scale(1.05);
}

.add-sub-btn:disabled {
  background: #cbd5e1;
  cursor: not-allowed;
}

/* Subscriptions List */
.subscriptions-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.subscription-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  background: #f8fafc;
  border-radius: 8px;
  margin-bottom: 6px;
  transition: background 0.15s;
}

.subscription-item:hover {
  background: #f1f5f9;
}

.color-bar {
  width: 4px;
  height: 32px;
  border-radius: 2px;
  flex-shrink: 0;
}

.sub-content {
  flex: 1;
  min-width: 0;
}

.sub-topic {
  display: block;
  font-size: 0.8rem;
  font-weight: 500;
  color: #334155;
  font-family: 'JetBrains Mono', monospace;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sub-qos {
  font-size: 0.7rem;
  color: #94a3b8;
}

.remove-sub-btn {
  background: none;
  border: none;
  color: #94a3b8;
  font-size: 1.2rem;
  cursor: pointer;
  padding: 0 4px;
  opacity: 0;
  transition: all 0.15s;
}

.subscription-item:hover .remove-sub-btn {
  opacity: 1;
}

.remove-sub-btn:hover {
  color: #ef4444;
}

.empty-subscriptions {
  text-align: center;
  padding: 24px;
  color: #94a3b8;
  font-size: 0.8rem;
}

/* ===== Main Content ===== */
.main-content {
  flex: 1 1 0;
  display: flex;
  flex-direction: column;
  min-width: 0;
  overflow: hidden;
}

/* Top Toolbar */
.top-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px;
  background: white;
  border-bottom: 1px solid #e2e8f0;
  gap: 16px;
}

.left-section, .right-section {
  display: flex;
  align-items: center;
  gap: 10px;
}

.center-section {
  flex: 1;
  display: flex;
  justify-content: center;
}

/* Toolbar Dropdown */
.toolbar-dropdown {
  position: relative;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  background: #f1f5f9;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 0.8rem;
  cursor: pointer;
  transition: all 0.15s;
}

.toolbar-dropdown:hover {
  border-color: #0ea5e9;
}

.toolbar-dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  min-width: 120px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  z-index: 100;
  margin-top: 4px;
  overflow: hidden;
}

.toolbar-dropdown-option {
  padding: 8px 12px;
  font-size: 0.8rem;
  cursor: pointer;
  transition: background 0.1s;
}

.toolbar-dropdown-option:hover {
  background: #f1f5f9;
}

.toolbar-dropdown-option.selected {
  background: #0ea5e9;
  color: white;
}

/* Publish bar dropdowns open upward */
.publish-header .toolbar-dropdown-menu {
  top: auto;
  bottom: 100%;
  margin-top: 0;
  margin-bottom: 4px;
}

.toolbar-dropdown-menu.wide {
  min-width: 160px;
}

.toolbar-dropdown-option .qos-num {
  font-weight: 600;
  color: #0ea5e9;
  margin-right: 6px;
}

.toolbar-dropdown-option .qos-text {
  color: #64748b;
  font-size: 0.7rem;
}

.toolbar-dropdown-option.selected .qos-num,
.toolbar-dropdown-option.selected .qos-text {
  color: white;
}

/* Filter Tabs */
.filter-tabs {
  display: flex;
  background: #f1f5f9;
  border-radius: 8px;
  padding: 4px;
}

.filter-tabs button {
  padding: 6px 16px;
  background: transparent;
  border: none;
  border-radius: 6px;
  font-size: 0.8rem;
  font-weight: 500;
  color: #64748b;
  cursor: pointer;
  transition: all 0.15s;
}

.filter-tabs button:hover {
  color: #334155;
}

.filter-tabs button.active {
  background: white;
  color: #0ea5e9;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.stats-compact {
  display: flex;
  gap: 8px;
}

.stat-item {
  font-size: 0.75rem;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.stat-item.rx { color: #0ea5e9; }
.stat-item.tx { color: #f59e0b; }

.clear-btn {
  padding: 6px 12px;
  background: #f1f5f9;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.15s;
}

.clear-btn:hover {
  background: #e2e8f0;
}

/* Messages Area */
.messages-area {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 4px 8px;
  background: white;
  /* GPU acceleration hints */
  will-change: scroll-position;
  contain: layout style;
  -webkit-overflow-scrolling: touch;
}

.message-bubble {
  background: #f8fafc;
  border-radius: 0;
  padding: 4px 8px;
  margin-bottom: 2px;
  border-left: 3px solid transparent;
  contain: content;
}

.message-bubble.tx {
  border-left-color: #f59e0b;
}

.message-bubble.rx {
  border-left-color: #0ea5e9;
}

.message-bubble.tx.warning {
  border-left-color: #f59e0b;
  background: #fffbeb;
}

.message-bubble.rx.warning {
  border-left-color: #ef4444;
  background: #fef2f2;
}

.message-header {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-bottom: 2px;
}

.message-type-badge {
  padding: 2px 5px;
  border-radius: 3px;
  font-size: 0.6rem;
  font-weight: 600;
  text-transform: uppercase;
  flex-shrink: 0;
}

.message-type-badge.tx {
  background: #fef3c7;
  color: #92400e;
}

.message-type-badge.rx {
  background: #dbeafe;
  color: #1e40af;
}

.message-type-badge.tx.warning {
  background: #fef3c7;
  color: #b45309;
}

.message-type-badge.rx.warning {
  background: #fee2e2;
  color: #dc2626;
}

.message-topic {
  font-size: 0.75rem;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.message-qos {
  font-size: 0.6rem;
  color: #64748b;
  padding: 1px 4px;
  background: #f1f5f9;
  border-radius: 3px;
  flex-shrink: 0;
}

.message-retain {
  font-size: 0.55rem;
  color: white;
  padding: 1px 4px;
  background: #8b5cf6;
  border-radius: 3px;
  font-weight: 600;
  flex-shrink: 0;
}

.message-time {
  margin-left: auto;
  font-size: 0.65rem;
  color: #94a3b8;
  flex-shrink: 0;
}

.format-error-badge {
  font-size: 0.55rem;
  padding: 1px 5px;
  border-radius: 3px;
  font-weight: 600;
  flex-shrink: 0;
  margin-left: 4px;
}

.message-bubble.tx .format-error-badge {
  background: #fef3c7;
  color: #b45309;
}

.message-bubble.rx .format-error-badge {
  background: #fee2e2;
  color: #dc2626;
}

.message-warning {
  font-size: 0.65rem;
  color: #dc2626;
  padding: 2px 0;
}

.message-payload {
  margin: 0;
  padding: 2px 0;
  background: transparent;
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.7rem;
  line-height: 1.3;
  color: #334155;
  white-space: pre-wrap;
  word-break: break-all;
  overflow-x: auto;
  max-height: 120px;
  overflow-y: auto;
}

.empty-messages {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #94a3b8;
  text-align: center;
}

.empty-messages svg {
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-messages p {
  margin: 0 0 4px 0;
  font-size: 1rem;
  font-weight: 500;
  color: #64748b;
}

.empty-messages span {
  font-size: 0.85rem;
}

/* Publish Bar */
.publish-bar {
  background: white;
  border-top: 1px solid #e2e8f0;
  padding: 8px 12px;
}

.publish-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 15px;
  margin-top: 5px;
}

.publish-header .topic-input {
  flex: 1;
  min-width: 100px;
  padding: 6px 10px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 0.75rem;
  font-family: 'JetBrains Mono', monospace;
  height: 30px;
}

.publish-header .topic-input:focus {
  outline: none;
  border-color: #0ea5e9;
}

.publish-header .topic-input:disabled {
  background: #f1f5f9;
  color: #94a3b8;
}

.toolbar-dropdown.compact {
  padding: 4px 8px;
  font-size: 0.7rem;
  height: 30px;
}

.retain-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 10px;
  height: 30px;
  background: #f1f5f9;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
  flex-shrink: 0;
}

.retain-toggle input {
  display: none;
}

.retain-toggle span {
  font-size: 0.7rem;
  font-weight: 500;
  color: #64748b;
}

.retain-toggle:hover {
  border-color: #0ea5e9;
}

.retain-toggle.active {
  background: #0ea5e9;
  border-color: #0ea5e9;
}

.retain-toggle.active span {
  color: white;
  font-weight: 600;
}

.auto-interval {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-shrink: 0;
}

.auto-interval input {
  width: 60px;
  height: 30px;
  padding: 4px 6px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 0.7rem;
  text-align: center;
}

.auto-interval input:focus {
  outline: none;
  border-color: #0ea5e9;
}

.auto-interval span {
  font-size: 0.65rem;
  color: #94a3b8;
}

.message-input-wrapper {
  display: flex;
  gap: 20px;
  align-items: flex-end;
  margin-bottom: 10px;
}

.message-textarea {
  flex: 1;
  min-height: 72px;
  max-height: 120px;
  padding: 8px 10px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.75rem;
  line-height: 1.5;
  resize: vertical;
  background: #f8fafc;
}

.message-textarea:focus {
  outline: none;
  border-color: #0ea5e9;
  background: white;
}

.message-textarea:disabled {
  background: #f1f5f9;
  color: #94a3b8;
}

.publish-actions {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex-shrink: 0;
}

.auto-btn,
.send-btn {
  min-width: 120px;
  height: 32px;
  padding: 0 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 5px;
  font-size: 0.75rem;
  font-weight: 600;
}

.auto-btn {
  background: #f1f5f9;
  border: 1px solid #e2e8f0;
  color: #64748b;
  margin-bottom: 0.225rem;
}

.auto-btn:hover:not(:disabled):not(.active) {
  background: #e2e8f0;
  border-color: #0ea5e9;
  transform: translateY(-1px);
}

.auto-btn.active {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  border-color: #dc2626;
  color: white;
}

.auto-btn.active:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.4);
}

.auto-btn:disabled,
.send-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.send-btn {
  background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
  color: white;
  border: none;
}

.send-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(14, 165, 233, 0.4);
}
</style>
