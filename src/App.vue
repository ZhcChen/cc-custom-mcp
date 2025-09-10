<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import { useFeedbackStore, type FeedbackData } from './stores/feedback'
import Sidebar from './components/Sidebar.vue'
import NotificationContainer from './components/NotificationContainer.vue'

const router = useRouter()
const feedbackStore = useFeedbackStore()

let unlistenFeedbackRequest: (() => void) | null = null


// 全局事件监听器
onMounted(async () => {
  console.log('🚀 App mounted, setting up global event listeners...')

  try {
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

        // 简化的自动切换逻辑：只有在第一个tab或没有活动tab时才自动切换
        const shouldAutoSwitch = currentTabCount === 0 || !feedbackStore.activeTabId

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
    console.log('🧹 Global event listeners cleaned up')
  }
})
</script>

<template>
  <div class="mcp-manager">
    <!-- 左侧菜单栏组件 -->
    <Sidebar />

    <!-- 右侧主内容区域 -->
    <main class="main-content">
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
}

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