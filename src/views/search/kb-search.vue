<script setup lang="ts">
import SvgIcon from "@components/SvgIcon/index.vue";
import {convertImageSrc} from "@/utils/commands.ts";
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
const highlightKeyword = (text, keyword) => {
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
}</script>

<template>
  <div v-if="!kw && searchResult.items.length===0" class="flex-column-v flex-center" style="height: 300px">
    <svg-icon icon-class="empty2" size="150"></svg-icon>
    <el-text type="info">在知识库中检索</el-text>
  </div>
  <div v-for="item in searchResult.items" class="mb10">
    <div class="bg-card br5 item">
      <div>
        <el-text>
          <!-- 使用 v-html 显示高亮内容 -->
          <span v-html="highlightKeyword(item.content, kw)"></span>
        </el-text>
      </div>
      <div>
        <el-divider border-style="dashed"></el-divider>
        <el-text type="info" size="small">{{t('知识库')}}：{{ item.refKb.name }}</el-text>
        <el-text type="info" class="ml20" size="small">{{t('文件')}}：{{ item.refImportRecord.file_name }}</el-text>
        <el-text type="info" class="ml20" size="small">{{ t('匹配度')}}：{{ (item.score * 100).toFixed(2) }} %</el-text>
      </div>
    </div>
  </div>

</template>

<style scoped lang="scss">
.item {
  line-height: 24px;
}

.el-divider {
  margin: 6px 0;
}


</style>
<style lang="scss">
.highlight {
  background-color: #ffff00;
  border-radius: 2px;
}
</style>