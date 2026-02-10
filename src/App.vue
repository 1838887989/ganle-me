<script setup lang="ts">
import Sidebar from './components/Sidebar.vue'
import TitleBar from './components/TitleBar.vue'
import TaskInput from './components/TaskInput.vue'
import TaskList from './components/TaskList.vue'
import MinimalView from './components/MinimalView.vue'
import ResizeHandle from './components/ResizeHandle.vue'
import ReminderModal from './components/ReminderModal.vue'
import { useAppStore } from './stores/app'
import { onMounted } from 'vue'

const store = useAppStore()

onMounted(() => {
  // 检查系统主题偏好
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    store.isDark = true
    document.documentElement.classList.add('dark')
  }

  // 监听系统主题变化
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    store.isDark = e.matches
    document.documentElement.classList.toggle('dark', e.matches)
  })
})
</script>

<template>
  <div class="h-screen w-screen bg-transparent">
    <!-- 极简模式 -->
    <div v-if="store.isMinimalMode" class="h-full w-full p-3">
      <MinimalView />
    </div>

    <!-- 标准模式 -->
    <div v-else class="h-full w-full p-3">
      <div class="relative h-full w-full flex overflow-hidden bg-cream dark:bg-dark-bg transition-colors duration-300 rounded-2xl">
        <!-- 侧边栏 -->
        <Sidebar />

        <!-- 主内容区 -->
        <div class="flex-1 flex flex-col min-w-0">
          <!-- 标题栏 -->
          <TitleBar />

          <!-- 主内容 -->
          <main class="flex-1 overflow-y-auto p-6">
            <div class="max-w-3xl mx-auto">
              <TaskInput />

              <div class="flex items-center justify-between mb-4 px-1">
                <span class="text-sm text-gray-500 dark:text-dark-muted">
                  共 {{ store.filteredTasks.length }} 个任务
                </span>
              </div>

              <TaskList />
            </div>
          </main>
        </div>

        <!-- Resize 手柄 -->
        <ResizeHandle />
      </div>
    </div>

    <!-- 提醒弹窗 -->
    <ReminderModal />
  </div>
</template>
