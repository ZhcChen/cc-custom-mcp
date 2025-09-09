<template>
  <div v-if="tabs.length > 0" class="tab-container">
    <!-- Tab 头部 -->
    <div class="tab-header">
      <div class="tab-list">
        <div
          v-for="tab in tabs"
          :key="tab.id"
          class="tab-item"
          :class="{
            'active': tab.id === currentActiveTabId,
            'has-new': tab.isNew,
            'high-priority': tab.priority === 'high'
          }"
          :data-ai-source="tab.aiSource || 'unknown'"
          @click="setActiveTab(tab.id)"
          :title="getTabTooltip(tab)"
        >
          <span class="tab-title">
            {{ tab.title }}
          </span>
          <span class="ai-source-tag" :data-source="tab.aiSource || 'unknown ai'">
            {{ tab.aiSource || 'unknown ai' }}
          </span>
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
      <!-- 无活动 tab 时的提示 -->
      <div v-if="!currentTab" class="no-tab-info">
        <p>没有活动的 Tab</p>
      </div>

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
  isNew?: boolean
  priority?: 'normal' | 'high'
  aiSource?: string // AI 来源标识
  aiSourceDisplay?: string // AI 来源显示名称
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

const currentActiveTabId = computed(() => {
  // 如果有明确的 activeTabId 且对应的 tab 存在，使用它
  if (props.activeTabId && props.tabs.find(tab => tab.id === props.activeTabId)) {
    return props.activeTabId
  }
  // 如果没有明确的 activeTabId 或对应的 tab 不存在，但有 tabs，使用第一个
  // 这确保了总是有一个活动的 tab
  if (props.tabs.length > 0) {
    return props.tabs[0].id
  }
  return ''
})

const currentTab = computed((): TabItem | null => {
  const activeId = currentActiveTabId.value
  const tab = props.tabs.find(tab => tab.id === activeId)
  console.log('🔍 TabContainer currentTab computed:', {
    activeId,
    tab: tab?.id,
    totalTabs: props.tabs.length,
    propsActiveTabId: props.activeTabId
  })
  return tab || null
})

function setActiveTab(tabId: string) {
  console.log('🔄 TabContainer setActiveTab called:', tabId)
  console.log('📊 Current state:', {
    propsActiveTabId: props.activeTabId,
    newTabId: tabId,
    tabExists: props.tabs.find(tab => tab.id === tabId) ? 'yes' : 'no'
  })

  // 直接发送事件，不修改本地状态
  emit('update:activeTabId', tabId)
  emit('update:active-tab-id', tabId)
  emit('tab-changed', tabId)
  console.log('✅ TabContainer events emitted for tab:', tabId)
}

function closeTab(tabId: string) {
  emit('close-tab', tabId)
}

// 生成tab提示信息
function getTabTooltip(tab: TabItem): string {
  const source = tab.aiSourceDisplay || tab.aiSource || 'Unknown AI Tool'
  return `${tab.title} - 来源: ${source}`
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
  position: relative;
}

.tab-item.high-priority {
  border-color: rgba(239, 68, 68, 0.5);
}

.tab-item:hover {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(248, 250, 252, 0.95) 100%);
  border-color: rgba(102, 126, 234, 0.4);
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(102, 126, 234, 0.2);
}

.tab-item.active {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.2) 0%, rgba(118, 75, 162, 0.2) 100%);
  border-color: #667eea;
  border-bottom: 3px solid #667eea;
  font-weight: 700;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4), inset 0 1px 0 rgba(255, 255, 255, 0.9);
  transform: translateY(-2px);
  position: relative;
}

.tab-item.active::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
  border-radius: inherit;
  z-index: -1;
}



.tab-title {
  flex: 1;
  font-size: 0.875rem;
  font-weight: 500;
  color: #374151;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-right: 0.75rem;
}

/* AI 来源文字标签样式 - 简洁现代设计 */
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
  /* 柔和的渐变背景，无边框 */
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
  color: #667eea;
  border: none;
  box-shadow: 0 1px 3px rgba(102, 126, 234, 0.1);
}

/* 悬停效果 */
.ai-source-tag:hover {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.2) 0%, rgba(118, 75, 162, 0.2) 100%);
  color: #5a67d8;
  box-shadow: 0 2px 4px rgba(102, 126, 234, 0.15);
  transform: translateY(-1px);
}

/* Tab 统一的主题色左边框 - 与底部颜色一致 */
.tab-item[data-ai-source] {
  border-left: 3px solid rgba(102, 126, 234, 0.4);
}

.tab-item.active[data-ai-source] {
  border-left: 3px solid #667eea;
}

/* 激活状态下的源标签强化 - 更精致的效果 */
.tab-item.active .ai-source-tag {
  font-weight: 700;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.25) 0%, rgba(118, 75, 162, 0.25) 100%);
  color: #5a67d8;
  box-shadow: 0 2px 6px rgba(102, 126, 234, 0.2);
  transform: scale(1.05);
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
    background: linear-gradient(135deg, rgba(51, 65, 85, 0.95) 0%, rgba(30, 41, 59, 0.95) 100%);
    border-color: rgba(129, 140, 248, 0.5);
    transform: translateY(-1px);
    box-shadow: 0 2px 6px rgba(129, 140, 248, 0.2);
  }

  .tab-item.active {
    background: linear-gradient(135deg, rgba(129, 140, 248, 0.25) 0%, rgba(168, 85, 247, 0.25) 100%);
    border-color: #818cf8;
    border-bottom: 3px solid #818cf8;
    font-weight: 700;
    box-shadow: 0 4px 12px rgba(129, 140, 248, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.15);
    transform: translateY(-2px);
    position: relative;
  }

  .tab-item.active::before {
    background: linear-gradient(135deg, rgba(129, 140, 248, 0.15) 0%, rgba(168, 85, 247, 0.15) 100%);
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
  background: linear-gradient(135deg, rgba(51, 65, 85, 0.95) 0%, rgba(30, 41, 59, 0.95) 100%);
  border-color: rgba(129, 140, 248, 0.5);
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(129, 140, 248, 0.2);
}

:global(.dark) .tab-item.active {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.25) 0%, rgba(168, 85, 247, 0.25) 100%);
  border-color: #818cf8;
  border-bottom: 3px solid #818cf8;
  font-weight: 700;
  box-shadow: 0 4px 12px rgba(129, 140, 248, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.15);
  transform: translateY(-2px);
  position: relative;
}

:global(.dark) .tab-item.active::before {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.15) 0%, rgba(168, 85, 247, 0.15) 100%);
}



:global(.dark) .tab-title {
  color: #f1f5f9;
}

/* 深色模式下的 AI 来源标签 - 精致现代设计 */
:global(.dark) .ai-source-tag {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.2) 0%, rgba(168, 85, 247, 0.2) 100%);
  color: #a5b4fc;
  border: none;
  box-shadow: 0 1px 3px rgba(129, 140, 248, 0.15);
}

:global(.dark) .ai-source-tag:hover {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.25) 0%, rgba(168, 85, 247, 0.25) 100%);
  color: #c7d2fe;
  box-shadow: 0 2px 4px rgba(129, 140, 248, 0.2);
  transform: translateY(-1px);
}

:global(.dark) .tab-item.active .ai-source-tag {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.3) 0%, rgba(168, 85, 247, 0.3) 100%);
  color: #c7d2fe;
  box-shadow: 0 2px 6px rgba(129, 140, 248, 0.25);
  transform: scale(1.05);
}

/* 深色模式下 Tab 统一的主题色左边框 */
:global(.dark) .tab-item[data-ai-source] {
  border-left: 3px solid rgba(129, 140, 248, 0.4);
}

:global(.dark) .tab-item.active[data-ai-source] {
  border-left: 3px solid #818cf8;
}

:global(.dark) .tab-close {
  color: #94a3b8;
}

:global(.dark) .tab-close:hover {
  background: rgba(239, 68, 68, 0.2);
  color: #fca5a5;
}

.tab-fade-enter-active,
.tab-fade-leave-active {
  transition: opacity 0.2s ease;
}

.tab-fade-enter-from,
.tab-fade-leave-to {
  opacity: 0;
}
</style>
