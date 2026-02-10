<script setup lang="ts">
import { ref } from 'vue'
import { useAppStore } from '@/stores/app'
import {
  Sun,
  Moon,
  CalendarDays,
  ListTodo,
  CheckCircle,
  Star,
  Plus,
  ChevronLeft,
  ChevronRight,
  Edit3,
  Trash2,
  X,
  Check,
  Settings,
  Sparkles,
} from 'lucide-vue-next'

const store = useAppStore()

// 设置面板状态
const showSettings = ref(false)

// 标签编辑状态
const editingTagId = ref<string | null>(null)
const editingTagName = ref('')
const editingTagColor = ref('')

// 新建标签状态
const showNewTagModal = ref(false)
const newTagName = ref('')
const newTagColor = ref('#3B82F6')

// 预设颜色
const presetColors = [
  '#EF4444', '#F97316', '#F59E0B', '#84CC16',
  '#22C55E', '#14B8A6', '#3B82F6', '#8B5CF6',
  '#EC4899', '#6B7280',
]

// 开始编辑标签
function startEditTag(tagId: string, name: string, color: string) {
  editingTagId.value = tagId
  editingTagName.value = name
  editingTagColor.value = color
}

// 保存标签编辑
function saveTagEdit() {
  if (editingTagId.value && editingTagName.value.trim()) {
    store.updateTag(editingTagId.value, {
      name: editingTagName.value.trim(),
      color: editingTagColor.value,
    })
  }
  cancelTagEdit()
}

// 取消编辑
function cancelTagEdit() {
  editingTagId.value = null
  editingTagName.value = ''
  editingTagColor.value = ''
}

// 删除标签
function handleDeleteTag(tagId: string) {
  if (confirm('确定要删除这个标签吗？')) {
    store.deleteTag(tagId)
  }
}

// 创建新标签
function createNewTag() {
  if (newTagName.value.trim()) {
    store.addTag(newTagName.value.trim(), newTagColor.value)
    closeNewTagModal()
  }
}

// 关闭新建标签弹窗
function closeNewTagModal() {
  showNewTagModal.value = false
  newTagName.value = ''
  newTagColor.value = '#3B82F6'
}
</script>

<template>
  <aside
    class="h-full flex flex-col bg-cream-100/80 dark:bg-dark-surface/80 backdrop-blur-sm border-r border-cream-300 dark:border-dark-border transition-all duration-300"
    :class="store.sidebarCollapsed ? 'w-16' : 'w-64'"
  >
    <!-- 顶部 Logo 区域 -->
    <div class="flex items-center justify-between p-4 border-b border-cream-300/50 dark:border-dark-border/50">
      <div v-if="!store.sidebarCollapsed" class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-organic bg-primary flex items-center justify-center">
          <ListTodo class="w-5 h-5 text-white" />
        </div>
        <span class="font-heading font-bold text-lg text-gray-800 dark:text-dark-text">
          TodoList Pro
        </span>
      </div>
      <div v-else class="w-8 h-8 rounded-organic bg-primary flex items-center justify-center mx-auto">
        <ListTodo class="w-5 h-5 text-white" />
      </div>
      <button
        v-if="!store.sidebarCollapsed"
        class="p-1.5 rounded-lg hover:bg-cream-300 dark:hover:bg-dark-border cursor-pointer transition-colors"
        @click="store.toggleSidebar"
      >
        <ChevronLeft class="w-4 h-4 text-gray-500" />
      </button>
    </div>

    <!-- 展开按钮（折叠时显示） -->
    <button
      v-if="store.sidebarCollapsed"
      class="mx-auto mt-4 p-1.5 rounded-lg hover:bg-cream-300 dark:hover:bg-dark-border cursor-pointer transition-colors"
      @click="store.toggleSidebar"
    >
      <ChevronRight class="w-4 h-4 text-gray-500" />
    </button>

    <!-- 智能清单 -->
    <div class="flex-1 overflow-y-auto py-4 px-3">
      <div v-if="!store.sidebarCollapsed" class="mb-6">
        <h3 class="px-3 mb-2 text-xs font-semibold text-secondary/70 dark:text-dark-muted uppercase tracking-wider">
          智能清单
        </h3>
        <nav class="space-y-1">
          <button
            class="sidebar-item w-full"
            :class="{ active: store.currentView.type === 'smart' && store.currentView.id === 'today' }"
            @click="store.setView({ type: 'smart', id: 'today' })"
          >
            <Sun class="w-5 h-5 text-accent" />
            <span class="flex-1 text-left">今日待办</span>
            <span class="text-sm text-gray-400">{{ store.todayCount }}</span>
          </button>
          <button
            class="sidebar-item w-full"
            :class="{ active: store.currentView.type === 'smart' && store.currentView.id === 'week' }"
            @click="store.setView({ type: 'smart', id: 'week' })"
          >
            <CalendarDays class="w-5 h-5 text-primary" />
            <span class="flex-1 text-left">近 7 天</span>
            <span class="text-sm text-gray-400">{{ store.weekCount }}</span>
          </button>
          <button
            class="sidebar-item w-full"
            :class="{ active: store.currentView.type === 'smart' && store.currentView.id === 'all' }"
            @click="store.setView({ type: 'smart', id: 'all' })"
          >
            <ListTodo class="w-5 h-5 text-secondary" />
            <span class="flex-1 text-left">全部任务</span>
            <span class="text-sm text-gray-400">{{ store.allCount }}</span>
          </button>
          <button
            class="sidebar-item w-full"
            :class="{ active: store.currentView.type === 'smart' && store.currentView.id === 'favorite' }"
            @click="store.setView({ type: 'smart', id: 'favorite' })"
          >
            <Star class="w-5 h-5 text-accent" />
            <span class="flex-1 text-left">收藏任务</span>
            <span class="text-sm text-gray-400">{{ store.favoriteCount }}</span>
          </button>
          <button
            class="sidebar-item w-full"
            :class="{ active: store.currentView.type === 'smart' && store.currentView.id === 'completed' }"
            @click="store.setView({ type: 'smart', id: 'completed' })"
          >
            <CheckCircle class="w-5 h-5 text-primary-600" />
            <span class="flex-1 text-left">已完成</span>
            <span class="text-sm text-gray-400">{{ store.completedCount }}</span>
          </button>
        </nav>
      </div>

      <!-- 折叠模式下的图标 -->
      <div v-else class="space-y-2">
        <button
          v-for="item in [
            { id: 'today', icon: Sun, color: 'text-accent' },
            { id: 'week', icon: CalendarDays, color: 'text-primary' },
            { id: 'all', icon: ListTodo, color: 'text-secondary' },
            { id: 'favorite', icon: Star, color: 'text-accent' },
            { id: 'completed', icon: CheckCircle, color: 'text-primary-600' },
          ]"
          :key="item.id"
          class="w-10 h-10 mx-auto flex items-center justify-center rounded-organic cursor-pointer transition-colors"
          :class="[
            store.currentView.type === 'smart' && store.currentView.id === item.id
              ? 'bg-primary/10 dark:bg-primary/20'
              : 'hover:bg-cream-300 dark:hover:bg-dark-border'
          ]"
          @click="store.setView({ type: 'smart', id: item.id })"
        >
          <component :is="item.icon" class="w-5 h-5" :class="item.color" />
        </button>
      </div>

      <!-- 标签列表 -->
      <div v-if="!store.sidebarCollapsed">
        <div class="flex items-center justify-between px-3 mb-2">
          <h3 class="text-xs font-semibold text-secondary/70 dark:text-dark-muted uppercase tracking-wider">
            标签
          </h3>
          <button
            class="p-1 rounded hover:bg-cream-300 dark:hover:bg-dark-border cursor-pointer transition-colors"
            @click="showNewTagModal = true"
          >
            <Plus class="w-4 h-4 text-gray-400" />
          </button>
        </div>
        <nav class="space-y-1">
          <!-- 编辑模式 -->
          <div
            v-for="tag in store.tags"
            :key="tag.id"
            class="group"
          >
            <!-- 编辑状态 -->
            <div v-if="editingTagId === tag.id" class="p-2 rounded-organic bg-cream-200 dark:bg-dark-border">
              <input
                v-model="editingTagName"
                type="text"
                class="w-full px-2 py-1 text-sm rounded bg-white dark:bg-dark-surface border border-cream-300 dark:border-dark-border outline-none focus:border-primary"
                @keydown.enter="saveTagEdit"
                @keydown.escape="cancelTagEdit"
              />
              <div class="flex flex-wrap gap-1 mt-2">
                <button
                  v-for="color in presetColors"
                  :key="color"
                  class="w-5 h-5 rounded-full transition-transform hover:scale-110"
                  :class="{ 'ring-2 ring-primary ring-offset-1': editingTagColor === color }"
                  :style="{ backgroundColor: color }"
                  @click="editingTagColor = color"
                />
              </div>
              <div class="flex justify-end gap-1 mt-2">
                <button
                  class="p-1 rounded hover:bg-cream-300 dark:hover:bg-dark-surface"
                  @click="cancelTagEdit"
                >
                  <X class="w-4 h-4 text-gray-400" />
                </button>
                <button
                  class="p-1 rounded hover:bg-primary/20"
                  @click="saveTagEdit"
                >
                  <Check class="w-4 h-4 text-primary" />
                </button>
              </div>
            </div>

            <!-- 正常显示 -->
            <button
              v-else
              class="sidebar-item w-full"
              :class="{ active: store.currentView.type === 'tag' && store.currentView.id === tag.id }"
              @click="store.setView({ type: 'tag', id: tag.id })"
            >
              <div
                class="w-3 h-3 rounded-full flex-shrink-0"
                :style="{ backgroundColor: tag.color }"
              />
              <span class="flex-1 text-left truncate">{{ tag.name }}</span>
              <span class="text-sm text-gray-400 group-hover:hidden">{{ store.getTagTaskCount(tag.id) }}</span>
              <!-- 编辑/删除按钮 -->
              <div class="hidden group-hover:flex items-center gap-0.5">
                <button
                  class="p-1 rounded hover:bg-cream-300 dark:hover:bg-dark-surface"
                  @click.stop="startEditTag(tag.id, tag.name, tag.color)"
                >
                  <Edit3 class="w-3.5 h-3.5 text-gray-400" />
                </button>
                <button
                  class="p-1 rounded hover:bg-red-100 dark:hover:bg-red-900/20"
                  @click.stop="handleDeleteTag(tag.id)"
                >
                  <Trash2 class="w-3.5 h-3.5 text-gray-400 hover:text-red-500" />
                </button>
              </div>
            </button>
          </div>
        </nav>
      </div>
    </div>

    <!-- 底部设置区域 -->
    <div class="p-4 border-t border-cream-300/50 dark:border-dark-border/50">
      <div class="relative">
        <button
          class="sidebar-item w-full justify-center"
          :class="{ 'bg-cream-300 dark:bg-dark-border': showSettings }"
          @click="showSettings = !showSettings"
        >
          <Settings class="w-5 h-5 text-gray-600 dark:text-dark-text" />
          <span v-if="!store.sidebarCollapsed">设置</span>
        </button>

        <!-- 设置下拉菜单 -->
        <div
          v-if="showSettings"
          class="absolute left-0 bottom-full mb-2 w-56 p-3 bg-white dark:bg-dark-surface rounded-organic shadow-organic-lg border border-cream-300 dark:border-dark-border z-50"
        >
          <!-- 极简模式切换 -->
          <div class="flex items-center justify-between py-2 px-2 hover:bg-cream-200 dark:hover:bg-dark-border rounded-lg cursor-pointer transition-colors" @click="store.toggleMinimalMode">
            <div class="flex items-center gap-3">
              <Sparkles class="w-5 h-5 text-primary" />
              <span class="text-sm text-gray-700 dark:text-dark-text">极简模式</span>
            </div>
            <div
              class="relative w-11 h-6 rounded-full transition-colors"
              :class="store.isMinimalMode ? 'bg-primary' : 'bg-gray-300 dark:bg-dark-border'"
            >
              <div
                class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-transform"
                :class="{ 'translate-x-5': store.isMinimalMode }"
              ></div>
            </div>
          </div>

          <!-- 主题切换 -->
          <div class="flex items-center justify-between py-2 px-2 hover:bg-cream-200 dark:hover:bg-dark-border rounded-lg cursor-pointer transition-colors mt-1" @click="store.toggleTheme">
            <div class="flex items-center gap-3">
              <Sun v-if="store.isDark" class="w-5 h-5 text-accent" />
              <Moon v-else class="w-5 h-5 text-secondary" />
              <span class="text-sm text-gray-700 dark:text-dark-text">
                {{ store.isDark ? '浅色模式' : '深色模式' }}
              </span>
            </div>
            <div
              class="relative w-11 h-6 rounded-full transition-colors"
              :class="store.isDark ? 'bg-secondary' : 'bg-gray-300'"
            >
              <div
                class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-transform"
                :class="{ 'translate-x-5': store.isDark }"
              ></div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 点击外部关闭设置 -->
    <div
      v-if="showSettings"
      class="fixed inset-0 z-40"
      @click="showSettings = false"
    ></div>
  </aside>

  <!-- 新建标签弹窗 -->
  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-150"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <div
        v-if="showNewTagModal"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/30"
        @click.self="closeNewTagModal"
      >
        <div class="w-72 p-4 bg-white dark:bg-dark-surface rounded-2xl shadow-xl">
          <h3 class="text-lg font-semibold text-gray-800 dark:text-dark-text mb-4">新建标签</h3>

          <input
            v-model="newTagName"
            type="text"
            placeholder="标签名称"
            class="w-full px-3 py-2 text-sm rounded-lg bg-cream-100 dark:bg-dark-border border-0 outline-none focus:ring-2 focus:ring-primary/30"
            @keydown.enter="createNewTag"
          />

          <div class="mt-3">
            <p class="text-xs text-gray-500 mb-2">选择颜色</p>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="color in presetColors"
                :key="color"
                class="w-6 h-6 rounded-full transition-transform hover:scale-110"
                :class="{ 'ring-2 ring-primary ring-offset-2': newTagColor === color }"
                :style="{ backgroundColor: color }"
                @click="newTagColor = color"
              />
            </div>
          </div>

          <div class="flex justify-end gap-2 mt-4">
            <button
              class="px-3 py-1.5 text-sm text-gray-500 hover:bg-cream-200 dark:hover:bg-dark-border rounded-lg"
              @click="closeNewTagModal"
            >
              取消
            </button>
            <button
              class="px-3 py-1.5 text-sm text-white bg-primary hover:bg-primary-600 rounded-lg"
              @click="createNewTag"
            >
              创建
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
