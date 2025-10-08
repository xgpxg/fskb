<script setup lang="ts">
import {nextTick, provide, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import en from "element-plus/es/locale/lang/en";
import zh from "element-plus/es/locale/lang/zh-cn";
import {useI18n} from "vue-i18n";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", {name: name.value});
}

const isRouteAlive = ref(true);
const reload = () => {
  isRouteAlive.value = false
  nextTick(() => {
    isRouteAlive.value = true
  })
}

provide("reload", reload)


const {locale: i18nLanguage} = useI18n()

const elementLanguage = ref()
watch(i18nLanguage, (language) => {
  switch (language) {
    case 'zh':
      elementLanguage.value = zh
      break;
    case 'en':
      elementLanguage.value = en
      break;
    default:
      elementLanguage.value = zh
  }
})
</script>

<template>
  <el-config-provider :locale="elementLanguage">
    <router-view v-if="isRouteAlive"/>
  </el-config-provider>
</template>

<style scoped>

</style>
