<template>
  <div class="fill-width editor" :style="{height:height}" :class="{'hide-all-menu':hideAllMenu}">
    <v-md-editor v-model="value"
                 v-on="$listeners"
                 :disabled-menus="disabledMenus"
                 :includeLevel="includeLevel"
                 :height="height"
                 @upload-image="handleUploadImage"
                 mode="edit"
                 v-bind="$attrs"
                 left-toolbar="customToolbar h bold italic strikethrough quote ul ol table hr link image code"
                 right-toolbar=""
    ></v-md-editor>
  </div>
</template>

<script>


import {call, convertImageSrc} from "@/utils/commands.js";

export default {
  name: "md-editor",
  components: {},
  props: {
    /**
     * 高度
     */
    height: String,
    /**
     * 目录级别
     */
    includeLevel: {
      type: Array,
      default: () => {
        return [1, 2, 3, 4]
      }
    },
    /**
     * 禁用的菜单
     */
    disabledMenus: {
      type: Array,
      default: () => {
        return ['save']
      }
    },
    hideAllMenu: {
      type: Boolean,
      default: false
    },
  },
  data() {
    return {
      value: '',
    }
  },
  watch: {
    value(newVal) {
      this.$emit('update:value', newVal)
    }
  },
  mounted() {
    document.addEventListener('keydown', this.saveByShortcut)
  },
  beforeUnmount() {
    document.removeEventListener('keydown', this.saveByShortcut)
  },
  methods: {
    save() {
      this.$emit('save-content', this.value)
    },
    saveByShortcut(e) {
      const key = window.event.keyCode
      if (key === 83 && e.ctrlKey) {
        e.preventDefault()
        this.save()
      }
    },
    handleUploadImage(event, insertImage, files) {
      const file = files[0]
      const reader = new FileReader();

      reader.onload = async (e) => {
        const arrayBuffer = e.target.result;
        // 调用后端方法，传递 ArrayBuffer
        const path = await call('save_note_file', {
          noteId: 0,
          fileName: file.name,
          bytes: arrayBuffer
        })
        insertImage({
          url: convertImageSrc(path),
          desc: file.name,
        });
      };

      reader.readAsArrayBuffer(file);
    },
  }
}
</script>


<style scoped lang="scss">
:deep(.v-md-editor) {
  box-shadow: unset !important;

  textarea {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", "Liberation Sans", "Helvetica Neue", Helvetica, Tahoma, Arial, "PingFang SC", "Hiragino Sans GB", "Heiti SC", "Microsoft YaHei", "WenQuanYi Micro Hei", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
  }
}

.hide-all-menu {
  :deep(.v-md-editor__toolbar) {
    display: none;
  }
}


:deep(.v-md-textarea-editor textarea) {
  padding: 10px 0;
  //background: #fafafa;
}

:deep(.v-md-editor__toolbar) {
  border-bottom: none;

  &::-webkit-scrollbar {
    width: 0 !important;
    height: 0;
  }
}

:deep(.v-md-editor__toolbar-left) {
  //display: flex;
  //flex-wrap: nowrap;
  //overflow: auto;
  //width: 100%;
}

:deep(.v-md-editor__toolbar-right) {
  //display: flex;
  //flex-wrap: nowrap;
  //overflow: auto;
  //width: 100%;
}

:deep(.v-md-editor__tooltip) {
  //display: none;
}

:deep(.v-md-editor-preview) {
  .github-markdown-body {
    padding: 0 0;
    font-size: 14px;
    line-height: 1.8;
    color: #333;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", "Liberation Sans", "Helvetica Neue", Helvetica, Tahoma, Arial, "PingFang SC", "Hiragino Sans GB", "Heiti SC", "Microsoft YaHei", "WenQuanYi Micro Hei", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"
  }
}
</style>