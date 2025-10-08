<script setup lang="ts">
import SvgIcon from "@components/SvgIcon/index.vue";
import {openPath, revealItemInDir} from "@tauri-apps/plugin-opener";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const props = defineProps({
  kw: {
    type: String,
    default: ''
  },
  searchResult: {
    type: Object,
    default: () => ({items: []})
  },
})

const stopWords = new Set(['的', '了', '在', '是',])
const highlightKeyword = (text: string, keyword: string) => {
  if (!keyword) return text

  // 使用 Intl.Segmenter 进行分词 (现代浏览器支持)
  const segmenter = new Intl.Segmenter('zh', {granularity: 'word'})
  const segments = Array.from(segmenter.segment(keyword))

  // 提取分词结果并过滤掉标点符号、空格和停用词
  const keywords = segments
      .map(segment => segment.segment.trim())
      .filter(segment => segment && !/^[\s\p{P}\p{S}]+$/u.test(segment) && !stopWords.has(segment))

  if (keywords.length === 0) return text

  // 对每个分词结果进行高亮处理
  let highlightedText = text
  keywords.forEach(kw => {
    // 转义特殊字符
    const escapedKeyword = kw.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
    const regex = new RegExp(`(${escapedKeyword})`, 'gi')
    highlightedText = highlightedText.replace(regex, '<span class="highlight">$1</span>')
  })

  return highlightedText
}
const getIcon = (filename: string) => {
  let ext = filename.substring(filename.lastIndexOf('.') + 1);
  if (['png', 'jpg', 'jpeg', 'gif'].indexOf(ext) > -1) {
    return 'image'
  }
  if (['pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'txt', 'md', 'zip'].indexOf(ext) > -1) {
    return ext
  }
  return 'file'
}

const canAddToKb = (filename: string) => {
  let ext = filename.substring(filename.lastIndexOf('.') + 1);
  return ['png', 'jpg', 'jpeg', 'gif', 'pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'txt', 'md'].indexOf(ext) > -1;
}
</script>

<template>
  <div class="fill-width">
    <el-row :gutter="10">
      <el-col :span="12" :xs="12" :sm="12" :md="8" :lg="6" :xl="4" v-for="item in searchResult.items"
              class="item mb10 ">
        <div class="bg-card br5">
          <div>
            <div class="flex-v">
              <div>
                <svg-icon :icon-class="getIcon(item.filename)" class="mr5"></svg-icon>
              </div>
              <el-text truncated>
                {{ item.filename }}
              </el-text>
            </div>
            <div>
              <el-text type="info" size="small" truncated>
                {{ item.filepath }}
              </el-text>
            </div>
          </div>
          <div>
            <el-button text type="text" size="small" @click="openPath(item.filepath)">
              <el-icon class="mr5">
                <svg-icon icon-class="open"></svg-icon>
              </el-icon>
              {{ t('打开')}}
            </el-button>
            <el-button text type="text" icon="folder" size="small" @click="revealItemInDir(item.filepath)">
              {{t('位置')}}
            </el-button>
            <el-button v-if="canAddToKb(item.filename)" text type="text" icon="plus" size="small">
              {{ t('添加')}}
            </el-button>
          </div>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<style scoped lang="scss">
.item {
  line-height: 24px;
}

.el-divider {
  margin: 10px 0;
}


</style>
<style lang="scss">
.highlight {
  background-color: #ffff00;
  border-radius: 2px;
}
</style>