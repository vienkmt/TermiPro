<script setup>
import { inject } from 'vue';
import { CONNECTION_TYPES } from '../stores/tabStore';

const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['select', 'cancel']);

const t = inject('t');

function selectType(type) {
  emit('select', type);
}
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="visible" class="dialog-overlay" @click.self="emit('cancel')">
        <div class="dialog-content">
          <div class="dialog-header">
            <h3>{{ t.selectConnectionType }}</h3>
            <button class="close-btn" @click="emit('cancel')">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>

          <div class="type-cards">
            <!-- Serial -->
            <div class="type-card" @click="selectType(CONNECTION_TYPES.SERIAL)">
              <div class="type-icon serial">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <rect x="4" y="4" width="16" height="16" rx="2"/>
                  <circle cx="9" cy="9" r="1" fill="currentColor"/>
                  <circle cx="15" cy="9" r="1" fill="currentColor"/>
                  <circle cx="9" cy="15" r="1" fill="currentColor"/>
                  <circle cx="15" cy="15" r="1" fill="currentColor"/>
                  <path d="M12 9v6"/>
                  <path d="M9 12h6"/>
                </svg>
              </div>
              <div class="type-info">
                <h4>{{ t.connectionTypeSerial }}</h4>
                <p>{{ t.serialDesc }}</p>
              </div>
            </div>

            <!-- TCP Client -->
            <div class="type-card" @click="selectType(CONNECTION_TYPES.TCP_CLIENT)">
              <div class="type-icon tcp-client">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M5 12h14"/>
                  <path d="M13 5l7 7-7 7"/>
                  <rect x="2" y="8" width="6" height="8" rx="1"/>
                </svg>
              </div>
              <div class="type-info">
                <h4>{{ t.connectionTypeTcpClient }}</h4>
                <p>{{ t.tcpClientDesc }}</p>
              </div>
            </div>

            <!-- TCP Server -->
            <div class="type-card" @click="selectType(CONNECTION_TYPES.TCP_SERVER)">
              <div class="type-icon tcp-server">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <rect x="2" y="3" width="20" height="6" rx="1"/>
                  <rect x="2" y="15" width="20" height="6" rx="1"/>
                  <circle cx="6" cy="6" r="1" fill="currentColor"/>
                  <circle cx="6" cy="18" r="1" fill="currentColor"/>
                  <path d="M12 9v6"/>
                  <path d="M9 12h6"/>
                </svg>
              </div>
              <div class="type-info">
                <h4>{{ t.connectionTypeTcpServer }}</h4>
                <p>{{ t.tcpServerDesc }}</p>
              </div>
            </div>

            <!-- Modbus Master -->
            <div class="type-card" @click="selectType(CONNECTION_TYPES.MODBUS)">
              <div class="type-icon modbus">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                </svg>
              </div>
              <div class="type-info">
                <h4>{{ t.connectionTypeModbus }}</h4>
                <p>{{ t.modbusDesc }}</p>
              </div>
            </div>

            <!-- Modbus Slave -->
            <div class="type-card" @click="selectType(CONNECTION_TYPES.MODBUS_SLAVE)">
              <div class="type-icon modbus-slave">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <rect x="4" y="2" width="16" height="6" rx="1"/>
                  <rect x="4" y="9" width="16" height="6" rx="1"/>
                  <rect x="4" y="16" width="16" height="6" rx="1"/>
                  <circle cx="7" cy="5" r="1" fill="currentColor"/>
                  <circle cx="7" cy="12" r="1" fill="currentColor"/>
                  <circle cx="7" cy="19" r="1" fill="currentColor"/>
                  <line x1="11" y1="5" x2="17" y2="5"/>
                  <line x1="11" y1="12" x2="17" y2="12"/>
                  <line x1="11" y1="19" x2="17" y2="19"/>
                </svg>
              </div>
              <div class="type-info">
                <h4>{{ t.connectionTypeModbusSlave }}</h4>
                <p>{{ t.modbusSlaveDesc }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.dialog-content {
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  padding: 24px;
  max-width: 520px;
  width: 90%;
  box-shadow: var(--shadow-lg);
  animation: dialogSlide 0.2s ease;
}

@keyframes dialogSlide {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(-10px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.dialog-header h3 {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.type-cards {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.type-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
}

.type-card:hover {
  border-color: var(--accent-primary);
  background: var(--bg-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(14, 165, 233, 0.15);
}

.type-icon {
  width: 56px;
  height: 56px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.type-icon.serial {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
}

.type-icon.tcp-client {
  background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
  color: white;
}

.type-icon.tcp-server {
  background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
  color: white;
}

.type-icon.modbus {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
}

.type-icon.modbus-slave {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
}

.type-info {
  flex: 1;
}

.type-info h4 {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.type-info p {
  font-size: 0.8rem;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.4;
}

/* Transition */
.dialog-enter-active,
.dialog-leave-active {
  transition: opacity 0.2s ease;
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}
</style>
