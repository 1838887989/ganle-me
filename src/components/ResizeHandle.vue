<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow, LogicalSize, LogicalPosition } from '@tauri-apps/api/window'

const props = defineProps<{
  opacity?: number
}>()

const isResizing = ref(false)
const resizeDir = ref('')
const startPos = ref({ x: 0, y: 0 })
const startBounds = ref({ x: 0, y: 0, w: 0, h: 0 })

const MIN_WIDTH = 320
const MIN_HEIGHT = 480

function handleResizeStart(e: MouseEvent, dir: string) {
  e.preventDefault()
  isResizing.value = true
  resizeDir.value = dir
  startPos.value = { x: e.screenX, y: e.screenY }
  startBounds.value = {
    x: window.screenX,
    y: window.screenY,
    w: window.outerWidth,
    h: window.outerHeight
  }

  document.body.style.cursor = dir === 'se' ? 'nwse-resize' : 'nesw-resize'
  document.body.style.userSelect = 'none'
}

function handleResizeMove(e: MouseEvent) {
  if (!isResizing.value) return

  requestAnimationFrame(() => {
    const deltaX = e.screenX - startPos.value.x
    const deltaY = e.screenY - startPos.value.y
    const dir = resizeDir.value

    let newW = startBounds.value.w
    let newH = startBounds.value.h
    let newX = startBounds.value.x
    const newY = startBounds.value.y

    if (dir === 'se') {
      // 右下角
      newW = Math.max(MIN_WIDTH, startBounds.value.w + deltaX)
      newH = Math.max(MIN_HEIGHT, startBounds.value.h + deltaY)
    } else if (dir === 'sw') {
      // 左下角
      const rawW = startBounds.value.w - deltaX
      if (rawW >= MIN_WIDTH) {
        newW = rawW
        newX = startBounds.value.x + deltaX
      } else {
        newW = MIN_WIDTH
        newX = startBounds.value.x + (startBounds.value.w - MIN_WIDTH)
      }
      newH = Math.max(MIN_HEIGHT, startBounds.value.h + deltaY)
    }

    const win = getCurrentWindow()
    win.setSize(new LogicalSize(newW, newH))
    win.setPosition(new LogicalPosition(newX, newY))
  })
}

function handleResizeEnd() {
  isResizing.value = false
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
}

onMounted(() => {
  document.addEventListener('mousemove', handleResizeMove)
  document.addEventListener('mouseup', handleResizeEnd)
})

onUnmounted(() => {
  document.removeEventListener('mousemove', handleResizeMove)
  document.removeEventListener('mouseup', handleResizeEnd)
})
</script>

<template>
  <!-- 右下角 resize 手柄 -->
  <div
    class="absolute bottom-0 right-0 w-8 h-8 z-50 group cursor-nwse-resize"
    style="-webkit-app-region: no-drag"
    @mousedown="handleResizeStart($event, 'se')"
  >
    <div
      class="absolute bottom-0 right-0 w-full h-full rounded-tl-3xl transition-all duration-300 ease-out opacity-0 group-hover:opacity-100"
      :style="{
        background: `radial-gradient(circle at bottom right, rgba(34, 139, 34, ${Math.max(0.4, (props.opacity ?? 1) * 0.8)}) 0%, transparent 70%)`
      }"
    />
  </div>

  <!-- 左下角 resize 手柄 -->
  <div
    class="absolute bottom-0 left-0 w-8 h-8 z-50 group cursor-nesw-resize"
    style="-webkit-app-region: no-drag"
    @mousedown="handleResizeStart($event, 'sw')"
  >
    <div
      class="absolute bottom-0 left-0 w-full h-full rounded-tr-3xl transition-all duration-300 ease-out opacity-0 group-hover:opacity-100"
      :style="{
        background: `radial-gradient(circle at bottom left, rgba(34, 139, 34, ${Math.max(0.4, (props.opacity ?? 1) * 0.8)}) 0%, transparent 70%)`
      }"
    />
  </div>
</template>
