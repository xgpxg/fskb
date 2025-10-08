<script setup lang="ts">
import {computed, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {ElMessage} from "element-plus";
import {emit} from "@tauri-apps/api/event";
import {useI18n} from "vue-i18n";

const {t} = useI18n()

const isShow = ref(false)
const form = ref({
  name: null,
  description: null,
  icon: null
})
const formRef = ref()
const rules = computed(() => {
  return {
    name: {required: true, message: t('知识库的名字是必填的'), trigger: 'blur'}
  }
})
const show = () => {
  isShow.value = true
}

const addKb = async () => {
  await formRef.value.validate()

  invoke("add_kb", {
    req: {
      name: form.value.name,
      description: form.value.description,
      icon: form.value.icon
    }
  }).then(res => {
    PubSub.publish('kb/list/refresh')
  })
  isShow.value = false
}

defineExpose({
  show
})
</script>

<template>
  <el-dialog v-model="isShow" :title="t('新建知识库')" append-to-body v-if="isShow" :modal="false" draggable
             width="400">
    <el-form ref="formRef" :model="form" :rules="rules" size="large" label-width="50" @submit="addKb">
      <el-form-item :label="t('名称')" prop="name">
        <el-input v-model="form.name" :placeholder="t('请输入知识库名称')"></el-input>
      </el-form-item>
      <el-form-item :label="t('描述')">
        <el-input v-model="form.description" type="textarea" :placeholder="t('请输入知识库描述')" :rows="3"></el-input>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="isShow = false">{{ t('取消') }}</el-button>
      <el-button type="primary" @click="addKb">{{ t('创建') }}</el-button>
    </template>
  </el-dialog>
</template>

<style scoped lang="scss">

</style>