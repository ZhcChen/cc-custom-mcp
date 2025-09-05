import { createI18n } from 'vue-i18n'
import zhCN from '../locales/zh-CN'
import enUS from '../locales/en-US'

// 获取浏览器语言或从本地存储获取用户设置
function getDefaultLocale(): string {
  // 首先检查本地存储
  const savedLocale = localStorage.getItem('mcp-manager-locale')
  if (savedLocale) {
    return savedLocale
  }
  
  // 然后检查浏览器语言
  const browserLang = navigator.language || navigator.languages[0]
  
  // 支持的语言列表
  const supportedLocales = ['zh-CN', 'en-US']
  
  // 精确匹配
  if (supportedLocales.includes(browserLang)) {
    return browserLang
  }
  
  // 语言代码匹配（如 zh 匹配 zh-CN）
  const langCode = browserLang.split('-')[0]
  const matchedLocale = supportedLocales.find(locale => 
    locale.startsWith(langCode)
  )
  
  if (matchedLocale) {
    return matchedLocale
  }
  
  // 默认返回简体中文
  return 'zh-CN'
}

const messages = {
  'zh-CN': zhCN,
  'en-US': enUS
}

const i18n = createI18n({
  legacy: false,
  locale: getDefaultLocale(),
  fallbackLocale: 'zh-CN',
  messages,
  globalInjection: true
})

export default i18n

// 导出切换语言的函数
export function setLocale(locale: string) {
  i18n.global.locale.value = locale as 'zh-CN' | 'en-US'
  localStorage.setItem('mcp-manager-locale', locale)
  document.documentElement.lang = locale
}

// 导出获取当前语言的函数
export function getCurrentLocale(): string {
  return i18n.global.locale.value
}

// 导出支持的语言列表
export const supportedLocales = [
  { code: 'zh-CN', name: '简体中文' },
  { code: 'en-US', name: 'English' }
]
