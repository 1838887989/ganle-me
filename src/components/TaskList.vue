<script setup lang="ts">
import { ref } from 'vue'
import { useAppStore } from '@/stores/app'
import TaskItem from './TaskItem.vue'
import { InboxIcon } from 'lucide-vue-next'

const store = useAppStore()

// 拖拽状态
const draggedIndex = ref<number | null>(null)
const targetIndex = ref<number | null>(null)

function handleDragStart(e: DragEvent, index: number) {
  draggedIndex.value = index
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move'
    e.dataTransfer.setData('text/plain', index.toString())
  }
}

function handleDragEnd() {
  if (draggedIndex.value !== null && targetIndex.value !== null && draggedIndex.value !== targetIndex.value) {
    store.reorderByIndex(draggedIndex.value, targetIndex.value)
  }
  draggedIndex.value = null
  targetIndex.value = null
}

function handleDragOver(e: DragEvent, index: number) {
  e.preventDefault()
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'move'
  }
  if (draggedIndex.value !== null && index !== targetIndex.value) {
    targetIndex.value = index
  }
}
</script>

<template>
  <div class="space-y-3">
    <!-- 任务列表 -->
    <div class="space-y-3 relative">
      <div
        v-for="(task, index) in store.filteredTasks"
        :key="task.id"
        class="relative"
        draggable="true"
        @dragstart="handleDragStart($event, index)"
        @dragend="handleDragEnd"
        @dragover="handleDragOver($event, index)"
      >
        <!-- 放置指示线 - 顶部 -->
        <div
          v-if="targetIndex === index && draggedIndex !== null && draggedIndex > index"
          class="absolute -top-1.5 left-4 right-4 h-0.5 bg-primary rounded-full z-10 transition-all"
        />

        <!-- 放置指示线 - 底部 -->
        <div
          v-if="targetIndex === index && draggedIndex !== null && draggedIndex < index"
          class="absolute -bottom-1.5 left-4 right-4 h-0.5 bg-primary rounded-full z-10 transition-all"
        />

        <TaskItem
          :task="task"
          :class="{ 'opacity-50 scale-[1.02]': draggedIndex === index }"
        />
      </div>
    </div>

    <!-- 空状态 -->
    <div
      v-if="store.filteredTasks.length === 0"
      class="flex flex-col items-center justify-center py-16 text-center"
    >
      <div class="w-20 h-20 rounded-full bg-cream-200 dark:bg-dark-border flex items-center justify-center mb-4">
        <InboxIcon class="w-10 h-10 text-secondary/50" />
      </div>
      <h3 class="text-lg font-heading font-semibold text-gray-600 dark:text-dark-muted mb-2">
        暂无任务
      </h3>
      <p class="text-gray-400 dark:text-dark-muted max-w-xs">
        {{ store.searchQuery ? '未找到匹配的任务' : '点击上方添加新任务开始你的待办清单' }}
      </p>
    </div>
  </div>
</template>

<style scoped>
/* 列表过渡动画 */
.task-list-move,
.task-list-enter-active,
.task-list-leave-active {
  transition: all 0.3s ease;
}

.task-list-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}

.task-list-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}

.task-list-leave-active {
  position: absolute;
  width: 100%;
}
</style>
