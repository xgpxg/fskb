<script setup lang="ts">

import {onMounted, ref, useAttrs, watch} from "vue";
import {call, convertImageSrc} from "@/utils/commands.ts";
import {UserMessageContent} from "@/views/chat/chat.ts";
import {sep} from '@tauri-apps/api/path';

const props = defineProps({
  knowledgeBase: {
    type: Object,
    required: true
  }
})
const attrs = useAttrs()
const inputRef = ref(null)
const contentData = ref({
  text: '',
  images: [] as string[],
  files: [] as string[]
})
const content = defineModel<UserMessageContent>('content')

watch(contentData, (value) => {
  content.value = new UserMessageContent(value.text, value.images, value.files)
}, {deep: true})

// 粘贴处理
const handlePaste = async (event: ClipboardEvent) => {
  const items = event.clipboardData?.items
  const text = event.clipboardData?.getData('text/plain')

  // 阻止默认粘贴行为
  event.preventDefault()
  event.stopPropagation()

  // 插入文本内容
  if (text && inputRef.value) {
    insertText(text)
    contentData.value.text += text
  }

  if (!items) {
    return
  }

  for (const item of items) {
    if (item.type.startsWith('image/')) {
      const blob = item.getAsFile()
      if (blob) {
        const {blobUrl, filePath} = await gen_image_path(blob)
        insertImage(blobUrl, filePath)
        contentData.value.images.push(filePath)
      }
    } else if (item.kind === 'file') {
      const blob = item.getAsFile()
      if (blob) {
        const {filePath, fileName} = await gen_file_path(blob)
        insertFile(filePath, fileName)
        contentData.value.files.push(filePath)
      }
    }
  }
}

// 保存图片到临时目录
const gen_image_path = async (blob: Blob) => {
  const arrayBuffer = await blob.arrayBuffer()
  const fileName = `image_${Date.now()}.${blob.type.split('/')[1]}`
  const filePath = await saveChatFileToDataDir(arrayBuffer, fileName)
  return {
    blobUrl: URL.createObjectURL(blob),
    filePath: filePath,
  }
}

// 保存文件到临时目录
const gen_file_path = async (file: File) => {
  const arrayBuffer = await file.arrayBuffer()
  const fileName = file.name
  const filePath = await saveChatFileToDataDir(arrayBuffer, fileName)
  return {
    filePath: filePath,
    fileName: fileName,
  }
}

// 保存文件到数据目录
const saveChatFileToDataDir = async (content: ArrayBuffer, fileName: string): Promise<string> => {
  return await call('save_chat_file_to_data_dir', {
    kbId: props.knowledgeBase.id,
    fileName: fileName,
    bytes: new Uint8Array(content)
  })
}

// 插入文本
const insertText = (text: string) => {
  const selection = window.getSelection()
  if (selection && selection.rangeCount > 0) {
    const range = selection.getRangeAt(0)
    range.deleteContents()
    range.insertNode(document.createTextNode(text))
    // 移动光标到插入内容之后
    range.collapse(false)
    selection.removeAllRanges()
    selection.addRange(range)
  }
}

// 插入图片
const insertImage = (src: string, filePath: string) => {
  if (inputRef.value) {
    const img = document.createElement('img')
    img.src = convertImageSrc(filePath)
    img.dataset.type = 'image'
    img.dataset.path = filePath
    img.style.maxWidth = '100px'
    img.addEventListener('load', () => {
      URL.revokeObjectURL(src) // 释放内存
    })

    inputRef.value.appendChild(img)

    const textNode = document.createTextNode('')
    inputRef.value.appendChild(textNode)

    moveCursorTo(textNode)
  }
}

// 插入文件
const insertFile = (filePath: string, fileName: string) => {
  if (inputRef.value) {
    const fileDiv = document.createElement('div')
    fileDiv.classList.add('file-item')
    fileDiv.dataset.type = 'file'
    fileDiv.dataset.path = filePath
    fileDiv.innerText = fileName
    fileDiv.contentEditable = 'false'

    inputRef.value.appendChild(fileDiv)

    const textNode = document.createTextNode('\u200B')
    inputRef.value.appendChild(textNode)

    moveCursorTo(textNode)
  }
}

const moveCursorTo = (node: Text) => {
  const selection = window.getSelection()
  const range = document.createRange()
  range.setStart(node, node.length)
  range.collapse(true)

  selection.removeAllRanges()
  selection.addRange(range)
}


const handleInput = () => {
  const container = inputRef.value
  if (!container) return

  // 清理所有 data-type 节点中已被删除的项
  const imagePaths: string[] = []
  const filePaths: string[] = []

  const elements = container.querySelectorAll('[data-type]')
  elements.forEach((el: HTMLElement) => {
    if (el.dataset.type === 'image') {
      imagePaths.push(el.dataset.path)
    } else if (el.dataset.type === 'file') {
      filePaths.push(el.dataset.path)
    }
  })

  // 同步数据
  contentData.value.text = extractText()
  contentData.value.images = [...imagePaths]
  contentData.value.files = [...filePaths]
}

// 提取文本内容
const extractText = () => {
  if (inputRef.value) {
    let textContent = ''

    inputRef.value.childNodes.forEach(node => {
      if (node.nodeType === Node.TEXT_NODE && node.textContent.trim()) {
        textContent += node.textContent
      } else if (node.nodeType === Node.ELEMENT_NODE) {
        // 可选：处理 br 换行
        if (node.nodeName === 'BR') {
          textContent += '\n'
        }
      }
    })

    return textContent.trim()
  }
}

const clean = () => {
  if (inputRef.value) {
    contentData.value.text = ''
    contentData.value.images = []
    contentData.value.files = []
    content.value = {}
    inputRef.value.innerHTML = ''
    inputRef.value.focus()
  }
}

defineExpose({
  insertImage,
  clean
})


PubSub.unsubscribe('chat/message/command/input')
PubSub.subscribe('chat/message/command/input', (event: string, message: any) => {
  const {command, data} = message
  switch (command) {
    case 'QuoteFile':
      const filePath = data.filePath
      const fileName = filePath.split(sep()).pop()
      const fileExt = fileName.split('.').pop()
      if (['png', 'jpg', 'jpeg', 'webp', 'bmp'].indexOf(fileExt) > -1) {
        insertImage(filePath, filePath)
      } else {
        insertFile(filePath, fileName)
      }
      contentData.value.files.push(filePath)
      break;
  }

})

</script>

<template>
  <div class="message-input">
    <div
        ref="inputRef"
        class="input"
        contenteditable="true"
        @paste="handlePaste"
        @input="handleInput"
        v-on="attrs.on"
    ></div>
  </div>
</template>

<style lang="scss">
.message-input {
  .input {
    padding: 12px 16px;
    height: 95px;
    max-height: 95px;
    overflow-y: auto;
    white-space: pre-wrap;
    word-wrap: break-word;
    border-radius: 8px;
    font-size: 14px;
    line-height: 1.5;
    color: #333;
    transition: all 0.2s ease;

    &:focus {
      outline: none;
    }

    &:hover {
      border-color: #ccc;
    }

    /* 滚动条样式（可选） */
    &::-webkit-scrollbar {
      width: 6px;
      height: 6px;
    }

    &::-webkit-scrollbar-thumb {
      background-color: rgba(0, 0, 0, 0.1);
      border-radius: 3px;
    }

    /* 图片样式 */
    img {
      display: inline-block;
      max-width: 100px;
      max-height: 100px;
      margin: 4px 8px 4px 0;
      border-radius: 4px;
      vertical-align: middle;
      object-fit: cover;
    }

    /* 文件插入样式 */
    .file-item {
      display: inline-block;
      margin: 0 4px 4px 4px;
      padding: 4px 12px;
      background-color: #f0f0f0;
      border-radius: 6px;
      font-size: 13px;
      color: #555;
      border: 1px solid #ddd;
      cursor: default;
      vertical-align: middle;

      * {
        user-select: none !important;
      }
    }

  }

}


</style>