<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {relaunch} from '@tauri-apps/plugin-process';

const currentVersion = ref()
const lastVersion = ref()
const isShowUpdateDialog = ref(false)
const isUpdating = ref(false)
const getCurrentVersion = () => {
  invoke('get_current_version').then(res => {
    currentVersion.value = res
  })
}
const getLatestVersion = () => {
  invoke('get_latest_version').then(res => {
    lastVersion.value = res
  })
}

onMounted(() => {
  getCurrentVersion()
  getLatestVersion()
})

const showUpdateDialog = () => {
  isShowUpdateDialog.value = true
}

const updateNow = () => {
  invoke('make_update_flag_file')
  // 重启
  relaunch()
}

const updateAfterReboot = () => {
  invoke('make_update_flag_file')
}

</script>

<template>
  <div class="fill-width">
    <div class="flex-space-between">
      <div>
        {{ currentVersion }}
      </div>
      <div>
        <el-tag v-if="currentVersion === lastVersion?.version" type="success">最新</el-tag>
        <el-tag v-if="currentVersion !== lastVersion?.version && lastVersion" type="primary" class="cursor-pointer"
                @click="showUpdateDialog">
          新版本 {{ lastVersion?.version }}
        </el-tag>
      </div>
    </div>
  </div>
  <el-dialog v-model="isShowUpdateDialog" title="升级" width="500px" draggable append-to-body>
    <el-text size="large">
      版本{{ lastVersion.version }}更新日志
    </el-text>
    <div class="bg-card br5 mt10 mb20">
      {{ lastVersion.description }}
    </div>
    <template #footer>
      <el-button type="primary" @click="updateNow" text>立即更新</el-button>
      <el-button type="primary" @click="updateAfterReboot" text>重启后更新</el-button>
    </template>
  </el-dialog>
</template>

<style scoped lang="scss">
:deep(.el-dialog__header) {
  padding-bottom: 0 !important;
}
</style>
