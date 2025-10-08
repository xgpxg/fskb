<script setup lang="ts">

import {computed, onMounted, ref} from "vue";
import {call} from "@/utils/commands.ts";
import {ElMessage} from "element-plus";
import {useI18n} from "vue-i18n";
import HelpTip from "@components/Tip/HelpTip.vue";

const {t} = useI18n()
const isShow = ref(false)
const show = () => {
  isShow.value = true
}
defineExpose({
  show
})

const userProfile = ref({
  enableProfileMemory: 0,
  profileMemoryModelId: null,
})
const rules = computed(() => {
  return {
    profileMemoryModelId:
        {
          required: true,
          message: '请选择知识库模型',
          trigger: 'change'
        }
  }
})
const models = ref([])

onMounted(() => {
  loadAllAvailableModels()
  loadUserProfile()
})


const loadAllAvailableModels = async () => {
  models.value = await call('all_available_models', {taskType: 1})
}
const loadUserProfile = async () => {
  userProfile.value = await call('get_user_profile')
}

const updateUserProfile = async () => {
  await call('update_user_profile', {
    req: userProfile.value
  })
  ElMessage.success(t('已保存'))
}
</script>

<template>
  <el-drawer v-model="isShow" title="个性化" append-to-body direction="ltr" size="250px"
             @open="loadUserProfile();loadAllAvailableModels()">
    <el-form :model="userProfile" :rules="rules" label-width="80px" label-position="left" class="mt10"
             require-asterisk-position="right">
      <div class="title-block">
        记忆设置
      </div>
      <el-form-item prop="enableProfileMemory">
        <template #label>
          记住我
          <help-tip width="250px" class="ml5">
            开启后，系统将根据您与AI的对话实时总结您关键信息，这有助于大模型快速了解您的个人偏好信息，并给出更准确的回复。这些信息将在您电脑上加密存储，不会外泄。
          </help-tip>
        </template>
        <el-switch v-model="userProfile.enableProfileMemory" :active-value="1" :inactive-value="0"></el-switch>

      </el-form-item>
      <el-form-item label="提取模型" prop="profileMemoryModelId" v-if="userProfile.enableProfileMemory">
        <el-select v-model="userProfile.profileMemoryModelId" placeholder="选择记忆提取模型">
          <el-option v-for="item in models" :key="item.id" :label="item.name" :value="item.id"></el-option>
        </el-select>
      </el-form-item>
    </el-form>
    <div class="fill-width">
      <el-button type="primary" @click="updateUserProfile" class="fill-width">{{ t('保存') }}</el-button>
    </div>
  </el-drawer>
</template>

<style scoped lang="scss">

</style>