<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ChevronLeft, ChevronRight } from 'lucide-vue-next'

const props = defineProps<{
  modelValue: Date | null
}>()

const emit = defineEmits<{
  'update:modelValue': [value: Date | null]
  'confirm': []
}>()

// 状态
const selectedDate = ref('')
const hour = ref('12')
const minute = ref('00')
const calendarMonth = ref(new Date())

// 初始化
watch(() => props.modelValue, (val) => {
  if (val) {
    const d = new Date(val)
    selectedDate.value = formatDateLocal(d)
    hour.value = d.getHours().toString().padStart(2, '0')
    minute.value = d.getMinutes().toString().padStart(2, '0')
    calendarMonth.value = new Date(d.getFullYear(), d.getMonth(), 1)
  }
}, { immediate: true })

// 格式化日期为 YYYY-MM-DD
function formatDateLocal(d: Date) {
  const year = d.getFullYear()
  const month = (d.getMonth() + 1).toString().padStart(2, '0')
  const day = d.getDate().toString().padStart(2, '0')
  return `${year}-${month}-${day}`
}

// 快速选项
const quickOptions = [
  { label: '今天', days: 0 },
  { label: '明天', days: 1 },
  { label: '3天后', days: 3 },
  { label: '下周', days: 7 },
]

function selectQuickOption(days: number) {
  const d = new Date()
  d.setDate(d.getDate() + days)
  selectedDate.value = formatDateLocal(d)
  updateValue()
}

// 时间处理
function normalizeHour(val: string) {
  const num = parseInt(val) || 0
  return Math.min(23, Math.max(0, num)).toString().padStart(2, '0')
}

function normalizeMinute(val: string) {
  const num = parseInt(val) || 0
  return Math.min(59, Math.max(0, num)).toString().padStart(2, '0')
}

function setQuickTime(h: string) {
  hour.value = h
  minute.value = '00'
  updateValue()
}

// 日历相关
const getDaysInMonth = (date: Date) => new Date(date.getFullYear(), date.getMonth() + 1, 0).getDate()
const getFirstDayOfMonth = (date: Date) => new Date(date.getFullYear(), date.getMonth(), 1).getDay()

function prevMonth() {
  calendarMonth.value = new Date(calendarMonth.value.getFullYear(), calendarMonth.value.getMonth() - 1, 1)
}

function nextMonth() {
  calendarMonth.value = new Date(calendarMonth.value.getFullYear(), calendarMonth.value.getMonth() + 1, 1)
}

function selectDay(day: number) {
  const d = new Date(calendarMonth.value.getFullYear(), calendarMonth.value.getMonth(), day)
  selectedDate.value = formatDateLocal(d)
  updateValue()
}

function isToday(day: number) {
  const today = new Date()
  return calendarMonth.value.getFullYear() === today.getFullYear() &&
         calendarMonth.value.getMonth() === today.getMonth() &&
         day === today.getDate()
}

function isPast(day: number) {
  const d = new Date(calendarMonth.value.getFullYear(), calendarMonth.value.getMonth(), day)
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  return d < today
}

function isSelected(day: number) {
  if (!selectedDate.value) return false
  const d = new Date(calendarMonth.value.getFullYear(), calendarMonth.value.getMonth(), day)
  return formatDateLocal(d) === selectedDate.value
}

// 日历数据
const calendarData = computed(() => {
  const daysInMonth = getDaysInMonth(calendarMonth.value)
  const firstDay = getFirstDayOfMonth(calendarMonth.value)
  const days = []

  // 空白格子
  for (let i = 0; i < firstDay; i++) {
    days.push({ day: 0, empty: true })
  }

  // 日期格子
  for (let day = 1; day <= daysInMonth; day++) {
    days.push({
      day,
      empty: false,
      past: isPast(day),
      today: isToday(day),
      selected: isSelected(day)
    })
  }

  return days
})

const monthLabel = computed(() => {
  return calendarMonth.value.toLocaleDateString('zh-CN', { year: 'numeric', month: 'long' })
})

const hasDue = computed(() => selectedDate.value !== '')

const previewText = computed(() => {
  if (!selectedDate.value) return ''
  const [year, month, day] = selectedDate.value.split('-').map(Number)
  const d = new Date(year, month - 1, day)
  const dateStr = d.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric', weekday: 'short' })
  return `${dateStr} ${normalizeHour(hour.value)}:${normalizeMinute(minute.value)}`
})

// 更新值
function updateValue() {
  if (selectedDate.value) {
    const h = normalizeHour(hour.value)
    const m = normalizeMinute(minute.value)
    const due = new Date(`${selectedDate.value}T${h}:${m}`)
    emit('update:modelValue', due)
  } else {
    emit('update:modelValue', null)
  }
}

function clearDue() {
  selectedDate.value = ''
  hour.value = '12'
  minute.value = '00'
  emit('update:modelValue', null)
}

function handleHourChange(e: Event) {
  const val = (e.target as HTMLInputElement).value.replace(/\D/g, '').slice(0, 2)
  hour.value = val
}

function handleHourBlur() {
  hour.value = normalizeHour(hour.value)
  updateValue()
}

function handleMinuteChange(e: Event) {
  const val = (e.target as HTMLInputElement).value.replace(/\D/g, '').slice(0, 2)
  minute.value = val
}

function handleMinuteBlur() {
  minute.value = normalizeMinute(minute.value)
  updateValue()
}

const weekdays = ['日', '一', '二', '三', '四', '五', '六']
</script>

<template>
  <div class="p-3 bg-white dark:bg-dark-surface rounded-2xl border border-primary/20 shadow-lg space-y-3">
    <!-- 快速选项 -->
    <div class="grid grid-cols-4 gap-2">
      <button
        v-for="opt in quickOptions"
        :key="opt.label"
        type="button"
        class="py-2 rounded-xl text-xs font-semibold transition-colors cursor-pointer"
        :class="selectedDate === formatDateLocal(new Date(Date.now() + opt.days * 86400000))
          ? 'bg-primary text-white'
          : 'bg-primary/10 text-primary hover:bg-primary/20'"
        @click="selectQuickOption(opt.days)"
      >
        {{ opt.label }}
      </button>
    </div>

    <!-- 日历 -->
    <div class="space-y-2">
      <!-- 月份导航 -->
      <div class="flex items-center justify-between">
        <button
          type="button"
          class="w-7 h-7 rounded-lg flex items-center justify-center text-gray-400 hover:bg-primary/10 hover:text-primary transition-colors cursor-pointer"
          @click="prevMonth"
        >
          <ChevronLeft class="w-4 h-4" />
        </button>
        <span class="text-sm font-semibold text-gray-700 dark:text-gray-200">
          {{ monthLabel }}
        </span>
        <button
          type="button"
          class="w-7 h-7 rounded-lg flex items-center justify-center text-gray-400 hover:bg-primary/10 hover:text-primary transition-colors cursor-pointer"
          @click="nextMonth"
        >
          <ChevronRight class="w-4 h-4" />
        </button>
      </div>

      <!-- 星期标题 -->
      <div class="grid grid-cols-7 gap-1">
        <div
          v-for="w in weekdays"
          :key="w"
          class="text-[10px] font-semibold text-gray-400 text-center py-1"
        >
          {{ w }}
        </div>
      </div>

      <!-- 日期网格 -->
      <div class="grid grid-cols-7 gap-1">
        <template v-for="(item, idx) in calendarData" :key="idx">
          <div v-if="item.empty" />
          <button
            v-else
            type="button"
            :disabled="item.past"
            class="w-7 h-7 rounded-lg text-xs font-semibold transition-all cursor-pointer"
            :class="{
              'bg-primary text-white shadow-md': item.selected,
              'bg-primary/20 text-primary': item.today && !item.selected,
              'text-gray-300 cursor-not-allowed': item.past,
              'text-gray-600 dark:text-gray-300 hover:bg-primary/10': !item.selected && !item.today && !item.past
            }"
            @click="selectDay(item.day)"
          >
            {{ item.day }}
          </button>
        </template>
      </div>
    </div>

    <!-- 时间输入 -->
    <div class="flex flex-col gap-3 pt-3 border-t border-primary/10">
      <div class="flex items-center justify-between">
        <span class="text-[10px] font-bold text-gray-400 uppercase tracking-wider">时间</span>
        <div class="flex items-center gap-1.5">
          <input
            type="text"
            inputmode="numeric"
            maxlength="2"
            :value="hour"
            class="w-8 h-8 rounded-lg bg-primary/5 border border-primary/20 text-center text-xs font-bold text-primary outline-none focus:border-primary/50 focus:bg-white transition-colors"
            @input="handleHourChange"
            @blur="handleHourBlur"
            @focus="($event.target as HTMLInputElement).select()"
          />
          <span class="text-primary/50 font-bold">:</span>
          <input
            type="text"
            inputmode="numeric"
            maxlength="2"
            :value="minute"
            class="w-8 h-8 rounded-lg bg-primary/5 border border-primary/20 text-center text-xs font-bold text-primary outline-none focus:border-primary/50 focus:bg-white transition-colors"
            @input="handleMinuteChange"
            @blur="handleMinuteBlur"
            @focus="($event.target as HTMLInputElement).select()"
          />
        </div>
      </div>

      <!-- 快速时间 -->
      <div class="grid grid-cols-3 gap-2">
        <button
          v-for="h in ['09', '12', '18']"
          :key="h"
          type="button"
          class="px-2 py-1.5 rounded-lg text-[10px] font-semibold transition-all cursor-pointer"
          :class="hour === h && minute === '00'
            ? 'bg-primary text-white shadow-md'
            : 'bg-primary/5 text-primary hover:bg-primary/15'"
          @click="setQuickTime(h)"
        >
          {{ h }}:00
        </button>
      </div>
    </div>

    <!-- 预览 -->
    <div
      v-if="hasDue"
      class="flex items-center justify-between py-2 px-3 bg-primary/10 rounded-xl"
    >
      <div class="flex items-center gap-2">
        <div class="w-1.5 h-1.5 rounded-full bg-primary" />
        <span class="text-[11px] font-semibold text-primary">
          {{ previewText }}
        </span>
      </div>
      <button
        type="button"
        class="text-[10px] font-medium text-primary hover:text-red-500 cursor-pointer"
        @click="clearDue"
      >
        清除
      </button>
    </div>
  </div>
</template>
