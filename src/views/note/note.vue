<script setup lang="ts">

import NoteList from "@/views/note/note-list.vue";
import MdEditor from "@components/Editor/MdEditor.vue";
import {computed, ref, watch} from "vue";
import {useRouter} from "vue-router";
import {call} from "@/utils/commands.ts";
import {U} from "@/utils/util";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const router = useRouter()
const route = router.currentRoute

const content = ref("");

const knowledgeBaseId = computed(() => {
  const id = route.value.params.knowledgeBaseId;
  return parseInt(Array.isArray(id) ? id[0] : id);
})

const addNote = async () => {
  if (content.value.trim() === '') {
    return
  }
  await call('add_note', {
    req: {
      kbId: knowledgeBaseId.value,
      title: null,
      content: content.value,
      summary: null
    }
  })
  PubSub.publish('note/list/reload')

  content.value = ''
}

watch(knowledgeBaseId, () => {
  PubSub.unsubscribe('chat/message/command/note')
  PubSub.subscribe('chat/message/command/note', (event: string, message: any) => {
    const {command, data} = message
    switch (command) {
      case 'AddToNote':
        content.value = data.content
        break;
    }
  })
}, {immediate: true})
</script>

<template>
  <div class="note-page">
    <div class="small-editor">
      <md-editor v-model="content" max-height="150px" :placeholder="t('记个笔记吧')" hide-all-menu></md-editor>
      <div class="mt10">
        <div class="flex-space-between">
          <div>
            <el-checkbox v-if="U.isDev()">AI助写</el-checkbox>
          </div>
          <el-button @click="addNote" type="primary" text>
            {{ t('记一下') }}
          </el-button>
        </div>
      </div>
    </div>
    <div class="mt20">
      <note-list></note-list>
    </div>

  </div>
</template>

<style scoped lang="scss">
.note-page {
  padding: 10px;
  overflow-y: auto;
  height: calc(100vh - 120px);

  &::-webkit-scrollbar {
    width: 0 !important;
    height: 0;
  }
}

.small-editor {
  :deep(.v-md-editor) {
    max-height: 50vh;
    min-height: 30vh;

    textarea {
      padding-right: 8px;
    }
  }
}

</style>