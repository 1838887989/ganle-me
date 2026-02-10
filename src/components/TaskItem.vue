<script setup lang="ts">
import { computed, ref, h } from 'vue'
import { useAppStore } from '@/stores/app'
import type { Task } from '@/types'
import { NCountdown } from 'naive-ui'
import {
  Star,
  Trash2,
  Check,
  RefreshCw,
  ChevronDown,
  ChevronRight,
  GripVertical,
  Plus,
  X,
  Repeat,
  Pencil,
  Clock,
  ChevronLeft,
  ListTodo,
} from 'lucide-vue-next'

const props = defineProps<{
  task: Task
}>()

const store = useAppStore()

// 子任务展开状态
const isExpanded = ref(false)
const newSubtaskTitle = ref('')

// 编辑状态
const showDatePicker = ref(false)
const isEditingTitle = ref(false)
const editTitle = ref('')

// 时间选择器模态框状态
const showDueModal = ref(false)
const selectedDate = ref('')
const selectedHour = ref('12')
const selectedMinute = ref('00')
const calendarMonth = ref(new Date())

// 优先级选项
const priorityOptions = ['high', 'medium', 'low'] as const
const priorityLabels = { high: '高', medium: '中', low: '低' }

// 重复选项
const repeatOptions = ['none', 'daily', 'weekly', 'monthly'] as const
const repeatLabels = { none: '不重复', daily: '每天', weekly: '每周', monthly: '每月' }

// 切换优先级（滚动效果）
function cyclePriority() {
  const currentIndex = priorityOptions.indexOf(props.task.priority)
  const nextIndex = (currentIndex + 1) % priorityOptions.length
  store.updateTask(props.task.id, { priority: priorityOptions[nextIndex] })
}

// 切换重复（滚动效果）
function cycleRepeat() {
  const currentIndex = repeatOptions.indexOf(props.task.repeat)
  const nextIndex = (currentIndex + 1) % repeatOptions.length
  store.updateTask(props.task.id, { repeat: repeatOptions[nextIndex] })
}

// 切换标签（包含"无标签"选项）
function cycleTag() {
  const allTags = store.tags
  if (allTags.length === 0) return

  const currentTagId = props.task.tags[0] || null

  if (!currentTagId) {
    // 当前无标签，切换到第一个标签
    store.updateTask(props.task.id, { tags: [allTags[0].id] })
  } else {
    const currentIndex = allTags.findIndex(t => t.id === currentTagId)
    if (currentIndex === allTags.length - 1) {
      // 最后一个标签，切换到无标签
      store.updateTask(props.task.id, { tags: [] })
    } else {
      // 切换到下一个标签
      store.updateTask(props.task.id, { tags: [allTags[currentIndex + 1].id] })
    }
  }
}

// 当前显示的标签
const currentTag = computed(() => {
  if (props.task.tags.length === 0) return null
  return store.tags.find(t => t.id === props.task.tags[0]) || null
})

// 更新日期
function updateDueDate(e: Event) {
  const target = e.target as HTMLInputElement
  store.updateTask(props.task.id, { dueDate: target.value || null })
  showDatePicker.value = false
}

// 开始编辑标题
function startEditTitle() {
  editTitle.value = props.task.title
  isEditingTitle.value = true
}

// 保存标题
function saveTitle() {
  if (editTitle.value.trim()) {
    store.updateTask(props.task.id, { title: editTitle.value.trim() })
  }
  isEditingTitle.value = false
}

// 取消编辑
function cancelEditTitle() {
  isEditingTitle.value = false
  editTitle.value = ''
}

// 打开时间选择器模态框
function openDueModal() {
  if (props.task.dueDate) {
    // 解析本地时间字符串 YYYY-MM-DDTHH:mm:ss
    if (props.task.dueDate.includes('T')) {
      const [datePart, timePart] = props.task.dueDate.split('T')
      selectedDate.value = datePart
      const [hour, minute] = timePart.split(':')
      selectedHour.value = hour
      selectedMinute.value = minute
      const [year, month] = datePart.split('-').map(Number)
      calendarMonth.value = new Date(year, month - 1, 1)
    } else {
      // 旧格式，仅日期
      selectedDate.value = props.task.dueDate
      selectedHour.value = '12'
      selectedMinute.value = '00'
      const [year, month] = props.task.dueDate.split('-').map(Number)
      calendarMonth.value = new Date(year, month - 1, 1)
    }
  } else {
    selectedDate.value = ''
    selectedHour.value = '12'
    selectedMinute.value = '00'
    calendarMonth.value = new Date()
  }
  showDueModal.value = true
}

// 保存时间（存储本地时间字符串，避免时区问题）
function saveDue() {
  if (selectedDate.value) {
    const h = Math.min(23, Math.max(0, parseInt(selectedHour.value) || 0))
    const m = Math.min(59, Math.max(0, parseInt(selectedMinute.value) || 0))
    // 直接存储本地时间字符串，格式：YYYY-MM-DDTHH:mm:ss
    const dueDateTime = `${selectedDate.value}T${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:00`
    store.updateTask(props.task.id, { dueDate: dueDateTime })
  } else {
    store.updateTask(props.task.id, { dueDate: null })
  }
  showDueModal.value = false
}

// 清除时间
function clearDue() {
  store.updateTask(props.task.id, { dueDate: null })
  showDueModal.value = false
}

// 日历辅助函数
function formatDateLocal(d: Date) {
  const year = d.getFullYear()
  const month = (d.getMonth() + 1).toString().padStart(2, '0')
  const day = d.getDate().toString().padStart(2, '0')
  return `${year}-${month}-${day}`
}

function getDaysInMonth(date: Date) {
  return new Date(date.getFullYear(), date.getMonth() + 1, 0).getDate()
}

function getFirstDayOfMonth(date: Date) {
  return new Date(date.getFullYear(), date.getMonth(), 1).getDay()
}

function isPastDay(day: number) {
  const d = new Date(calendarMonth.value.getFullYear(), calendarMonth.value.getMonth(), day)
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  return d < today
}

function selectDate(day: number) {
  selectedDate.value = formatDateLocal(
    new Date(calendarMonth.value.getFullYear(), calendarMonth.value.getMonth(), day)
  )
}

function prevMonth() {
  calendarMonth.value = new Date(
    calendarMonth.value.getFullYear(),
    calendarMonth.value.getMonth() - 1,
    1
  )
}

function nextMonth() {
  calendarMonth.value = new Date(
    calendarMonth.value.getFullYear(),
    calendarMonth.value.getMonth() + 1,
    1
  )
}

const isCompleted = computed(() => props.task.status === 'completed')

const priorityClass = computed(() => {
  switch (props.task.priority) {
    case 'high':
      return 'priority-high'
    case 'medium':
      return 'priority-medium'
    case 'low':
      return 'priority-low'
    default:
      return ''
  }
})

const priorityLabel = computed(() => {
  switch (props.task.priority) {
    case 'high':
      return '高'
    case 'medium':
      return '中'
    case 'low':
      return '低'
    default:
      return ''
  }
})

// 倒计时信息（使用 globalNow 触发响应式更新）
const timeInfo = computed(() => {
  if (!props.task.dueDate) return null

  // 使用 globalNow 触发响应式更新（每分钟）
  const now = store.globalNow
  let dueTime: Date

  // 解析本地时间字符串
  if (props.task.dueDate.includes('T')) {
    const [datePart, timePart] = props.task.dueDate.split('T')
    const [year, month, day] = datePart.split('-').map(Number)
    const [hour, minute, second] = timePart.split(':').map(Number)
    dueTime = new Date(year, month - 1, day, hour, minute, second || 0)
  } else {
    const [year, month, day] = props.task.dueDate.split('-').map(Number)
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

  // 剩余时间 > 5分钟，普通显示
  const mins = Math.floor(diff / 60000)
  if (mins < 60) return { text: `${mins}m 剩余`, urgent: mins < 30, overdue: false, useCountdown: false, duration: 0 }

  const hours = Math.floor(mins / 60)
  if (hours < 24) return { text: `${hours}h ${mins % 60}m`, urgent: hours < 2, overdue: false, useCountdown: false, duration: 0 }

  const days = Math.floor(hours / 24)
  return { text: `${days}d ${hours % 24}h`, urgent: false, overdue: false, useCountdown: false, duration: 0 }
})

// n-countdown 自定义渲染
const renderCountdown = ({ minutes, seconds }: { minutes: number; seconds: number }) => {
  return h('span', {}, `${minutes}m ${seconds}s`)
}

const formattedDueDate = computed(() => {
  return timeInfo.value?.text || ''
})

const isOverdue = computed(() => {
  if (!props.task.dueDate || isCompleted.value) return false
  return props.task.dueDate < new Date().toISOString().split('T')[0]
})

// 获取任务标签
const taskTags = computed(() => {
  return store.tags.filter(tag => props.task.tags.includes(tag.id))
})

// 子任务完成进度
const subtaskProgress = computed(() => {
  if (props.task.subtasks.length === 0) return null
  const completed = props.task.subtasks.filter(s => s.completed).length
  return `${completed}/${props.task.subtasks.length}`
})

// 切换子任务展开
function toggleExpand() {
  if (props.task.subtasks.length > 0 || !isCompleted.value) {
    isExpanded.value = !isExpanded.value
  }
}

// 添加子任务
function addSubtask() {
  if (!newSubtaskTitle.value.trim()) return
  store.addSubtask(props.task.id, newSubtaskTitle.value.trim())
  newSubtaskTitle.value = ''
}

// 处理子任务输入回车
function handleSubtaskKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault()
    addSubtask()
  }
}
</script>

<template>
  <div class="task-item-wrapper">
    <div
      v-motion
      :initial="{ opacity: 0, y: -10, scale: 0.95 }"
      :enter="{ opacity: 1, y: 0, scale: 1, transition: { type: 'spring', stiffness: 300, damping: 25 } }"
      :hovered="{ scale: 1.01 }"
      class="task-item group"
      :class="{ completed: isCompleted }"
    >
    <!-- 拖拽手柄 -->
    <div class="drag-handle flex-shrink-0 cursor-grab opacity-0 group-hover:opacity-100 transition-opacity duration-200 -ml-1 mr-1">
      <GripVertical class="w-4 h-4 text-gray-400" />
    </div>

    <!-- 复选框 -->
    <button
      class="flex-shrink-0 w-6 h-6 rounded-full border-2 flex items-center justify-center cursor-pointer transition-all duration-200"
      :class="[
        isCompleted
          ? 'bg-primary border-primary'
          : 'border-secondary/40 hover:border-primary hover:bg-primary/10'
      ]"
      @click="store.toggleTask(task.id)"
    >
      <Check v-if="isCompleted" class="w-4 h-4 text-white" />
    </button>

    <!-- 任务内容 -->
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2 mb-1">
        <!-- 编辑模式 -->
        <form
          v-if="isEditingTitle"
          class="flex-1"
          @submit.prevent="saveTitle"
        >
          <input
            v-model="editTitle"
            type="text"
            class="w-full bg-transparent border-none outline-none font-medium text-gray-800 dark:text-dark-text"
            autofocus
            @blur="saveTitle"
            @keydown.esc="cancelEditTitle"
          />
        </form>
        <!-- 显示模式 -->
        <span
          v-else
          class="task-title font-medium text-gray-800 dark:text-dark-text truncate"
          :class="{ 'line-through text-gray-400 dark:text-dark-muted': isCompleted }"
        >
          {{ task.title }}
        </span>

        <!-- 重复任务标记 -->
        <RefreshCw
          v-if="task.repeat !== 'none'"
          class="w-3.5 h-3.5 text-secondary flex-shrink-0"
        />
      </div>

      <!-- 描述和元信息 -->
      <div class="flex items-center gap-3 text-sm">
        <!-- 截止日期（普通显示） -->
        <span
          v-if="timeInfo && !isCompleted && !timeInfo.useCountdown"
          class="flex items-center gap-1 text-xs font-medium"
          :class="[
            timeInfo.overdue ? 'text-red-500' :
            timeInfo.urgent ? 'text-amber-500' :
            'text-gray-500 dark:text-dark-muted'
          ]"
        >
          <Clock class="w-3.5 h-3.5" />
          {{ timeInfo.text }}
        </span>

        <!-- 截止日期（n-countdown 秒级倒计时，≤5分钟） -->
        <span
          v-if="timeInfo && !isCompleted && timeInfo.useCountdown"
          class="flex items-center gap-1 text-xs font-medium text-red-500"
        >
          <Clock class="w-3.5 h-3.5" />
          <n-countdown
            :duration="timeInfo.duration"
            :render="renderCountdown"
          />
        </span>

        <!-- 子任务进度 -->
        <button
          v-if="task.subtasks.length > 0"
          class="flex items-center gap-1 text-gray-500 dark:text-dark-muted hover:text-primary transition-colors"
          @click.stop="toggleExpand"
        >
          <component :is="isExpanded ? ChevronDown : ChevronRight" class="w-3.5 h-3.5" />
          {{ subtaskProgress }}
        </button>

        <!-- 标签（点击切换） -->
        <button
          class="tag flex-shrink-0 transition-all hover:scale-105"
          :style="currentTag ? {
            backgroundColor: `${currentTag.color}20`,
            color: currentTag.color
          } : {
            backgroundColor: '#9ca3af20',
            color: '#9ca3af'
          }"
          @click.stop="cycleTag"
          title="点击切换标签"
        >
          {{ currentTag ? currentTag.name : '无标签' }}
        </button>

        <!-- 优先级（可点击切换） -->
        <button
          class="tag flex-shrink-0 transition-all hover:scale-105"
          :class="priorityClass"
          @click.stop="cyclePriority"
          title="点击切换优先级"
        >
          {{ priorityLabel }}
        </button>

        <!-- 重复（可点击切换） -->
        <button
          v-if="task.repeat !== 'none'"
          class="tag flex-shrink-0 bg-primary/10 text-primary transition-all hover:scale-105"
          @click.stop="cycleRepeat"
          title="点击切换重复"
        >
          <Repeat class="w-3 h-3 mr-1" />
          {{ repeatLabels[task.repeat] }}
        </button>
      </div>
    </div>

    <!-- 操作按钮组（悬停显示，已完成时隐藏） -->
    <div
      v-if="!isCompleted"
      class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
    >
      <!-- 时间选择 -->
      <button
        class="p-2 rounded-lg hover:bg-cream-200 dark:hover:bg-dark-border cursor-pointer transition-colors"
        @click.stop="openDueModal"
        title="修改截止时间"
      >
        <Clock class="w-4 h-4 text-gray-400 hover:text-primary" />
      </button>

      <!-- 子任务 -->
      <button
        class="p-2 rounded-lg hover:bg-cream-200 dark:hover:bg-dark-border cursor-pointer transition-colors"
        @click.stop="toggleExpand"
        title="子任务"
      >
        <ListTodo class="w-4 h-4" :class="task.subtasks.length > 0 ? 'text-primary' : 'text-gray-400 hover:text-primary'" />
      </button>

      <!-- 编辑标题 -->
      <button
        class="p-2 rounded-lg hover:bg-cream-200 dark:hover:bg-dark-border cursor-pointer transition-colors"
        @click.stop="startEditTitle"
        title="编辑任务"
      >
        <Pencil class="w-4 h-4 text-gray-400 hover:text-primary" />
      </button>

      <!-- 收藏 -->
      <button
        class="p-2 rounded-lg hover:bg-cream-200 dark:hover:bg-dark-border cursor-pointer transition-colors"
        @click.stop="store.toggleFavorite(task.id)"
      >
        <Star
          class="w-4 h-4"
          :class="task.favorite ? 'text-accent fill-accent' : 'text-gray-400'"
        />
      </button>

      <!-- 删除 -->
      <button
        class="p-2 rounded-lg hover:bg-red-100 dark:hover:bg-red-900/20 cursor-pointer transition-colors"
        @click.stop="store.deleteTask(task.id)"
        title="删除"
      >
        <Trash2 class="w-4 h-4 text-gray-400 hover:text-red-500" />
      </button>
    </div>
    </div>

    <!-- 子任务列表（展开时显示） -->
    <Transition
      enter-active-class="transition-all duration-200 ease-out"
      leave-active-class="transition-all duration-150 ease-in"
      enter-from-class="opacity-0 max-h-0"
      enter-to-class="opacity-100 max-h-96"
      leave-from-class="opacity-100 max-h-96"
      leave-to-class="opacity-0 max-h-0"
    >
      <div v-if="isExpanded" class="overflow-hidden">
        <div class="subtask-container">
          <!-- 子任务列表 -->
          <div v-if="task.subtasks.length > 0" class="space-y-1 mb-2">
            <div
              v-for="subtask in task.subtasks"
              :key="subtask.id"
              class="subtask-item group/subtask"
            >
              <button
                class="subtask-checkbox"
                :class="{ checked: subtask.completed }"
                @click="store.toggleSubtask(task.id, subtask.id)"
              >
                <Check v-if="subtask.completed" class="w-2.5 h-2.5 text-white" />
              </button>
              <span
                class="flex-1 text-sm"
                :class="subtask.completed ? 'line-through text-gray-400' : 'text-gray-600 dark:text-dark-text'"
              >
                {{ subtask.title }}
              </span>
              <button
                class="p-1 rounded opacity-0 group-hover/subtask:opacity-100 hover:bg-red-100 dark:hover:bg-red-900/20 transition-all"
                @click="store.deleteSubtask(task.id, subtask.id)"
              >
                <X class="w-3 h-3 text-gray-400 hover:text-red-500" />
              </button>
            </div>
          </div>

          <!-- 添加子任务输入框 -->
          <div v-if="!isCompleted" class="flex items-center gap-2">
            <input
              v-model="newSubtaskTitle"
              type="text"
              placeholder="添加子任务..."
              class="subtask-input"
              @keydown="handleSubtaskKeydown"
            />
            <button
              class="p-1.5 rounded-lg bg-primary/10 hover:bg-primary/20 transition-colors"
              @click="addSubtask"
            >
              <Plus class="w-4 h-4 text-primary" />
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </div>

  <!-- 时间选择器模态框 -->
  <Teleport to="body">
    <Transition
      enter-active-class="transition-all duration-200"
      leave-active-class="transition-all duration-150"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="showDueModal"
        class="fixed inset-0 bg-black/20 backdrop-blur-sm z-50 flex items-center justify-center p-4"
        @click="showDueModal = false"
      >
        <div
          class="w-full max-w-[280px] bg-white dark:bg-dark-surface rounded-3xl shadow-2xl border border-white/50 dark:border-dark-border overflow-hidden"
          @click.stop
        >
          <div class="p-4 space-y-4">
            <!-- 标题 -->
            <div class="flex items-center justify-between">
              <h3 class="text-sm font-bold text-gray-800 dark:text-dark-text">修改截止时间</h3>
              <button
                class="p-1 hover:bg-gray-100 dark:hover:bg-dark-border rounded-full"
                @click="showDueModal = false"
              >
                <X class="w-4 h-4 text-gray-400" />
              </button>
            </div>

            <!-- 日历导航 -->
            <div class="flex items-center justify-between">
              <button
                class="p-1 hover:bg-primary/10 rounded-lg text-gray-400"
                @click="prevMonth"
              >
                <ChevronLeft class="w-4 h-4" />
              </button>
              <span class="text-xs font-bold text-gray-700 dark:text-dark-text">
                {{ calendarMonth.toLocaleDateString('zh-CN', { year: 'numeric', month: 'long' }) }}
              </span>
              <button
                class="p-1 hover:bg-primary/10 rounded-lg text-gray-400"
                @click="nextMonth"
              >
                <ChevronRight class="w-4 h-4" />
              </button>
            </div>

            <!-- 星期标题 -->
            <div class="grid grid-cols-7 gap-1">
              <div
                v-for="day in ['日', '一', '二', '三', '四', '五', '六']"
                :key="day"
                class="text-[10px] font-semibold text-gray-400 text-center py-1"
              >
                {{ day }}
              </div>
            </div>

            <!-- 日历日期 -->
            <div class="grid grid-cols-7 gap-1">
              <!-- 空白填充 -->
              <div
                v-for="i in getFirstDayOfMonth(calendarMonth)"
                :key="'empty-' + i"
              />
              <!-- 日期按钮 -->
              <button
                v-for="day in getDaysInMonth(calendarMonth)"
                :key="day"
                type="button"
                :disabled="isPastDay(day)"
                class="w-7 h-7 rounded-lg text-xs font-semibold transition-all duration-100"
                :class="[
                  selectedDate === formatDateLocal(new Date(calendarMonth.getFullYear(), calendarMonth.getMonth(), day))
                    ? 'bg-primary text-white shadow-md'
                    : formatDateLocal(new Date()) === formatDateLocal(new Date(calendarMonth.getFullYear(), calendarMonth.getMonth(), day))
                      ? 'bg-primary/10 text-primary'
                      : isPastDay(day)
                        ? 'text-gray-300 cursor-not-allowed'
                        : 'text-gray-700 dark:text-dark-text hover:bg-primary/10 cursor-pointer'
                ]"
                @click="selectDate(day)"
              >
                {{ day }}
              </button>
            </div>

            <!-- 时间选择器 -->
            <div class="flex items-center justify-center gap-2 pt-3 border-t border-gray-100 dark:border-dark-border">
              <input
                v-model="selectedHour"
                type="text"
                maxlength="2"
                class="w-10 h-10 rounded-xl bg-gray-50 dark:bg-dark-border text-center text-sm font-bold text-primary outline-none focus:bg-white dark:focus:bg-dark-surface border border-transparent focus:border-primary/20"
                @focus="($event.target as HTMLInputElement).select()"
              />
              <span class="font-bold text-gray-300">:</span>
              <input
                v-model="selectedMinute"
                type="text"
                maxlength="2"
                class="w-10 h-10 rounded-xl bg-gray-50 dark:bg-dark-border text-center text-sm font-bold text-primary outline-none focus:bg-white dark:focus:bg-dark-surface border border-transparent focus:border-primary/20"
                @focus="($event.target as HTMLInputElement).select()"
              />
            </div>

            <!-- 操作按钮 -->
            <div class="flex gap-2 pt-2">
              <button
                class="flex-1 py-2.5 text-xs font-bold text-red-500 bg-red-50 dark:bg-red-900/20 rounded-2xl hover:bg-red-100 dark:hover:bg-red-900/30 transition-colors"
                @click="clearDue"
              >
                清除
              </button>
              <button
                class="flex-1 py-2.5 text-xs font-bold text-white bg-primary rounded-2xl hover:bg-primary/90 shadow-lg shadow-primary/20 transition-colors"
                @click="saveDue"
              >
                保存
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
