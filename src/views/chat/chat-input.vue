<script setup lang="ts">
import {computed, ref} from 'vue'
import SvgIcon from "../../components/SvgIcon/index.vue";
import ModelSwitch from "./model-switch.vue";
import {UserMessageContent} from "./chat.ts";
import AutoAvatar from "../../components/avatar/AutoAvatar.vue";
import {open} from "@tauri-apps/plugin-dialog";
import MessageInput from "./message-input.vue";
import {call, convertImageSrc} from "@/utils/commands.ts";
import {useI18n} from "vue-i18n";
import {U} from "@/utils/util";

const {t} = useI18n()

const props = defineProps({
  knowledgeBase: {
    type: Object,
    required: true
  }
})
// 已选择的模型
const selectedModel = ref({
  id: null,
  name: null,
  icon: null,
  description: null,
})
// 消息输入框
const messageInputRef = ref(null)
// 消息输入框中的内容
const content = ref<UserMessageContent>(new UserMessageContent())

const imageDialogContent = ref<UserMessageContent>(new UserMessageContent())


const showSendImageDialog = ref(false)

const message = computed(() => {
  return {
    text: content.value.text,
    images: content.value.images,
    files: content.value.files,
  }
})

const modelSwitchRef = ref(null)
const selectedModelPopoverRef = ref(null)

const openImageFileDialog = async () => {
  const files = await open({
    multiple: true, // 允许选择多个文件
    filters: [
      {
        name: 'Images',
        extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp'],
      },
    ],
  })

  if (Array.isArray(files)) {
    imageDialogContent.value.images.push(...files)
  } else if (files !== null) {
    imageDialogContent.value.images.push(files)
  }
  if (imageDialogContent.value.images.length === 0) {
    return
  }
  showSendImageDialog.value = true
}


const emit = defineEmits(['sendMessage'])

const sendMessage = () => {
  if (content.value.isEmpty()) {
    return
  }
  emit('sendMessage', message.value)
  setTimeout(() => {
    messageInputRef.value.clean()
  }, 50)
}
const sendMessageWithEnter = (event) => {
  if (event.shiftKey && event.keyCode === 13) {
    return;
  }
  if (!event.shiftKey && event.keyCode === 13) {
    if (content.value.isEmpty()) {
      event.preventDefault()
      return
    }
    sendMessage()
  }
}

// 拷贝文件到数据目录
const copyChatFileToDataDir = async (path: string): Promise<string> => {
  return await call('copy_chat_file_to_data_dir', {
    kbId: props.knowledgeBase.id,
    path: path,
  })
}

// 从图片选择弹窗发送消息
const sendImageMessage = async () => {
  if (imageDialogContent.value.images.length === 0) {
    return
  }

  // 拷贝文件到数据目录
  const images = []
  for (let imagePath of imageDialogContent.value.images) {
    images.push(await copyChatFileToDataDir(imagePath))
  }

  const message = {
    text: imageDialogContent.value.text,
    images: images,
  }
  emit('sendMessage', message)
  showSendImageDialog.value = false
  setTimeout(() => {
    imageDialogContent.value = new UserMessageContent()
  }, 50)
}
</script>


<template>
  <div class="chat-input">
    <div class="input-area">
      <!--      <el-input v-model="text"
                      type="textarea"
                      placeholder="输入消息..."
                      :rows="5"
                      @keydown="sendMessageWithEnter"></el-input>-->
      <message-input ref="messageInputRef"
                     v-model:content="content"
                     :knowledge-base="knowledgeBase"
                     @keydown="sendMessageWithEnter"></message-input>
      <div class="flex-space-between">
        <div class="actions">
          <el-popover ref="selectedModelPopoverRef"
                      placement="right-end"
                      trigger="click"
                      width="300"
                      @show="modelSwitchRef.refresh()"
                      transition="none">
            <template #reference>
              <template v-if="selectedModel?.id">
                <auto-avatar :src="selectedModel.icon" :text="selectedModel.name" size="20px"
                             class="action-item" transparent></auto-avatar>
              </template>
              <template v-else>
                <div>
                  <svg-icon icon-class="question"
                            size="20"
                            class="action-item"
                  ></svg-icon>
                </div>
              </template>
            </template>
            <model-switch ref="modelSwitchRef"
                          v-model="selectedModel"
                          @hide="selectedModelPopoverRef.hide()"
                          :knowledge-base="knowledgeBase">
            </model-switch>
          </el-popover>
          <!--          <svg-icon icon-class="internet"
                              size="20"
                              class="action-item"></svg-icon>-->
          <svg-icon icon-class="image"
                    size="20"
                    @click="openImageFileDialog"
                    class="action-item"></svg-icon>
          <svg-icon icon-class="file"
                    size="20"
                    class="action-item"
                    v-if="U.isDev()"></svg-icon>
        </div>
        <div>
          <el-button class="send-btn" @click="sendMessage" type="primary">
            <svg-icon icon-class="send" size="20" class="mr5"></svg-icon>
            {{ t('发送') }}
          </el-button>
        </div>
      </div>
    </div>
  </div>
  <el-dialog title="发送图片" v-model="showSendImageDialog" width="500px" :close-on-click-modal="false"
             @close="imageDialogContent.images = []">
    <div class="flex-v mb10">
      <div style="height: 20px;width: 30px" class="bg-card flex-center br5 border-solid mr10"
           @click="openImageFileDialog">
        <el-icon>
          <Plus></Plus>
        </el-icon>
      </div>
      <div class="scrollbar-flex-content" style="overflow-x: auto">
        <div v-for="(image,index) in imageDialogContent.images.slice(0,23)" class=" scrollbar-demo-item">
          <div class="flex-center box-shadow br5">
            <el-image :src="convertImageSrc(image)" style="height: 40px;"></el-image>
          </div>
        </div>
      </div>
    </div>
    <div class="fill-width">
      <div class="fill-width">
        <el-input v-model="imageDialogContent.text" type="textarea" rows="5" class="fill-width"
                  :placeholder="t('你希望对这些图片做什么操作')+'？'"></el-input>
      </div>
    </div>
    <template #footer>
      <el-button @click="showSendImageDialog = false">取消</el-button>
      <el-button type="primary" @click="sendImageMessage">发送</el-button>
    </template>
  </el-dialog>
</template>

<style scoped lang="scss">
.chat-input {
  padding: 10px;
  border-top: 1px solid #ddd;
  width: calc(100vw - 271px - 25vw);
  position: absolute;
  bottom: 0;
  //transform: translateX(calc(-50vw + 140px));

  .input-area {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;


    .actions {
      display: flex;
      align-items: center;
      gap: 1.5rem;
      margin-left: 10px;
      margin-bottom: -10px;

      .action-item {
        &:hover {
          scale: 1.2;
          transition: all 0.3s;
        }
      }
    }
  }
}

.file-area {
  margin-left: 20px;
}

.scrollbar-flex-content {
  display: flex;
  width: 100%;
}

.scrollbar-demo-item {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 50px;
  margin: 0 10px;
  text-align: center;
  border-radius: 4px;
  color: var(--el-color-danger);
}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.1);
  border-radius: 3px;
}
</style>
