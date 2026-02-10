// 任务优先级
export type Priority = 'high' | 'medium' | 'low'

// 任务状态
export type TaskStatus = 'todo' | 'in_progress' | 'completed'

// 重复类型
export type RepeatType = 'daily' | 'weekly' | 'monthly' | 'custom' | 'none'

// 标签
export interface Tag {
  id: string
  name: string
  color: string
}

// 子任务
export interface SubTask {
  id: string
  title: string
  completed: boolean
}

// 任务
export interface Task {
  id: string
  title: string
  description: string
  status: TaskStatus
  priority: Priority
  projectId: string | null
  tags: string[]       // 标签ID列表
  subtasks: SubTask[]
  dueDate: string | null
  reminder: string | null
  repeat: RepeatType
  favorite: boolean
  createdAt: string
  updatedAt: string
  completedAt: string | null
}

// 项目
export interface Project {
  id: string
  name: string
  color: string
  icon: string
  archived: boolean
  createdAt: string
}

// 智能清单类型
export type SmartListType = 'today' | 'week' | 'all' | 'completed' | 'favorite'

// 当前视图
export interface CurrentView {
  type: 'smart' | 'project' | 'tag'
  id: string
}

// 视图模式
export type ViewMode = 'list' | 'board' | 'calendar' | 'compact'
