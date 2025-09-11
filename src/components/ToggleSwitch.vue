<template>
  <div class="setting-control">
    <label class="switch">
      <input
        type="checkbox"
        :checked="modelValue"
        @change="handleChange"
      >
      <span class="slider"></span>
    </label>
  </div>
</template>

<script setup lang="ts">
interface Props {
  modelValue: boolean
  description?: string
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'change', value: boolean): void
}

defineProps<Props>()
const emit = defineEmits<Emits>()

function handleChange(event: Event) {
  const target = event.target as HTMLInputElement
  const value = target.checked
  emit('update:modelValue', value)
  emit('change', value)
}
</script>

<style scoped>
.setting-control {
  display: flex;
  align-items: center;
  flex-shrink: 0; /* 防止开关被挤压 */
}

/* 开关样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #cbd5e1;
  transition: 0.3s;
  border-radius: 24px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  top: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

input:checked + .slider {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

input:checked + .slider:before {
  transform: translateX(26px);
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .slider {
    background-color: #4b5563;
  }

  input:checked + .slider {
    background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%);
  }
}

:global(.dark) .slider {
  background-color: #4b5563;
}

:global(.dark) input:checked + .slider {
  background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%);
}
</style>
