<script setup>
import { inject } from 'vue';

const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
  title: {
    type: String,
    default: '',
  },
  message: {
    type: String,
    default: '',
  },
});

const emit = defineEmits(['confirm', 'cancel']);

const t = inject('t');
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="visible" class="dialog-overlay" @click.self="emit('cancel')">
        <div class="dialog-content">
          <div class="dialog-header">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <line x1="12" y1="8" x2="12" y2="12"/>
              <line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
            <h3>{{ title || t.closeTabConfirm }}</h3>
          </div>
          <p class="dialog-message">{{ message || t.closeTabWarning }}</p>
          <div class="dialog-actions">
            <button class="btn-cancel" @click="emit('cancel')">
              {{ t.cancel }}
            </button>
            <button class="btn-confirm" @click="emit('confirm')">
              {{ t.disconnect }}
            </button>
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
  max-width: 400px;
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
  gap: 12px;
  margin-bottom: 16px;
}

.dialog-header svg {
  color: var(--warning);
  flex-shrink: 0;
}

.dialog-header h3 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.dialog-message {
  font-size: 0.875rem;
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0 0 24px 0;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.btn-cancel {
  padding: 10px 20px;
  font-size: 0.85rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.btn-confirm {
  padding: 10px 20px;
  font-size: 0.85rem;
  font-weight: 600;
  font-family: var(--font-sans);
  background: var(--danger);
  border: none;
  border-radius: var(--radius-md);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-confirm:hover {
  background: #dc2626;
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
