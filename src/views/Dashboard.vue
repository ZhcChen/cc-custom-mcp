<template>
  <div class="dashboard">
    <header class="page-header">
      <h2>{{ $t('dashboard.title') }}</h2>
      <p class="page-subtitle">{{ $t('dashboard.subtitle') }}</p>
    </header>

    <!-- 服务器控制面板 -->
    <section class="server-control">
      <div class="card">
        <div class="card-header">
          <h3>{{ $t('dashboard.serverControl.title') }}</h3>
          <div class="server-status" :class="serverStatus">
            <span class="status-dot"></span>
            {{ serverStatus === 'running' ? $t('dashboard.serverControl.serverRunning') : $t('dashboard.serverControl.serverStopped') }}
          </div>
        </div>
        <div class="card-content">
          <div class="control-buttons">
            <button
              @click="startServer"
              :disabled="serverStatus === 'running' || loading"
              class="btn btn-primary"
            >
              {{ loading ? $t('status.starting') : $t('dashboard.serverControl.startServer') }}
            </button>
            <button
              @click="stopServer"
              :disabled="serverStatus === 'stopped' || loading"
              class="btn btn-secondary"
            >
              {{ $t('dashboard.serverControl.stopServer') }}
            </button>
            <button @click="refreshStatus" class="btn btn-outline">
              {{ $t('dashboard.serverControl.refreshStatus') }}
            </button>
          </div>
        </div>
      </div>
    </section>

    <!-- 工具列表 -->
    <section class="tools-overview">
      <div class="card">
        <div class="card-header">
          <h3>{{ $t('dashboard.toolsOverview.title') }}</h3>
          <span class="tool-count">{{ $t('dashboard.toolsOverview.toolsCount', { count: tools.length }) }}</span>
        </div>
        <div class="card-content">
          <div class="tool-grid" v-if="tools.length > 0">
            <div v-for="tool in tools" :key="tool.name" class="tool-card">
              <div class="tool-header">
                <h4>{{ tool.name }}</h4>
                <span class="tool-badge">{{ $t('dashboard.toolsOverview.toolBadge') }}</span>
              </div>
              <p class="tool-description">{{ getToolDescription(tool.name, tool.description) }}</p>
              <details class="tool-schema">
                <summary>{{ $t('dashboard.toolsOverview.schema') }}</summary>
                <pre>{{ JSON.stringify(tool.inputSchema, null, 2) }}</pre>
              </details>
            </div>
          </div>
          <div v-else class="empty-state">
            <div class="empty-icon">🛠️</div>
            <p>{{ $t('dashboard.toolsOverview.noTools') }}</p>
            <p class="empty-hint">{{ $t('dashboard.toolsOverview.noToolsHint') }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- 配置信息 -->
    <section class="config-section">
      <div class="card">
        <div class="card-header">
          <h3>{{ $t('dashboard.configuration.title') }}</h3>
          <div class="config-buttons">
            <button @click="copyCursorConfig" class="btn btn-outline btn-sm">
              {{ copiedCursor ? $t('dashboard.configuration.copied') : '📋 ' + $t('dashboard.configuration.cursorConfig') }}
            </button>
            <button @click="copyAugmentConfig" class="btn btn-outline btn-sm">
              {{ copiedAugment ? $t('dashboard.configuration.copied') : '📋 ' + $t('dashboard.configuration.augmentConfig') }}
            </button>
            <button @click="copyClaudeConfig" class="btn btn-outline btn-sm">
              {{ copiedClaude ? $t('dashboard.configuration.copied') : '📋 Claude Desktop' }}
            </button>
            <button @click="copyChatGptConfig" class="btn btn-outline btn-sm">
              {{ copiedChatGpt ? $t('dashboard.configuration.copied') : '📋 ChatGPT' }}
            </button>
          </div>
        </div>
        <div class="card-content">
          <div class="config-tabs">
            <button
              @click="activeConfigTab = 'cursor'"
              :class="['config-tab', { active: activeConfigTab === 'cursor' }]"
            >
              {{ $t('dashboard.configuration.cursorTab') }}
            </button>
            <button
              @click="activeConfigTab = 'augment'"
              :class="['config-tab', { active: activeConfigTab === 'augment' }]"
            >
              {{ $t('dashboard.configuration.augmentTab') }}
            </button>
            <button
              @click="activeConfigTab = 'claude'"
              :class="['config-tab', { active: activeConfigTab === 'claude' }]"
            >
              Claude Desktop
            </button>
            <button
              @click="activeConfigTab = 'chatgpt'"
              :class="['config-tab', { active: activeConfigTab === 'chatgpt' }]"
            >
              ChatGPT
            </button>
            <button
              @click="activeConfigTab = 'custom'"
              :class="['config-tab', { active: activeConfigTab === 'custom' }]"
            >
              自定义
            </button>
          </div>

          <div v-if="activeConfigTab === 'cursor'" class="config-content">
            <p class="config-description">
              {{ $t('dashboard.configuration.cursorDescription') }}
            </p>
            <div class="config-container">
              <pre class="config-text">{{ cursorConfig }}</pre>
            </div>
            <div class="config-help">
              <h4>{{ $t('dashboard.configuration.howToUse') }}</h4>
              <ol>
                <li>{{ $t('dashboard.configuration.cursorSteps.step1') }}</li>
                <li>{{ $t('dashboard.configuration.cursorSteps.step2') }}</li>
                <li>{{ $t('dashboard.configuration.cursorSteps.step3') }}</li>
                <li>{{ $t('dashboard.configuration.cursorSteps.step4') }}</li>
              </ol>
            </div>
          </div>

          <div v-if="activeConfigTab === 'augment'" class="config-content">
            <p class="config-description">
              {{ $t('dashboard.configuration.augmentDescription') }}
            </p>
            <div class="config-container">
              <pre class="config-text">{{ augmentConfig }}</pre>
            </div>
            <div class="config-help">
              <h4>{{ $t('dashboard.configuration.howToUse') }}</h4>
              <ol>
                <li>{{ $t('dashboard.configuration.augmentSteps.step1') }}</li>
                <li>{{ $t('dashboard.configuration.augmentSteps.step2') }}</li>
                <li>{{ $t('dashboard.configuration.augmentSteps.step3') }}</li>
                <li>{{ $t('dashboard.configuration.augmentSteps.step4') }}</li>
              </ol>
            </div>
          </div>

          <div v-if="activeConfigTab === 'claude'" class="config-content">
            <p class="config-description">
              为 Claude Desktop 配置 MCP 服务器，自动标记来源为 "claude-desktop"。
            </p>
            <div class="config-container">
              <pre class="config-text">{{ claudeConfig }}</pre>
            </div>
            <div class="config-help">
              <h4>使用方法</h4>
              <ol>
                <li>复制上面的配置</li>
                <li>在 Claude Desktop 中添加 MCP 服务器配置</li>
                <li>重启 Claude Desktop</li>
                <li>现在 feedback 将显示来源为 "Claude Desktop"</li>
              </ol>
            </div>
          </div>

          <div v-if="activeConfigTab === 'chatgpt'" class="config-content">
            <p class="config-description">
              为 ChatGPT 配置 MCP 服务器，自动标记来源为 "chatgpt"。
            </p>
            <div class="config-container">
              <pre class="config-text">{{ chatGptConfig }}</pre>
            </div>
            <div class="config-help">
              <h4>使用方法</h4>
              <ol>
                <li>复制上面的配置</li>
                <li>在 ChatGPT 中添加 MCP 服务器配置</li>
                <li>重启 ChatGPT</li>
                <li>现在 feedback 将显示来源为 "ChatGPT"</li>
              </ol>
            </div>
          </div>

          <div v-if="activeConfigTab === 'custom'" class="config-content">
            <p class="config-description">
              为自定义 AI 工具配置 MCP 服务器，可以自定义来源名称。
            </p>
            <div class="custom-source-input">
              <label>来源名称：</label>
              <input 
                v-model="customSourceName" 
                type="text" 
                placeholder="输入 AI 工具名称"
                @input="generateCustomConfig"
                class="custom-input"
              />
              <button @click="copyCustomConfig" class="btn btn-outline btn-sm">
                {{ copiedCustom ? '已复制' : '📋 复制配置' }}
              </button>
            </div>
            <div class="config-container" v-if="customConfig">
              <pre class="config-text">{{ customConfig }}</pre>
            </div>
            <div class="config-help">
              <h4>使用方法</h4>
              <ol>
                <li>输入你的 AI 工具名称</li>
                <li>复制生成的配置</li>
                <li>在你的 AI 工具中添加 MCP 服务器配置</li>
                <li>现在 feedback 将显示你的自定义来源名称</li>
              </ol>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface McpTool {
  name: string
  description: string
  inputSchema: any
}

const serverStatus = ref<string>('stopped')
const tools = ref<McpTool[]>([])
const mcpConfig = ref<string>('')
const cursorConfig = ref<string>('')
const augmentConfig = ref<string>('')
const claudeConfig = ref<string>('')
const chatGptConfig = ref<string>('')
const customConfig = ref<string>('')
const customSourceName = ref<string>('')
const loading = ref<boolean>(false)
const copiedCursor = ref<boolean>(false)
const copiedAugment = ref<boolean>(false)
const copiedClaude = ref<boolean>(false)
const copiedChatGpt = ref<boolean>(false)
const copiedCustom = ref<boolean>(false)
const activeConfigTab = ref<string>('cursor')

async function startServer() {
  loading.value = true
  try {
    const result = await invoke('start_mcp_server')
    console.log(result)
    await refreshStatus()
    await loadTools()
  } catch (error) {
    console.error('Failed to start server:', error)
  } finally {
    loading.value = false
  }
}

async function stopServer() {
  loading.value = true
  try {
    const result = await invoke('stop_mcp_server')
    console.log(result)
    await refreshStatus()
    tools.value = []
  } catch (error) {
    console.error('Failed to stop server:', error)
  } finally {
    loading.value = false
  }
}

async function refreshStatus() {
  try {
    const status = await invoke<string>('get_server_status')
    serverStatus.value = status
  } catch (error) {
    console.error('Failed to get server status:', error)
  }
}

async function loadTools() {
  try {
    const toolList = await invoke<McpTool[]>('list_available_tools')
    tools.value = toolList
  } catch (error) {
    console.error('Failed to load tools:', error)
  }
}

async function loadConfig() {
  try {
    const config = await invoke<string>('get_mcp_config')
    mcpConfig.value = config
  } catch (error) {
    console.error('Failed to load config:', error)
  }
}

async function loadCursorConfig() {
  try {
    const config = await invoke<string>('get_cursor_config')
    cursorConfig.value = config
  } catch (error) {
    console.error('Failed to load cursor config:', error)
  }
}

async function loadAugmentConfig() {
  try {
    const config = await invoke<string>('get_augment_config')
    augmentConfig.value = config
  } catch (error) {
    console.error('Failed to load augment config:', error)
  }
}

async function loadClaudeConfig() {
  try {
    const config = await invoke<string>('get_claude_desktop_config')
    claudeConfig.value = config
  } catch (error) {
    console.error('Failed to load claude config:', error)
  }
}

async function loadChatGptConfig() {
  try {
    const config = await invoke<string>('get_chatgpt_config')
    chatGptConfig.value = config
  } catch (error) {
    console.error('Failed to load chatgpt config:', error)
  }
}

async function generateCustomConfig() {
  if (!customSourceName.value.trim()) {
    customConfig.value = ''
    return
  }
  
  try {
    const config = await invoke<string>('get_custom_config', { sourceName: customSourceName.value })
    customConfig.value = config
  } catch (error) {
    console.error('Failed to generate custom config:', error)
  }
}

function copyCursorConfig() {
  navigator.clipboard.writeText(cursorConfig.value)
  copiedCursor.value = true
  setTimeout(() => {
    copiedCursor.value = false
  }, 2000)
}

function copyAugmentConfig() {
  navigator.clipboard.writeText(augmentConfig.value)
  copiedAugment.value = true
  setTimeout(() => {
    copiedAugment.value = false
  }, 2000)
}

function copyClaudeConfig() {
  navigator.clipboard.writeText(claudeConfig.value)
  copiedClaude.value = true
  setTimeout(() => {
    copiedClaude.value = false
  }, 2000)
}

function copyChatGptConfig() {
  navigator.clipboard.writeText(chatGptConfig.value)
  copiedChatGpt.value = true
  setTimeout(() => {
    copiedChatGpt.value = false
  }, 2000)
}

function copyCustomConfig() {
  if (!customConfig.value) {
    return
  }
  navigator.clipboard.writeText(customConfig.value)
  copiedCustom.value = true
  setTimeout(() => {
    copiedCustom.value = false
  }, 2000)
}

function getToolDescription(toolName: string, originalDescription: string): string {
  // 尝试获取翻译的描述，如果没有则使用原始描述
  const translationKey = `tools.${toolName}.description`
  const translatedDesc = t(translationKey)

  // 如果翻译键不存在，t() 会返回键本身，所以我们检查是否等于键
  if (translatedDesc === translationKey) {
    return originalDescription
  }

  return translatedDesc
}



onMounted(async () => {
  await refreshStatus()
  await loadConfig()
  await loadCursorConfig()
  await loadAugmentConfig()
  await loadClaudeConfig()
  await loadChatGptConfig()

  // 自动启动服务器
  if (serverStatus.value === 'stopped') {
    console.log('Auto-starting MCP server...')
    await startServer()
  }

  // 加载工具列表
  await loadTools()
})
</script>

<style scoped>
.dashboard {
  animation: fadeIn 0.5s ease-in-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.server-control {
  animation: slideInLeft 0.6s ease-out;
}

.tools-overview {
  animation: slideInUp 0.7s ease-out;
}

.config-section {
  animation: slideInRight 0.8s ease-out;
}

@keyframes slideInLeft {
  from {
    opacity: 0;
    transform: translateX(-30px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes slideInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(30px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* 配置相关样式 */
.config-buttons {
  display: flex;
  gap: 0.5rem;
}

.config-tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
  border-bottom: 1px solid rgba(226, 232, 240, 0.3);
}

.config-tab {
  padding: 0.75rem 1.5rem;
  border: none;
  background: transparent;
  color: #718096;
  font-weight: 500;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.3s ease;
}

.config-tab:hover {
  color: #4a5568;
  background: rgba(255, 255, 255, 0.1);
}

.config-tab.active {
  color: #667eea;
  border-bottom-color: #667eea;
  background: rgba(102, 126, 234, 0.1);
}

.config-content {
  animation: fadeIn 0.3s ease-in-out;
}

.config-help {
  margin-top: 1.5rem;
  padding: 1rem;
  background: rgba(102, 126, 234, 0.05);
  border-radius: 0.5rem;
  border-left: 3px solid #667eea;
}

.config-help h4 {
  margin: 0 0 0.75rem 0;
  color: #4a5568;
  font-size: 0.875rem;
  font-weight: 600;
}

.config-help ol {
  margin: 0;
  padding-left: 1.25rem;
  color: #718096;
  font-size: 0.875rem;
  line-height: 1.5;
}

.config-help li {
  margin-bottom: 0.25rem;
}

/* 自定义配置输入框样式 */
.custom-source-input {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
  padding: 1rem;
  background: rgba(102, 126, 234, 0.05);
  border-radius: 0.5rem;
  border: 1px solid rgba(102, 126, 234, 0.2);
}

.custom-source-input label {
  color: #4a5568;
  font-weight: 500;
  white-space: nowrap;
}

.custom-input {
  flex: 1;
  padding: 0.5rem 0.75rem;
  border: 1px solid rgba(226, 232, 240, 0.5);
  border-radius: 0.375rem;
  background: rgba(255, 255, 255, 0.8);
  color: #4a5568;
  font-size: 0.875rem;
  transition: all 0.3s ease;
}

.custom-input:focus {
  outline: none;
  border-color: #667eea;
  background: rgba(255, 255, 255, 1);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

@media (prefers-color-scheme: dark) {
  .config-tab {
    color: #a0aec0;
  }

  .config-tab:hover {
    color: #e2e8f0;
    background: rgba(45, 55, 72, 0.3);
  }

  .config-tab.active {
    color: #90cdf4;
    border-bottom-color: #90cdf4;
    background: rgba(144, 205, 244, 0.1);
  }

  .config-help {
    background: rgba(144, 205, 244, 0.1);
    border-left-color: #90cdf4;
  }

  .config-help h4 {
    color: #e2e8f0;
  }

  .config-help ol {
    color: #a0aec0;
  }

  .custom-source-input {
    background: rgba(144, 205, 244, 0.1);
    border-color: rgba(144, 205, 244, 0.3);
  }

  .custom-source-input label {
    color: #e2e8f0;
  }

  .custom-input {
    background: rgba(45, 55, 72, 0.8);
    border-color: rgba(113, 128, 150, 0.5);
    color: #e2e8f0;
  }

  .custom-input:focus {
    background: rgba(45, 55, 72, 1);
    border-color: #90cdf4;
    box-shadow: 0 0 0 3px rgba(144, 205, 244, 0.1);
  }
}
</style>