<script setup lang="ts">
import intro from 'intro.js'
import 'intro.js/introjs.css'
import 'intro.js/themes/introjs-modern.css'
import {TourOptions} from "intro.js/src/packages/tour/option";
import {onMounted} from "vue";

const steps = [
  {title: '新建知识库', element: '#step1', intro: `点击“+”按钮即可创建一个知识库`},
  {
    title: '模型配置',
    element: '#step2',
    intro: `这里可以配置个各种你喜欢的模型，支持添加离线模型哦，再也不用担心Token不够用啦 🤖`,
  },
  {title: 'MCP工具', element: '#step3', intro: `这里配置你需要使用的MCP工具，为你的知识库提供更强大的功能 💪`,},
  {title: '导入文件', element: '#step4', intro: `点击此处或从任何地方将文件拖入到这里即可导入到知识库啦 😎`,},
  {title: '完成', element: 'ok', intro: `开始创建你的第一个知识库吧 😊`,},
]

const options: Partial<TourOptions> = { // 参数对象
  prevLabel: '上一步',
  nextLabel: '下一步',
  skipLabel: '跳过',
  doneLabel: '完成',
  tooltipClass: 'intro-tooltip',
  highlightClass: 'intro-highlight',
  overlayOpacity: 0, // 遮罩层的透明度 0-1之间
  steps: steps,
}

const tip = intro()
    .setOptions(options)
    // 点击结束按钮后执行的事件
    .onComplete(() => {
      localStorage.setItem('intro-done', 'true')
    })
    // 点击跳过按钮后执行的事件
    .onExit(() => {
      localStorage.setItem('intro-done', 'true')
    })
    // 确认完毕之后执行的事件
    .onBeforeExit(() => {
      return true
    })

onMounted(() => {
  if (localStorage.getItem('intro-done') === 'true') {
    return
  }
  tip.start()
})
</script>

<template>

</template>

<style scoped lang="scss">


</style>
<style lang="scss">
.introjs-overlay {
  margin-top: 40px;
}

.introjs-tooltip {
  font-size: 14px !important;

}

.introjs-tooltip-title {
  font-size: 16px !important;
}

.introjs-tooltiptext {
  padding: 10px 20px;
}

.introjs-tooltipbuttons {
  border-top: unset !important;
}

.introjs-prevbutton {
  font-size: 14px !important;
  padding: 2px 6px;
  background: var(--el-color-primary) !important;
  color: white;

  &:hover {
    background: var(--el-color-primary) !important;
    border: var(--el-color-primary) solid 1px !important;
    box-shadow: unset !important;
  }
}

.introjs-nextbutton {
  font-size: 14px !important;
  padding: 2px 6px;
  background: var(--el-color-primary) !important;
  color: white;

  &:hover {
    background: var(--el-color-primary) !important;
    border: var(--el-color-primary) solid 1px !important;
    box-shadow: unset !important;
  }
}

.introjs-skipbutton {
  font-size: 14px !important;
  color: #ffffff;
}

.intro-highlight {
  // 创建一个更加现代化的高亮效果
  border: unset !important;
  box-shadow: 0 0 0 2px var(--el-color-primary),
  0 0 20px 5px rgba(64, 158, 255, 0.3),
  0 5px 15px rgba(0, 0, 0, 0.2) !important;
  border-radius: 6px !important;
}

.introjs-skipbutton {
  font-weight: 400;

  &:hover {
    color: unset;
  }
}

.introjs-arrow {
  display: none !important;
}

</style>