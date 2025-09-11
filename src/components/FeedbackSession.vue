<template>
  <div class="feedback-session">
    <!-- 左侧：AI 回答 -->
    <div class="ai-response-panel">
      <div class="panel-header">
        <div class="header-content">
          <h3>{{ $t('feedback.aiResponse') }}</h3>
          <div class="header-meta">
            <span class="context-info">{{ context }}</span>
            <span v-if="props.aiSourceDisplay" class="ai-source-info">
              🤖 {{ props.aiSourceDisplay }}
            </span>
          </div>
        </div>
      </div>
      <div class="ai-response-content">
        <div class="response-text">{{ aiResponse }}</div>
        <div class="response-meta">
          <span class="timestamp">{{ formatTime(timestamp) }}</span>
          <span class="session-id">{{ sessionId.slice(0, 8) }}</span>
          <span v-if="props.aiSource" class="ai-source-tag" :data-source="props.aiSource">
            {{ props.aiSource }}
          </span>
        </div>
      </div>
    </div>

    <!-- 右侧：用户反馈 -->
    <div class="user-feedback-panel">
      <div class="panel-header">
        <h3>{{ $t('feedback.userFeedback') }}</h3>
        <button
          class="close-button"
          @click="handleClose"
          :title="$t('common.close')"
        >
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>
      
      <div class="feedback-content">
        <!-- 提交成功提示 -->
        <div v-if="submitted" class="success-message">
          <div class="success-icon">✓</div>
          <div class="success-text">{{ $t('feedback.submitted') }}</div>
        </div>

        <div v-else class="feedback-input-area">
          <textarea
            ref="feedbackInput"
            v-model="feedbackText"
            :placeholder="$t('feedback.placeholder')"
            class="feedback-textarea"
            @keydown="handleKeydown"
          ></textarea>
          
          <div class="input-actions">
            <div class="input-hint">
              {{ $t('feedback.hint') }}
            </div>
            <button
              class="send-button"
              @click="sendFeedback"
              :disabled="sending"
            >
              <svg v-if="!sending" viewBox="0 0 20 20" fill="currentColor">
                <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.429a1 1 0 001.169-1.409l-7-14z" />
              </svg>
              <svg v-else class="loading-spinner" viewBox="0 0 20 20">
                <path d="M10 3a7 7 0 100 14 7 7 0 000-14zM2 10a8 8 0 1116 0 8 8 0 01-16 0z" fill="currentColor" opacity="0.3"/>
                <path d="M10 2a8 8 0 018 8" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              </svg>
              {{ sending ? $t('feedback.sending') : $t('feedback.send') }}
            </button>
          </div>
        </div>

        <!-- 反馈历史 -->
        <div v-if="feedbackHistory.length > 0" class="feedback-history">
          <h4>{{ $t('feedback.history') }}</h4>
          <div class="history-list">
            <div
              v-for="(item, index) in feedbackHistory"
              :key="index"
              class="history-item"
            >
              <div class="history-content">{{ item.content }}</div>
              <div class="history-time">{{ formatTime(item.timestamp) }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Props {
  aiResponse: string
  context: string
  sessionId: string
  timestamp: string
  aiSource?: string
  aiSourceDisplay?: string
}

interface Emits {
  (e: 'close'): void
  (e: 'feedback', data: { content: string; sessionId: string }): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const feedbackInput = ref<HTMLTextAreaElement>()
const feedbackText = ref('')
const sending = ref(false)
const submitted = ref(false)
const feedbackHistory = ref<Array<{ content: string; timestamp: string }>>([])

// 标记会话是否已结束（提交或取消），防止重复操作
const sessionEnded = ref(false)



function formatTime(timestamp: string) {
  return new Date(timestamp).toLocaleString()
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Enter' && event.shiftKey) {
    event.preventDefault()
    sendFeedback()
  }
}





async function sendFeedback() {
  if (sending.value || sessionEnded.value) return

  sending.value = true
  // 不要立即设置 sessionEnded，等提交成功后再设置

  try {
    const feedbackContent = feedbackText.value.trim() || '(无内容)'
    await invoke('submit_feedback', {
      sessionId: props.sessionId,
      feedbackContent: feedbackContent
    })

    feedbackHistory.value.push({
      content: feedbackContent,
      timestamp: new Date().toISOString()
    })

    emit('feedback', {
      content: feedbackContent,
      sessionId: props.sessionId
    })

    submitted.value = true
    sessionEnded.value = true  // 只有提交成功后才设置为已结束

    setTimeout(() => {
      emit('close')
    }, 1500) // 缩短延迟

  } catch (error) {
    console.error('Failed to send feedback:', error)
    // 失败时不设置 sessionEnded，允许重试
  } finally {
    sending.value = false
  }
}

async function cancelFeedback() {
  if (sessionEnded.value) {
    console.log(`⚠️ Session ${props.sessionId} already ended, skipping cancel`)
    return
  }
  
  console.log(`🚫 Cancelling feedback session: ${props.sessionId}`)
  sessionEnded.value = true

  try {
    await invoke('cancel_feedback', { sessionId: props.sessionId })
    console.log(`✅ Feedback session cancelled successfully: ${props.sessionId}`)
  } catch (error) {
    console.error(`❌ Failed to cancel feedback session ${props.sessionId}:`, error)
    // 即便取消失败，也认为会话已尝试结束
  }
}

function handleClose() {
  // 用户通过组件内关闭按钮关闭 tab
  console.log(`🚫 User closing feedback session via component close button: ${props.sessionId}`)
  cancelFeedback()
  emit('close')
}

onMounted(() => {
  nextTick(() => {
    feedbackInput.value?.focus()
  })
})

onUnmounted(() => {
  // 移除自动取消逻辑，只在用户主动关闭时才取消
  // 组件卸载时不再自动取消 feedback 会话
  console.log(`📝 FeedbackSession component unmounted for session: ${props.sessionId}`)
})
</script>

<style scoped>
.feedback-session {
  display: flex;
  height: 100%;
  gap: 1rem;
  padding: 1rem;
  min-height: 0; /* 确保 flex 子元素可以收缩 */
  max-height: calc(100vh - 180px); /* 更严格的高度限制，减去页面padding和tab高度 */
  overflow: hidden; /* 防止整个会话区域溢出 */
}

.ai-response-panel,
.user-feedback-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 0.75rem;
  border: 1px solid rgba(209, 213, 219, 0.3);
  overflow: hidden;
  min-height: 0; /* 确保 flex 子元素可以收缩 */
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.9) 0%, rgba(248, 250, 252, 0.9) 100%);
  border-bottom: 1px solid rgba(209, 213, 219, 0.3);
}

.header-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.header-meta {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.panel-header h3 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: #374151;
}

.context-info {
  font-size: 0.75rem;
  color: #6b7280;
  background: rgba(102, 126, 234, 0.1);
  padding: 0.25rem 0.5rem;
  border-radius: 0.375rem;
}

.ai-source-info {
  font-size: 0.75rem;
  color: #059669;
  background: rgba(16, 185, 129, 0.1);
  padding: 0.25rem 0.5rem;
  border-radius: 0.375rem;
  font-weight: 500;
}

.ai-source-tag {
  font-size: 0.7rem;
  padding: 0.25rem 0.6rem;
  border-radius: 0.4rem;
  font-weight: 600;
  border: none;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
  color: #667eea;
  box-shadow: 0 1px 3px rgba(102, 126, 234, 0.1);
  transition: all 0.2s ease;
}

.ai-source-tag:hover {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.2) 0%, rgba(118, 75, 162, 0.2) 100%);
  color: #5a67d8;
  box-shadow: 0 2px 4px rgba(102, 126, 234, 0.15);
  transform: translateY(-1px);
}

.close-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: none;
  color: #6b7280;
  cursor: pointer;
  border-radius: 0.25rem;
  transition: all 0.2s ease;
}

.close-button:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.close-button svg {
  width: 16px;
  height: 16px;
}

.ai-response-content {
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
  min-height: 0; /* 确保可以收缩到内容区域 */
  /* 使用更保守的高度计算，确保不会导致页面滚动 */
  max-height: calc(100vh - 320px); /* 进一步减少高度，给其他元素留出足够空间 */
}

.response-text {
  font-size: 0.875rem;
  line-height: 1.6;
  color: #374151;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.response-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid rgba(209, 213, 219, 0.3);
  font-size: 0.75rem;
  color: #6b7280;
}

.feedback-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 1rem;
  gap: 1rem;
  min-height: 0; /* 确保可以收缩 */
  overflow-y: auto; /* 右侧反馈区域也可以独立滚动 */
}

.success-message {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  text-align: center;
  gap: 1rem;
}

.success-icon {
  width: 64px;
  height: 64px;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2rem;
  color: white;
  font-weight: bold;
  box-shadow: 0 8px 25px rgba(16, 185, 129, 0.3);
  animation: successPulse 0.6s ease-out;
}

.success-text {
  font-size: 1.125rem;
  font-weight: 600;
  color: #10b981;
}

@keyframes successPulse {
  0% {
    transform: scale(0.8);
    opacity: 0;
  }
  50% {
    transform: scale(1.1);
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}

.feedback-input-area {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.feedback-textarea {
  width: 100%;
  min-height: 240px; /* 从 120px 增加到 240px，高度翻倍 */
  padding: 0.75rem;
  border: 1px solid rgba(209, 213, 219, 0.5);
  border-radius: 0.5rem;
  background: rgba(255, 255, 255, 0.9);
  font-size: 0.875rem;
  line-height: 1.5;
  resize: vertical;
  transition: all 0.2s ease;
}

.feedback-textarea:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.input-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.input-hint {
  font-size: 0.75rem;
  color: #6b7280;
}

.send-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.send-button:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.send-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.send-button svg {
  width: 16px;
  height: 16px;
}

.loading-spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.feedback-history {
  flex: 1;
  overflow: hidden;
}

.feedback-history h4 {
  margin: 0 0 0.75rem 0;
  font-size: 0.875rem;
  font-weight: 600;
  color: #374151;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  max-height: 200px;
  overflow-y: auto;
}

.history-item {
  padding: 0.75rem;
  background: rgba(248, 250, 252, 0.8);
  border-radius: 0.5rem;
  border: 1px solid rgba(209, 213, 219, 0.2);
}

.history-content {
  font-size: 0.875rem;
  color: #374151;
  margin-bottom: 0.25rem;
}

.history-time {
  font-size: 0.75rem;
  color: #6b7280;
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .ai-response-panel,
  .user-feedback-panel {
    background: rgba(30, 41, 59, 0.8);
    border-color: rgba(129, 140, 248, 0.2);
  }

  .panel-header {
    background: linear-gradient(135deg, rgba(30, 41, 59, 0.9) 0%, rgba(15, 23, 42, 0.9) 100%);
    border-bottom-color: rgba(129, 140, 248, 0.2);
  }

  .panel-header h3 {
    color: #f1f5f9;
  }

  .context-info {
    color: #cbd5e0;
    background: rgba(129, 140, 248, 0.2);
  }

  .ai-source-info {
    color: #6ee7b7;
    background: rgba(16, 185, 129, 0.2);
  }

  .ai-source-tag {
    background: linear-gradient(135deg, rgba(129, 140, 248, 0.2) 0%, rgba(168, 85, 247, 0.2) 100%);
    color: #a5b4fc;
    box-shadow: 0 1px 3px rgba(129, 140, 248, 0.15);
  }

  .ai-source-tag:hover {
    background: linear-gradient(135deg, rgba(129, 140, 248, 0.25) 0%, rgba(168, 85, 247, 0.25) 100%);
    color: #c7d2fe;
    box-shadow: 0 2px 4px rgba(129, 140, 248, 0.2);
  }

  .close-button {
    color: #94a3b8;
  }

  .response-text {
    color: #e2e8f0;
  }

  .response-meta {
    border-top-color: rgba(129, 140, 248, 0.2);
    color: #94a3b8;
  }

  .feedback-textarea {
    background: rgba(51, 65, 85, 0.8);
    border-color: rgba(129, 140, 248, 0.3);
    color: #f1f5f9;
  }

  .feedback-textarea:focus {
    border-color: #818cf8;
    box-shadow: 0 0 0 3px rgba(129, 140, 248, 0.2);
  }

  .input-hint {
    color: #94a3b8;
  }

  .feedback-history h4 {
    color: #f1f5f9;
  }

  .history-item {
    background: rgba(51, 65, 85, 0.6);
    border-color: rgba(129, 140, 248, 0.2);
  }

  .history-content {
    color: #e2e8f0;
  }

  .history-time {
    color: #94a3b8;
  }
}

:global(.dark) .ai-response-panel,
:global(.dark) .user-feedback-panel {
  background: rgba(30, 41, 59, 0.8);
  border-color: rgba(129, 140, 248, 0.2);
}

:global(.dark) .panel-header {
  background: linear-gradient(135deg, rgba(30, 41, 59, 0.9) 0%, rgba(15, 23, 42, 0.9) 100%);
  border-bottom-color: rgba(129, 140, 248, 0.2);
}

:global(.dark) .panel-header h3 {
  color: #f1f5f9;
}

:global(.dark) .context-info {
  color: #cbd5e0;
  background: rgba(129, 140, 248, 0.2);
}

:global(.dark) .ai-source-tag {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.2) 0%, rgba(168, 85, 247, 0.2) 100%);
  color: #a5b4fc;
  box-shadow: 0 1px 3px rgba(129, 140, 248, 0.15);
}

:global(.dark) .ai-source-tag:hover {
  background: linear-gradient(135deg, rgba(129, 140, 248, 0.25) 0%, rgba(168, 85, 247, 0.25) 100%);
  color: #c7d2fe;
  box-shadow: 0 2px 4px rgba(129, 140, 248, 0.2);
}

:global(.dark) .close-button {
  color: #94a3b8;
}

:global(.dark) .response-text {
  color: #e2e8f0;
}

:global(.dark) .response-meta {
  border-top-color: rgba(129, 140, 248, 0.2);
  color: #94a3b8;
}

:global(.dark) .feedback-textarea {
  background: rgba(51, 65, 85, 0.8);
  border-color: rgba(129, 140, 248, 0.3);
  color: #f1f5f9;
}

:global(.dark) .feedback-textarea:focus {
  border-color: #818cf8;
  box-shadow: 0 0 0 3px rgba(129, 140, 248, 0.2);
}

:global(.dark) .input-hint {
  color: #94a3b8;
}

:global(.dark) .feedback-history h4 {
  color: #f1f5f9;
}

:global(.dark) .history-item {
  background: rgba(51, 65, 85, 0.6);
  border-color: rgba(129, 140, 248, 0.2);
}

:global(.dark) .history-content {
  color: #e2e8f0;
}

:global(.dark) .history-time {
  color: #94a3b8;
}
</style>
