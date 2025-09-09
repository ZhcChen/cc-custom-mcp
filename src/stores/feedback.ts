import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import FeedbackSession from '../components/FeedbackSession.vue'

export interface FeedbackData {
  sessionId: string
  aiResponse: string
  context: string
  timestamp: string
  aiSource?: string
  aiSourceDisplay?: string
}

export interface FeedbackTab {
  id: string
  title: string
  component: any
  props: any
  events?: Record<string, Function>
  isNew?: boolean // 标记是否为新的未查看的 feedback
  priority?: 'normal' | 'high' // 优先级
  aiSource?: string // AI 来源标识
  aiSourceDisplay?: string // AI 来源显示名称
}

export const useFeedbackStore = defineStore('feedback', () => {
  // 状态
  const feedbackTabs = ref<FeedbackTab[]>([])
  const activeTabId = ref<string>('')
  const notifications = ref<string[]>([]) // 新增：通知队列

  // 计算属性
  const currentTab = computed(() => {
    return feedbackTabs.value.find(tab => tab.id === activeTabId.value) || null
  })

  const hasActiveFeedback = computed(() => {
    // 检查是否有活跃的（未提交且未取消的）反馈会话
    return feedbackTabs.value.some(() => {
      // 如果 tab 的 props 中有状态信息，检查状态
      // 这里我们简单检查 tab 是否存在，因为已完成的 tab 会被自动移除
      return true
    })
  })

  const pendingFeedbackCount = computed(() => {
    return feedbackTabs.value.length
  })

  const newFeedbackCount = computed(() => {
    return feedbackTabs.value.filter(tab => tab.isNew).length
  })

  // 方法
  function addFeedbackSession(data: FeedbackData, options: { autoSwitch?: boolean } = {}) {
    console.log('📝 Adding feedback session to store:', data)
    console.log('📊 Current state - tabs:', feedbackTabs.value.length, 'activeTabId:', activeTabId.value)

    // 检查是否已存在相同的会话
    const existingIndex = feedbackTabs.value.findIndex(tab => tab.id === data.sessionId)

    if (existingIndex !== -1) {
      console.log('⚠️ Session already exists, updating:', data.sessionId)
      // 更新现有会话
      feedbackTabs.value[existingIndex].props = {
        sessionId: data.sessionId,
        aiResponse: data.aiResponse,
        context: data.context,
        timestamp: data.timestamp,
        aiSource: data.aiSource,
        aiSourceDisplay: data.aiSourceDisplay
      }
      // 更新tab的AI来源信息
      feedbackTabs.value[existingIndex].aiSource = data.aiSource
      feedbackTabs.value[existingIndex].aiSourceDisplay = data.aiSourceDisplay
      // 标记为新的
      feedbackTabs.value[existingIndex].isNew = true

      // 如果当前没有活动 tab 或明确要求自动切换，则切换到这个 tab
      if (!activeTabId.value || options.autoSwitch === true) {
        console.log('🔄 Switching to updated session:', data.sessionId)
        activeTabId.value = data.sessionId
      }

      return
    }

    // 创建新的 tab
    const newTab: FeedbackTab = {
      id: data.sessionId,
      title: data.context || `Feedback ${feedbackTabs.value.length + 1}`,
      component: FeedbackSession,
      props: {
        sessionId: data.sessionId,
        aiResponse: data.aiResponse,
        context: data.context,
        timestamp: data.timestamp,
        aiSource: data.aiSource,
        aiSourceDisplay: data.aiSourceDisplay
      },
      events: {
        feedback: handleFeedbackSubmit
      },
      isNew: true, // 标记为新的
      priority: 'normal',
      aiSource: data.aiSource, // 添加AI来源信息到tab
      aiSourceDisplay: data.aiSourceDisplay // 添加AI来源显示名称到tab
    }

    // 优化的并发切换逻辑：
    // 1. 如果是第一个 tab，总是切换
    // 2. 如果当前没有活动 tab，切换到新 tab
    // 3. 如果明确要求自动切换，切换到新 tab
    // 4. 对于并发场景：
    //    - 如果当前活动 tab 是新的（未查看），不自动切换，让用户先处理当前的
    //    - 如果当前活动 tab 已查看，可以自动切换
    const isFirstTab = feedbackTabs.value.length === 0
    const hasNoActiveTab = !activeTabId.value
    const explicitAutoSwitch = options.autoSwitch === true
    
    // 检查当前活动 tab 的状态
    const currentActiveTab = feedbackTabs.value.find(tab => tab.id === activeTabId.value)
    const currentTabIsNew = currentActiveTab?.isNew || false
    
    // 智能切换逻辑：在并发场景下更加智能
    const shouldAutoSwitch = isFirstTab || 
                            hasNoActiveTab || 
                            explicitAutoSwitch ||
                            (!currentTabIsNew && feedbackTabs.value.length < 2) // 当前 tab 已查看且 tab 数量少时自动切换

    feedbackTabs.value.push(newTab)
    console.log('➕ New tab added. Total tabs:', feedbackTabs.value.length)

    if (shouldAutoSwitch) {
      console.log('🔄 Auto-switching to new session:', data.sessionId)
      activeTabId.value = data.sessionId
    } else {
      console.log('📢 New tab added without switching - better UX for concurrent requests')
      // 不自动切换，但添加通知让用户知道有新的feedback
      addNotification(`新的 Feedback 请求: ${data.context || 'Feedback'}`)

      // 确保至少有一个活动tab
      if (!activeTabId.value) {
        console.log('🎯 No active tab, forcing activation of new tab:', data.sessionId)
        activeTabId.value = data.sessionId
      }
    }

    console.log('✅ Feedback session added to store:', {
      sessionId: data.sessionId,
      totalTabs: feedbackTabs.value.length,
      activeTabId: activeTabId.value,
      autoSwitched: shouldAutoSwitch,
      currentTabWasNew: currentTabIsNew
    })
  }

  function removeFeedbackSession(sessionId: string) {
    console.log('🗑️ Removing feedback session from store:', sessionId)
    
    const index = feedbackTabs.value.findIndex(tab => tab.id === sessionId)
    if (index !== -1) {
      feedbackTabs.value.splice(index, 1)
      
      // 如果关闭的是当前活动的 tab，切换到其他 tab
      if (activeTabId.value === sessionId) {
        if (feedbackTabs.value.length > 0) {
          // 优先选择下一个 tab，如果没有则选择上一个
          const nextIndex = index < feedbackTabs.value.length ? index : index - 1
          activeTabId.value = feedbackTabs.value[nextIndex].id
        } else {
          activeTabId.value = ''
        }
      }

      console.log('✅ Feedback session removed from store:', {
        sessionId,
        remainingTabs: feedbackTabs.value.length,
        newActiveTabId: activeTabId.value
      })
    }
  }

  function setActiveTab(tabId: string) {
    console.log('🔄 Setting active tab in store:', tabId)

    // 验证 tab 是否存在
    const tab = feedbackTabs.value.find(t => t.id === tabId)
    if (!tab) {
      console.warn('⚠️ Attempted to set active tab that does not exist:', tabId)
      return
    }

    activeTabId.value = tabId

    // 标记当前 tab 为已查看
    if (tab.isNew) {
      tab.isNew = false
      console.log('✅ Tab marked as viewed:', tabId)
    }

    console.log('✅ Active tab set successfully:', tabId)
  }

  function handleFeedbackSubmit(data: { content: string; sessionId: string }) {
    console.log('📤 Feedback submitted from store:', data)
    
    // 这里可以添加额外的处理逻辑
    // 比如通知其他组件、更新状态等
  }

  function clearAllSessions() {
    console.log('🧹 Clearing all feedback sessions from store')
    feedbackTabs.value = []
    activeTabId.value = ''
  }

  // 获取特定会话的信息
  function getSession(sessionId: string) {
    return feedbackTabs.value.find(tab => tab.id === sessionId)
  }

  // 检查会话是否存在
  function hasSession(sessionId: string): boolean {
    return feedbackTabs.value.some(tab => tab.id === sessionId)
  }

  // 新增：通知管理
  function addNotification(message: string) {
    notifications.value.push(message)
    console.log('🔔 Notification added:', message)

    // 自动清除通知（5秒后）
    setTimeout(() => {
      removeNotification(message)
    }, 5000)
  }

  function removeNotification(message: string) {
    const index = notifications.value.indexOf(message)
    if (index > -1) {
      notifications.value.splice(index, 1)
      console.log('🔕 Notification removed:', message)
    }
  }

  function clearNotifications() {
    notifications.value = []
    console.log('🧹 All notifications cleared')
  }

  // 新增：批量操作
  function markAllAsViewed() {
    feedbackTabs.value.forEach(tab => {
      tab.isNew = false
    })
    console.log('👁️ All feedback sessions marked as viewed')
  }

  // 新增：强制刷新 tab 内容
  function refreshTabContent(tabId: string) {
    const tab = feedbackTabs.value.find(t => t.id === tabId)
    if (tab) {
      // 通过更新 props 来强制重新渲染
      tab.props = { ...tab.props }
      console.log('🔄 Tab content refreshed:', tabId)
    }
  }

  // 新增：确保活动 tab 的内容可见
  function ensureActiveTabVisible() {
    if (activeTabId.value && feedbackTabs.value.length > 0) {
      const activeTab = feedbackTabs.value.find(t => t.id === activeTabId.value)
      if (!activeTab) {
        // 如果当前活动 tab 不存在，切换到第一个可用的 tab
        const firstTab = feedbackTabs.value[0]
        if (firstTab) {
          console.log('🔄 Active tab not found, switching to first available:', firstTab.id)
          setActiveTab(firstTab.id)
        }
      } else {
        // 刷新活动 tab 的内容
        refreshTabContent(activeTabId.value)
      }
    }
  }



  return {
    // 状态
    feedbackTabs,
    activeTabId,
    notifications,

    // 计算属性
    currentTab,
    hasActiveFeedback,
    pendingFeedbackCount,
    newFeedbackCount,

    // 方法
    addFeedbackSession,
    removeFeedbackSession,
    setActiveTab,
    handleFeedbackSubmit,
    clearAllSessions,
    getSession,
    hasSession,
    addNotification,
    removeNotification,
    clearNotifications,
    markAllAsViewed,
    refreshTabContent,
    ensureActiveTabVisible
  }
})
