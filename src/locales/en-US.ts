export default {
  // App title
  app: {
    title: 'MCP Manager',
    subtitle: 'Model Context Protocol Server Management Tool'
  },
  
  // Navigation menu
  nav: {
    dashboard: 'Dashboard',
    settings: 'Settings',
    feedback: 'Feedback'
  },
  
  // Status
  status: {
    running: 'Running',
    stopped: 'Stopped',
    loading: 'Loading...',
    starting: 'Starting...',
    stopping: 'Stopping...'
  },
  
  // Dashboard
  dashboard: {
    title: 'Dashboard',
    subtitle: 'Monitor your MCP server status and available tools',
    
    // Server control
    serverControl: {
      title: 'Server Control',
      startServer: 'Start Server',
      stopServer: 'Stop Server',
      refreshStatus: 'Refresh Status',
      serverRunning: 'Server Running',
      serverStopped: 'Server Stopped'
    },
    
    // Tools overview
    toolsOverview: {
      title: 'Available Tools',
      toolsCount: '{count} tools',
      noTools: 'No tools available',
      noToolsHint: 'Start the server to see available tools',
      toolBadge: 'Tool',
      schema: 'Schema'
    },
    
    // Configuration
    configuration: {
      title: 'Quick Configuration',
      cursorConfig: 'Cursor Config',
      augmentConfig: 'Augment Config',
      copied: 'Copied!',
      copyConfig: 'Copy Config',
      
      // Tabs
      cursorTab: 'Cursor IDE',
      augmentTab: 'Augment Code',
      
      // Descriptions
      cursorDescription: 'Copy this configuration to your Cursor IDE MCP settings:',
      augmentDescription: 'Copy this configuration to your Augment Code MCP settings:',
      
      // How to use
      howToUse: 'How to use:',
      cursorSteps: {
        step1: 'Open Cursor IDE',
        step2: 'Go to Settings → Features → Model Context Protocol',
        step3: 'Paste the configuration above',
        step4: 'Restart Cursor to load the tools'
      },
      augmentSteps: {
        step1: 'Open Augment Code',
        step2: 'Go to Settings → MCP Servers',
        step3: 'Add a new server with the configuration above',
        step4: 'Enable the server to load the tools'
      }
    }
  },
  
  // Settings page
  settings: {
    title: 'Settings',
    subtitle: 'Configure your MCP server preferences',
    
    // General settings
    general: {
      title: 'General Settings',
      language: 'Language',
      selectLanguage: 'Select Language',
      theme: 'Theme',
      selectTheme: 'Select Theme',
      autoStart: 'Auto Start',
      autoStartDesc: 'Automatically start MCP server when app launches',
      compactMode: 'Compact Mode',
      compactModeDesc: 'Set window width to 800px, height maximized, Feedback page uses vertical layout'
    },
    
    // Theme options
    themes: {
      auto: 'Follow System',
      light: 'Light Mode',
      dark: 'Dark Mode'
    },
    
    // Language options
    languages: {
      'zh-CN': '简体中文',
      'en-US': 'English'
    },
    
    // Placeholder content
    comingSoon: 'More settings coming soon',
    comingSoonHint: 'Configuration options will be available here'
  },

  // Feedback page
  feedback: {
    title: 'Interactive Feedback',
    subtitle: 'Real-time interaction and feedback with AI',
    aiResponse: 'AI Response',
    userFeedback: 'User Feedback',
    placeholder: 'Enter your feedback...',
    hint: 'Enter for new line, Shift + Enter to send',
    send: 'Send',
    sending: 'Sending...',
    submitted: 'Feedback Submitted!',
    history: 'Feedback History',
    customEmphasis: 'Custom Emphasis',
    customEmphasisPlaceholder: 'Enter content to emphasize...',
    empty: {
      title: 'No feedback sessions',
      description: 'Interactive feedback interface will appear here when AI calls the feedback tool'
    }
  },

  // Common buttons and actions
  common: {
    save: 'Save',
    cancel: 'Cancel',
    confirm: 'Confirm',
    close: 'Close',
    refresh: 'Refresh',
    copy: 'Copy',
    edit: 'Edit',
    delete: 'Delete',
    add: 'Add',
    remove: 'Remove',
    enable: 'Enable',
    disable: 'Disable',
    reset: 'Reset'
  },
  
  // Tool descriptions
  tools: {
    echo: {
      description: 'Echo the input text, useful for testing and debugging'
    },
    file_read: {
      description: 'Read the contents of a specified file'
    },
    system_info: {
      description: 'Get system information including OS, architecture, etc.'
    },
    feedback: {
      description: 'Interactive feedback tool - displays AI response and allows user to provide feedback'
    }
  },

  // Message prompts
  messages: {
    success: {
      serverStarted: 'Server started successfully',
      serverStopped: 'Server stopped successfully',
      configCopied: 'Configuration copied to clipboard',
      settingsSaved: 'Settings saved successfully'
    },
    error: {
      serverStartFailed: 'Failed to start server',
      serverStopFailed: 'Failed to stop server',
      loadToolsFailed: 'Failed to load tools',
      loadConfigFailed: 'Failed to load configuration',
      copyFailed: 'Failed to copy'
    }
  }
}
