<script setup>
import "codemirror/mode/javascript/javascript.js";
import "codemirror/mode/python/python.js";
import Codemirror from "codemirror-editor-vue3";
import SvgIcon from "../SvgIcon/index.vue";
import {computed} from "vue";

const modeMap = {
  javascript: "text/javascript",
  python: "text/x-python",
}

const code = defineModel('code')
const language = defineModel('language')

const cmOptions = computed(() => {
  return {
    mode: modeMap[language.value || 'javascript'],
  }
})

</script>

<template>
  <div class="editor">
    <div class="flex-space-between toolbar">
      <el-radio-group v-model="language">
        <el-radio label="JS" value="javascript"></el-radio>
        <el-radio label="Python" value="python"></el-radio>
      </el-radio-group>
      <svg-icon icon-class="fullscreen"></svg-icon>
    </div>
    <Codemirror
        v-model:value="code"
        :options="cmOptions"
        border
        ref="cmRef"
        height="300"
    >
    </Codemirror>
  </div>

</template>

<style scoped lang="scss">
.editor {
  border: 1px solid #f2e3ff !important;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  background: #ffffff;
}

.codemirror-container {
  border: none !important;
}

:deep(.CodeMirror-lines) {
  line-height: 20px;
}

.toolbar {
  border-radius: 5px 5px 0 0;

  background: #ffffff;
  border-bottom: 1px solid #dcdfe6 !important;
  padding: 0 10px;
}

:deep(.CodeMirror-gutters) {
  background: #ffffff;
}
</style>