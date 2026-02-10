<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore } from '@/stores/app'
import { Clock, X, Check } from 'lucide-vue-next'

const store = useAppStore()

const task = computed(() => store.pendingReminder)

function dismiss() {
  store.dismissReminder()
}

function completeTask() {
  if (task.value) {
    store.toggleTask(task.value.id)
  }
  dismiss()
}
</script>

<template>
  <Transition
    enter-active-class="transition-all duration-300 ease-out"
    leave-active-class="transition-all duration-200 ease-in"
    enter-from-class="opacity-0 scale-95"
    enter-to-class="opacity-100 scale-100"
    leave-from-class="opacity-100 scale-100"
    leave-to-class="opacity-0 scale-95"
  >
    <div
      v-if="task"
      class="fixed inset-0 z-[200] flex items-center justify-center p-4 bg-black/30 backdrop-blur-sm"
      @click.self="dismiss"
    >
      <div class="w-full max-w-sm bg-white dark:bg-dark-surface rounded-2xl shadow-2xl overflow-hidden">
        <!-- 头部 -->
        <div class="bg-gradient-to-r from-red-500 to-orange-500 px-5 py-4 flex items-center justify-between">
          <div class="flex items-center gap-2 text-white">
            <Clock class="w-5 h-5" />
            <span class="font-bold">任务即将到期</span>
          </div>
          <button
            class="p-1 rounded-lg hover:bg-white/20 transition-colors"
            @click="dismiss"
          >
            <X class="w-5 h-5 text-white" />
          </button>
        </div>

        <!-- 内容 -->
        <div class="p-5">
          <h3 class="text-lg font-bold text-gray-800 dark:text-white mb-2">
            {{ task.title }}
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            请尽快完成此任务
          </p>
        </div>

        <!-- 操作按钮 -->
        <div class="px-5 pb-5 flex gap-3">
          <button
            class="flex-1 py-2.5 px-4 rounded-xl bg-gray-100 dark:bg-dark-border text-gray-600 dark:text-gray-300 font-medium hover:bg-gray-200 dark:hover:bg-dark-border/80 transition-colors"
            @click="dismiss"
          >
            稍后提醒
          </button>
          <button
            class="flex-1 py-2.5 px-4 rounded-xl bg-primary text-white font-medium hover:bg-primary/90 transition-colors flex items-center justify-center gap-2"
            @click="completeTask"
          >
            <Check class="w-4 h-4" />
            完成任务
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>
