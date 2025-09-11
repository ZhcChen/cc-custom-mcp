<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import { useFeedbackStore, type FeedbackData } from './stores/feedback'
import Sidebar from './components/Sidebar.vue'
import NotificationContainer from './components/NotificationContainer.vue'

const router = useRouter()
const feedbackStore = useFeedbackStore()

let unlistenFeedbackRequest: (() => void) | null = null
let unlistenWindowResized: (() => void) | null = null
let unlistenWindowMoved: (() => void) | null = null

// 窗口尺寸保存相关
let saveWindowSizeTimeout: NodeJS.Timeout | null = null

// 保存窗口尺寸（防抖处理）
async function saveWindowSizeDebounced() {
  if (saveWindowSizeTimeout) {
    clearTimeout(saveWindowSizeTimeout)
  }
  
  saveWindowSizeTimeout = setTimeout(async () => {
    try {
      await invoke('save_current_window_size')
      console.log('✅ Window size saved')
    } catch (error) {
      console.error('❌ Failed to save window size:', error)
    }
  }, 500) // 500ms防抖延迟
}

// 小窗口模式状态
const isCompactMode = ref(false)

// 初始化小窗口模式状态
async function initCompactMode() {
  const savedCompactMode = localStorage.getItem('mcp-manager-compact-mode')
  const shouldBeCompact = savedCompactMode === 'true'
  isCompactMode.value = shouldBeCompact
  
  console.log('🔧 Initializing compact mode on startup:', { savedCompactMode, shouldBeCompact })
  
  // 无论当前状态如何，都应用保存的设置以确保窗口大小正确
  try {
    await invoke('set_window_compact_mode', { compact: shouldBeCompact })
    console.log('✅ Applied window mode on startup:', shouldBeCompact ? 'compact' : 'normal')
  } catch (error) {
    console.error('❌ Failed to apply window mode on startup:', error)
  }
}

// 监听小窗口模式变化
function handleCompactModeChange(event: CustomEvent) {
  isCompactMode.value = event.detail.compactMode
}


// 全局事件监听器
onMounted(async () => {
  console.log('🚀 App mounted, setting up global event listeners...')

  // 初始化小窗口模式
  await initCompactMode()
  
  // 监听小窗口模式变化事件
  window.addEventListener('compact-mode-changed', handleCompactModeChange as EventListener)

  try {
    // 监听窗口尺寸变化事件
    unlistenWindowResized = await listen('tauri://resize', async () => {
      console.log('📐 Window resized, saving size...')
      await saveWindowSizeDebounced()
    })
    
    // 监听窗口移动事件
    unlistenWindowMoved = await listen('tauri://move', async () => {
      console.log('📐 Window moved, saving position...')
      await saveWindowSizeDebounced()
    })
    
    // 监听来自 Tauri 后端的反馈请求事件
    unlistenFeedbackRequest = await listen<FeedbackData>('feedback-request', async (event) => {
      console.log('📡 Global: Received feedback-request event:', event)
      console.log('📦 Event payload:', event.payload)
      console.log('📊 Current feedback tabs before adding:', feedbackStore.feedbackTabs.length)
      console.log('📊 Current active tab ID:', feedbackStore.activeTabId)

      try {
        // 播放系统提示音通知用户有新的feedback请求
        console.log('🔔 Playing notification sound for new feedback request...')
        invoke('play_notification_sound').then(() => {
          console.log('✅ Notification sound played successfully')
        }).catch((error) => {
          console.error('❌ Failed to play notification sound:', error)
        })

        // 将窗口置顶到所有应用前面（异步，不阻塞事件处理）
        console.log('🔝 Bringing window to front...')
        invoke('bring_window_to_front').then(() => {
          console.log('✅ Window brought to front successfully')
        }).catch((error) => {
          console.error('❌ Failed to bring window to front:', error)
        })

        // 添加到全局 store - 关键修复：优化并发场景下的切换逻辑
        console.log('📦 Adding feedback session to global store...')
        const isOnFeedbackPage = router.currentRoute.value.path === '/feedback'
        const currentTabCount = feedbackStore.feedbackTabs.length
        console.log('📊 Current state - isOnFeedbackPage:', isOnFeedbackPage, 'tabCount:', currentTabCount)

        // 智能自动切换逻辑：考虑输入框聚焦状态
        const hasNoTabs = currentTabCount === 0
        const hasNoActiveTab = !feedbackStore.activeTabId
        const isInputFocused = feedbackStore.isCurrentTabInputFocused()
        
        // 自动切换条件：
        // 1. 没有任何 tab 时
        // 2. 没有活动 tab 时  
        // 3. 有 tab 但用户没有在输入反馈时
        const shouldAutoSwitch = hasNoTabs || hasNoActiveTab || !isInputFocused

        console.log('🔄 Auto-switch decision:', {
          hasNoTabs,
          hasNoActiveTab, 
          isInputFocused,
          shouldAutoSwitch
        })

        feedbackStore.addFeedbackSession(event.payload, {
          autoSwitch: shouldAutoSwitch
        })

        console.log('📊 Feedback tabs after adding:', feedbackStore.feedbackTabs.length)
        console.log('📊 New active tab ID:', feedbackStore.activeTabId)

        // 确保 tab 内容可见性
        setTimeout(() => {
          feedbackStore.ensureActiveTabVisible()
          console.log('📊 After ensuring visibility - tabs:', feedbackStore.feedbackTabs.length)
        }, 200)

        // 导航到 Feedback 页面（如果不在该页面）
        if (!isOnFeedbackPage) {
          console.log('🔄 Navigating to Feedback page...')
          router.push('/feedback').then(() => {
            console.log('✅ Navigation completed')
            // 导航完成后，确保 tab 状态正确
            setTimeout(() => {
              feedbackStore.ensureActiveTabVisible()
            }, 200)
          }).catch((error) => {
            console.error('❌ Navigation failed:', error)
          })
        } else {
          console.log('✅ Already on Feedback page, session added to store')
        }
      } catch (error) {
        console.error('❌ Error handling feedback-request event:', error)
      }
    })

    console.log('✅ Global event listeners setup complete')
    
    // 在事件监听器设置完成后，手动扫描 pending 的 feedback 请求
    // 这确保了重启 GUI 后能正确加载之前未处理的请求
    console.log('🔍 Scanning for pending feedback requests...')
    try {
      const result = await invoke('scan_pending_feedback')
      console.log('✅ Pending feedback scan completed:', result)
    } catch (error) {
      console.error('❌ Failed to scan pending feedback requests:', error)
    }
  } catch (error) {
    console.error('❌ Failed to setup global event listeners:', error)
  }
})

onUnmounted(() => {
  // 清理事件监听器
  if (unlistenFeedbackRequest) {
    unlistenFeedbackRequest()
  }
  
  if (unlistenWindowResized) {
    unlistenWindowResized()
  }
  
  if (unlistenWindowMoved) {
    unlistenWindowMoved()
  }
  
  // 清理小窗口模式事件监听器
  window.removeEventListener('compact-mode-changed', handleCompactModeChange as EventListener)
  
  // 清理定时器
  if (saveWindowSizeTimeout) {
    clearTimeout(saveWindowSizeTimeout)
  }
  
  console.log('🧹 Global event listeners cleaned up')
})
</script>

<template>
  <div class="mcp-manager" :class="{ 'compact-mode': isCompactMode }">
    <!-- 左侧菜单栏组件 -->
    <Sidebar :compact="isCompactMode" />

    <!-- 右侧主内容区域 -->
    <main class="main-content" :class="{ 'compact-content': isCompactMode }">
      <router-view />
    </main>

    <!-- 通知容器 -->
    <NotificationContainer />
  </div>
</template>

<style scoped>
.mcp-manager {
  display: flex;
  min-height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  color: #2d3748;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

/* 右侧主内容区域 */
.main-content {
  flex: 1;
  margin-left: 280px; /* 为固定侧边栏留出空间 */
  padding: 2rem;
  overflow-y: auto;
  min-height: 100vh;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  transition: margin-left 0.3s ease, padding 0.3s ease;
}

/* 小窗口模式样式 */
.mcp-manager.compact-mode {
  /* 移除max-width限制，让内容占满窗口宽度 */
  overflow-x: hidden;
}

.main-content.compact-content {
  margin-left: 60px; /* 为压缩的侧边栏留出空间 */
  padding: 1rem; /* 减少内边距 */
}

/* 小窗口模式动画 - 已移除，因为窗口大小由Tauri控制 */

/* 响应式调整 - 已移除，因为不再限制内容宽度 */

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .mcp-manager {
    background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
    color: #e2e8f0;
  }

  .main-content {
    background: rgba(0, 0, 0, 0.2);
  }
}

/* 手动深色模式 */
.dark .mcp-manager {
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  color: #e2e8f0;
}

.dark .main-content {
  background: rgba(0, 0, 0, 0.2);
}
</style>