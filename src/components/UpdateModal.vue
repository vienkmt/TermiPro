<script setup>
import { inject, computed } from 'vue';

const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
  updateInfo: {
    type: Object,
    default: null,
  },
  currentVersion: {
    type: String,
    default: '',
  },
  status: {
    type: String,
    default: 'idle', // idle, downloading, installing, ready, error
  },
  progress: {
    type: Number,
    default: 0,
  },
});

const emit = defineEmits(['confirm', 'cancel']);

const t = inject('t');

const statusText = computed(() => {
  switch (props.status) {
    case 'downloading':
      return `${t.value.downloading} ${Math.round(props.progress)}%`;
    case 'installing':
      return t.value.installing;
    case 'ready':
      return t.value.updateReady;
    case 'error':
      return t.value.updateFailed;
    default:
      return '';
  }
});

const isProcessing = computed(() =>
  ['downloading', 'installing', 'ready'].includes(props.status)
);

// Simple markdown to HTML conversion for changelog
function formatChangelog(text) {
  if (!text) return '<p>No changelog available</p>';

  return text
    .replace(/^### (.+)$/gm, '<h5>$1</h5>')
    .replace(/^## (.+)$/gm, '<h4>$1</h4>')
    .replace(/^# (.+)$/gm, '<h3>$1</h3>')
    .replace(/^\* (.+)$/gm, '<li>$1</li>')
    .replace(/^- (.+)$/gm, '<li>$1</li>')
    .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    .replace(/`(.+?)`/g, '<code>$1</code>')
    .replace(/\n\n/g, '</p><p>');
}
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="visible" class="dialog-overlay" @click.self="emit('cancel')">
        <div class="dialog-content update-modal">
          <div class="dialog-header">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
              <polyline points="7 10 12 15 17 10"/>
              <line x1="12" y1="15" x2="12" y2="3"/>
            </svg>
            <h3>{{ t.updateTitle }}</h3>
          </div>

          <!-- Version info -->
          <div class="version-info">
            <div class="version-row">
              <span class="version-label">{{ t.currentVersion }}:</span>
              <span class="version-value current">v{{ currentVersion }}</span>
            </div>
            <div class="version-arrow">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="9 18 15 12 9 6"/>
              </svg>
            </div>
            <div class="version-row">
              <span class="version-label">{{ t.newVersion }}:</span>
              <span class="version-value new">v{{ updateInfo?.version }}</span>
            </div>
          </div>

          <!-- Changelog -->
          <div class="changelog-section">
            <h4>{{ t.changelog }}</h4>
            <div class="changelog-content" v-html="formatChangelog(updateInfo?.body)"></div>
          </div>

          <!-- Progress bar -->
          <div v-if="isProcessing" class="progress-section">
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: progress + '%' }"></div>
            </div>
            <span class="progress-text">{{ statusText }}</span>
          </div>

          <!-- Actions -->
          <div class="dialog-actions" v-if="!isProcessing">
            <button class="btn-cancel" @click="emit('cancel')">
              {{ t.updateLater }}
            </button>
            <button class="btn-confirm update-confirm" @click="emit('confirm')">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              {{ t.updateNow }}
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
  max-width: 480px;
  width: 90%;
  box-shadow: var(--shadow-lg);
  animation: dialogSlide 0.2s ease;
}

.update-modal {
  max-height: 80vh;
  overflow-y: auto;
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
  margin-bottom: 20px;
}

.dialog-header svg {
  color: var(--accent-primary);
  flex-shrink: 0;
}

.dialog-header h3 {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

/* Version Info */
.version-info {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 16px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  margin-bottom: 20px;
}

.version-row {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.version-label {
  font-size: 0.75rem;
  color: var(--text-tertiary);
}

.version-value {
  font-size: 1rem;
  font-weight: 600;
  font-family: var(--font-mono);
}

.version-value.current {
  color: var(--text-secondary);
}

.version-value.new {
  color: var(--success);
}

.version-arrow {
  color: var(--text-tertiary);
}

/* Changelog */
.changelog-section {
  margin-bottom: 20px;
}

.changelog-section h4 {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 12px 0;
}

.changelog-content {
  max-height: 200px;
  overflow-y: auto;
  padding: 12px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  font-size: 0.85rem;
  line-height: 1.6;
  color: var(--text-secondary);
}

.changelog-content :deep(h3),
.changelog-content :deep(h4),
.changelog-content :deep(h5) {
  margin: 8px 0 4px 0;
  color: var(--text-primary);
}

.changelog-content :deep(ul) {
  margin: 8px 0;
  padding-left: 20px;
}

.changelog-content :deep(li) {
  margin: 4px 0;
}

.changelog-content :deep(code) {
  background: var(--bg-hover);
  padding: 2px 6px;
  border-radius: 3px;
  font-family: var(--font-mono);
  font-size: 0.8rem;
}

/* Progress */
.progress-section {
  margin-bottom: 16px;
}

.progress-bar {
  height: 8px;
  background: var(--bg-tertiary);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-primary), var(--accent-secondary));
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 0.8rem;
  color: var(--text-secondary);
  text-align: center;
  display: block;
}

/* Actions */
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
  background: var(--accent-primary);
  border: none;
  border-radius: var(--radius-md);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-confirm:hover {
  background: var(--accent-secondary);
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
