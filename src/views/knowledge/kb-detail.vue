<script setup lang="ts">
import {computed, onActivated, onMounted, ref, watch} from "vue";
import KbSetting from "./kb-setting.vue";
import KbContentList from "./kb-content-list.vue";
import {useRouter} from "vue-router";
import {call} from "@/utils/commands.ts";
import {ElMessage} from "element-plus";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const props = defineProps({})

const router = useRouter()
const route = router.currentRoute
const isShow = ref(false)
const active = ref('content')
const knowledgeBase = ref({})
const knowledgeBaseId = computed(() => {
  const id = route.value.params.knowledgeBaseId;
  return parseInt(Array.isArray(id) ? id[0] : id);
})
const show = () => {
  isShow.value = true
}

const refresh = async () => {
  await loadKbDetail()
}

const loadKbDetail = async () => {
  knowledgeBase.value = await call('kb_detail', {
    id: knowledgeBaseId.value
  })
}
onMounted(async () => {
  await loadKbDetail()
})

defineExpose({
  show,
  refresh
})

</script>
<template>
  <el-drawer v-model="isShow" :title="t('知识库管理')" size="650px" :with-header="false" destroy-on-close
             @open="loadKbDetail">
    <el-page-header @back="isShow=false" :title="t('返回')">
      <template #content>
        <el-text truncated style="max-width: 300px;margin-top: 3px">{{ knowledgeBase.name }}</el-text>
      </template>
      <template #extra>
        <el-tabs v-model="active">
          <el-tab-pane :label="t('内容管理')" name="content">
          </el-tab-pane>
          <el-tab-pane :label="t('设置')" name="setting">
          </el-tab-pane>
        </el-tabs>
      </template>
    </el-page-header>

    <kb-content-list v-if="active==='content'" :knowledge-base="knowledgeBase"></kb-content-list>
    <kb-setting v-if="active==='setting'" class="pd10" :knowledge-base="knowledgeBase"></kb-setting>
  </el-drawer>
</template>

<style scoped lang="scss">
.el-tabs .el-tabs--top {
}

:deep(.el-page-header__left) {
  margin-top: -10px;
}

.el-tabs {
  --el-tabs-header-height: 30px;
  display: flex;
}
</style>