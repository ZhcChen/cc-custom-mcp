<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import Sidebar from './components/Sidebar.vue'

const router = useRouter()

interface FeedbackData {
  sessionId: string
  aiResponse: string
  context: string
  timestamp: string
}

let unlistenFeedbackRequest: (() => void) | null = null

// 全局事件监听器
onMounted(async () => {
  console.log('🚀 App mounted, setting up global event listeners...')

  try {
    // 监听来自 Tauri 后端的反馈请求事件
    unlistenFeedbackRequest = await listen<FeedbackData>('feedback-request', async (event) => {
      console.log('📡 Global: Received feedback-request event:', event)
      console.log('📦 Event payload:', event.payload)

      try {
        // 将窗口置顶到所有应用前面
        console.log('🔝 Bringing window to front...')
        try {
          await invoke('bring_window_to_front')
          console.log('✅ Window brought to front successfully')
        } catch (error) {
          console.error('❌ Failed to bring window to front:', error)
        }

        // 始终导航到 Feedback 页面（简化逻辑）
        console.log('🔄 Navigating to Feedback page...')
        router.push({
          path: '/feedback',
          query: {
            sessionId: event.payload.sessionId,
            aiResponse: encodeURIComponent(event.payload.aiResponse || ''),
            context: encodeURIComponent(event.payload.context || ''),
            timestamp: event.payload.timestamp || new Date().toISOString()
          }
        }).then(() => {
          console.log('✅ Navigation completed')
        }).catch((error) => {
          console.error('❌ Navigation failed:', error)
        })
      } catch (error) {
        console.error('❌ Error handling feedback-request event:', error)
      }
    })

    console.log('✅ Global event listeners setup complete')
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