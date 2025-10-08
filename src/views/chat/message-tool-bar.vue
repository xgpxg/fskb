<script setup>

import SvgIcon from "../../components/SvgIcon/index.vue";
import {ref} from "vue";
import {R} from "../../utils/R";

const props = defineProps({
  sessionId: {
    type: String,
    required: true
  },
  messageId: {
    type: Number,
    required: true
  }
})
const isShowExport = ref(false);
const isExporting = ref(false);
const exportForm = ref({
  format: 'pdf'
})
const exportMessage = () => {
  isExporting.value = true
  R.postJson('/chat/export-message', {
    sessionId: props.sessionId,
    messageId: props.messageId,
    format: exportForm.value.format
  }).then(res => {
    if (res.code === 0) {
      const link = document.createElement('a');
      link.href = res.data.url;
      link.download = res.data.url.substring(res.data.url.lastIndexOf('/') + 1).substring(20);
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);

      isExporting.value = false
      isShowExport.value = false
    }
  })
}
</script>

<template>
  <div class="tool-bar flex-v">
    <!--    <svg-icon icon-class="like" size="22" class="mr20 cursor-pointer"></svg-icon>-->
    <svg-icon icon-class="export" size="22" class="mr20 cursor-pointer" @click="isShowExport = true"></svg-icon>
  </div>
  <el-dialog v-model="isShowExport" title="导出回复" append-to-body width="660px" :modal="false" draggable>
    <el-form label-width="60">
      <el-form-item label="导出为">
        <div class="export-format-item" style="width: 100px" @click="exportForm.format='pdf'"
             :class="exportForm.format==='pdf'?'export-format-selected':''">
          <svg-icon icon-class="pdf" class="mr10" size="26"></svg-icon>
          <el-text size="large">PDF</el-text>
        </div>
        <div class="export-format-item" style="width: 100px" @click="exportForm.format='docx'"
             :class="exportForm.format==='docx'?'export-format-selected':''">
          <svg-icon icon-class="doc" class="mr10" size="26"></svg-icon>
          <el-text size="large">DOCX</el-text>
        </div>
        <div class="export-format-item" style="width: 100px" @click="exportForm.format='html'"
             :class="exportForm.format==='html'?'export-format-selected':''">
          <svg-icon icon-class="html" class="mr10" size="26"></svg-icon>
          <el-text size="large">HTML</el-text>
        </div>
        <div class="export-format-item" style="width: 100px" @click="exportForm.format='md'"
             :class="exportForm.format==='md'?'export-format-selected':''">
          <svg-icon icon-class="md" class="mr10" size="26"></svg-icon>
          <el-text size="large">MD</el-text>
        </div>
      </el-form-item>
      <el-form-item>
        <div v-if="isExporting">
          <el-text>
            <el-icon class="is-loading">
              <Loading/>
            </el-icon>
            正在导出，请稍后...
          </el-text>
        </div>
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="isShowExport = false">取消</el-button>
      <el-button type="primary" @click="exportMessage" :loading="isExporting">导出</el-button>
    </template>
  </el-dialog>
</template>

<style scoped lang="scss">
.tool-bar {
  margin-top: 10px;
  //float: right;
  //margin-right: 20px;
}

.export-format-item {
  padding: 10px;
  margin-right: 10px;
  margin-bottom: 10px;
  border-radius: 5px;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  border: 1px #e7e7e7 solid;
}

.export-format-selected {
  background: var(--el-color-primary-light-10);
}
</style>