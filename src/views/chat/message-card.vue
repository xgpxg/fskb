<script setup lang="ts">
import SvgIcon from "@components/SvgIcon/index.vue";
import MessageMd from "@components/Editor/message-md.vue";
import MessageToolBar from "./message-tool-bar.vue";
import AutoAvatar from "../../components/avatar/AutoAvatar.vue";
import {AssistantMessageContent, UserMessageContent} from "./chat.js";
import {computed, h} from "vue";
import store from "@/store";
import {U} from '@/utils/util'
import ContextMenu from "@imengyu/vue3-context-menu";
import {revealItemInDir} from "@tauri-apps/plugin-opener";
import {call} from "@/utils/commands.ts";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const props = defineProps({
  message: Object,
  assistant: Object,
  sessionId: String
})

const user = computed(() => {
  return store.state.user
})

const onContextMenu = (e: MouseEvent) => {
  let message = props.message
  ContextMenu.showContextMenu({
    x: e.x, y: e.y,
    theme: '',
    items: [
      {
        label: t('添加到笔记'),
        icon: h(SvgIcon, {iconClass: 'add-to-note', size: '14px'}),
        onClick: () => {
          // 当前选中内容
          const selectText = window.getSelection().toString()
          PubSub.publish('chat/message/command/note', {
            command: 'AddToNote',
            data: {
              content: selectText
            }
          })
        },
        hidden: window.getSelection().toString().trim() === '',
      },
      {
        label: '生成分享卡片',
        icon: h(SvgIcon, {iconClass: 'share-card', size: '14px'}),
        onClick: () => {
          PubSub.publish('chat/message/command/share/card', {
            text: window.getSelection().toString()
          })
        },
      },
      {
        label: '导出为',
        icon: h(SvgIcon, {iconClass: 'summarize'}),
        onClick: () => alert('Click Simple item'),
        hidden: !U.isDev(),
        children: [
          {
            label: 'Markdown',
            icon: h(SvgIcon, {iconClass: 'summarize'}),
            onClick: () => {

            },
          },
          {
            label: 'PDF',
            icon: h(SvgIcon, {iconClass: 'summarize'}),
            onClick: () => {

            },
          },
          {
            label: 'Word',
            icon: h(SvgIcon, {iconClass: 'summarize'}),
            onClick: () => {

            },
          },
        ],
        divided: true,
      },
      {
        label: t('删除'),
        onClick: () => {
          call('delete_message', {
            id: message.id
          }).then(() => {
            PubSub.publish('chat/message/list/delete', {
              id: message.id
            })
          })
        },
      },
    ]
  });
}

</script>

<template>
  <div class="message-card">
    <div v-if="message['role']==='assistant'" class="left">
      <div>
        <auto-avatar :src="assistant?.icon" :text="assistant?.name" size="36px"></auto-avatar>
      </div>
      <div @contextmenu="onContextMenu($event)">
        <div v-if="message['status']==='waiting'" class="flex-v ml10 thinking">
          <el-icon class="is-loading" size="30">
            <svg-icon icon-class="loading"></svg-icon>
          </el-icon>
          思考中
        </div>
        <div class="message-md">
          <message-md v-if="message['status']!=='error'" :enable-header="false"
                      :value="AssistantMessageContent.from(message['content']).toMd()">
          </message-md>
        </div>
        <div class="ml20 br5 message-error-card" v-if="message['status']==='error'">
          <el-text type="danger">
            {{ message['content'] }}
          </el-text>
        </div>
        <div class="ml20 br5 message-tool-bar" v-if="message['status']==='finished'">
          <!--          <message-tool-bar class="message-tool-bar" :sessionId="sessionId"
                                      :messageId="message.messageId"></message-tool-bar>-->
          <div>
            <el-text type="info" size="small">
              {{ U.dateUtil.formatDate(new Date(message['create_time']), 'yyyy/MM/dd hh:mm') }}
            </el-text>
          </div>
        </div>
      </div>
    </div>
    <div v-if="message['role']==='user'" class="right">
      <div>
        <auto-avatar :src="user.avatar" :text="user.avatar" size="36px"></auto-avatar>
      </div>
      <div class="message-md" @contextmenu="onContextMenu($event)">
        <message-md :enable-header="false"
                    :value="UserMessageContent.from(message['content']).toMd()">
        </message-md>
      </div>
    </div>
  </div>
</template>


<style scoped lang="scss">
.message-card {
  border-radius: 15px;
  padding: 20px;

  .left {
    display: flex;
    justify-content: flex-start;

    .message-md {
      margin-top: 5px;
    }

    :hover {
      .message-tool-bar {
        visibility: visible;
      }
    }

    :deep(.v-md-editor-preview) {
      margin-bottom: 10px;
    }
  }

  .right {
    display: flex;
    justify-content: flex-start;
    flex-direction: row-reverse;

    .message-md {
      background: var(--el-color-primary-light-10) !important;
      padding: 10px 0;
      border-radius: 5px;
      max-width: calc(100% - 60px);
      margin-left: 60px;
      margin-right: 10px;
    }
  }

  .thinking {
    font-size: 14px;
    color: #6a737d;
  }

  .message-error-card {
    background: #faebeb;
    padding: 10px;
    font-weight: bold;
  }

  :deep(img) {
    max-width: 25vw;
  }
}

.message-tool-bar {
  visibility: hidden;
}


</style>

<style lang="scss">
.file-card {
  display: flex;
  align-items: center;
  padding: 6px 10px;
  border: 1px solid #e8e8e8;
  border-radius: 4px;
  text-decoration: none;
  background-color: #f6f8fa;
  font-size: 13px;
  transition: all 0.15s ease;
  margin-top: 4px;

  .file-ext-card {
    width: 18px;
    height: 18px;
    border-radius: 3px;
    font-weight: bold;
    font-size: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 5px;
    margin-right: 8px;
  }

  // 文件类型颜色映射
  $file-colors: (
      "pdf": #f85959,
      "doc": #2b579a,
      "docx": #2b579a,
      "xls": #217346,
      "xlsx": #217346,
      "ppt": #d24726,
      "pptx": #d24726,
      "zip": #ffb900,
      "rar": #ffb900,
      "7z": #ffb900,
      "jpg": #64b446,
      "jpeg": #64b446,
      "png": #64b446,
      "gif": #64b446,
      "txt": #444444,
      "md": #444444,
      "json": #444444,
      "js": #f7df1e,
      "ts": #3178c6,
      "html": #e34f26,
      "css": #1572b6,
      "exe": #43a047,
      "mp3": #ef5350,
      "mp4": #ab47bc,
      "mov": #ab47bc,
      "avi": #ab47bc,
      "default": #444444
  );

  // 生成文件类型样式
  @each $ext, $color in $file-colors {
    &[data-ext="#{$ext}"] .file-ext-card {
      background-color: #{rgba($color, 0.12)};
      color: #{$color};
    }
  }
}

</style>