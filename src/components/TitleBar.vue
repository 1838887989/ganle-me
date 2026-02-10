<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { Minus, Square, X, Search } from 'lucide-vue-next'
import { getCurrentWindow } from '@tauri-apps/api/window'

const store = useAppStore()

// Tauri 窗口控制
function minimizeWindow() {
  getCurrentWindow().minimize()
}

function maximizeWindow() {
  getCurrentWindow().toggleMaximize()
}

function closeWindow() {
  getCurrentWindow().hide()
}
</script>

<template>
  <header
    class="h-14 flex items-center justify-between px-4 bg-cream-100/60 dark:bg-dark-surface/60 backdrop-blur-md border-b border-cream-300/50 dark:border-dark-border/50"
    style="-webkit-app-region: drag"
  >
    <!-- 左侧：窗口控制按钮 (macOS 风格) -->
    <div class="flex items-center gap-2" style="-webkit-app-region: no-drag">
      <button
        class="window-control window-control-close"
        @click="closeWindow"
        title="关闭"
      ></button>
      <button
        class="window-control window-control-minimize"
        @click="minimizeWindow"
        title="最小化"
      ></button>
      <button
        class="window-control window-control-maximize"
        @click="maximizeWindow"
        title="最大化"
      ></button>
    </div>

    <!-- 中间：标题 -->
    <div class="flex-1 flex items-center justify-center">
      <h1 class="font-heading font-semibold text-lg text-gray-800 dark:text-dark-text">
        {{ store.currentViewTitle }}
      </h1>
    </div>

    <!-- 右侧：搜索框 -->
    <div class="flex items-center gap-3" style="-webkit-app-region: no-drag">
      <div class="relative">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
        <input
          v-model="store.searchQuery"
          type="text"
          placeholder="搜索任务..."
          class="w-48 pl-9 pr-4 py-2 text-sm rounded-organic bg-white/70 dark:bg-dark-surface/70 border border-cream-300 dark:border-dark-border focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary transition-all"
        />
      </div>
    </div>
  </header>
</template>
