<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAppStore } from '@/stores/app'
import { Plus, Calendar, Repeat } from 'lucide-vue-next'
import DuePicker from './DuePicker.vue'
import type { Priority, RepeatType } from '@/types'

const store = useAppStore()

const newTaskTitle = ref('')
const selectedDue = ref<Date | null>(null)
const selectedPriority = ref<Priority>('medium')
const selectedTagId = ref<string | null>(null)
const selectedRepeat = ref<RepeatType>('none')
const showDatePicker = ref(false)

// 优先级选项
const priorityOptions = ['high', 'medium', 'low'] as const
const priorityLabels: Record<Priority, string> = { high: '高', medium: '中', low: '低' }

// 重复周期选项
const repeatOptions: RepeatType[] = ['none', 'daily', 'weekly', 'monthly']
const repeatLabels: Record<RepeatType, string> = {
  none: '不重复',
  daily: '每天',
  weekly: '每周',
  monthly: '每月',
  custom: '自定义'
}

// 切换优先级
function cyclePriority() {
  const currentIndex = priorityOptions.indexOf(selectedPriority.value)
  const nextIndex = (currentIndex + 1) % priorityOptions.length
  selectedPriority.value = priorityOptions[nextIndex]
}

// 切换重复周期
function cycleRepeat() {
  const currentIndex = repeatOptions.indexOf(selectedRepeat.value)
  const nextIndex = (currentIndex + 1) % repeatOptions.length
  selectedRepeat.value = repeatOptions[nextIndex]
}

// 优先级样式
const priorityClass = computed(() => {
  switch (selectedPriority.value) {
    case 'high': return 'bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400'
    case 'medium': return 'bg-amber-100 text-amber-600 dark:bg-amber-900/30 dark:text-amber-400'
    case 'low': return 'bg-primary/10 text-primary'
    default: return 'bg-gray-100 text-gray-600'
  }
})

// 当前标签
const currentTag = computed(() => {
  if (!selectedTagId.value) return null
  return store.tags.find(t => t.id === selectedTagId.value) || null
})

// 切换标签（包含"无标签"选项）
function cycleTag() {
  const allTags = store.tags
  if (allTags.length === 0) return

  if (!selectedTagId.value) {
    // 当前无标签，切换到第一个标签
    selectedTagId.value = allTags[0].id
  } else {
    const currentIndex = allTags.findIndex(t => t.id === selectedTagId.value)
    if (currentIndex === allTags.length - 1) {
      // 最后一个标签，切换到无标签
      selectedTagId.value = null
    } else {
      // 切换到下一个标签
      selectedTagId.value = allTags[currentIndex + 1].id
    }
  }
}

function addTask() {
  if (!newTaskTitle.value.trim()) return
  // 使用本地时间格式存储，避免时区问题
  let dueDate: string
  if (selectedDue.value) {
    const d = selectedDue.value
    const year = d.getFullYear()
    const month = (d.getMonth() + 1).toString().padStart(2, '0')
    const day = d.getDate().toString().padStart(2, '0')
    const hour = d.getHours().toString().padStart(2, '0')
    const minute = d.getMinutes().toString().padStart(2, '0')
    dueDate = `${year}-${month}-${day}T${hour}:${minute}:00`
  } else {
    // 默认为今天
    const today = new Date()
    const year = today.getFullYear()
    const month = (today.getMonth() + 1).toString().padStart(2, '0')
    const day = today.getDate().toString().padStart(2, '0')
    dueDate = `${year}-${month}-${day}`
  }
  store.addTask(newTaskTitle.value.trim(), dueDate, selectedPriority.value, selectedTagId.value, selectedRepeat.value)
  // 重置表单
  newTaskTitle.value = ''
  selectedDue.value = null
  selectedPriority.value = 'medium'
  selectedTagId.value = null
  selectedRepeat.value = 'none'
  showDatePicker.value = false
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    addTask()
  }
}

// 格式化日期为本地 YYYY-MM-DD
function formatDateLocal(d: Date): string {
  const year = d.getFullYear()
  const month = (d.getMonth() + 1).toString().padStart(2, '0')
  const day = d.getDate().toString().padStart(2, '0')
  return `${year}-${month}-${day}`
}

function formatDate(date: Date | null): string {
  if (!date) return ''
  const today = new Date()
  const tomorrow = new Date(today)
  tomorrow.setDate(tomorrow.getDate() + 1)
  const dateStr = formatDateLocal(date)
  const todayStr = formatDateLocal(today)
  const tomorrowStr = formatDateLocal(tomorrow)

  if (dateStr === todayStr) {
    return '今天'
  } else if (dateStr === tomorrowStr) {
    return '明天'
  } else {
    return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
  }
}
</script>

<template>
  <div class="mb-6 space-y-3">
    <div class="card-organic p-4">
      <div class="flex items-center gap-3">
        <!-- 主输入框 -->
        <div class="flex-1 relative">
          <input
            v-model="newTaskTitle"
            type="text"
            placeholder="添加新任务..."
            class="input-organic pr-24"
            @keydown="handleKeydown"
          />

          <!-- 内嵌按钮组 -->
          <div class="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-1">
            <!-- 日期选择 -->
            <button
              class="flex items-center gap-1 px-2 py-1 text-sm rounded-lg hover:bg-cream-200 dark:hover:bg-dark-border cursor-pointer transition-colors"
              :class="selectedDue ? 'text-primary' : 'text-gray-400'"
              @click="showDatePicker = !showDatePicker"
            >
              <Calendar class="w-4 h-4" />
              <span v-if="formatDate(selectedDue)" class="hidden sm:inline">{{ formatDate(selectedDue) }}</span>
            </button>

            <!-- 标签选择（点击切换） -->
            <button
              class="tag flex-shrink-0 text-xs transition-all hover:scale-105"
              :style="currentTag ? {
                backgroundColor: `${currentTag.color}20`,
                color: currentTag.color
              } : {
                backgroundColor: '#9ca3af20',
                color: '#9ca3af'
              }"
              @click="cycleTag"
              title="点击切换标签"
            >
              {{ currentTag ? currentTag.name : '标签' }}
            </button>

            <!-- 优先级选择（点击切换） -->
            <button
              class="tag flex-shrink-0 text-xs transition-all hover:scale-105"
              :class="priorityClass"
              @click="cyclePriority"
              title="点击切换优先级"
            >
              {{ priorityLabels[selectedPriority] }}
            </button>

            <!-- 重复周期选择（点击切换） -->
            <button
              class="tag flex-shrink-0 text-xs transition-all hover:scale-105"
              :class="selectedRepeat !== 'none' ? 'bg-primary/10 text-primary' : 'bg-gray-100 text-gray-500 dark:bg-dark-border dark:text-gray-400'"
              @click="cycleRepeat"
              title="点击切换重复周期"
            >
              <Repeat class="w-3 h-3 inline-block mr-0.5" />
              {{ repeatLabels[selectedRepeat] }}
            </button>
          </div>
        </div>

        <!-- 添加按钮 -->
        <button
          class="btn-primary flex-shrink-0"
          @click="addTask"
        >
          <Plus class="w-5 h-5" />
          <span class="hidden sm:inline">添加</span>
        </button>
      </div>
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
      <div v-if="showDatePicker" class="overflow-hidden">
        <DuePicker v-model="selectedDue" />
      </div>
    </Transition>

  </div>
</template>
