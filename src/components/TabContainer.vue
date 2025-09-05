<template>
  <div class="tab-container">
    <!-- Tab 头部 -->
    <div class="tab-header">
      <div class="tab-list">
        <div
          v-for="tab in tabs"
          :key="tab.id"
          class="tab-item"
          :class="{ active: tab.id === activeTabId }"
          @click="setActiveTab(tab.id)"
        >
          <span class="tab-title">{{ tab.title }}</span>
          <button
            class="tab-close"
            @click.stop="closeTab(tab.id)"
            :title="$t('common.close')"
          >
            <svg viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Tab 内容 -->
    <div class="tab-content">
      <transition
        name="tab-fade"
        mode="out-in"
        appear
      >
        <div
          v-if="currentTab"
          :key="currentTab.id"
          class="tab-pane"
        >
          <component
            :is="currentTab.component"
            v-bind="currentTab.props"
            @close="closeTab(currentTab.id)"
          />
        </div>
      </transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface TabItem {
  id: string
  title: string
  component: any
  props?: Record<string, any>
}

interface Props {
  tabs: TabItem[]
  activeTabId?: string
}

interface Emits {
  (e: 'update:activeTabId', value: string): void
  (e: 'update:active-tab-id', value: string): void
  (e: 'close-tab', tabId: string): void
  (e: 'tab-changed', tabId: string): void
}

const props = withDefaults(defineProps<Props>(), {
  activeTabId: ''
})

const emit = defineEmits<Emits>()

const activeTab = computed({
  get: () => props.activeTabId || (props.tabs.length > 0 ? props.tabs[0].id : ''),
  set: (value: string) => emit('update:activeTabId', value)
})

const currentTab = computed(() => {
  const activeId = activeTab.value
  return props.tabs.find(tab => tab.id === activeId) || null
})

function setActiveTab(tabId: string) {
  activeTab.value = tabId
  emit('update:activeTabId', tabId)
  emit('update:active-tab-id', tabId)
  emit('tab-changed', tabId)
}

function closeTab(tabId: string) {
  emit('close-tab', tabId)
}
</script>

<style scoped>
.tab-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 1rem;
  backdrop-filter: blur(20px);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.tab-header {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(248, 250, 252, 0.95) 100%);
  border-bottom: 1px solid rgba(209, 213, 219, 0.3);
  padding: 0.5rem 1rem 0;
}

.tab-list {
  display: flex;
  gap: 0.25rem;
  overflow-x: auto;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.tab-list::-webkit-scrollbar {
  display: none;
}

.tab-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  background: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(209, 213, 219, 0.3);
  border-bottom: none;
  border-radius: 0.5rem 0.5rem 0 0;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
  min-width: 120px;
  max-width: 200px;
}

.tab-item:hover {
  background: rgba(255, 255, 255, 0.9);
  border-color: rgba(102, 126, 234, 0.3);
}

.tab-item.active {
  background: rgba(255, 255, 255, 0.95);
  border-color: rgba(102, 126, 234, 0.5);
  box-shadow: 0 2px 4px rgba(102, 126, 234, 0.1);
}

.tab-title {
  flex: 1;
  font-size: 0.875rem;
  font-weight: 500;
  color: #374151;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tab-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  border: none;
  background: none;
  color: #6b7280;
  cursor: pointer;
  border-radius: 0.25rem;
  transition: all 0.2s ease;
}

.tab-close:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.tab-close svg {
  width: 12px;
  height: 12px;
}

.tab-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}

.tab-pane {
  height: 100%;
  overflow: hidden;
}

/* Tab 切换动画 - 现代平滑过渡 */
.tab-fade-enter-active {
  transition: all 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  transition-delay: 0.1s;
}

.tab-fade-leave-active {
  transition: all 0.25s cubic-bezier(0.55, 0.055, 0.675, 0.19);
}

.tab-fade-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
  filter: blur(2px);
}

.tab-fade-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(1.02);
  filter: blur(1px);
}

.tab-fade-enter-to,
.tab-fade-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
  filter: blur(0);
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .tab-container {
    background: rgba(30, 41, 59, 0.9);
  }

  .tab-header {
    background: linear-gradient(135deg, rgba(30, 41, 59, 0.95) 0%, rgba(15, 23, 42, 0.95) 100%);
    border-bottom-color: rgba(129, 140, 248, 0.2);
  }

  .tab-item {
    background: rgba(51, 65, 85, 0.7);
    border-color: rgba(129, 140, 248, 0.2);
  }

  .tab-item:hover {
    background: rgba(51, 65, 85, 0.9);
    border-color: rgba(129, 140, 248, 0.4);
  }

  .tab-item.active {
    background: rgba(51, 65, 85, 0.95);
    border-color: rgba(129, 140, 248, 0.6);
    box-shadow: 0 2px 4px rgba(129, 140, 248, 0.2);
  }

  .tab-title {
    color: #f1f5f9;
  }

  .tab-close {
    color: #94a3b8;
  }

  .tab-close:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #fca5a5;
  }
}

:global(.dark) .tab-container {
  background: rgba(30, 41, 59, 0.9);
}

:global(.dark) .tab-header {
  background: linear-gradient(135deg, rgba(30, 41, 59, 0.95) 0%, rgba(15, 23, 42, 0.95) 100%);
  border-bottom-color: rgba(129, 140, 248, 0.2);
}

:global(.dark) .tab-item {
  background: rgba(51, 65, 85, 0.7);
  border-color: rgba(129, 140, 248, 0.2);
}

:global(.dark) .tab-item:hover {
  background: rgba(51, 65, 85, 0.9);
  border-color: rgba(129, 140, 248, 0.4);
}

:global(.dark) .tab-item.active {
  background: rgba(51, 65, 85, 0.95);
  border-color: rgba(129, 140, 248, 0.6);
  box-shadow: 0 2px 4px rgba(129, 140, 248, 0.2);
}

:global(.dark) .tab-title {
  color: #f1f5f9;
}

:global(.dark) .tab-close {
  color: #94a3b8;
}

:global(.dark) .tab-close:hover {
  background: rgba(239, 68, 68, 0.2);
  color: #fca5a5;
}
</style>
