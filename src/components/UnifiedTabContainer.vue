<template>
  <div v-if="tabs.length > 0" class="unified-tab-container">
    <!-- Tab 头部 - 使用仪表盘样式 -->
    <div class="unified-tab-header">
      <div class="tab-header-content">
        <div class="unified-tab-list">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            :class="['unified-tab-button', { active: tab.id === currentActiveTabId }]"
            @click="setActiveTab(tab.id)"
            :title="getTabTooltip(tab)"
          >
            <span class="tab-title">{{ tab.title }}</span>
            <span v-if="tab.aiSource" class="ai-source-tag" :data-source="tab.aiSource">
              {{ tab.aiSourceDisplay || tab.aiSource }}
            </span>
            <button
              v-if="showCloseButton"
              class="tab-close-btn"
              @click.stop="closeTab(tab.id)"
              :title="$t('common.close')"
            >
              <svg viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </button>
          </button>
        </div>
        
        <!-- 关闭全部按钮 -->
        <div v-if="showCloseButton && tabs.length > 1" class="close-all-container">
          <button
            class="close-all-btn"
            @click="closeAllTabs"
            :title="$t('feedback.closeAll', '关闭全部')"
          >
            <svg viewBox="0 0 20 20" fill="currentColor" class="close-all-icon">
              <path fill-rule="evenodd" d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9zM4 5a2 2 0 012-2v1a1 1 0 001 1h6a1 1 0 001-1V3a2 2 0 012 2v6a2 2 0 01-2 2H6a2 2 0 01-2-2V5zM8 7a1 1 0 012 0v6a1 1 0 11-2 0V7zm4 0a1 1 0 10-2 0v6a1 1 0 102 0V7z" clip-rule="evenodd" />
            </svg>
            <span class="close-all-text">全部关闭</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Tab 内容 -->
    <div class="unified-tab-content">
      <!-- 无活动 tab 时的提示 -->
      <div v-if="!currentTab" class="no-tab-info">
        <p>没有活动的 Tab</p>
      </div>

      <transition
        name="unified-tab-fade"
        mode="out-in"
        appear
      >
        <div
          v-if="currentTab"
          :key="currentTab.id"
          class="unified-tab-pane"
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
  component?: any
  props?: Record<string, any>
  isNew?: boolean
  priority?: 'normal' | 'high'
  aiSource?: string // AI 来源标识
  aiSourceDisplay?: string // AI 来源显示名称
}

interface Props {
  tabs: TabItem[]
  activeTabId?: string
  showCloseButton?: boolean // 是否显示关闭按钮
}

interface Emits {
  (e: 'update:activeTabId', value: string): void
  (e: 'update:active-tab-id', value: string): void
  (e: 'close-tab', tabId: string): void
  (e: 'close-all-tabs'): void
  (e: 'tab-changed', tabId: string): void
}

const props = withDefaults(defineProps<Props>(), {
  activeTabId: '',
  showCloseButton: true
})

const emit = defineEmits<Emits>()

const currentActiveTabId = computed(() => {
  // 如果有明确的 activeTabId 且对应的 tab 存在，使用它
  if (props.activeTabId && props.tabs.find(tab => tab.id === props.activeTabId)) {
    return props.activeTabId
  }
  // 如果没有明确的 activeTabId 或对应的 tab 不存在，但有 tabs，使用第一个
  if (props.tabs.length > 0) {
    return props.tabs[0].id
  }
  return ''
})

const currentTab = computed((): TabItem | null => {
  const activeId = currentActiveTabId.value
  const tab = props.tabs.find(tab => tab.id === activeId)
  return tab || null
})

function setActiveTab(tabId: string) {
  // 直接发送事件，不修改本地状态
  emit('update:activeTabId', tabId)
  emit('update:active-tab-id', tabId)
  emit('tab-changed', tabId)
}

function closeTab(tabId: string) {
  emit('close-tab', tabId)
}

function closeAllTabs() {
  emit('close-all-tabs')
}

// 生成tab提示信息
function getTabTooltip(tab: TabItem): string {
  const source = tab.aiSourceDisplay || tab.aiSource || 'Unknown AI Tool'
  return tab.aiSource ? `${tab.title} - 来源: ${source}` : tab.title
}
</script>

<style scoped>
.unified-tab-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 1rem;
  backdrop-filter: blur(20px);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.unified-tab-header {
  padding: 1rem 1rem 0;
  background: transparent;
}

.tab-header-content {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
}

.unified-tab-list {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
  border-bottom: 1px solid rgba(226, 232, 240, 0.3);
  padding-bottom: 0.5rem;
  overflow-x: auto;
  scrollbar-width: none;
  -ms-overflow-style: none;
  flex: 1;
}

.unified-tab-list::-webkit-scrollbar {
  display: none;
}

/* 基于仪表盘config-tab样式 */
.unified-tab-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: none;
  background: transparent;
  color: #718096;
  font-weight: 500;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.3s ease;
  white-space: nowrap;
  min-width: fit-content;
  border-radius: 0.375rem 0.375rem 0 0;
}

.unified-tab-button:hover {
  color: #4a5568;
  background: rgba(255, 255, 255, 0.1);
}

.unified-tab-button.active {
  color: #667eea;
  border-bottom-color: #667eea;
  background: rgba(102, 126, 234, 0.1);
}

.tab-title {
  flex: 1;
  font-size: 0.875rem;
  font-weight: inherit;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* AI 来源标签样式 */
.ai-source-tag {
  font-size: 0.65rem;
  font-weight: 600;
  padding: 0.25rem 0.6rem;
  border-radius: 0.4rem;
  letter-spacing: 0.02em;
  flex-shrink: 0;
  transition: all 0.2s ease;
  min-width: 40px;
  text-align: center;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
  color: #667eea;
  border: none;
  box-shadow: 0 1px 3px rgba(102, 126, 234, 0.1);
}

.unified-tab-button.active .ai-source-tag {
  font-weight: 700;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.25) 0%, rgba(118, 75, 162, 0.25) 100%);
  color: #5a67d8;
  box-shadow: 0 2px 6px rgba(102, 126, 234, 0.2);
  transform: scale(1.05);
}

/* 关闭按钮样式 */
.tab-close-btn {
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
  margin-left: 0.25rem;
}

.tab-close-btn:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.tab-close-btn svg {
  width: 12px;
  height: 12px;
}

/* 关闭全部按钮样式 */
.close-all-container {
  display: flex;
  align-items: center;
  margin-bottom: 1.5rem;
  padding-bottom: 0.5rem;
}

.close-all-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  border: 1px solid rgba(239, 68, 68, 0.3);
  background: rgba(239, 68, 68, 0.05);
  color: #dc2626;
  font-size: 0.75rem;
  font-weight: 500;
  cursor: pointer;
  border-radius: 0.375rem;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.close-all-btn:hover {
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.5);
  color: #b91c1c;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(239, 68, 68, 0.2);
}

.close-all-btn:active {
  transform: translateY(0);
  box-shadow: 0 1px 2px rgba(239, 68, 68, 0.2);
}

.close-all-icon {
  width: 14px;
  height: 14px;
  flex-shrink: 0;
}

.close-all-text {
  font-size: 0.75rem;
  font-weight: 500;
}

.unified-tab-content {
  flex: 1;
  overflow: hidden;
  position: relative;
  min-height: 0;
  padding: 0 1rem 1rem;
}

.unified-tab-pane {
  height: 100%;
  overflow: hidden;
  min-height: 0;
}

.no-tab-info {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #6b7280;
  font-style: italic;
}

/* Tab 切换动画 */
.unified-tab-fade-enter-active {
  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.unified-tab-fade-leave-active {
  transition: all 0.2s cubic-bezier(0.55, 0.055, 0.675, 0.19);
}

.unified-tab-fade-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.unified-tab-fade-leave-to {
  opacity: 0;
  transform: translateY(-5px);
}

.unified-tab-fade-enter-to,
.unified-tab-fade-leave-from {
  opacity: 1;
  transform: translateY(0);
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .unified-tab-container {
    background: rgba(30, 41, 59, 0.9);
  }

  .unified-tab-list {
    border-bottom-color: rgba(129, 140, 248, 0.2);
  }

  .unified-tab-button {
    color: #a0aec0;
  }

  .unified-tab-button:hover {
    color: #e2e8f0;
    background: rgba(45, 55, 72, 0.3);
  }

  .unified-tab-button.active {
    color: #90cdf4;
    border-bottom-color: #90cdf4;
    background: rgba(144, 205, 244, 0.1);
  }

  .ai-source-tag {
    background: linear-gradient(135deg, rgba(129, 140, 248, 0.2) 0%, rgba(168, 85, 247, 0.2) 100%);
    color: #a5b4fc;
    box-shadow: 0 1px 3px rgba(129, 140, 248, 0.15);
  }

  .unified-tab-button.active .ai-source-tag {
    background: linear-gradient(135deg, rgba(129, 140, 248, 0.3) 0%, rgba(168, 85, 247, 0.3) 100%);
    color: #c7d2fe;
    box-shadow: 0 2px 6px rgba(129, 140, 248, 0.25);
  }

  .tab-close-btn {
    color: #94a3b8;
  }

  .tab-close-btn:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #fca5a5;
  }

  .no-tab-info {
    color: #9ca3af;
  }

  .close-all-btn {
    border-color: rgba(248, 113, 113, 0.3);
    background: rgba(248, 113, 113, 0.1);
    color: #f87171;
  }

  .close-all-btn:hover {
    background: rgba(248, 113, 113, 0.2);
    border-color: rgba(248, 113, 113, 0.5);
    color: #fca5a5;
    box-shadow: 0 2px 4px rgba(248, 113, 113, 0.3);
  }
}

:global(.dark) .unified-tab-container {
  background: rgba(30, 41, 59, 0.9);
}

:global(.dark) .unified-tab-list {
  border-bottom-color: rgba(129, 140, 248, 0.2);
}

:global(.dark) .unified-tab-button {
  color: #a0aec0;
}

:global(.dark) .unified-tab-button:hover {
  color: #e2e8f0;
  background: rgba(45, 55, 72, 0.3);
}

:global(.dark) .unified-tab-button.active {
  color: #90cdf4;
  border-bottom-color: #90cdf4;
  background: rgba(144, 205, 244, 0.1);
}

:global(.dark) .ai-source-tag {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.2) 0%, rgba(168, 85, 247, 0.2) 100%);
  color: #a5b4fc;
  box-shadow: 0 1px 3px rgba(129, 140, 248, 0.15);
}

:global(.dark) .unified-tab-button.active .ai-source-tag {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.3) 0%, rgba(168, 85, 247, 0.3) 100%);
  color: #c7d2fe;
  box-shadow: 0 2px 6px rgba(129, 140, 248, 0.25);
}

:global(.dark) .tab-close-btn {
  color: #94a3b8;
}

:global(.dark) .tab-close-btn:hover {
  background: rgba(239, 68, 68, 0.2);
  color: #fca5a5;
}

:global(.dark) .no-tab-info {
  color: #9ca3af;
}

:global(.dark) .close-all-btn {
  border-color: rgba(248, 113, 113, 0.3);
  background: rgba(248, 113, 113, 0.1);
  color: #f87171;
}

:global(.dark) .close-all-btn:hover {
  background: rgba(248, 113, 113, 0.2);
  border-color: rgba(248, 113, 113, 0.5);
  color: #fca5a5;
  box-shadow: 0 2px 4px rgba(248, 113, 113, 0.3);
}
</style>
