<script setup lang="ts">
import {useRoute} from "vue-router";
import {computed, onMounted, ref, watch} from "vue";
import {call} from "@/utils/commands.ts";
import KbSearch from "@/views/search/kb-search.vue";
import LocalSearch from "@/views/search/local-search.vue";
import {Channel} from "@tauri-apps/api/core";
import SvgIcon from "@components/SvgIcon/index.vue";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const route = useRoute()
const q = computed(() => {
  return route.query.q
})
watch(q, (q) => {
  form.value.kw = q
})
const form = ref({
  kw: q.value
})
const searchResult = ref({
  kb: {items: []},
  local: {items: [], hasNext: false},
})

onMounted(() => {
  search()
})

type ChatSearchResultEvent = {
  event: 'kb';
  data: {};
} | {
  event: 'local';
  data: {
    items: object[],
    has_next: boolean
  };
};


const search = async () => {
  if (!form.value.kw || form.value.kw.trim() === '') {
    return
  }
  searchResult.value = {
    kb: {items: []},
    local: {items: []},
  }
  // 创建一个通道用于接收消息
  const channel = new Channel<ChatSearchResultEvent>()
  channel.onmessage = onmessage
  await call('search', {
    req: {
      kw: form.value.kw
    },
    channel
  })
}

const onmessage = ({event, data}) => {
  switch (event) {
      // 调用接口后，会立即返回一个内容为空的助手消息，收到这个消息后，将消息渲染到UI
      // 并设置状态为等待中
    case 'kb': {
      searchResult.value.kb = data
      break;
    }
    case 'local': {
      searchResult.value.local.items.push(...data.items)
      searchResult.value.local.hasNext = data.hasNext
      break;
    }
  }
}

</script>

<template>
  <div class="pd10">
    <div>
      <div class="left">
        <div class="content">
          <el-input v-model="form.kw" prefix-icon="search"
                    :placeholder="t('输入关键词搜索知识库/本地文件/网页')"
                    @keydown.enter="search" clearable></el-input>
        </div>
      </div>
      <el-tabs>
        <el-tab-pane>
          <template #label>
            {{ t('知识库') }}
          </template>
          <el-scrollbar height="calc(100vh - 99px)">
            <kb-search :kw="form.kw" :search-result="searchResult.kb"></kb-search>
          </el-scrollbar>
        </el-tab-pane>
        <el-tab-pane>
          <template #label>
            {{ t('本地文件') }}
            <el-icon class="loading ml5" v-if="searchResult.local.hasNext">
              <svg-icon icon-class="loading2"></svg-icon>
            </el-icon>
          </template>
          <el-scrollbar height="calc(100vh - 99px)">
            <local-search :kw="form.kw" :search-result="searchResult.local"></local-search>
          </el-scrollbar>
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<style scoped lang="scss">
.left {
  position: relative;

  .content {
    position: fixed;
    left: 260px;
    top: 50px;
    width: 310px;
    z-index: 1;

    :deep(.el-input__wrapper) {
      box-shadow: unset;
      background: #f0f0f0;
    }
  }

}

:deep(.el-tabs__nav) {
  float: right;
  margin-top: -4px;
}

:deep(.el-tabs__header) {
  margin: 0 0 4px 0;
}
</style>