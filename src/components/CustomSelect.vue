<template>
  <div class="custom-select" :class="{ open: isOpen }" ref="selectRef">
    <div class="select-trigger" @click="toggleOpen">
      <span class="select-value">{{ selectedOption?.name || placeholder }}</span>
      <div class="select-arrow-container">
        <svg
          class="select-arrow"
          :class="{ rotated: isOpen }"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </div>
    </div>

    <transition name="dropdown">
      <div v-if="isOpen" class="select-dropdown">
        <div class="dropdown-content">
          <div
            v-for="option in options"
            :key="option.value"
            class="select-option"
            :class="{ selected: option.value === modelValue }"
            @click="selectOption(option)"
          >
            <span class="option-text">{{ option.name }}</span>
            <div v-if="option.value === modelValue" class="option-check">
              <svg viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
              </svg>
            </div>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

interface SelectOption {
  value: string
  name: string
}

interface Props {
  modelValue: string
  options: SelectOption[]
  placeholder?: string
}

interface Emits {
  (e: 'update:modelValue', value: string): void
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '请选择...'
})

const emit = defineEmits<Emits>()

const isOpen = ref(false)
const selectRef = ref<HTMLElement>()

const selectedOption = computed(() => {
  return props.options.find(option => option.value === props.modelValue)
})

function toggleOpen() {
  isOpen.value = !isOpen.value
}

function selectOption(option: SelectOption) {
  emit('update:modelValue', option.value)
  isOpen.value = false
}

function handleClickOutside(event: Event) {
  if (selectRef.value && !selectRef.value.contains(event.target as Node)) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.custom-select {
  position: relative;
  width: 100%;
  max-width: 220px;
}

.select-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.625rem 1rem;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.9) 0%, rgba(248, 250, 252, 0.9) 100%);
  border: none;
  border-radius: 1rem;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(20px);
  box-shadow:
    0 4px 6px -1px rgba(0, 0, 0, 0.1),
    0 2px 4px -1px rgba(0, 0, 0, 0.06);
  position: relative;
  overflow: hidden;
}

.select-trigger::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
  opacity: 0;
  transition: opacity 0.3s ease;
}



.custom-select.open .select-trigger {
  box-shadow:
    0 4px 6px -1px rgba(0, 0, 0, 0.1),
    0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.select-value {
  font-size: 0.875rem;
  color: #374151;
  font-weight: 500;
  position: relative;
  z-index: 1;
}

.select-arrow-container {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 0.375rem;
  background: rgba(102, 126, 234, 0.1);
  transition: all 0.3s ease;
}

.select-arrow {
  width: 14px;
  height: 14px;
  color: #667eea;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.select-arrow.rotated {
  transform: rotate(180deg);
}

.select-dropdown {
  position: absolute;
  top: calc(100% + 0.5rem);
  left: 0;
  right: 0;
  z-index: 99;
  background: rgba(255, 255, 255, 0.95);
  border: none;
  border-radius: 1rem;
  backdrop-filter: blur(20px);
  box-shadow:
    0 4px 6px -1px rgba(0, 0, 0, 0.1),
    0 2px 4px -1px rgba(0, 0, 0, 0.06),
    0 0 0 1px rgba(0, 0, 0, 0.05);
  overflow: hidden;
}

.dropdown-content {
  max-height: 200px;
  overflow-y: auto;
}

.dropdown-content::-webkit-scrollbar {
  width: 6px;
}

.dropdown-content::-webkit-scrollbar-track {
  background: rgba(243, 244, 246, 0.5);
}

.dropdown-content::-webkit-scrollbar-thumb {
  background: rgba(156, 163, 175, 0.5);
  border-radius: 3px;
}

.dropdown-content::-webkit-scrollbar-thumb:hover {
  background: rgba(107, 114, 128, 0.7);
}

.select-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  cursor: pointer;
  transition: color 0.2s ease;
  font-size: 0.875rem;
}

.select-option:hover {
  color: #667eea;
}

.select-option.selected {
  background: rgba(102, 126, 234, 0.1);
  color: #667eea;
  font-weight: 600;
}

.option-text {
  flex: 1;
  position: relative;
  z-index: 1;
}

.option-check {
  width: 18px;
  height: 18px;
  color: #667eea;
  position: relative;
  z-index: 1;
}

/* 下拉动画 */
.dropdown-enter-active {
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 1, 1);
}

.dropdown-enter-from {
  opacity: 0;
  transform: translateY(-20px) scale(0.9);
}

.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(0.95);
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .select-trigger {
    background: linear-gradient(135deg, rgba(30, 41, 59, 0.9) 0%, rgba(15, 23, 42, 0.9) 100%);
    border: none;
  }

  .select-trigger::before {
    background: linear-gradient(135deg, rgba(129, 140, 248, 0.15) 0%, rgba(165, 180, 252, 0.15) 100%);
  }

  .custom-select.open .select-trigger {
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .select-value {
    color: #f1f5f9;
  }

  .select-arrow-container {
    background: rgba(129, 140, 248, 0.15);
  }

  .select-arrow {
    color: #a5b4fc;
  }

  .select-dropdown {
    background: rgba(30, 41, 59, 0.95);
    border: none;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.3),
      0 2px 4px -1px rgba(0, 0, 0, 0.2),
      0 0 0 1px rgba(0, 0, 0, 0.1);
  }

  .dropdown-content::-webkit-scrollbar-track {
    background: rgba(51, 65, 85, 0.5);
  }

  .dropdown-content::-webkit-scrollbar-thumb {
    background: rgba(148, 163, 184, 0.5);
  }

  .dropdown-content::-webkit-scrollbar-thumb:hover {
    background: rgba(203, 213, 225, 0.7);
  }

  .select-option:hover {
    color: #a5b4fc;
  }

  .select-option.selected {
    background: rgba(129, 140, 248, 0.15);
    color: #a5b4fc;
  }

  .option-check {
    color: #818cf8;
  }
}

:global(.dark) .select-trigger {
  background: linear-gradient(135deg, rgba(30, 41, 59, 0.9) 0%, rgba(15, 23, 42, 0.9) 100%);
  border: none;
}

:global(.dark) .select-trigger::before {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.15) 0%, rgba(165, 180, 252, 0.15) 100%);
}

:global(.dark) .custom-select.open .select-trigger {
  box-shadow:
    0 4px 6px -1px rgba(0, 0, 0, 0.1),
    0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

:global(.dark) .select-value {
  color: #f1f5f9;
}

:global(.dark) .select-arrow-container {
  background: rgba(129, 140, 248, 0.15);
}

:global(.dark) .select-arrow {
  color: #a5b4fc;
}

:global(.dark) .select-dropdown {
  background: rgba(30, 41, 59, 0.95);
  border: none;
  box-shadow:
    0 4px 6px -1px rgba(0, 0, 0, 0.3),
    0 2px 4px -1px rgba(0, 0, 0, 0.2),
    0 0 0 1px rgba(0, 0, 0, 0.1);
}

:global(.dark) .dropdown-content::-webkit-scrollbar-track {
  background: rgba(51, 65, 85, 0.5);
}

:global(.dark) .dropdown-content::-webkit-scrollbar-thumb {
  background: rgba(148, 163, 184, 0.5);
}

:global(.dark) .dropdown-content::-webkit-scrollbar-thumb:hover {
  background: rgba(203, 213, 225, 0.7);
}

:global(.dark) .select-option:hover {
  color: #a5b4fc;
}

:global(.dark) .select-option.selected {
  background: rgba(129, 140, 248, 0.15);
  color: #a5b4fc;
}

:global(.dark) .option-check {
  color: #818cf8;
}
</style>
