<script setup lang="ts">

import {onMounted, ref} from "vue";
import {call} from "@/utils/commands.ts";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const installedMcpServers = ref([])
const selectedMcpServers = defineModel({default: []})
const loadInstallTools = async () => {
  installedMcpServers.value = await call('list_installed_mcp_server')
}

onMounted(async () => {
  await loadInstallTools()
})
</script>

<template>
  <div class="fill-width">
    <el-select v-model="selectedMcpServers" multiple :placeholder="t('选择工具')" class="fill-width">
      <el-option v-for="item in installedMcpServers" :value="item.id" :label="item.summary">
        <div class="fill-width">
          <div class="fl">
            {{ item.summary }}
          </div>
          <div class="mr10 fr">
            <el-text type="info" size="small" style="font-weight: 500;max-width: 200px" truncated>
              {{ item.description }}
            </el-text>
          </div>
        </div>
      </el-option>
    </el-select>
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