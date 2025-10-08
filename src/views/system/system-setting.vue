<script setup lang="ts">
import {ref, watch} from "vue";
import {useI18n} from "vue-i18n";
import ThemeSwitch from "@/views/system/theme-switch.vue";
import Updater from "@/views/system/updater.vue";

const {t} = useI18n()
const isShow = ref(false)

const show = () => {
  isShow.value = true
}
defineExpose({
  show
})

const {locale: i18nLanguage} = useI18n()

watch(i18nLanguage, (newLocale) => {
  localStorage.setItem('language', newLocale)
})


</script>

<template>
  <el-drawer v-model="isShow" :title="t('系统设置')" size="25vw" append-to-body destroy-on-close>
    <el-form class="mt10" label-position="left" label-width="70">
      <el-form-item :label="t('语言')">
        <el-select v-model="i18nLanguage">
          <el-option label="中文" value="zh"></el-option>
          <el-option label="English" value="en"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item :label="t('主题')">
        <theme-switch></theme-switch>
      </el-form-item>
      <el-form-item :label="t('版本')">
        <updater></updater>
      </el-form-item>
    </el-form>
  </el-drawer>
</template>

<style scoped lang="scss">

</style>