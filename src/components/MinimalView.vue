<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { useAppStore } from '@/stores/app'
import DuePicker from './DuePicker.vue'
import ResizeHandle from './ResizeHandle.vue'
import { NCountdown } from 'naive-ui'
import { getCurrentWindow } from '@tauri-apps/api/window'
import {
  Minus,
  X,
  Pin,
  Clock,
  Plus,
  Trash2,
  Settings,
  Sun,
  Moon,
  Sparkles,
  GripVertical,
  Pencil,
} from 'lucide-vue-next'

const store = useAppStore()
const showSettings = ref(false)
const newTaskTitle = ref('')
const opacity = ref(100)
const isPinned = ref(false)
const showDatePicker = ref(false)
const selectedDue = ref<Date | null>(null)

// 编辑状态
const editingTaskId = ref<string | null>(null)
const editTitle = ref('')

// 开始编辑任务
function startEditTask(taskId: string, title: string) {
  editingTaskId.value = taskId
  editTitle.value = title
}

// 保存编辑
function saveEditTask() {
  if (editingTaskId.value && editTitle.value.trim()) {
    store.updateTask(editingTaskId.value, { title: editTitle.value.trim() })
  }
  editingTaskId.value = null
  editTitle.value = ''
}

// 取消编辑
function cancelEditTask() {
  editingTaskId.value = null
  editTitle.value = ''
}

// 计算各元素的透明度
const containerBgStyle = computed(() => ({
  '--opacity-value': opacity.value / 100,
  backgroundColor: `rgba(248, 247, 252, ${opacity.value / 100})`,
}))

// 窗口控制
function minimizeWindow() {
  getCurrentWindow().minimize()
}

function closeWindow() {
  getCurrentWindow().hide()
}

// 置顶切换
function togglePin() {
  isPinned.value = !isPinned.value
  getCurrentWindow().setAlwaysOnTop(isPinned.value)
}

// 时间选择
function toggleDatePicker() {
  showDatePicker.value = !showDatePicker.value
}

// 添加任务
function addTask() {
  if (!newTaskTitle.value.trim()) return
  const dueDate = selectedDue.value
    ? selectedDue.value.toISOString().split('T')[0]
    : new Date().toISOString().split('T')[0]
  store.addTask(newTaskTitle.value.trim(), dueDate)
  newTaskTitle.value = ''
  selectedDue.value = null
  showDatePicker.value = false
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault()
    addTask()
  }
}

// 清空已完成任务
function clearCompleted() {
  store.tasks = store.tasks.filter(t => t.status !== 'completed')
}

// 计算剩余时间（使用 globalNow 触发响应式更新）
function getTimeRemaining(dueDate: string | null): { text: string; urgent: boolean; overdue: boolean; useCountdown: boolean; duration: number } | null {
  if (!dueDate) return null

  // 使用 globalNow 触发响应式更新
  const now = store.globalNow
  let dueTime: Date

  // 解析本地时间字符串
  if (dueDate.includes('T')) {
    const [datePart, timePart] = dueDate.split('T')
    const [year, month, day] = datePart.split('-').map(Number)
    const [hour, minute, second] = timePart.split(':').map(Number)
    dueTime = new Date(year, month - 1, day, hour, minute, second || 0)
  } else {
    const [year, month, day] = dueDate.split('-').map(Number)
    dueTime = new Date(year, month - 1, day, 23, 59, 59)
  }

  const diff = dueTime.getTime() - now

  // 逾期
  if (diff < 0) {
    const mins = Math.abs(Math.floor(diff / 60000))
    if (mins < 60) return { text: `${mins}m 逾期`, urgent: true, overdue: true, useCountdown: false, duration: 0 }
    const hours = Math.floor(mins / 60)
    if (hours < 24) return { text: `${hours}h 逾期`, urgent: true, overdue: true, useCountdown: false, duration: 0 }
    return { text: `${Math.floor(hours / 24)}d 逾期`, urgent: true, overdue: true, useCountdown: false, duration: 0 }
  }

  // 剩余 ≤ 5分钟，使用 n-countdown
  if (diff <= 300000) {
    return { text: '', urgent: true, overdue: false, useCountdown: true, duration: diff }
  }

  // 剩余时间 > 5分钟
  const mins = Math.floor(diff / 60000)
  if (mins < 60) return { text: `${mins}m 剩余`, urgent: mins < 30, overdue: false, useCountdown: false, duration: 0 }

  const hours = Math.floor(mins / 60)
  if (hours < 24) return { text: `${hours}h ${mins % 60}m`, urgent: hours < 2, overdue: false, useCountdown: false, duration: 0 }

  const days = Math.floor(hours / 24)
  return { text: `${days}d ${hours % 24}h`, urgent: false, overdue: false, useCountdown: false, duration: 0 }
}

// n-countdown 自定义渲染
const renderCountdown = ({ minutes, seconds }: { minutes: number; seconds: number }) => {
  return h('span', {}, `${minutes}m ${seconds}s`)
}

// 今日待办数量
const todayPendingCount = computed(() => {
  return store.filteredTasks.filter(t => t.status !== 'completed').length
})

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
  <div
    class="minimal-container backdrop-blur-xl"
    :style="containerBgStyle"
  >
    <!-- 顶部栏 -->
    <header class="minimal-header" style="-webkit-app-region: drag">
      <div class="flex items-center gap-2" style="-webkit-app-region: no-drag">
        <div class="w-2.5 h-2.5 rounded-full bg-primary"></div>
        <span class="font-bold text-sm text-gray-700 dark:text-cream">TodoList</span>
        <span class="text-xs text-gray-400">v1.0</span>
      </div>

      <div class="flex items-center gap-1" style="-webkit-app-region: no-drag">
        <!-- 设置按钮 -->
        <button
          class="minimal-icon-btn"
          :class="{ 'bg-primary/20': showSettings }"
          @click="showSettings = !showSettings"
        >
          <Settings class="w-4 h-4 text-primary" />
        </button>

        <!-- 置顶按钮 -->
        <button
          class="minimal-icon-btn"
          :class="{ 'bg-primary/20': isPinned }"
          @click="togglePin"
        >
          <Pin class="w-4 h-4" :class="isPinned ? 'text-primary' : 'text-gray-400'" />
        </button>

        <!-- 最小化 -->
        <button class="minimal-icon-btn" @click="minimizeWindow">
          <Minus class="w-4 h-4 text-gray-400" />
        </button>

        <!-- 关闭 -->
        <button class="minimal-icon-btn" @click="closeWindow">
          <X class="w-4 h-4 text-gray-400" />
        </button>
      </div>

      <!-- 设置面板 -->
      <div v-if="showSettings" class="minimal-settings-panel">
        <!-- 透明度 -->
        <div class="settings-item flex-col !items-start gap-2">
          <div class="flex items-center justify-between w-full">
            <span>透明度</span>
            <span class="text-primary">{{ opacity }}%</span>
          </div>
          <input
            v-model="opacity"
            type="range"
            min="20"
            max="100"
            class="w-full accent-primary"
          />
        </div>

        <!-- 标准模式切换 -->
        <div class="settings-item" @click="store.toggleMinimalMode">
          <div class="flex items-center gap-2">
            <Sparkles class="w-4 h-4 text-primary" />
            <span>标准模式</span>
          </div>
          <div class="minimal-toggle" :class="{ active: !store.isMinimalMode }">
            <div class="toggle-dot"></div>
          </div>
        </div>

        <!-- 主题切换 -->
        <div class="settings-item" @click="store.toggleTheme">
          <div class="flex items-center gap-2">
            <component :is="store.isDark ? Sun : Moon" class="w-4 h-4 text-primary" />
            <span>{{ store.isDark ? '浅色' : '深色' }}</span>
          </div>
          <div class="minimal-toggle" :class="{ active: store.isDark }">
            <div class="toggle-dot"></div>
          </div>
        </div>
      </div>
    </header>

    <!-- 点击外部关闭设置 -->
    <div v-if="showSettings" class="fixed inset-0 z-40" @click="showSettings = false"></div>

    <!-- 标题区域 -->
    <div class="minimal-title-section">
      <div>
        <h1 class="text-2xl font-bold text-gray-800 dark:text-white">{{ store.currentViewTitle }}</h1>
        <p class="text-sm text-gray-500">{{ todayPendingCount }} 项待办</p>
      </div>
      <button
        v-if="store.completedCount > 0"
        class="minimal-clear-btn"
        @click="clearCompleted"
      >
        <Trash2 class="w-4 h-4" />
        <span>清空</span>
      </button>
    </div>

    <!-- 任务输入区 -->
    <div class="minimal-input-section">
      <input
        v-model="newTaskTitle"
        type="text"
        placeholder="添加新任务..."
        class="minimal-input"
        @keydown="handleKeydown"
      />
      <button
        class="minimal-time-btn"
        :class="{ 'bg-primary/20': showDatePicker || selectedDue }"
        @click="toggleDatePicker"
      >
        <Clock class="w-5 h-5" :class="showDatePicker || selectedDue ? 'text-primary' : 'text-gray-400'" />
      </button>
      <button class="minimal-add-btn" @click="addTask">
        <Plus class="w-6 h-6 text-white" />
      </button>
    </div>

    <!-- 日期选择器 - 展开动画 -->
    <Transition
      enter-active-class="transition-all duration-200 ease-out"
      leave-active-class="transition-all duration-150 ease-in"
      enter-from-class="opacity-0 max-h-0"
      enter-to-class="opacity-100 max-h-[500px]"
      leave-from-class="opacity-100 max-h-[500px]"
      leave-to-class="opacity-0 max-h-0"
    >
      <div v-if="showDatePicker" class="overflow-hidden px-4 pb-2">
        <DuePicker v-model="selectedDue" />
      </div>
    </Transition>

    <!-- 任务列表 -->
    <div class="minimal-task-list">
      <div class="space-y-2">
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
            class="absolute -top-1 left-2 right-2 h-0.5 bg-primary rounded-full z-10"
          />

          <!-- 放置指示线 - 底部 -->
          <div
            v-if="targetIndex === index && draggedIndex !== null && draggedIndex < index"
            class="absolute -bottom-1 left-2 right-2 h-0.5 bg-primary rounded-full z-10"
          />

          <div
            v-motion
            :initial="{ opacity: 0, y: -8, scale: 0.95 }"
            :enter="{ opacity: 1, y: 0, scale: 1, transition: { type: 'spring', stiffness: 300, damping: 25 } }"
            :hovered="{ scale: 1.02 }"
            class="minimal-task-item group"
            :class="{
              completed: task.status === 'completed',
              'opacity-50 scale-[1.02]': draggedIndex === index
            }"
          >
            <!-- 拖拽手柄 -->
            <div class="drag-handle flex-shrink-0 cursor-grab opacity-0 group-hover:opacity-100 transition-opacity duration-200 -ml-1">
              <GripVertical class="w-4 h-4 text-gray-400" />
            </div>

            <button
              class="minimal-checkbox"
              :class="{ checked: task.status === 'completed' }"
              @click="store.toggleTask(task.id)"
            >
              <svg v-if="task.status === 'completed'" class="w-3 h-3 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                <polyline points="20 6 9 17 4 12"></polyline>
              </svg>
            </button>

            <div class="flex-1 min-w-0">
              <!-- 编辑模式 -->
              <form v-if="editingTaskId === task.id" class="flex-1" @submit.prevent="saveEditTask">
                <input
                  v-model="editTitle"
                  type="text"
                  class="w-full bg-transparent border-none outline-none text-sm font-medium text-gray-800 dark:text-white"
                  autofocus
                  @blur="saveEditTask"
                  @keydown.esc="cancelEditTask"
                />
              </form>
              <!-- 显示模式 -->
              <span v-else class="minimal-task-title" :class="{ 'line-through text-gray-400': task.status === 'completed' }">
                {{ task.title }}
              </span>
              <div class="flex items-center gap-2 mt-1">
                <!-- 标签 -->
                <span
                  v-for="tagId in task.tags.slice(0, 2)"
                  :key="tagId"
                  class="text-xs px-1.5 py-0.5 rounded-full"
                  :style="{ backgroundColor: store.tags.find(t => t.id === tagId)?.color + '20', color: store.tags.find(t => t.id === tagId)?.color }"
                >
                  {{ store.tags.find(t => t.id === tagId)?.name }}
                </span>
                <!-- 剩余时间（普通显示） -->
                <div
                  v-if="getTimeRemaining(task.dueDate) && task.status !== 'completed' && !getTimeRemaining(task.dueDate)?.useCountdown"
                  class="minimal-task-time"
                  :class="[
                    getTimeRemaining(task.dueDate)?.overdue ? 'text-red-500' :
                    getTimeRemaining(task.dueDate)?.urgent ? 'text-amber-500' :
                    'text-gray-500'
                  ]"
                >
                  <Clock class="w-3 h-3" />
                  <span>{{ getTimeRemaining(task.dueDate)?.text }}</span>
                </div>
                <!-- 剩余时间（n-countdown 秒级，≤5分钟） -->
                <div
                  v-if="getTimeRemaining(task.dueDate) && task.status !== 'completed' && getTimeRemaining(task.dueDate)?.useCountdown"
                  class="minimal-task-time text-red-500"
                >
                  <Clock class="w-3 h-3" />
                  <n-countdown
                    :duration="getTimeRemaining(task.dueDate)?.duration || 0"
                    :render="renderCountdown"
                  />
                </div>
              </div>
            </div>

            <!-- 操作按钮（悬停显示） -->
            <div
              v-if="task.status !== 'completed'"
              class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
            >
              <!-- 编辑 -->
              <button
                class="p-1.5 rounded-lg hover:bg-cream-200 dark:hover:bg-dark-border cursor-pointer transition-colors"
                @click.stop="startEditTask(task.id, task.title)"
                title="编辑"
              >
                <Pencil class="w-3.5 h-3.5 text-gray-400 hover:text-primary" />
              </button>
              <!-- 删除 -->
              <button
                class="p-1.5 rounded-lg hover:bg-red-100 dark:hover:bg-red-900/20 cursor-pointer transition-colors"
                @click.stop="store.deleteTask(task.id)"
                title="删除"
              >
                <Trash2 class="w-3.5 h-3.5 text-gray-400 hover:text-red-500" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-if="store.filteredTasks.length === 0" class="minimal-empty">
        <p class="text-gray-400">暂无任务</p>
      </div>
    </div>

    <!-- Resize 手柄 -->
    <ResizeHandle :opacity="opacity / 100" />
  </div>
</template>
