<script setup lang="ts">

import {onMounted, ref} from "vue";
import {call} from "@/utils/commands.ts";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const models = ref([])
const selectedMcpServers = defineModel({default: []})
const fileContentExtractType = defineModel<object>('fileContentExtractType')
const loadVLModels = async () => {
  models.value = await call('all_available_models', {
    taskType: 2
  })
  if (models.value.length === 0) {
    fileContentExtractType.value.model_id = null
  }
}

onMounted(async () => {
  await loadVLModels()
})


</script>

<template>
  <div class="fill-width">
    <el-form-item>
      <el-radio-group v-model="fileContentExtractType.type">
        <el-radio :label="t('仅文本')" value="text"></el-radio>
        <el-radio :label="t('内置OCR')" value="ocr"></el-radio>
        <el-radio :label="t('视觉模型')" value="vision_model"></el-radio>
      </el-radio-group>
    </el-form-item>
    <el-form-item v-if="fileContentExtractType.type === 'vision_model'"
                  prop="file_content_extract_type.model_id"
                  :rules="{required: true,message: t('请选择模型')}">
      <el-select
          v-model="fileContentExtractType.model_id"
          :placeholder="t('选择模型')"
          class="fill-width">
        <el-option v-for="item in models" :value="item.id" :label="item.name">
          <div class="fill-width">
            <div class="fl">
              {{ item.name }}
            </div>
            <div class="mr10 fr">
              <el-text type="info" size="small" style="font-weight: 500;max-width: 200px" truncated>
                {{ item.description }}
              </el-text>
            </div>
          </div>
        </el-option>
      </el-select>
    </el-form-item>


    <!--    <el-row :gutter="10" class="fill-width mt10">
          <el-col :span="12" v-for="item in selectedMcpServers">
            <div class="tool-card">
              <div class="name">{{ item.summary }}</div>
              <div class="desc">{{ item.description }}</div>
            </div>
          </el-col>
        </el-row>-->

  </div>
</template>

<style scoped lang="scss">
.tool-card {
  background: #f0f0f0;
  border-radius: 5px;
  padding: 10px;
  line-height: 20px;
  margin-bottom: 10px;

  .name {
    color: #333333;
    font-size: 14px;
    font-weight: bold;
  }

  .desc {
    font-size: 12px;
  }
}
</style>