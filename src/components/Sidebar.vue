<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <h1 class="app-title">🔧 {{ $t('app.title') }}</h1>
    </div>

    <nav class="sidebar-nav">
      <ul class="nav-list">
        <li class="nav-item">
          <router-link
            to="/dashboard"
            class="nav-button"
            active-class="active"
          >
            <span class="nav-icon">📊</span>
            <span class="nav-text">{{ $t('nav.dashboard') }}</span>
          </router-link>
        </li>
        <li class="nav-item">
          <router-link
            to="/settings"
            class="nav-button"
            active-class="active"
          >
            <span class="nav-icon">⚙️</span>
            <span class="nav-text">{{ $t('nav.settings') }}</span>
          </router-link>
        </li>
        <li class="nav-item">
          <router-link
            to="/feedback"
            class="nav-button"
            active-class="active"
          >
            <span class="nav-icon">💬</span>
            <span class="nav-text">{{ $t('nav.feedback') }}</span>
          </router-link>
        </li>
      </ul>
    </nav>
  </aside>
</template>

<script setup lang="ts">
// 简化的侧边栏组件，不再显示服务器状态
// 服务器状态统一在仪表盘中管理和显示
</script>

<style scoped>
.sidebar {
  width: 280px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  flex-direction: column;
  position: fixed;
  height: 100vh;
  left: 0;
  top: 0;
  z-index: 100;
  box-shadow: 4px 0 20px rgba(0, 0, 0, 0.1);
}

.sidebar-header {
  padding: 2rem 1.5rem 1.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(255, 255, 255, 0.05);
}

.app-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.sidebar-nav {
  flex: 1;
  padding: 1rem 0;
}

.nav-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.nav-item {
  margin: 0;
}

.nav-button {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem 1.5rem;
  border: none;
  background: transparent;
  color: rgba(255, 255, 255, 0.8);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  text-decoration: none;
  position: relative;
  overflow: hidden;
}

.nav-button:hover {
  background: rgba(255, 255, 255, 0.15);
  color: white;
  border-radius: 0 0.5rem 0.5rem 0;
}

.nav-button.active {
  background: rgba(255, 255, 255, 0.15);
  color: white;
  border-right: 3px solid #fbbf24;
  box-shadow: inset 0 0 20px rgba(255, 255, 255, 0.1);
}

.nav-icon {
  font-size: 1.125rem;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
}

.nav-text {
  font-weight: 500;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

@media (prefers-color-scheme: dark) {
  .sidebar {
    background: linear-gradient(135deg, #1a202c 0%, #2d3748 50%, #1a202c 100%);
    border-right: 1px solid rgba(255, 255, 255, 0.05);
  }

  .sidebar-header {
    background: rgba(0, 0, 0, 0.3);
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .app-title {
    color: #f7fafc;
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
  }

  .status-indicator.running {
    background: rgba(72, 187, 120, 0.2);
    color: #9ae6b4;
    border: 1px solid rgba(72, 187, 120, 0.3);
  }

  .status-indicator.stopped {
    background: rgba(245, 101, 101, 0.2);
    color: #feb2b2;
    border: 1px solid rgba(245, 101, 101, 0.3);
  }

  .nav-button {
    color: rgba(255, 255, 255, 0.7);
  }

  .nav-button:hover {
    background: rgba(255, 255, 255, 0.08);
    color: #e2e8f0;
    border-radius: 0 0.5rem 0.5rem 0;
  }

  .nav-button.active {
    background: rgba(129, 140, 248, 0.15);
    color: #a5b4fc;
    border-right: 3px solid #818cf8;
    box-shadow: inset 0 0 20px rgba(129, 140, 248, 0.1);
  }
}

/* 手动深色模式 */
.dark .sidebar {
  background: linear-gradient(135deg, #1a202c 0%, #2d3748 50%, #1a202c 100%);
  border-right: 1px solid rgba(255, 255, 255, 0.05);
}

.dark .sidebar-header {
  background: rgba(0, 0, 0, 0.3);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.dark .app-title {
  color: #f7fafc;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
}

.dark .status-indicator.running {
  background: rgba(72, 187, 120, 0.2);
  color: #9ae6b4;
  border: 1px solid rgba(72, 187, 120, 0.3);
}

.dark .status-indicator.stopped {
  background: rgba(245, 101, 101, 0.2);
  color: #feb2b2;
  border: 1px solid rgba(245, 101, 101, 0.3);
}

.dark .nav-button {
  color: rgba(255, 255, 255, 0.7);
}

.dark .nav-button:hover {
  background: rgba(255, 255, 255, 0.08);
  color: #e2e8f0;
  border-radius: 0 0.5rem 0.5rem 0;
}

.dark .nav-button.active {
  background: rgba(129, 140, 248, 0.15);
  color: #a5b4fc;
  border-right: 3px solid #818cf8;
  box-shadow: inset 0 0 20px rgba(129, 140, 248, 0.1);
}
</style>
