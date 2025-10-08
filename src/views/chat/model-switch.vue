<script setup lang="ts">
import {call} from "@/utils/commands.ts";
import {nextTick, onMounted, ref, watch} from "vue";
import AutoAvatar from "../../components/avatar/AutoAvatar.vue";
import SvgIcon from "@components/SvgIcon/index.vue";
import {ElMessage} from "element-plus";

const props = defineProps({
  knowledgeBase: {
    type: Object,
    required: true
  },
})
const models = ref([])
const value = defineModel<object>({})
const loadAllAvailableModels = async () => {
  models.value = await call('all_available_models', {taskType: 1})
}

// onMounted(async () => {
//   await loadAllAvailableModels()
// })

// 监听 props.knowledgeBase 是否初始化完成
watch(
    () => props.knowledgeBase,
    async (newVal) => {
      await loadAllAvailableModels()
      if (newVal && newVal.model_id) {
        await nextTick(() => {
          value.value = models.value.find(m => m.id === newVal.model_id)
        })
      }
    },
    {deep: true, immediate: true}
);


const refresh = async () => {
  await loadAllAvailableModels()
}

const emit = defineEmits(['hide'])

const onClick = async (model: object) => {
  // 更新使用的模型
  await call('update_kb', {
    req: {
      id: props.knowledgeBase.id,
      modelId: model.id
    }
  })

  emit('hide')
  setTimeout(() => {
    value.value = model
  }, 50)

}

defineExpose({
  refresh
})

</script>

<template>
  <div class="list">
    <div class="item" v-if="models.length>0" v-for="model in models" @click="onClick(model)"
         :class="value?.id === model.id?'selected':''">
      <div class="flex-v">
        <div>
          <auto-avatar :src="model.icon" :text="model.name" size="small" transparent></auto-avatar>
        </div>
        <div class="ml10">
          <div class="ellipsis" style="max-width: 220px">{{ model.name }}</div>
          <div class="small-font" :class="value?.id !== model.id?'color-secondary':''">{{ model.description }}</div>
        </div>
      </div>
    </div>
    <div v-else>
      <div class="flex-v">
        <svg-icon icon-class="empty" size="20"></svg-icon>
        <el-text type="info" class="ml10">暂无可用模型</el-text>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.item {
  padding: 10px;
  background: #f0f0f0;
  width: 250px;
  border-radius: 5px;
  margin-bottom: 10px;

  &.selected {
    background: var(--el-color-primary); // 蓝色系选中背景
    color: white;
  }
}

</style>