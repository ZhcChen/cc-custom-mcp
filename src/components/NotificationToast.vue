<template>
  <Transition name="toast" appear>
    <div v-if="visible" class="notification-toast">
      <div class="toast-content">
        <div class="toast-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-3.582 8-8 8a8.959 8.959 0 01-4.906-1.681L3 21l2.681-5.094A8.959 8.959 0 013 12c0-4.418 3.582-8 8-8s8 3.582 8 8z" />
          </svg>
        </div>
        <div class="toast-message">{{ message }}</div>
        <button class="toast-close" @click="close">
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Props {
  message: string
  duration?: number
}

interface Emits {
  (e: 'close'): void
}

const props = withDefaults(defineProps<Props>(), {
  duration: 5000
})

const emit = defineEmits<Emits>()

const visible = ref(false)

function close() {
  visible.value = false
  setTimeout(() => {
    emit('close')
  }, 300) // 等待动画完成
}

onMounted(() => {
  visible.value = true
  
  // 自动关闭
  if (props.duration > 0) {
    setTimeout(() => {
      close()
    }, props.duration)
  }
})
</script>

<style scoped>
.notification-toast {
  position: fixed;
  top: 1rem;
  right: 1rem;
  z-index: 1000;
  max-width: 400px;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 0.75rem;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(20px);
  overflow: hidden;
}

.toast-content {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
}

.toast-icon {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  color: #3b82f6;
}

.toast-message {
  flex: 1;
  font-size: 0.875rem;
  font-weight: 500;
  color: #374151;
  line-height: 1.4;
}

.toast-close {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: none;
  background: none;
  color: #6b7280;
  cursor: pointer;
  border-radius: 0.25rem;
  transition: all 0.2s ease;
}

.toast-close:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%);
}

/* 暗色主题 */
:global(.dark) .notification-toast {
  background: rgba(51, 65, 85, 0.95);
  border-color: rgba(129, 140, 248, 0.3);
}

:global(.dark) .toast-message {
  color: #f1f5f9;
}

:global(.dark) .toast-icon {
  color: #818cf8;
}

:global(.dark) .toast-close {
  color: #94a3b8;
}

:global(.dark) .toast-close:hover {
  background: rgba(239, 68, 68, 0.2);
  color: #fca5a5;
}
</style>
