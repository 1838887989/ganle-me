import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Task, Project, Tag, CurrentView, SmartListType, RepeatType, SubTask } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { sendNotification } from '@tauri-apps/plugin-notification'

// 全局时间（每分钟更新，用于倒计时显示）
const globalNow = ref(Date.now())
let globalTimer: ReturnType<typeof setInterval> | null = null

// 待显示的提醒任务
const pendingReminder = ref<Task | null>(null)

function startGlobalTimer() {
  if (globalTimer) return
  globalTimer = setInterval(() => {
    globalNow.value = Date.now()
  }, 60000)
}

// 启动全局定时器
startGlobalTimer()

export const useAppStore = defineStore('app', () => {
  // 主题状态
  const isDark = ref(false)

  // 极简模式状态
  const isMinimalMode = ref(false)

  // 侧边栏折叠状态
  const sidebarCollapsed = ref(false)

  // 当前视图
  const currentView = ref<CurrentView>({ type: 'smart', id: 'today' })

  // 搜索关键词
  const searchQuery = ref('')

  // 任务列表
  const tasks = ref<Task[]>([])

  // 项目列表
  const projects = ref<Project[]>([])

  // 标签列表
  const tags = ref<Tag[]>([])

  // 从 Rust 后端加载所有数据
  async function initFromBackend() {
    try {
      const [taskList, projectList, tagList, settings] = await Promise.all([
        invoke<Task[]>('get_all_tasks'),
        invoke<Project[]>('get_all_projects'),
        invoke<Tag[]>('get_all_tags'),
        invoke<{ isDark: boolean }>('get_settings'),
      ])
      tasks.value = taskList
      projects.value = projectList
      tags.value = tagList
      isDark.value = settings.isDark
      document.documentElement.classList.toggle('dark', settings.isDark)
    } catch (e) {
      console.error('从后端加载数据失败:', e)
    }
  }

  // 初始化
  initFromBackend()

  // 计算属性 - 根据当前视图过滤任务
  const filteredTasks = computed(() => {
    let result = tasks.value

    // 按搜索关键词过滤
    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase()
      result = result.filter(
        task =>
          task.title.toLowerCase().includes(query) ||
          task.description.toLowerCase().includes(query)
      )
    }

    const view = currentView.value
    if (view.type === 'smart') {
      const today = new Date().toISOString().split('T')[0]
      switch (view.id as SmartListType) {
        case 'today':
          // 支持 YYYY-MM-DD 和 YYYY-MM-DDTHH:mm:ss 两种格式
          result = result.filter(t => t.dueDate && t.dueDate.startsWith(today))
          break
        case 'week': {
          const weekLater = new Date()
          weekLater.setDate(weekLater.getDate() + 7)
          const weekStr = weekLater.toISOString().split('T')[0]
          // 取日期部分进行比较
          result = result.filter(t => {
            if (!t.dueDate) return false
            const taskDate = t.dueDate.split('T')[0]
            return taskDate <= weekStr
          })
          break
        }
        case 'all':
          break
        case 'completed':
          result = result.filter(t => t.status === 'completed')
          break
        case 'favorite':
          result = result.filter(t => t.favorite)
          break
      }
    } else if (view.type === 'project') {
      result = result.filter(t => t.projectId === view.id)
    } else if (view.type === 'tag') {
      result = result.filter(t => t.tags.includes(view.id))
    }

    return result
  })

  // 当前视图标题
  const currentViewTitle = computed(() => {
    const view = currentView.value
    if (view.type === 'smart') {
      const titles: Record<string, string> = {
        today: '今日待办',
        week: '近 7 天',
        all: '全部任务',
        completed: '已完成',
        favorite: '收藏任务',
      }
      return titles[view.id] || '任务列表'
    } else if (view.type === 'project') {
      const project = projects.value.find(p => p.id === view.id)
      return project?.name || '项目'
    } else if (view.type === 'tag') {
      const tag = tags.value.find(t => t.id === view.id)
      return tag?.name || '标签'
    }
    return '任务列表'
  })

  // 统计数据
  const todayCount = computed(() => {
    const today = new Date().toISOString().split('T')[0]
    // 支持 YYYY-MM-DD 和 YYYY-MM-DDTHH:mm:ss 两种格式
    return tasks.value.filter(t => t.dueDate && t.dueDate.startsWith(today) && t.status !== 'completed').length
  })

  const weekCount = computed(() => {
    const weekLater = new Date()
    weekLater.setDate(weekLater.getDate() + 7)
    const weekStr = weekLater.toISOString().split('T')[0]
    // 取日期部分进行比较
    return tasks.value.filter(t => {
      if (!t.dueDate || t.status === 'completed') return false
      const taskDate = t.dueDate.split('T')[0]
      return taskDate <= weekStr
    }).length
  })

  const allCount = computed(() => tasks.value.filter(t => t.status !== 'completed').length)
  const completedCount = computed(() => tasks.value.filter(t => t.status === 'completed').length)
  const favoriteCount = computed(() => tasks.value.filter(t => t.favorite && t.status !== 'completed').length)

  // ========== 基础操作 ==========
  async function toggleTheme() {
    isDark.value = !isDark.value
    document.documentElement.classList.toggle('dark', isDark.value)
    await invoke('update_settings', { key: 'isDark', value: String(isDark.value) })
  }

  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  async function toggleMinimalMode() {
    isMinimalMode.value = !isMinimalMode.value
    await invoke('set_window_mode', { mode: isMinimalMode.value ? 'minimal' : 'standard' })
  }

  function setView(view: CurrentView) {
    currentView.value = view
  }

  // ========== 任务操作 ==========
  async function addTask(title: string, dueDate: string | null = null, priority: 'high' | 'medium' | 'low' = 'medium', tagId: string | null = null, repeat: RepeatType = 'none') {
    // 确定项目ID
    const projectId = currentView.value.type === 'project' ? currentView.value.id : null
    // 确定标签ID
    const taskTagId = tagId || (currentView.value.type === 'tag' ? currentView.value.id : null)

    const newTask = await invoke<Task>('create_task', {
      title,
      dueDate,
      priority,
      projectId,
      tagId: taskTagId,
      repeat,
    })
    tasks.value.unshift(newTask)
  }

  async function updateTask(taskId: string, updates: Partial<Task>) {
    const updatedTask = await invoke<Task>('update_task', { id: taskId, updates })
    const index = tasks.value.findIndex(t => t.id === taskId)
    if (index !== -1) {
      tasks.value[index] = updatedTask
    }
  }

  async function toggleTask(taskId: string) {
    const updatedTask = await invoke<Task>('toggle_task_status', { id: taskId })
    const index = tasks.value.findIndex(t => t.id === taskId)
    if (index !== -1) {
      tasks.value[index] = updatedTask
    }
    // 重新加载任务列表（可能生成了重复任务）
    tasks.value = await invoke<Task[]>('get_all_tasks')
  }

  async function toggleFavorite(taskId: string) {
    const updatedTask = await invoke<Task>('toggle_favorite', { id: taskId })
    const index = tasks.value.findIndex(t => t.id === taskId)
    if (index !== -1) {
      tasks.value[index] = updatedTask
    }
  }

  async function deleteTask(taskId: string) {
    await invoke('delete_task', { id: taskId })
    tasks.value = tasks.value.filter(t => t.id !== taskId)
  }

  // ========== 子任务操作 ==========
  async function addSubtask(taskId: string, title: string) {
    await invoke('add_subtask', { taskId, title })
    // 重新加载该任务
    const allTasks = await invoke<Task[]>('get_all_tasks')
    tasks.value = allTasks
  }

  async function toggleSubtask(taskId: string, subtaskId: string) {
    await invoke('toggle_subtask', { taskId, subtaskId })
    const allTasks = await invoke<Task[]>('get_all_tasks')
    tasks.value = allTasks
  }

  async function deleteSubtask(taskId: string, subtaskId: string) {
    await invoke('delete_subtask', { taskId, subtaskId })
    const allTasks = await invoke<Task[]>('get_all_tasks')
    tasks.value = allTasks
  }

  async function updateSubtask(taskId: string, subtaskId: string, title: string) {
    await invoke('update_subtask', { taskId, subtaskId, title })
    const allTasks = await invoke<Task[]>('get_all_tasks')
    tasks.value = allTasks
  }

  // ========== 重复任务 ==========
  async function setTaskRepeat(taskId: string, repeat: RepeatType) {
    await invoke('update_task', { id: taskId, updates: { repeat } })
    const allTasks = await invoke<Task[]>('get_all_tasks')
    tasks.value = allTasks
  }

  // ========== 标签操作 ==========
  async function addTag(name: string, color: string) {
    const newTag = await invoke<Tag>('create_tag', { name, color })
    tags.value.push(newTag)
  }

  async function updateTag(tagId: string, updates: Partial<Tag>) {
    const updatedTag = await invoke<Tag>('update_tag', { id: tagId, updates })
    const index = tags.value.findIndex(t => t.id === tagId)
    if (index !== -1) {
      tags.value[index] = updatedTag
    }
  }

  async function deleteTag(tagId: string) {
    await invoke('delete_tag', { id: tagId })
    tags.value = tags.value.filter(t => t.id !== tagId)
    // 重新加载任务（后端已处理标签关联删除）
    tasks.value = await invoke<Task[]>('get_all_tasks')
  }

  // ========== 排序操作 ==========
  async function reorderTasks(orderedIds: string[]) {
    const taskMap = new Map(tasks.value.map(t => [t.id, t]))
    const reordered = orderedIds.map(id => taskMap.get(id)).filter(Boolean) as Task[]
    const remaining = tasks.value.filter(t => !orderedIds.includes(t.id))
    tasks.value = [...reordered, ...remaining]
  }

  async function reorderByIndex(fromIndex: number, toIndex: number) {
    const filtered = filteredTasks.value
    if (fromIndex < 0 || toIndex < 0 || fromIndex >= filtered.length || toIndex >= filtered.length) return

    const fromTask = filtered[fromIndex]
    const toTask = filtered[toIndex]
    const fromOriginalIndex = tasks.value.findIndex(t => t.id === fromTask.id)
    const toOriginalIndex = tasks.value.findIndex(t => t.id === toTask.id)

    if (fromOriginalIndex === -1 || toOriginalIndex === -1) return

    const [removed] = tasks.value.splice(fromOriginalIndex, 1)
    tasks.value.splice(toOriginalIndex, 0, removed)

    // 同步排序到后端
    await invoke('reorder_tasks', {
      fromIndex,
      toIndex,
      viewType: currentView.value.type,
      viewId: currentView.value.id,
    })
  }

  // ========== 统计方法 ==========
  function getProjectTaskCount(projectId: string) {
    return tasks.value.filter(t => t.projectId === projectId && t.status !== 'completed').length
  }

  function getTagTaskCount(tagId: string) {
    return tasks.value.filter(t => t.tags.includes(tagId) && t.status !== 'completed').length
  }

  // ========== 提醒功能 ==========
  // 检查并触发提醒（由 Rust 后端判断）
  async function checkTaskReminders() {
    try {
      const task = await invoke<Task | null>('check_reminders')
      if (task) {
        triggerReminder(task)
      }
    } catch (e) {
      console.error('检查提醒失败:', e)
    }
  }

  // 触发提醒（系统通知 + 应用内弹窗）
  function triggerReminder(task: Task) {
    // 设置应用内弹窗
    pendingReminder.value = task

    // 发送系统通知
    try {
      sendNotification({
        title: '任务即将到期',
        body: task.title,
      })
    } catch (e) {
      console.error('发送通知失败:', e)
    }
  }

  // 关闭提醒弹窗
  function dismissReminder() {
    pendingReminder.value = null
  }

  // 启动提醒检查定时器（每30秒检查一次）
  let reminderTimer: ReturnType<typeof setInterval> | null = null
  function startReminderTimer() {
    if (reminderTimer) return
    reminderTimer = setInterval(checkTaskReminders, 30000)
    // 立即检查一次
    checkTaskReminders()
  }
  startReminderTimer()

  return {
    // 状态
    isDark,
    isMinimalMode,
    sidebarCollapsed,
    currentView,
    searchQuery,
    tasks,
    projects,
    tags,
    globalNow,
    pendingReminder,
    // 计算属性
    filteredTasks,
    currentViewTitle,
    todayCount,
    weekCount,
    allCount,
    completedCount,
    favoriteCount,
    // 基础方法
    toggleTheme,
    toggleSidebar,
    toggleMinimalMode,
    setView,
    initFromBackend,
    // 任务方法
    addTask,
    updateTask,
    toggleTask,
    toggleFavorite,
    deleteTask,
    // 子任务方法
    addSubtask,
    toggleSubtask,
    deleteSubtask,
    updateSubtask,
    // 重复任务
    setTaskRepeat,
    // 标签方法
    addTag,
    updateTag,
    deleteTag,
    // 排序方法
    reorderTasks,
    reorderByIndex,
    // 统计方法
    getProjectTaskCount,
    getTagTaskCount,
    dismissReminder,
  }
})
