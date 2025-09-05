<template>
  <div class="feedback-page">
    <div class="page-header">
      <h2>{{ $t('feedback.title') }}</h2>
      <p class="page-subtitle">{{ $t('feedback.subtitle') }}</p>
    </div>

    <div class="feedback-container">
      <TabContainer
        v-model:active-tab-id="activeTabId"
        :tabs="feedbackTabs"
        @close-tab="closeFeedbackTab"
        @tab-changed="onTabChanged"
      />
      
      <!-- 空状态 -->
      <div v-if="feedbackTabs.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-3.582 8-8 8a8.959 8.959 0 01-4.906-1.681L3 21l2.681-5.094A8.959 8.959 0 013 12c0-4.418 3.582-8 8-8s8 3.582 8 8z" />
          </svg>
        </div>
        <h3>{{ $t('feedback.empty.title') }}</h3>
        <p>{{ $t('feedback.empty.description') }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { useRoute, useRouter } from 'vue-router'
import TabContainer from '../components/TabContainer.vue'
import FeedbackSession from '../components/FeedbackSession.vue'

// const { t } = useI18n() // 暂时注释掉未使用的 i18n
// const route = useRoute() // 暂时注释掉未使用的 route
const router = useRouter()

interface FeedbackData {
  sessionId: string
  aiResponse: string
  context: string
  timestamp: string
}

const feedbackTabs = ref<Array<{
  id: string
  title: string
  component: any
  props: any
}>>([])

const activeTabId = ref('')
let unlistenFeedbackRequest: (() => void) | null = null

// 监听来自 MCP 服务器的反馈请求
async function handleFeedbackRequest(data: FeedbackData) {
  console.log('🎯 Feedback request received:', data)

  const { sessionId, aiResponse, context, timestamp } = data

  console.log('📝 Creating new feedback tab:', {
    sessionId,
    context,
    aiResponseLength: aiResponse?.length || 0,
    timestamp
  })

  // 将窗口置顶到所有应用前面
  console.log('🔝 Bringing window to front...')
  try {
    await invoke('bring_window_to_front')
    console.log('✅ Window brought to front successfully')
  } catch (error) {
    console.error('❌ Failed to bring window to front:', error)
  }

  // 创建新的 tab
  const newTab = {
    id: sessionId,
    title: context || `Feedback ${feedbackTabs.value.length + 1}`,
    component: FeedbackSession,
    props: {
      sessionId,
      aiResponse,
      context,
      timestamp
    },
    events: {
      feedback: handleFeedbackSubmit
    }
  }

  feedbackTabs.value.push(newTab)
  activeTabId.value = sessionId

  console.log('✅ Tab created and activated:', {
    tabId: sessionId,
    totalTabs: feedbackTabs.value.length,
    activeTabId: activeTabId.value
  })

  // 播放系统提示音
  console.log('🔊 Playing notification sound...')
  try {
    await invoke('play_notification_sound')
    console.log('✅ Notification sound played successfully')
  } catch (error) {
    console.error('❌ Failed to play notification sound:', error)
  }
}

function closeFeedbackTab(tabId: string) {
  const index = feedbackTabs.value.findIndex(tab => tab.id === tabId)
  if (index !== -1) {
    feedbackTabs.value.splice(index, 1)
    
    // 如果关闭的是当前活动的 tab，切换到其他 tab
    if (activeTabId.value === tabId) {
      if (feedbackTabs.value.length > 0) {
        // 优先选择下一个 tab，如果没有则选择上一个
        const nextIndex = index < feedbackTabs.value.length ? index : index - 1
        activeTabId.value = feedbackTabs.value[nextIndex].id
      } else {
        activeTabId.value = ''
      }
    }
  }
}

function onTabChanged(tabId: string) {
  console.log('Tab changed to:', tabId)
}

function handleFeedbackSubmit(data: { content: string; sessionId: string }) {
  console.log('Feedback submitted:', data)
  
  // 这里可以将反馈发送回 AI 或保存到本地
  // 暂时只是记录日志
}

onMounted(async () => {
  console.log('🚀 Feedback page mounting...')

  // 检查是否有 URL 参数传递的反馈数据
  const route = useRoute()
  if (route.query.sessionId) {
    console.log('📋 Found feedback data in URL parameters')
    const feedbackData: FeedbackData = {
      sessionId: route.query.sessionId as string,
      aiResponse: decodeURIComponent(route.query.aiResponse as string || ''),
      context: decodeURIComponent(route.query.context as string || ''),
      timestamp: route.query.timestamp as string || new Date().toISOString()
    }

    console.log('📝 Processing feedback data from URL:', feedbackData)
    await handleFeedbackRequest(feedbackData)

    // 清理 URL 参数
    router.replace('/feedback')
  }

  try {
    // 监听来自 Tauri 后端的反馈请求事件
    unlistenFeedbackRequest = await listen<FeedbackData>('feedback-request', async (event) => {
      console.log('📡 Received feedback-request event:', event)
      await handleFeedbackRequest(event.payload)
    })

    console.log('✅ Feedback page mounted, listening for feedback requests')
    console.log('🎧 Event listener setup complete')
  } catch (error) {
    console.error('❌ Failed to setup feedback event listeners:', error)
  }
})

onUnmounted(() => {
  // 清理事件监听器
  if (unlistenFeedbackRequest) {
    unlistenFeedbackRequest()
    unlistenFeedbackRequest = null
  }
})
</script>

<style scoped>
.feedback-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 1.5rem;
  gap: 1.5rem;
}

.page-header {
  text-align: center;
}

.page-header h2 {
  margin: 0 0 0.5rem 0;
  font-size: 2rem;
  font-weight: 700;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.page-subtitle {
  margin: 0;
  font-size: 1rem;
  color: #6b7280;
  font-weight: 400;
}

.feedback-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 3rem;
  color: #6b7280;
}

.empty-icon {
  width: 64px;
  height: 64px;
  margin-bottom: 1rem;
  color: #d1d5db;
}

.empty-icon svg {
  width: 100%;
  height: 100%;
}

.empty-state h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: #374151;
}

.empty-state p {
  margin: 0;
  font-size: 0.875rem;
  max-width: 400px;
  line-height: 1.5;
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .page-subtitle {
    color: #9ca3af;
  }

  .empty-state {
    color: #9ca3af;
  }

  .empty-icon {
    color: #4b5563;
  }

  .empty-state h3 {
    color: #f3f4f6;
  }
}

:global(.dark) .page-subtitle {
  color: #9ca3af;
}

:global(.dark) .empty-state {
  color: #9ca3af;
}

:global(.dark) .empty-icon {
  color: #4b5563;
}

:global(.dark) .empty-state h3 {
  color: #f3f4f6;
}
</style>
