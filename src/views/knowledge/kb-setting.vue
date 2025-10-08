<script setup lang="ts">
import {computed, onMounted, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {call, convertImageSrc} from "../../utils/commands.ts";
import PubSub from 'pubsub-js'
import {useRouter} from "vue-router";
import KbToolSetting from "@/views/knowledge/kb-tool-setting.vue";
import {ElMessage} from "element-plus";
import AutoAvatar from "@components/avatar/AutoAvatar.vue";
import KbFileExtractSetting from "@/views/knowledge/kb-file-extract-setting.vue";
import {useI18n} from "vue-i18n";
import {open} from "@tauri-apps/plugin-dialog";

const {t} = useI18n()
const props = defineProps({
  knowledgeBase: {
    type: Object,
    required: true
  }
})

const router = useRouter()
const form = ref({
  ...props.knowledgeBase,
  mcpServers: props.knowledgeBase.mcp_server_ids || []
})
watch(
    () => props.knowledgeBase,
    (newVal) => {
      form.value = {
        ...newVal,
        mcpServers: newVal.mcp_server_ids || []
      }
    },
    {deep: true, immediate: true}
)
const rules = computed(() => {
  return {
    name: {required: true, message: t('请填写知识库名称'), trigger: 'blur'},
    'fileContentExtractType.type': {required: true, message: t('请选择文件内容提取方式'), trigger: 'change'},
  }
})
const formRef = ref()

const delKb = async () => {
  await call('delete_kb', {
    id: props.knowledgeBase.id
  })
  PubSub.publish('kb/list/refresh')

  removeRecentKb()

  await router.push({name: 'Welcome'})
}

const removeRecentKb = () => {
  let recentKbsStr = localStorage.getItem('RECENT_KBS') || '[]'
  let recentKbs: object[] = JSON.parse(recentKbsStr)
  recentKbs = recentKbs.filter(v => v.id !== props.knowledgeBase.id)
  localStorage.setItem('RECENT_KBS', JSON.stringify(recentKbs))
}

const updateKb = async () => {
  const valid = await formRef.value.validate()
  if (!valid) {
    return
  }
  await call('update_kb', {
    req: {
      id: props.knowledgeBase.id,
      name: form.value.name,
      description: form.value.description,
      mcpServerIds: form.value.mcp_server_ids,
      fileContentExtractType: form.value.file_content_extract_type,
      icon: form.value.icon
    }
  })
  PubSub.publish('kb/list/refresh')
  ElMessage.success(t('已保存'))

  props.knowledgeBase.name = form.value.name
}

const clearMessage = async () => {
  await call('clear_message', {
    kbId: props.knowledgeBase.id
  })
  PubSub.publish('chat/message/list/reload')
}

const openIconSelectFileDialog = async () => {
  const file = await open({
    multiple: false, // 允许选择多个文件
    filters: [
      {
        name: 'Images',
        extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp'],
      },
    ],
  })
  if (file) {
    const savedFile = await call('copy_file_to_file_dir', {
      src: file
    })
    form.value.icon = convertImageSrc(savedFile)
  }
}
</script>

<template>
  <el-form ref="formRef" :model="form" label-width="90" :rules="rules" label-position="left">
    <div class="title-block">{{ t('基础信息') }}</div>
    <div class="pdt10 br5">
      <el-form-item :label="t('名称')">
        <div class="fill-width flex-v">
          <div class="fill-width">
            <el-form-item prop="name">
              <el-input v-model="form.name" :placeholder="t('知识库名称')">
                <template #prefix>
                  <auto-avatar :src="form.icon" :text="form.name" shape="square" size="24px"
                               @click="openIconSelectFileDialog" class=""></auto-avatar>
                </template>
              </el-input>
            </el-form-item>
          </div>
        </div>
      </el-form-item>
      <el-form-item :label="t('描述')">
        <el-input v-model="form.description" type="textarea" :placeholder="t('知识库描述')" rows="3"></el-input>
      </el-form-item>
    </div>

    <div class="title-block">{{ t('内容解析') }}</div>
    <div class="pdt10 br5">
      <el-form-item :label="t('文本抽取')" prop="file_content_extract_type">
        <kb-file-extract-setting v-model:file-content-extract-type="form.file_content_extract_type">
        </kb-file-extract-setting>
      </el-form-item>
      <el-form-item :label="t('MCP工具')">
        <kb-tool-setting :kb-id="form.id" v-model="form.mcp_server_ids"></kb-tool-setting>
        <el-text type="info" size="small" class="compact mt5">💡
          {{ t('MCP可作为知识库的扩展工具，在对话过程中LLM可自动调用这些工具') }}
        </el-text>
      </el-form-item>
    </div>
    <el-form-item label="">
      <el-button type="primary" @click="updateKb">{{ t('保存') }}</el-button>
    </el-form-item>
  </el-form>

  <div class="bg-card br5 mt50" style="bottom: 40px;right:0;position:absolute;">
    <el-popconfirm :title="t('清空对话消息提示')" @confirm="clearMessage" width="200">
      <template #reference>
        <el-button type="danger" text>{{ t('清空对话记录') }}</el-button>
      </template>
    </el-popconfirm>
    <el-popconfirm :title="t('确定删除知识库')" v-if="form.id!==0" @confirm="delKb" width="200">
      <template #reference>
        <el-button type="danger" text>{{ t('删除知识库') }}</el-button>
      </template>
    </el-popconfirm>
  </div>

</template>

<style scoped lang="scss">

</style>