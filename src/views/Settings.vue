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
            <div class="setting-item">
              <label class="setting-label">{{ $t('settings.general.autoStart') }}</label>
              <div class="setting-control">
                <label class="switch">
                  <input
                    type="checkbox"
                    v-model="autoStart"
                    @change="saveAutoStart"
                  >
                  <span class="slider"></span>
                </label>
                <span class="setting-description">{{ $t('settings.general.autoStartDesc') }}</span>
              </div>
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
import { setLocale, getCurrentLocale, supportedLocales } from '../i18n'
import { useI18n } from 'vue-i18n'
import CustomSelect from '../components/CustomSelect.vue'

const { t } = useI18n()

const selectedLanguage = ref(getCurrentLocale())
const selectedTheme = ref('auto')
const autoStart = ref(true)

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

.setting-label {
  font-weight: 600;
  color: #374151;
  font-size: 0.875rem;
}





.setting-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.setting-description {
  color: #6b7280;
  font-size: 0.875rem;
  line-height: 1.4;
}

/* 开关样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #cbd5e1;
  transition: 0.3s;
  border-radius: 24px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

input:checked + .slider {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

input:checked + .slider:before {
  transform: translateX(26px);
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .setting-label {
    color: #f3f4f6;
  }



  .setting-description {
    color: #9ca3af;
  }



  .slider {
    background-color: #4b5563;
  }

  input:checked + .slider {
    background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%);
  }
}

:global(.dark) .setting-label {
  color: #f3f4f6;
}



:global(.dark) .setting-description {
  color: #9ca3af;
}

:global(.dark) .slider {
  background-color: #4b5563;
}

:global(.dark) input:checked + .slider {
  background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%);
}


</style>
