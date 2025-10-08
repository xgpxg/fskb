<script setup lang="ts">

import ChatInput from "./chat-input.vue";
import {computed, nextTick, onMounted, reactive, ref} from "vue";
import MessageCard from "./message-card.vue";
import ChatHeader from "./chat-header.vue";
import {useRouter} from "vue-router";
import {Channel} from "@tauri-apps/api/core";
import {call} from "../../utils/commands.ts";
import {ElMessage} from "element-plus";
import {toMdMessage, UserMessageContent} from "./chat.ts";
import {debounce} from 'lodash-es'
import GenShareCard from "@/views/chat/components/gen-share-card.vue";

const router = useRouter()
const route = router.currentRoute
const knowledgeBaseId = computed(() => {
  const id = route.value.params.knowledgeBaseId;
  return parseInt(Array.isArray(id) ? id[0] : id);
})
const knowledgeBase = ref({})

const messages = ref([])

// 获取知识库详情
const loadKbDetail = async () => {
  knowledgeBase.value = await call('kb_detail', {
    id: knowledgeBaseId.value
  })
}

// 获取知识库下的所有历史消息
const lastMessageId = ref(null)
const hasLoadAll = ref(false)
const debouncedLoadAllHistoryMessages = ref(null);
const loadAllHistoryMessages = async () => {
  // 如果已经加载完所有数据，则不再加载
  if (hasLoadAll.value) {
    return;
  }

  const container = messageListRef.value;
  if (!container) return;

  // 记录当前滚动位置和滚动高度
  const scrollTopBefore = container.scrollTop;
  const scrollHeightBefore = container.scrollHeight;

  const list = await call('list_all_history_messages', {
    kbId: knowledgeBaseId.value,
    lastMessageId: lastMessageId.value
  })
  messages.value.unshift(...list)
  // 如果返回的数据为空，说明已经加载完所有数据
  if (list.length === 0) {
    hasLoadAll.value = true
  } else {
    // 更新 lastMessageId 为第一条消息的 ID
    lastMessageId.value = list[0].message_id

    // 在下次 DOM 更新后恢复滚动位置
    await nextTick(() => {
      const container = messageListRef.value;
      if (container) {
        // 计算新增内容导致的高度变化，并调整滚动位置
        const heightDiff = container.scrollHeight - scrollHeightBefore;
        container.scrollTop = scrollTopBefore + heightDiff;
      }
    });
  }
}

onMounted(async () => {
  await loadKbDetail()
  loadAllHistoryMessages().then(() => {
    autoScrollEnabled.value = true
    scrollToBottom();

    scrollToBottom('smooth');

    resumeMessages()
  })
  setTimeout(() => {
    scrollToBottom('smooth');
  }, 500)
})

type ChatMessageChunk = {
  id: number;
  messageId: number;
  content: string;
  status: MessageStatus;
  role: 'user' | 'assistant';
};
// 定义会话事件
type ChatEvent = {
  event: 'start';
  data: ChatMessageChunk;
} | {
  event: 'message';
  data: ChatMessageChunk;
} | {
  event: 'done';
  data: ChatMessageChunk
};

// 定义消息状态
enum MessageStatus {
  // 等待回复(前端定义状态)
  Waiting = 'waiting',
  // 回复中
  Pending = 'pending',
  // 完成
  Finished = 'finished',
  // 错误
  Error = 'error'
}

const sendMessage = async (content: UserMessageContent) => {
  // 添加用户消息到消息列表
  messages.value.push({
    role: 'user',
    content: content,
  })

  // 滚到底部
  autoScrollEnabled.value = true
  setTimeout(() => {
    scrollToBottom('smooth');
  }, 100)

// 创建一个通道用于接收消息
  const channel = new Channel<ChatEvent>()
  // 绑定消息处理
  channel.onmessage = onmessage
  // 发送消息
  await call('chat', {
    kbId: knowledgeBaseId.value,
    content: content,
    channel: channel
  })

}
// 监听消息
const onmessage = ({event, data}) => {
  switch (event) {
      // 调用接口后，会立即返回一个内容为空的助手消息，收到这个消息后，将消息渲染到UI
      // 并设置状态为等待中
    case 'start': {
      data.status = MessageStatus.Waiting
      messages.value.push(data);
      break;
    }
      // 模型回复中，状态修改为回复中，并追加到消息列表
    case 'message': {
      const curr = messages.value.find(m => m.id === data.id);
      curr.status = data.status;
      curr.content += data.content;
      break;
    }
      // 模型回复完成，状态修改为完成
    case 'done': {
      const curr = messages.value.find(m => m.id === data.id);
      curr.status = data.status;
      break;
    }
  }

  if (autoScrollEnabled.value) {
    scrollToBottom('smooth');
  }

};

// 恢复某个消息流
const resume = async (message) => {
  // 创建一个通道用于接收消息
  const channel = new Channel<ChatEvent>()
  // 绑定消息处理
  channel.onmessage = onmessage
  await call('resume', {kbId: knowledgeBaseId.value, messageId: message.message_id, channel: channel});
}

// 恢复所有回复中的消息
const resumeMessages = async () => {
  let needResumeMessages = messages.value.filter(message => message.status === MessageStatus.Pending)
  for (let message of needResumeMessages) {
    await resume(message)
  }
}


const messageListRef = ref(null);
const autoScrollEnabled = ref(true);
const lastScrollTop = ref(0);

const isNearBottom = (): boolean => {
  const container = messageListRef.value;
  if (!container) return false;
  const threshold = 10;
  return container.scrollHeight - container.scrollTop - container.clientHeight < threshold;
};

const handleScroll = () => {
  const container = messageListRef.value;
  if (!container) return;

  const scrollTop = container.scrollTop;

  if (scrollTop < lastScrollTop.value) {
    autoScrollEnabled.value = false;
  } else if (isNearBottom()) {
    autoScrollEnabled.value = true;
  }

  lastScrollTop.value = scrollTop;
};

const handleWheel = (event: WheelEvent) => {
  const container = messageListRef.value;
  if (!container) return;
  const scrollTop = container.scrollTop;
  if (scrollTop < 50) {
    if (!debouncedLoadAllHistoryMessages.value) {
      debouncedLoadAllHistoryMessages.value = debounce(loadAllHistoryMessages, 200);
    }
    debouncedLoadAllHistoryMessages.value();
  }
};
const scrollToBottom = (behavior: string = 'auto') => {
  nextTick(() => {
    const container = messageListRef.value;
    if (container && autoScrollEnabled.value) {
      container.scrollTo({
        top: container.scrollHeight,
        behavior: behavior
      });
    }
  })
}
PubSub.unsubscribe('chat/message/list/reload')
PubSub.subscribe('chat/message/list/reload', () => {
  loadAllHistoryMessages()
})
PubSub.unsubscribe('chat/message/list/delete')
PubSub.subscribe('chat/message/list/delete', (event: string, data: any) => {
  const {id} = data
  messages.value = messages.value.filter(item => item.id !== id)
})

PubSub.unsubscribe('chat/message/command')
PubSub.subscribe('chat/message/command', (event, message: any) => {
  const {command, data} = message
  const m = new UserMessageContent()
  switch (command) {
    case 'GetText':
      m.text = '提取这个文件中的文字'
      m.command = command
      m.files = [data.filePath]
      break;
  }
  sendMessage(m)
})
</script>

<template>
  <div class="chat">
    <div class="header">
      <chat-header :knowledge-base="knowledgeBase"></chat-header>
    </div>
    <div ref="messageListRef"
         class="message-list"
         @scroll="handleScroll"
         @wheel="handleWheel">
      <div v-for="message in messages">
        <message-card :message="message" :assistant="knowledgeBase"></message-card>
      </div>
    </div>
    <chat-input @send-message="sendMessage" :knowledge-base="knowledgeBase"></chat-input>
    <gen-share-card></gen-share-card>
  </div>
</template>

<style scoped lang="scss">
.message-list {
  height: calc(100vh - 270px);
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
</style>