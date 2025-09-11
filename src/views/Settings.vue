<template>
  <div class="settings">
    <header class="page-header">
      <h2>{{ $t('settings.title') }}</h2>
      <p class="page-subtitle">{{ $t('settings.subtitle') }}</p>
    </header>

    <section class="settings-content">
      <div class="card">
        <div class="card-header">
          <h3>{{ $t('settings.general.title') }}</h3>
        </div>
        <div class="card-content">
          <div class="settings-form">
            <!-- 语言设置 -->
            <div class="setting-item">
              <label class="setting-label">{{ $t('settings.general.language') }}</label>
              <CustomSelect
                v-model="selectedLanguage"
                :options="languageOptions"
                :placeholder="$t('settings.general.selectLanguage')"
                @update:modelValue="changeLanguage"
              />
            </div>

            <!-- 主题设置 -->
            <div class="setting-item">
              <label class="setting-label">{{ $t('settings.general.theme') }}</label>
              <CustomSelect
                v-model="selectedTheme"
                :options="themeOptions"
                :placeholder="$t('settings.general.selectTheme')"
                @update:modelValue="changeTheme"
              />
            </div>

            <!-- 自动启动设置 -->
            <div class="setting-item setting-item-toggle">
              <div class="setting-header">
                <label class="setting-label">{{ $t('settings.general.autoStart') }}</label>
                <ToggleSwitch
                  v-model="autoStart"
                  @change="saveAutoStart"
                />
              </div>
              <p class="setting-description">{{ $t('settings.general.autoStartDesc') }}</p>
            </div>

            <!-- 小窗口模式设置 -->
            <div class="setting-item setting-item-toggle">
              <div class="setting-header">
                <label class="setting-label">{{ $t('settings.general.compactMode') }}</label>
                <ToggleSwitch
                  v-model="compactMode"
                  @change="saveCompactMode"
                />
              </div>
              <p class="setting-description">{{ $t('settings.general.compactModeDesc') }}</p>
            </div>
            
            <!-- 窗口尺寸测试按钮 -->
            <div class="setting-item">
              <label class="setting-label">窗口尺寸管理</label>
              <div class="button-group">
                <button @click="saveWindowSize" class="test-btn">保存当前窗口尺寸</button>
                <button @click="loadWindowSize" class="test-btn">加载保存的尺寸</button>
                <button @click="applyWindowSize" class="test-btn">应用保存的尺寸</button>
              </div>
              <p class="setting-description">测试窗口尺寸的保存和恢复功能</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 占位卡片 -->
      <div class="card">
        <div class="card-header">
          <h3>{{ $t('settings.comingSoon') }}</h3>
        </div>
        <div class="card-content">
          <div class="empty-state">
            <div class="empty-icon">⚙️</div>
            <p>{{ $t('settings.comingSoon') }}</p>
            <p class="empty-hint">{{ $t('settings.comingSoonHint') }}</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { setLocale, getCurrentLocale, supportedLocales } from '../i18n'
import { useI18n } from 'vue-i18n'
import CustomSelect from '../components/CustomSelect.vue'
import ToggleSwitch from '../components/ToggleSwitch.vue'

const { t } = useI18n()

const selectedLanguage = ref(getCurrentLocale())
const selectedTheme = ref('auto')
const autoStart = ref(true)
const compactMode = ref(false)

// 窗口尺寸状态
const savedWindowSize = ref<any>(null)

const supportedLanguages = supportedLocales

// 语言选项
const languageOptions = computed(() =>
  supportedLanguages.map(lang => ({
    value: lang.code,
    name: lang.name
  }))
)

// 主题选项
const themeOptions = computed(() => [
  { value: 'auto', name: t('settings.themes.auto') },
  { value: 'light', name: t('settings.themes.light') },
  { value: 'dark', name: t('settings.themes.dark') }
])

function changeLanguage() {
  setLocale(selectedLanguage.value)
}

function changeTheme() {
  localStorage.setItem('mcp-manager-theme', selectedTheme.value)
  applyTheme(selectedTheme.value)
}

function applyTheme(theme: string) {
  const html = document.documentElement

  if (theme === 'dark') {
    html.classList.add('dark')
  } else if (theme === 'light') {
    html.classList.remove('dark')
  } else {
    // auto - follow system
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    if (prefersDark) {
      html.classList.add('dark')
    } else {
      html.classList.remove('dark')
    }
  }
}

function saveAutoStart() {
  localStorage.setItem('mcp-manager-auto-start', autoStart.value.toString())
}

async function saveCompactMode() {
  localStorage.setItem('mcp-manager-compact-mode', compactMode.value.toString())
  
  try {
    // 调用 Tauri 命令来实际调整窗口大小
    await invoke('set_window_compact_mode', { compact: compactMode.value })
    console.log('✅ Window size updated for compact mode:', compactMode.value)
  } catch (error) {
    console.error('❌ Failed to update window size:', error)
  }
  
  // 触发全局事件来通知应用布局变化
  window.dispatchEvent(new CustomEvent('compact-mode-changed', { 
    detail: { compactMode: compactMode.value } 
  }))
}

// 窗口尺寸管理函数
async function saveWindowSize() {
  try {
    await invoke('save_current_window_size')
    console.log('✅ Window size saved manually')
    alert('窗口尺寸已保存！')
  } catch (error) {
    console.error('❌ Failed to save window size:', error)
    alert('保存窗口尺寸失败：' + error)
  }
}

async function loadWindowSize() {
  try {
    const windowSize = await invoke('load_saved_window_size')
    savedWindowSize.value = windowSize
    console.log('✅ Window size loaded:', windowSize)
    alert('窗口尺寸已加载：' + JSON.stringify(windowSize, null, 2))
  } catch (error) {
    console.error('❌ Failed to load window size:', error)
    alert('加载窗口尺寸失败：' + error)
  }
}

async function applyWindowSize() {
  if (!savedWindowSize.value) {
    await loadWindowSize()
  }
  
  if (savedWindowSize.value) {
    try {
      await invoke('apply_window_size', { windowSize: savedWindowSize.value })
      console.log('✅ Window size applied:', savedWindowSize.value)
      alert('窗口尺寸已应用！')
    } catch (error) {
      console.error('❌ Failed to apply window size:', error)
      alert('应用窗口尺寸失败：' + error)
    }
  }
}

onMounted(() => {
  // 加载保存的主题设置
  const savedTheme = localStorage.getItem('mcp-manager-theme')
  if (savedTheme) {
    selectedTheme.value = savedTheme
    applyTheme(savedTheme)
  }

  // 加载自动启动设置
  const savedAutoStart = localStorage.getItem('mcp-manager-auto-start')
  if (savedAutoStart !== null) {
    autoStart.value = savedAutoStart === 'true'
  }

  // 加载小窗口模式设置
  const savedCompactMode = localStorage.getItem('mcp-manager-compact-mode')
  if (savedCompactMode !== null) {
    compactMode.value = savedCompactMode === 'true'
  }

  // 监听系统主题变化
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    if (selectedTheme.value === 'auto') {
      applyTheme('auto')
    }
  })
})
</script>

<style scoped>
.settings {
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

.settings-content {
  animation: slideInUp 0.6s ease-out;
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

/* 设置表单样式 */
.settings-form {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

/* 开关类设置项使用紧凑布局 */
.setting-item-toggle {
  flex-direction: column;
  align-items: flex-start;
  gap: 0.75rem;
}

.setting-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  width: fit-content;
  max-width: 100%;
}

.setting-label {
  font-weight: 600;
  color: #374151;
  font-size: 0.875rem;
}

.setting-description {
  color: #6b7280;
  font-size: 0.875rem;
  line-height: 1.4;
  margin: 0;
}

/* 按钮组样式 */
.button-group {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.test-btn {
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 0.375rem;
  background: #f9fafb;
  color: #374151;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.test-btn:hover {
  background: #f3f4f6;
  border-color: #9ca3af;
}

.test-btn:active {
  transform: translateY(1px);
}






/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .setting-label {
    color: #f3f4f6;
  }
  
  .setting-description {
    color: #9ca3af;
  }
}

:global(.dark) .setting-label {
  color: #f3f4f6;
}

:global(.dark) .setting-description {
  color: #9ca3af;
}

:global(.dark) .test-btn {
  background: #374151;
  color: #f3f4f6;
  border-color: #4b5563;
}

:global(.dark) .test-btn:hover {
  background: #4b5563;
  border-color: #6b7280;
}


</style>
