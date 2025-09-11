export default {
  // 应用标题
  app: {
    title: 'MCP 管理器',
    subtitle: 'Model Context Protocol 服务器管理工具'
  },
  
  // 导航菜单
  nav: {
    dashboard: '仪表板',
    settings: '设置',
    feedback: 'Feedback'
  },
  
  // 状态
  status: {
    running: '运行中',
    stopped: '已停止',
    loading: '加载中...',
    starting: '启动中...',
    stopping: '停止中...'
  },
  
  // 仪表板
  dashboard: {
    title: '仪表板',
    subtitle: '监控您的 MCP 服务器状态和可用工具',
    
    // 服务器控制
    serverControl: {
      title: '服务器控制',
      startServer: '启动服务器',
      stopServer: '停止服务器',
      refreshStatus: '刷新状态',
      serverRunning: '服务器运行中',
      serverStopped: '服务器已停止'
    },
    
    // 工具概览
    toolsOverview: {
      title: '可用工具',
      toolsCount: '{count} 个工具',
      noTools: '暂无可用工具',
      noToolsHint: '启动服务器以查看可用工具',
      toolBadge: '工具',
      schema: '架构'
    },
    
    // 配置
    configuration: {
      title: '快速配置',
      cursorConfig: 'Cursor 配置',
      augmentConfig: 'Augment 配置',
      copied: '已复制！',
      copyConfig: '复制配置',
      
      // 标签页
      cursorTab: 'Cursor IDE',
      augmentTab: 'Augment Code',
      
      // 说明
      cursorDescription: '将此配置复制到您的 Cursor IDE MCP 设置中：',
      augmentDescription: '将此配置复制到您的 Augment Code MCP 设置中：',
      
      // 使用指南
      howToUse: '使用方法：',
      cursorSteps: {
        step1: '打开 Cursor IDE',
        step2: '进入 设置 → 功能 → Model Context Protocol',
        step3: '粘贴上述配置',
        step4: '重启 Cursor 以加载工具'
      },
      augmentSteps: {
        step1: '打开 Augment Code',
        step2: '进入 设置 → MCP 服务器',
        step3: '添加新服务器并使用上述配置',
        step4: '启用服务器以加载工具'
      }
    }
  },
  
  // 设置页面
  settings: {
    title: '设置',
    subtitle: '配置您的 MCP 服务器偏好设置',
    
    // 通用设置
    general: {
      title: '通用设置',
      language: '语言',
      selectLanguage: '选择语言',
      theme: '主题',
      selectTheme: '选择主题',
      autoStart: '自动启动',
      autoStartDesc: '应用启动时自动启动 MCP 服务器',
      compactMode: '小窗口模式',
      compactModeDesc: '窗口宽度设为800px，高度最大化，Feedback页面使用上下布局'
    },
    
    // 主题选项
    themes: {
      auto: '跟随系统',
      light: '浅色模式',
      dark: '深色模式'
    },
    
    // 语言选项
    languages: {
      'zh-CN': '简体中文',
      'en-US': 'English'
    },
    
    // 占位内容
    comingSoon: '更多设置即将推出',
    comingSoonHint: '配置选项将在此处提供'
  },

  // 反馈页面
  feedback: {
    title: '交互式反馈',
    subtitle: '与 AI 进行实时交互和反馈',
    aiResponse: 'AI 回答',
    userFeedback: '用户反馈',
    placeholder: '请输入您的反馈...',
    hint: 'Enter 换行，Shift + Enter 发送',
    send: '发送',
    sending: '发送中...',
    submitted: '反馈已提交！',
    history: '反馈历史',
    customEmphasis: '自定义强调语',
    customEmphasisPlaceholder: '请输入要强调的内容...',
    empty: {
      title: '暂无反馈会话',
      description: '当 AI 调用 feedback 工具时，会在这里显示交互式反馈界面'
    }
  },

  // 通用按钮和操作
  common: {
    save: '保存',
    cancel: '取消',
    confirm: '确认',
    close: '关闭',
    refresh: '刷新',
    copy: '复制',
    edit: '编辑',
    delete: '删除',
    add: '添加',
    remove: '移除',
    enable: '启用',
    disable: '禁用',
    reset: '重置'
  },
  
  // 工具描述翻译
  tools: {
    echo: {
      description: '回显输入的文本，用于测试和调试'
    },
    file_read: {
      description: '读取指定文件的内容'
    },
    system_info: {
      description: '获取系统信息，包括操作系统、架构等'
    },
    feedback: {
      description: '交互式反馈工具 - 显示 AI 回答并允许用户提供反馈'
    }
  },

  // 消息提示
  messages: {
    success: {
      serverStarted: '服务器启动成功',
      serverStopped: '服务器停止成功',
      configCopied: '配置已复制到剪贴板',
      settingsSaved: '设置已保存'
    },
    error: {
      serverStartFailed: '服务器启动失败',
      serverStopFailed: '服务器停止失败',
      loadToolsFailed: '加载工具失败',
      loadConfigFailed: '加载配置失败',
      copyFailed: '复制失败'
    }
  }
}
