<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import AutoAvatar from "../../components/avatar/AutoAvatar.vue";
import {call} from "../../utils/commands.ts";
import {ElMessage} from "element-plus";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const isShow = ref(false)
const installedMcpServers = ref([])
const allMcpServers = ref([])
const form = ref({})
const formRef = ref({})
const isShowAdd = ref(false)
const isShowUpdate = ref(false)
const rules = computed(() => {
  return {
    summary: {required: true, message: t('请填写工具名称'), trigger: 'blur'},
    config: {required: true, message: t('请填写工具配置'), trigger: 'blur'},
  }
})
const show = () => {
  isShow.value = true
}
defineExpose({
  show
})

const loadAllMcpServers = async () => {
  allMcpServers.value = await call("list_all_mcp_server", {})
}
const loadInstalledMcpServers = async () => {
  installedMcpServers.value = await call("list_installed_mcp_server", {})
}
onMounted(async () => {
  await loadAllMcpServers()
  await loadInstalledMcpServers()
  refreshOfflineStatus()
})

const installMcpServer = async (item) => {
  let timer = setInterval(() => {
    loadInstalledMcpServers()
  }, 1000)

  item.installing = true
  await call("install_mcp_server", {name: item.name})
  await loadInstalledMcpServers()
  await loadAllMcpServers()
  clearInterval(timer)

  item.installing = false
}

const uninstallMcpServer = async (item) => {
  await call("uninstall_mcp_server", {name: item.name})
  await loadInstalledMcpServers()
  await loadAllMcpServers()
}

const runMcpServer = async (item) => {
  item.status = 4
  refreshOfflineStatus()
  await call("run_mcp_server", {name: item.name})
  await loadInstalledMcpServers()
}
const stopMcpServer = async (item) => {
  await call("stop_mcp_server", {name: item.name})
  await loadInstalledMcpServers()
}

const addMcpServer = async () => {
  if (!(await formRef.value.validate())) {
    return
  }
  const name = Object.keys(JSON.parse(form.value.config)['mcpServers'])[0]
  await call("add_mcp_server", {req: {...form.value, name}})
  await loadInstalledMcpServers()
  isShowAdd.value = false
}
const updateMcpServer = async () => {
  if (!(await formRef.value.validate())) {
    return
  }
  await call("update_mcp_server", {req: {...form.value}})
  await loadInstalledMcpServers()
  isShowUpdate.value = false
}

const refreshOfflineStatus = () => {
  if (allMcpServers.value.find(m => [3, 4, 5, 6].includes(m.status))) {
    let timer = setInterval(async () => {
      await loadAllMcpServers()
      if (!allMcpServers.value.find(m => [3, 4, 5, 6].includes(m.status))) {
        clearInterval(timer)
      }
    }, 3000)
  }
}
</script>

<template>
  <el-drawer v-model="isShow" :title="t('工具管理')" size="650px" append-to-body @open="loadAllMcpServers()">
    <div class="title-block">
      {{ t('已安装') }}
    </div>
    <div class="list">
      <el-row :gutter="20">
        <el-col :span="12" v-for="item in installedMcpServers">
          <div class="item">
            <div class="flex">
              <auto-avatar :src="item.icon" :text="item.summary" size="small"></auto-avatar>
              <div class="ellipsis ml10" :title="item.summary">
                <el-text>{{ item.summary }}</el-text>
              </div>
            </div>
            <div class="ellipsis">
              <el-text type="info" size="small">{{ item.description ?? t('暂无描述') }}</el-text>
            </div>
            <div class="action flex-space-between">
              <div>
                <el-text class="mr20" v-if="item.status===0" size="small" type="warning">{{ t('未启用') }}</el-text>
                <el-text class="mr20" v-if="item.status===1" size="small" type="success">{{ t('运行中') }}</el-text>
                <el-tooltip v-if="item.status===2" :content="item.statusMsg">
                  <el-text class="mr20" size="small" type="danger">{{ t('异常') }}</el-text>
                </el-tooltip>
                <el-text class="mr20" v-if="item.status===3" size="small" type="primary">{{ t('正在安装') }}</el-text>
                <el-text class="mr20" v-if="item.status===4" size="small" type="primary">{{ t('启动中') }}</el-text>
                <el-text class="mr20" v-if="item.status===5" size="small" type="primary">{{ t('停止中') }}</el-text>
                <el-text class="mr20" v-if="item.status===6" size="small" type="primary">{{ t('升级中') }}</el-text>
              </div>
              <div>
                <el-button size="small" @click="runMcpServer(item)" v-if="[0,2].indexOf(item.status)>-1"
                           icon="VideoPlay"
                           text></el-button>
                <el-button size="small" @click="stopMcpServer(item)" v-if="[1,4].indexOf(item.status)>-1"
                           icon="VideoPause"
                           text></el-button>
                <el-button size="small" @click="form = item;isShowUpdate=true" icon="setting" text></el-button>
                <el-popconfirm title="确定删除" @confirm="uninstallMcpServer(item)">
                  <template #reference>
                    <el-button size="small" icon="delete" text></el-button>
                  </template>
                </el-popconfirm>
              </div>
            </div>
          </div>
        </el-col>
        <el-col :span="12">
          <div class="item flex-column-v  flex-center" @click="form={};isShowAdd=true">
            <div class="name flex-center">
              <el-text size="large">+</el-text>
            </div>
            <div class="flex-center">
              <el-text type="info">{{ t('添加新工具') }}</el-text>
            </div>
          </div>
        </el-col>
      </el-row>
    </div>

    <div class="title-block">
      {{ t('可用工具') }}
    </div>
    <div class="list">
      <el-row :gutter="20">
        <el-col :span="12" v-for="item in allMcpServers">
          <div class="item">
            <div class="flex">
              <auto-avatar :src="item.icon" :text="item.summary" size="small"></auto-avatar>
              <div class="ellipsis ml10" :title="item.summary">
                <el-text>{{ item.summary }}</el-text>
              </div>
            </div>
            <div class="summary ellipsis">
              <el-text type="info" size="small">{{ item.description ?? t('暂无描述') }}</el-text>
            </div>
            <div class="action flex-space-between">
              <div>

              </div>
              <div>
                <el-button v-if="!item.installed" size="small" @click="installMcpServer(item)" icon="download"
                           text :loading="item.installing">
                  <template v-if="!item.installing">{{ t('安装') }}</template>
                  <template v-if="item.installing">{{ t('正在安装') }}...</template>
                </el-button>
                <el-button v-else-if="item.installed && item.version != item.installedVersion" size="small"
                           @click="installMcpServer(item)" icon="download"
                           text>
                  更新
                </el-button>
                <el-button v-else size="small" text>
                  {{ t('已安装') }}
                </el-button>
              </div>
            </div>
          </div>
        </el-col>
      </el-row>
    </div>
  </el-drawer>
  <el-drawer v-model="isShowAdd" :title="t('添加新工具')" size="400px" append-to-body>
    <div>
      <el-form ref="formRef" :model="form" label-width="50" :rules="rules">
        <el-form-item :label="t('名称')" prop="summary">
          <el-input v-model="form.summary" :placeholder="t('请填写工具名称')"></el-input>
        </el-form-item>
        <el-form-item :label="t('描述')" prop="description">
          <el-input v-model="form.description" :placeholder="t('请填写工具描述')"></el-input>
        </el-form-item>
        <el-form-item :label="t('配置')" prop="config">
          <el-input v-model="form.config" type="textarea" rows="10"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="addMcpServer">{{ t('保存') }}</el-button>
        </el-form-item>
      </el-form>
    </div>
  </el-drawer>
  <el-drawer v-model="isShowUpdate" :title="t('修改工具')" size="400px" append-to-body>
    <div>
      <el-form ref="formRef" :model="form" label-width="80" :rules="rules">
        <el-form-item :label="t('名称')" prop="summary">
          <el-input v-model="form.summary" :placeholder="t('工具名称')"></el-input>
        </el-form-item>
        <el-form-item :label="t('描述')" prop="description">
          <el-input v-model="form.description" :placeholder="t('工具描述')"></el-input>
        </el-form-item>
        <el-form-item :label="t('配置')" prop="config">
          <el-input v-model="form.config" type="textarea" rows="10"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="updateMcpServer" type="primary">保存</el-button>
        </el-form-item>
      </el-form>
    </div>
  </el-drawer>
</template>

<style scoped lang="scss">
.item {
  padding: 10px;
  background: #f0f0f0;
  border-radius: 5px;
  margin-bottom: 20px;
  height: 80px;

  .action {
    margin-top: 5px;
  }
}
</style>