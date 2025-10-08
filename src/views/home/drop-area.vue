<script setup lang="ts">
import {open} from "@tauri-apps/plugin-dialog";
import {call} from "@/utils/commands.ts";
import {ref} from "vue";
import {useI18n} from "vue-i18n";

const {t} = useI18n()

const selectedFiles = ref([])
const openFileDialog = async () => {
  const files = await open({
    multiple: true, // 允许选择多个文件
    filters: [
      {
        name: 'All Files',
        extensions: ['*'],
      },
    ],
  })

  if (Array.isArray(files)) {
    selectedFiles.value = files
  } else if (files !== null) {
    selectedFiles.value = [files]
  }

  if (selectedFiles.value.length === 0) {
    return
  }

  PubSub.publish('kb/drop/files', {
    files: selectedFiles.value
  })
}
</script>

<template>
  <div id="step4" class="drop-area" @click="openFileDialog">
    {{ t('将文件拖到此处，或点击导入')}}
  </div>
</template>

<style scoped lang="scss">
.drop-area {
  width: 100%;
  height: 100px;
  border: 2px dashed #c0c4cc;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #fff;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;

  &:hover {
    border-color: var(--el-color-primary);
    background-color: var(--el-color-primary-light-11);
  }

  .drop-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    .upload-icon {
      font-size: 36px;
      color: #c0c4cc;
      margin-bottom: 10px;
      transition: all 0.3s ease;
    }

    .drop-text {
      color: #606266;
      font-size: 14px;
      transition: all 0.3s ease;
    }
  }

  &:hover {
    .upload-icon {
      color: #409eff;
      transform: translateY(-3px);
    }

    .drop-text {
      color: #409eff;
      font-weight: 500;
    }
  }
}
</style>