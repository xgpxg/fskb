<script setup lang="ts">
import {ref, onMounted} from 'vue'
import {useI18n} from 'vue-i18n'
import {ElMessage} from 'element-plus'

const {t} = useI18n()

// 主题选项
const themes = [
  {
    name: 'default',
    color: '#10B981'
  },
  {
    name: 'dark',
    color: '#3e3e3e'
  },
  {
    name: 'blue',
    color: '#409eff'
  },
  {
    name: 'red',
    color: '#f56c6c'
  },
  {
    name: 'purple',
    color: '#a66ec6'
  },
  {
    name: 'orange',
    color: '#ebb563'
  },
]

// 当前主题
const currentTheme = ref(localStorage.getItem('theme') || 'default')

// 切换主题
const switchTheme = (theme: string) => {
  currentTheme.value = theme

  applyTheme(theme)

  // 保存到本地存储
  localStorage.setItem('theme', theme)
}

// 应用主题
const applyTheme = (theme: string) => {
  const html = document.documentElement
  themes.forEach(t => {
    html.classList.remove(t.name)
  })
  html.classList.add(theme)
  document.body.removeAttribute('class')
}

// 初始化主题
onMounted(() => {

})
</script>

<template>
  <div class="theme-switch">
    <div class="flex" v-for="theme in themes">
      <div class="theme"
           :style="{background: theme.color}"
           @click="switchTheme(theme.name)">
        <div v-if="currentTheme===theme.name" class="flex-column-v mt10">
          <el-icon color="white"><Select></Select></el-icon>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.theme-switch {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.theme {
  width: 32px;
  height: 32px;
  border-radius: 5px;
  margin-bottom: 10px;
}

.selected {
  border: 1px solid #fff;
}
</style>
