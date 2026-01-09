<script setup>
import { ref, watch, computed } from 'vue';

const props = defineProps({
  message: { type: String, default: '' },
  type: { type: String, default: 'error' }, // error, success, warning, info
  visible: { type: Boolean, default: false },
  duration: { type: Number, default: 1500 },
});

const emit = defineEmits(['close']);

const isVisible = ref(false);
let timer = null;

watch(() => props.visible, (val) => {
  isVisible.value = val;
  if (val && props.duration > 0) {
    clearTimeout(timer);
    timer = setTimeout(() => {
      emit('close');
    }, props.duration);
  }
}, { immediate: true });

function close() {
  clearTimeout(timer);
  emit('close');
}

const iconPath = computed(() => {
  switch (props.type) {
    case 'success':
      return 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z';
    case 'warning':
      return 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z';
    case 'info':
      return 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z';
    default: // error
      return 'M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z';
  }
});
</script>

<template>
  <Teleport to="body">
    <Transition name="toast">
      <div v-if="isVisible" class="toast-container" :class="type">
        <div class="toast-content">
          <svg class="toast-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" :d="iconPath" />
          </svg>
          <span class="toast-message">{{ message }}</span>
          <button class="toast-close" @click="close">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.toast-container {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 10000;
  max-width: 500px;
  min-width: 300px;
}

.toast-content {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  font-size: 0.85rem;
  font-weight: 500;
}

.toast-container.error .toast-content {
  background: #fee2e2;
  border: 1px solid #fca5a5;
  color: #b91c1c;
}

.toast-container.success .toast-content {
  background: #d1fae5;
  border: 1px solid #6ee7b7;
  color: #047857;
}

.toast-container.warning .toast-content {
  background: #fef3c7;
  border: 1px solid #fcd34d;
  color: #b45309;
}

.toast-container.info .toast-content {
  background: #dbeafe;
  border: 1px solid #93c5fd;
  color: #1d4ed8;
}

.toast-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.toast-message {
  flex: 1;
  line-height: 1.4;
}

.toast-close {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  opacity: 0.7;
  color: currentColor;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: opacity 0.2s, background 0.2s;
}

.toast-close:hover {
  opacity: 1;
  background: rgba(0, 0, 0, 0.1);
}

.toast-close svg {
  width: 16px;
  height: 16px;
}

/* Animations */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translate(-50%, -50%) scale(0.9);
}

.toast-leave-to {
  opacity: 0;
  transform: translate(-50%, -50%) scale(0.9);
}
</style>
