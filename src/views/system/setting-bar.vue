<script setup lang="ts">
import {ref} from "vue";
import SvgIcon from "../../components/SvgIcon/index.vue";
import KbAdd from "../knowledge/kb-add.vue";
import ModelList from "../model/model-list.vue";
import ToolList from "../tool/tool-list.vue";
import SystemSetting from "./system-setting.vue";

const kbAddRef = ref(null);
const modelListRef = ref(null);
const toolListRef = ref(null);
const systemSettingRef = ref(null);
const showAddKb = () => {
  kbAddRef.value.show()
}
const showModelList = () => {
  modelListRef.value.show()
}
const showToolList = () => {
  toolListRef.value.show()
}
const showSystemSetting = () => {
  systemSettingRef.value.show()
}
</script>

<template>
  <div class="pdb5 pdl20 pdr20 flex-space-between ml10 mr10">
    <el-button @click="showAddKb" text type="text" id="step1">
      <svg-icon icon-class="plus" size="20" color="var(--el-color-primary)"></svg-icon>
    </el-button>
    <el-button @click="showModelList" text type="text" id="step2">
      <svg-icon icon-class="model" size="20"></svg-icon>
    </el-button>
    <el-button @click="showToolList" text type="text" id="step3">
      <svg-icon icon-class="mcp" size="20"></svg-icon>
    </el-button>
    <el-button @click="showSystemSetting" text type="text">
      <svg-icon icon-class="setting" size="20"></svg-icon>
    </el-button>
  </div>
  <kb-add ref="kbAddRef"></kb-add>
  <model-list ref="modelListRef"></model-list>
  <tool-list ref="toolListRef"></tool-list>
  <system-setting ref="systemSettingRef"></system-setting>
</template>

<style scoped lang="scss">

</style>