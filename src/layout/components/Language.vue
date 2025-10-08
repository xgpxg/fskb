<script setup lang="ts">

import {useI18n} from "vue-i18n";
import {onMounted} from "vue";

const {locale: i18nLanguage} = useI18n()

// 获取系统/浏览器语言
const getSystemLanguage = () => {
  return navigator.language || navigator.languages[0] || 'en'
}

// 获取简化语言代码（如 zh-CN -> zh）
const getSimpleLanguageCode = () => {
  const systemLang = getSystemLanguage()
  return systemLang.split('-')[0]
}


onMounted(() => {
  const lang = localStorage.getItem('language')
  if (lang) {
    i18nLanguage.value = lang
  } else {
    const systemLang = getSimpleLanguageCode()
    localStorage.setItem('language', systemLang)
  }
})


</script>

<template>

</template>

<style scoped lang="scss">

</style>