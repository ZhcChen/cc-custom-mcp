<template>
  <div class="feedback-page">
    <div class="feedback-container">
      <UnifiedTabContainer
        :active-tab-id="feedbackStore.activeTabId"
        :tabs="feedbackStore.feedbackTabs"
        :show-close-button="true"
        @close-tab="closeFeedbackTab"
        @close-all-tabs="closeAllFeedbackTabs"
        @tab-changed="onTabChanged"
        @update:active-tab-id="onTabChanged"
      />

      <!-- 空状态 - 修复：使用 computed 属性确保反应式更新 -->
      <div v-if="!feedbackStore.hasActiveFeedback" class="empty-state">
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
import { onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useFeedbackStore, type FeedbackData } from '../stores/feedback'
import UnifiedTabContainer from '../components/UnifiedTabContainer.vue'

const router = useRouter()
const feedbackStore = useFeedbackStore()

// 处理来自 URL 参数的反馈请求
function handleFeedbackRequest(data: FeedbackData) {
  console.log('🎯 Feedback request received in Feedback page:', data)
  feedbackStore.addFeedbackSession(data)
}

async function closeFeedbackTab(tabId: string) {
  console.log('🚫 Closing feedback tab:', tabId)
  
  // 首先尝试取消 feedback 请求
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('cancel_feedback', { sessionId: tabId })
    console.log(`✅ Feedback session cancelled successfully: ${tabId}`)
  } catch (error) {
    console.error(`❌ Failed to cancel feedback session ${tabId}:`, error)
    // 即使取消失败，也继续移除 tab
  }
  
  // 然后从 store 中移除
  feedbackStore.removeFeedbackSession(tabId)
}

async function closeAllFeedbackTabs() {
  console.log('🚫 Closing all feedback tabs...')
  
  const tabCount = feedbackStore.feedbackTabs.length
  if (tabCount === 0) {
    console.log('📭 No feedback tabs to close')
    return
  }

  console.log(`📊 Closing ${tabCount} feedback tabs`)
  
  try {
    // 使用 store 中的 closeAllSessions 方法
    await feedbackStore.closeAllSessions()
    console.log('✅ All feedback tabs closed successfully')
  } catch (error) {
    console.error('❌ Error closing all feedback tabs:', error)
    // 即使出错，也显示一个通知
    feedbackStore.addNotification('关闭全部会话时出现错误')
  }
}

function onTabChanged(tabId: string) {
  console.log('🔄 Feedback page onTabChanged:', tabId)
  console.log('📊 Before change - activeTabId:', feedbackStore.activeTabId)
  console.log('📊 Available tabs:', feedbackStore.feedbackTabs.map(t => ({ id: t.id, title: t.title })))

  feedbackStore.setActiveTab(tabId)

  // 验证更改是否生效
  setTimeout(() => {
    console.log('📊 After change - activeTabId:', feedbackStore.activeTabId)
    console.log('📊 Current tab:', feedbackStore.currentTab?.id)
  }, 100)
}





onMounted(() => {
  console.log('🚀 Feedback page mounting...')

  // 检查是否有 URL 参数传递的反馈数据
  const route = useRoute()
  if (route.query.sessionId) {
    console.log('📋 Found feedback data in URL parameters')
    const feedbackData: FeedbackData = {
      sessionId: route.query.sessionId as string,
      aiResponse: decodeURIComponent(route.query.aiResponse as string || ''),
      context: decodeURIComponent(route.query.context as string || ''),
      timestamp: route.query.timestamp as string || new Date().toISOString(),
      aiSource: route.query.aiSource as string,
      aiSourceDisplay: decodeURIComponent(route.query.aiSourceDisplay as string || '')
    }

    console.log('📝 Processing feedback data from URL:', feedbackData)
    handleFeedbackRequest(feedbackData)

    // 清理 URL 参数
    router.replace('/feedback')
  }

  console.log('✅ Feedback page mounted')
})


</script>

<style scoped>
.feedback-page {
  height: 100vh; /* 使用视口高度而不是 100% */
  display: flex;
  flex-direction: column;
  padding: 1.5rem;
  gap: 1.5rem;
  overflow: hidden; /* 防止整个页面滚动 */
}

.feedback-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0; /* 确保 flex 子元素可以收缩 */
  gap: 1rem;
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



@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
