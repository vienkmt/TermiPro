<script setup>
import { inject, computed } from 'vue';

const props = defineProps({
  tabs: {
    type: Map,
    required: true,
  },
  tabOrder: {
    type: Array,
    required: true,
  },
  activeTabId: {
    type: String,
    default: null,
  },
  canAddTab: {
    type: Boolean,
    default: true,
  },
});

const emit = defineEmits(['selectTab', 'closeTab', 'addTab']);

const t = inject('t');

// Get connection type for tab
function getConnectionType(tabId) {
  const tab = props.tabs.get(tabId);
  return tab?.connectionType || 'serial';
}

// Get display name for tab
function getTabDisplayName(tabId) {
  const tab = props.tabs.get(tabId);
  if (!tab) return t.value.newTab;

  // Serial tab
  if (tab.connectionType === 'serial') {
    if (tab.selectedPort && tab.isConnected) {
      return tab.selectedPort.split('/').pop();
    } else if (tab.selectedPort) {
      return tab.selectedPort.split('/').pop();
    }
    return t.value.newTab;
  }

  // TCP Client tab
  if (tab.connectionType === 'tcp_client') {
    if (tab.isConnected) {
      return `${tab.host}:${tab.port}`;
    }
    return t.value.tcpClient;
  }

  // TCP Server tab
  if (tab.connectionType === 'tcp_server') {
    if (tab.isConnected) {
      return `:${tab.listenPort}`;
    }
    return t.value.tcpServer;
  }

  return t.value.newTab;
}

// Check if tab is connected
function isTabConnected(tabId) {
  const tab = props.tabs.get(tabId);
  return tab?.isConnected || false;
}
</script>

<template>
  <div class="tab-bar">
    <div class="tabs-container">
      <div
        v-for="tabId in tabOrder"
        :key="tabId"
        class="tab"
        :class="{
          active: tabId === activeTabId,
          connected: isTabConnected(tabId),
          'type-serial': getConnectionType(tabId) === 'serial',
          'type-tcp-client': getConnectionType(tabId) === 'tcp_client',
          'type-tcp-server': getConnectionType(tabId) === 'tcp_server'
        }"
        @click="emit('selectTab', tabId)"
      >
        <!-- Serial Icon -->
        <svg v-if="getConnectionType(tabId) === 'serial'" class="tab-icon" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
        <!-- TCP Client Icon -->
        <svg v-else-if="getConnectionType(tabId) === 'tcp_client'" class="tab-icon tcp-client-icon" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M5 12h14"/>
          <path d="M13 5l7 7-7 7"/>
          <rect x="2" y="8" width="6" height="8" rx="1"/>
        </svg>
        <!-- TCP Server Icon -->
        <svg v-else-if="getConnectionType(tabId) === 'tcp_server'" class="tab-icon tcp-server-icon" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="3" width="20" height="6" rx="1"/>
          <rect x="2" y="15" width="20" height="6" rx="1"/>
          <circle cx="6" cy="6" r="1" fill="currentColor"/>
          <circle cx="6" cy="18" r="1" fill="currentColor"/>
          <path d="M12 9v6"/>
        </svg>
        <span class="tab-name">{{ getTabDisplayName(tabId) }}</span>
        <span v-if="isTabConnected(tabId)" class="connection-dot"></span>
        <button
          class="tab-close"
          @click.stop="emit('closeTab', tabId)"
          :title="t.closeTab"
        >
          <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>
    </div>

    <button
      class="add-tab-btn"
      :disabled="!canAddTab"
      @click="emit('addTab')"
      :title="canAddTab ? t.newTab : t.maxTabsReached"
    >
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="12" y1="5" x2="12" y2="19"/>
        <line x1="5" y1="12" x2="19" y2="12"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.tab-bar {
  display: flex;
  align-items: center;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  padding: 0 8px;
  gap: 6px;
  height: 30px;
  flex-shrink: 0;
}

.tabs-container {
  display: flex;
  gap: 3px;
  flex: 1;
  overflow-x: auto;
  padding: 3px 0;
}

.tabs-container::-webkit-scrollbar {
  height: 3px;
}

.tabs-container::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 2px;
}

.tab {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 4px 8px;
  background: var(--bg-tertiary);
  border: 1px solid transparent;
  border-radius: var(--radius-sm) var(--radius-sm) 0 0;
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 90px;
  max-width: 150px;
  position: relative;
}

.tab:hover {
  background: var(--bg-hover);
}

.tab.active {
  background: var(--accent-light);
  border-color: var(--accent-primary);
  border-bottom-color: var(--accent-light);
  margin-bottom: -1px;
  box-shadow: 0 -2px 6px rgba(14, 165, 233, 0.15);
}

.tab.active .tab-icon {
  color: var(--accent-primary);
}

.tab.active .tab-name {
  color: var(--accent-primary);
  font-weight: 700;
}

.tab-icon {
  flex-shrink: 0;
  color: var(--text-tertiary);
  transition: color 0.2s ease;
  width: 10px;
  height: 10px;
}

/* Connection type icon colors */
.tab.type-tcp-client.active .tab-icon {
  color: #0ea5e9;
}

.tab.type-tcp-server.active .tab-icon {
  color: #8b5cf6;
}

.tab.type-serial.active .tab-icon {
  color: #10b981;
}

.tab-name {
  font-size: 0.65rem;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  font-family: var(--font-mono);
}

.connection-dot {
  width: 5px;
  height: 5px;
  background: var(--success);
  border-radius: 50%;
  flex-shrink: 0;
  animation: pulse-dot 2s infinite;
}

@keyframes pulse-dot {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.2); opacity: 0.7; }
}

.tab-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 14px;
  height: 14px;
  background: transparent;
  border: none;
  border-radius: 2px;
  color: var(--text-tertiary);
  cursor: pointer;
  opacity: 0;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.tab:hover .tab-close {
  opacity: 1;
}

.tab-close:hover {
  background: var(--danger-light);
  color: var(--danger);
}

.add-tab-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.add-tab-btn:hover:not(:disabled) {
  background: var(--accent-light);
  color: var(--accent-primary);
  border-color: var(--accent-primary);
}

.add-tab-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
