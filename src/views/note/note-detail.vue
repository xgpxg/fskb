<script setup lang="ts">
import {useI18n} from "vue-i18n";
import {ref, onMounted, onBeforeUnmount, watch} from "vue";
import MdEditor from "@components/Editor/MdEditor.vue";
import {call} from "@/utils/commands.ts";
import {debounce} from "lodash-es";

const {t} = useI18n()

const props = defineProps({
  note: {
    type: Object,
    required: true
  }
})

const isShow = ref(false)
const mode = ref('preview')

const show = () => {
  isShow.value = true
}
const close = () => {
  isShow.value = false
}
defineExpose({
  show, close
})

const updateNote = () => {
  call('update_note', {
    req: {
      id: props.note.id,
      title: props.note.title,
      content: props.note.content,
      summary: props.note.summary
    }
  })
}


const debounceUpdateNote = debounce(updateNote, 1000)
const lastAutoSaveTime = ref()
watch(() => props.note, (newValue, oldValue) => {
  debounceUpdateNote()
  lastAutoSaveTime.value = new Date().toLocaleString()
}, {deep: true})

// 切换编辑/预览模式
const toggleMode = () => {
  mode.value = mode.value === 'edit' ? 'preview' : 'edit'
}
// 处理键盘事件
const handleKeyDown = (e: KeyboardEvent) => {
  // 检查是否按下了 Alt 键 和 空格键
  if (e.altKey && e.ctrlKey) {
    toggleMode()
  }
}

// 组件挂载时添加事件监听器
onMounted(() => {
  document.addEventListener('keydown', handleKeyDown)
})

// 组件卸载前移除事件监听器
onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleKeyDown)
})

</script>

<template>
  <el-drawer v-model="isShow"
             direction="btt"
             size="100vh"
             modal-class="note-modal"
             custom-class="notebook-drawer"
             destroy-on-close
             v-if="props.note"
             append-to-body
  >
    <template #title>
      <div class="fill-width flex-space-between flex-v">
        <div>
          <el-input v-model="props.note.title" v-if="mode==='edit'" :placeholder="t('笔记标题')"
                    style="width: calc(75vw - 500px)"></el-input>
          <h4 v-if="mode==='preview'" class="title">{{ props.note.title }}</h4>
        </div>
        <div>
          <el-button text v-if="mode==='edit'" type="primary" class="ml10 mr20" @click="mode='preview'">
            {{ t('预览') }}
          </el-button>
          <el-button text v-if="mode==='preview'" type="primary" class="ml10 mr20" @click="mode='edit'">
            {{ t('编辑') }}
          </el-button>
        </div>
      </div>
    </template>
    <div class="note-detail">
      <md-editor v-model="props.note.content" :placeholder="t('记个笔记吧')" :mode="mode"></md-editor>
    </div>
    <template #footer>
      <div class="footer">
        <template v-if="mode==='edit'">
          <div>
            <el-text size="small" type="info"> {{ props.note.content.length }} {{ t('字')}}</el-text>
          </div>
          <div>
            <el-text size="small" type="info">
              <el-divider direction="vertical"></el-divider>
            </el-text>
          </div>
          <div v-if="lastAutoSaveTime">
            <el-text size="small" type="info">{{ lastAutoSaveTime }} {{t('已自动保存')}}</el-text>
          </div>
          <div>
            <el-text size="small" type="info">
              <el-divider direction="vertical"></el-divider>
            </el-text>
          </div>
          <div>
            <el-text size="small" type="info">{{ t('ctrl+alt切换编辑/预览模式')}}</el-text>
          </div>
        </template>
      </div>
    </template>
  </el-drawer>
</template>

<style scoped lang="scss">
.item {
  background-color: #fafafa;
  border-radius: 5px;
  padding: 10px;
}

:deep(.v-md-textarea-editor ) {
  height: calc(100vh - 170px);
  overflow: hidden;

  textarea {
    padding: 10px 0;
    overflow: auto;
    z-index: 2;

    &::-webkit-scrollbar {
      width: 8px;
      height: 8px;
    }

    &::-webkit-scrollbar-thumb {
      background-color: rgba(0, 0, 0, 0.1);
      border-radius: 3px;
    }
  }
}

:deep(.el-input__wrapper) {
  box-shadow: unset;
  background: #f0f0f0;
}

.note-detail {
  padding: 0 10px;
}

:deep(.v-md-editor__toolbar) {
  padding: 6px 0;
}

</style>

<style lang="scss">
.note-modal {
  margin-right: 25vw;
  //margin-left: 250px;
  width: calc(75vw - 0.1px);

  .title {
    margin-bottom: 10px;
  }

  .el-drawer__body {
    padding: 0 0 0 10px;
  }

  .footer {
    margin-bottom: 24px;
    display: flex;
    align-items: center;
    justify-items: center;
  }
}


</style>
