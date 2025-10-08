<script setup lang="ts">

import Note from "@/views/note/note.vue";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
</script>

<template>
  <el-tabs>
    <el-tab-pane :label="t('笔记')">
      <Note></Note>
    </el-tab-pane>
  </el-tabs>
</template>

<style scoped lang="scss">
:deep(.el-tabs__header) {
  margin-bottom: 0;
}
</style>