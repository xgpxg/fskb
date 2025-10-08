<script setup lang="ts">
import {useRouter} from "vue-router";
import {ref} from "vue";
import {useI18n} from "vue-i18n";
const { t } = useI18n()

const router = useRouter()
const q = ref(null)
const toSearchPage = () => {
  router.push({
    name: 'Search',
    query: {
      q: q.value
    }
  })
}
</script>

<template>
  <el-input v-model="q" prefix-icon="search"  :placeholder="t('搜索内容')" @keydown.enter="toSearchPage"></el-input>

</template>

<style scoped lang="scss">
:deep(.el-input__wrapper) {
  box-shadow: unset !important;
  background: #f0f0f0;
}
</style>