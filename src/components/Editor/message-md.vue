<template>
  <div class="fill-width">
    <template v-for="item in messages">
      <!--  深度思考区域  -->
      <think-message v-if="item.type==='think'"
                     v-model:value="item.content"
                     :expand="expandThink"
                     :thinking="!item.finished"></think-message>
      <!--  消息内容区域  -->
      <v-md-preview
          v-if="item.type==='content'"
          ref="editor"
          :text="item.content"
          height="400px"
          @image-click="imageClick">
      </v-md-preview>
    </template>
  </div>
  <!-- 图片放大预览弹窗 -->
  <el-image-viewer v-if="isShowImagePreview"
                   :url-list="previewImages"
                   show-progress
                   hide-on-click-modal
                   teleported
                   @close="isShowImagePreview = false"></el-image-viewer>
</template>

<script>


import ThinkMessage from "@components/Editor/think-message.vue";

export default {
  name: "message-md",
  components: {ThinkMessage},
  props: {
    value: String,
    enableHeader: Boolean
  },
  data() {
    return {
      isShowImagePreview: false,
      previewImages: [],
      expandThink: false,
      messages: [],
    }
  },
  watch: {
    value: {
      handler(newVal) {
        this.messages = []
        // 提取think
        const thinkReg = /<think>([\s\S]*?)<\/think>/g;
        // 提取非think
        const notThinkReg = /<\/think>([\s\S]*?)<think>/g
        // 匹配<think>个数
        const thinkCount = (this.value.match(/<think>/g) || []).length;
        // 匹配</think>个数
        const thinkEndCount = (this.value.match(/<\/think>/g) || []).length;

        const thinkMessages = [...newVal.matchAll(thinkReg)].map(match => match[1])
        const messages = [...newVal.matchAll(notThinkReg)].map(match => match[1])
        let splits = newVal.split(/<think>([\s\S]*?)<\/think>/g)
        splits.forEach((split, index) => {
          if (thinkMessages.includes(split)) {
            this.messages.push({
              type: 'think',
              content: split,
              finished: true
            })
          } else if (messages.includes(split)) {
            this.messages.push({
              type: 'content',
              content: split,
            })
          } else if (thinkCount !== thinkEndCount) {
            this.messages.push({
              type: 'think',
              content: split.replace('<think>', ''),
              finished: false,
            })
            this.expandThink = true
          } else {
            this.messages.push({
              type: 'content',
              content: split,
            })
          }
        })
      },
      deep: true,
      immediate: true
    }
  },
  computed: {},
  mounted() {
    this.init()
  },
  methods: {
    init() {
    },
    imageClick(images) {
      this.previewImages = images
      this.isShowImagePreview = true
    },
  }
}
</script>


<style scoped lang="scss">
:deep(.v-md-editor-preview) {
  .github-markdown-body {
    padding: 0 20px;
    font-size: 14px;
    line-height: 1.8;
    color: #333;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", "Liberation Sans", "Helvetica Neue", Helvetica, Tahoma, Arial, "PingFang SC", "Hiragino Sans GB", "Heiti SC", "Microsoft YaHei", "WenQuanYi Micro Hei", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
    max-width: calc(75vw - 400px);
  }

  ::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  ::-webkit-scrollbar-thumb {
    background-color: rgba(0, 0, 0, 0.1);
    border-radius: 3px;
  }

  .v-md-pre-wrapper {
    font-size: 14px;
  }

  table {
    border: #eeeeee 1px solid;
    width: 100%;
    display: table;

    tr {
      border: unset;

      th {
        font-weight: normal;
        background: #f3f3f3;
        min-width: 50px;
        border: unset;
      }

      td {
        font-size: 15px;
        border: unset;
      }
    }
  }

  a {
    &:hover {
      cursor: pointer;
    }
  }

  p {
    margin-bottom: 0;
  }

  hr {
    height: 2px;
  }


  table {
    max-width: calc(100vw - 400px - 25vw); // 限制最大宽度
    overflow-x: auto; // 超出时显示横向滚动条
    white-space: nowrap; // 防止表格内容换行
    display: inline-block;
    border-collapse: collapse;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
    transition: box-shadow 0.3s;
    margin-top: 10px;

    &:hover {
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    th,
    td {
      padding: 12px 16px;
      text-align: left;
      border-bottom: 1px solid #ebedf0;
    }

    th {
      background: linear-gradient(135deg, #f3f3f3, #ebedf0);
      font-weight: bold;
      color: #333;
    }

    tr:nth-child(even) {
      background: #f8f8f8;
    }

    tr:hover {
      background: #eef5ff;
    }

    tr:last-child td {
      border-bottom: none;
    }

    td {
      position: relative;
      width: 100%;

      &:before {
        content: '';
        position: absolute;
        left: 0;
        bottom: 0;
        width: 100%;
        height: 1px;
        background: #ebedf0;
        transform: scaleX(0);
        transition: transform 0.3s;
      }
    }

    tr:hover td:before {
      transform: scaleX(1);
    }
  }

  blockquote {
    // 基础样式
    padding: 10px 20px;
    margin: 16px 0;
    background: #f8f8f8;
    border-left: 6px solid #2f93f8;
    border-radius: 8px;
    position: relative;
    color: #333;
    font-size: 15px;
    line-height: 1.6;

    // 悬停效果
    &:hover {
      background: #eef5ff;
      border-left-color: #66b1ff;
      box-shadow: 0 2px 8px rgba(10, 132, 255, 0.15);
    }

  }

  .copy-code-mode {
    border-radius: 10px;
    margin-top: 10px;
  }

  img {
    border-radius: 6px;
    max-width: 100px;
  }

  div[data-tool] {
    background-color: #f6f6f6;
    border: 1px solid #e3e3e3;
    padding: 2px 4px;
    font-size: 12px;
    font-family: inherit;
    border-radius: 4px;
  }

  .katex-display {
    overflow-x: auto;
    padding: 6px;
    max-width: calc(100vw - 430px);
  }
}


</style>