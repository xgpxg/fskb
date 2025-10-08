<script setup lang="ts">
import {computed, inject, nextTick, onMounted, reactive, ref, watch} from "vue";
import AutoAvatar from "../../components/avatar/AutoAvatar.vue";
import {onBeforeRouteLeave, onBeforeRouteUpdate, useRoute, useRouter} from "vue-router";
import {invoke} from "@tauri-apps/api/core";
import PubSub from 'pubsub-js'
import DropHandler from "@/views/home/drop-handler.vue";
import {call} from "@/utils/commands.ts";
import {ElMessage} from "element-plus";
import {useI18n} from "vue-i18n";
import KbRecordList from "@/views/knowledge/kb-record-list.vue";

const {t} = useI18n()
const router = useRouter()
const kbs = ref([])
const route = useRoute()
const currKnowledgeBaseId = computed(() => {
  const id = route.params.knowledgeBaseId;
  return parseInt(Array.isArray(id) ? id[0] : id);
})
const showMenu = inject<() => void>('showMenu')
const hideMenu = inject<() => void>('hideMenu')
const listAllKb = async () => {
  kbs.value = await call('list_all_kb')
  if (kbs.value.length > 0 && !isNaN(currKnowledgeBaseId.value)) {
    onKbClick(kbs.value.find(kb => kb.id === currKnowledgeBaseId.value))
  }

  if (kbs.value.length > 0) {
    showMenu()
  }
  if (kbs.value.length === 0) {
    hideMenu()
    localStorage.setItem('RECENT_KBS', '[]')
    PubSub.publish('kb/recent/reload')
  }
}
onMounted(async () => {
  await listAllKb()
})

PubSub.unsubscribe('kb/list/refresh')
PubSub.subscribe('kb/list/refresh', async () => {
  await listAllKb()
})
const onKbClick = (item) => {
  // 设置未选中
  kbs.value.forEach((item) => {
    item.selected = false
  })
  // 当前选中
  item.selected = true
  // 转到chat页面
  router.push({name: 'Chat', params: {knowledgeBaseId: item.id}})

  // 添加最近使用的知识库
  let recentKbsStr = localStorage.getItem('RECENT_KBS') || '[]'
  let recentKbs: object[] = JSON.parse(recentKbsStr)
  // 过滤掉已存在的项
  recentKbs = recentKbs.filter(v => v.id !== item.id)
  // 将当前项添加到数组开头
  recentKbs.unshift({id: item.id, name: item.name, description: item.description})
  // 只保留前3个元素
  recentKbs = recentKbs.slice(0, 3)
  localStorage.setItem('RECENT_KBS', JSON.stringify(recentKbs))

  kbs.value.map(v => {
    if (v.id !== item.id) {
      v['isShowKbRecordList'] = false
    }
  })
}

// 离开页面时取消选中
onBeforeRouteUpdate((to, from, next) => {
  if (to.name !== 'Chat') {
    kbs.value.forEach(item => {
      item.selected = false
    })
  }
  next()
})

watch(currKnowledgeBaseId, (newVal) => {
  if (newVal) {
    kbs.value.find(item => item.id === newVal).selected = true
  }
})

const showKbRecordList = (item: any) => {
  kbs.value.map(v => {
    if (v.id !== item.id) {
      v['isShowKbRecordList'] = false
    }
  })
  item['isShowKbRecordList'] = !item['isShowKbRecordList']
}
</script>

<template>
  <div class="list">
    <template v-for="item in kbs">
      <div class="item" @click="onKbClick(item)" :class="{selected:item.selected}" @dblclick="showKbRecordList(item)">
        <div>
          <auto-avatar :src="item.icon" :text="item.name" size="36px"></auto-avatar>
        </div>
        <div>
          <div class="title ellipsis" style="max-width: 200px">{{ item.name }}</div>
          <div class="summary">
            {{ item.description ?? t('暂无描述') }}
          </div>
        </div>
      </div>
      <div class="kb-record-list">
        <kb-record-list v-if="item['isShowKbRecordList']" :kb-id="item.id"></kb-record-list>
      </div>
    </template>
  </div>
  <drop-handler></drop-handler>
</template>

<style scoped lang="scss">
.list {
  height: calc(100vh - 150px);
  overflow-y: auto;

  &::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  &::-webkit-scrollbar-thumb {
    background-color: rgba(0, 0, 0, 0.1);
    border-radius: 3px;
  }
}

.item {
  padding: 10px 10px;
  display: flex;
  gap: 10px;

  .title {
    font-weight: 500;
    font-size: 14px;
    color: #2b2b2b;
  }

  .summary {
    color: #8e8e8e;
    font-size: 12px;
    width: 180px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    word-break: break-all;
  }

  &:hover {
    background: #eae9e9;
  }

  &.selected {
    background: #eae9e9;
  }
}

.drop-active {
  background: var(--el-color-primary);
}

.kb-recored-list {

}

</style>