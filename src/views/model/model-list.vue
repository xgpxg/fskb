<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import SingleImageUpload from "../../components/Upload/single-image-upload.vue";
import {call} from "../../utils/commands.ts";
import AutoAvatar from "../../components/avatar/AutoAvatar.vue";
import {ElMessage} from "element-plus";
import {U} from "../../utils/util";
import {useI18n} from "vue-i18n";

const {t} = useI18n()

const isShow = ref(false)
const modelList = ref([])
const form = ref({})
const formRef = ref()
const validateUrl = (rule, value, callback) => {
  if (!value) {
    return callback(new Error(t('请填写模型API')))
  }
  if (!U.checkUrl(value)) {
    return callback(new Error(t('请填写正确的API地址')))
  }
  callback();
}
const rules = computed(() => {
  return {
    name: {required: true, message: t('请填写模型名称'), trigger: 'blur'},
    baseUrl: {required: true, trigger: 'blur', validator: validateUrl},
    taskType: {required: true, message: t('请选择模型适用的任务类型'), trigger: 'blur'},
  }
})
const isShowAdd = ref(false)

const show = () => {
  isShow.value = true
}
defineExpose({
  show
})

const loadModelList = async () => {
  modelList.value = await call('list_all_models', null, {showError: false})
}
onMounted(async () => {
  await loadModelList()
  await listAllOfflineModels()
  refreshOfflineStatus()
})

const saveModel = async () => {
  if (!(await formRef.value.validate())) {
    return
  }
  if (form.value.id) {
    await call('update_model', {req: {...form.value}})
  } else {
    await call('add_model', {req: {...form.value}})
  }
  isShowAdd.value = false
  await loadModelList()
}

const deleteModel = async (item) => {
  await call('delete_model', {req: {id: item.id}})
  await loadModelList()
}

const updateStatus = async (item, status) => {
  await call('update_model', {req: {id: item.id, status: status}})
  await loadModelList()
}


const offlineModels = ref([])

enum ModelStatus {
  Disable = 0,
  Enable = 1,
  Error = 2,
  Installing = 3,
  Starting = 4,
}

const listAllOfflineModels = async () => {
  offlineModels.value = await call('list_all_offline_models', null, {showError: false})
}
const installOfflineModel = async (item: any) => {
  item.status = ModelStatus.Installing
  await call('install_offline_model', {name: item.name})
  refreshOfflineStatus()
}
const runOfflineModel = async (item: any) => {
  item.status = ModelStatus.Starting
  await call('run_offline_model', {name: item.name})
  await loadModelList()
}
const stopOfflineModel = async (item: any) => {
  await call('stop_offline_model', {name: item.name})
  await loadModelList()
}
const uninstallOfflineModel = async (item: any) => {
  await call('uninstall_offline_model', {name: item.name})
  await loadModelList()
}

const refreshOfflineStatus = () => {
  if (offlineModels.value.find(m => [ModelStatus.Installing, ModelStatus.Starting].includes(m.status))) {
    let timer = setInterval(async () => {
      await loadModelList()
      await listAllOfflineModels()
      if (!offlineModels.value.find(m => [ModelStatus.Installing, ModelStatus.Starting].includes(m.status))) {
        clearInterval(timer)
      }
    }, 3000)
  }
}
</script>

<template>
  <el-drawer v-model="isShow" :title="t('模型配置')" size="650px" append-to-body>
    <div class="list">
      <el-row :gutter="20">
        <el-col :span="12" v-for="item in modelList">
          <div class="item">
            <div class="flex-space-between">
              <div class="flex">
                <auto-avatar :src="item.icon" :text="item.name" size="small" transparent></auto-avatar>
                <div class="ellipsis ml10" :title="item.name">
                  <el-text truncated style="max-width: 175px">{{ item.name }}</el-text>
                </div>
              </div>
              <div style="margin-top: -6px">
                <el-tag v-if="item.taskType===1" size="small" effect="plain" type="info" round>LLM</el-tag>
                <el-tag v-if="item.taskType===2" size="small" effect="plain" type="info" round>VL</el-tag>
              </div>
            </div>
            <div>
              <el-text type="info" size="small" v-if="item.description" truncated style="max-width: 250px">
                {{ item.description }}
              </el-text>
              <el-text type="info" size="small" v-else>{{ t('暂无描述') }}</el-text>
            </div>
            <div class="action flex-space-between">
              <div>
                <el-text class="mr20" v-if="item.source===1" size="small" type="info">{{ t('内置模型') }}</el-text>
                <el-text class="mr20" v-if="item.source===2" size="small" type="info">{{ t('自建模型') }}</el-text>
                <el-text class="mr20" v-if="item.source===3" size="small" type="info">{{ t('离线模型') }}</el-text>
                <el-text class="mr20" v-if="item.status===0" size="small" type="warning">{{ t('未启用') }}</el-text>
                <el-text class="mr20" v-if="item.status===1" size="small" type="success">{{ t('运行中') }}</el-text>
                <el-text class="mr20" v-if="item.status===2" size="small" type="danger" :title="item.statusMsg">
                  {{ t('异常') }}
                </el-text>
                <el-text class="mr20" v-if="item.status===3" size="small" type="primary">{{ t('正在安装') }}</el-text>
              </div>
              <div v-if="[1,2].indexOf(item.source)>-1">
                <el-button size="small" @click="updateStatus(item,1)" v-if="item.status === 0" icon="VideoPlay"
                           text></el-button>
                <el-button size="small" @click="updateStatus(item,0)" v-if="item.status === 1" icon="VideoPause"
                           text></el-button>
                <el-button size="small" @click="form = item;isShowAdd=true" icon="edit" text></el-button>
                <el-popconfirm :title="t('确定删除')" @confirm="deleteModel(item)">
                  <template #reference>
                    <el-button size="small" icon="delete" text></el-button>
                  </template>
                </el-popconfirm>
              </div>
              <!-- 离线模型操作 -->
              <div v-if="item.source===3">
                <el-button size="small" @click="runOfflineModel(item)" v-if="item.status === 0" icon="VideoPlay"
                           text></el-button>
                <el-button size="small" @click="stopOfflineModel(item)" v-if="item.status === 1" icon="VideoPause"
                           text></el-button>
                <el-button size="small" @click="form = item;isShowAdd=true" icon="edit" text></el-button>
                <el-popconfirm :title="t('确定删除')" @confirm="uninstallOfflineModel(item)">
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
              <el-text type="info">{{ t('添加新模型') }}</el-text>
            </div>
          </div>
        </el-col>
      </el-row>
    </div>
  </el-drawer>

  <el-drawer v-model="isShowAdd" :title="form.id?t('修改模型'):t('添加模型')" size="400px" append-to-body
             @open="listAllOfflineModels">
    <el-tabs>
      <el-tab-pane :label="t('线上模型')">
        <el-form ref="formRef" :model="form" label-width="85" :rules="rules" class="mt10">
          <el-form-item :label="t('模型名称')" prop="name">
            <el-input v-model="form.name" :placeholder="t('请填写模型名称')" maxlength="100" show-word-limit></el-input>
            <el-text type="info" size="small" style="line-height: 16px;margin-top: 5px">
              {{ t('模型名称需全局唯一，请向您的模型服务提供商获取模型名称，一般为模型服务商提供的唯一模型ID') }}
            </el-text>
          </el-form-item>
          <el-form-item :label="t('任务类型')" prop="taskType">
            <el-select v-model="form.taskType" :placeholder="t('请选择模型适用的任务类型')">
              <el-option :label="t('文本生成')" :value="1"></el-option>
              <el-option :label="t('视觉问答')" :value="2"></el-option>
            </el-select>
          </el-form-item>
          <el-form-item :label="t('图标')" prop="icon">
            <single-image-upload :size="50" v-model:value="form.icon"></single-image-upload>
          </el-form-item>
          <el-form-item :label="t('描述')" prop="description">
            <el-input v-model="form.description" :placeholder="t('请填写模型描述，可选')" maxlength="50"
                      show-word-limit></el-input>
          </el-form-item>
          <el-form-item :label="t('API')" prop="baseUrl">
            <el-input v-model="form.baseUrl" :placeholder="t('请填写模型API')"></el-input>
            <el-text type="info" size="small" style="line-height: 16px;margin-top: 5px">
              {{ t('支持OpenAI规范的任何API，请向您的模型服务提供商获取API，或填写您自建模型的API地址') }}
            </el-text>
          </el-form-item>
          <el-form-item label="API KEY" prop="apiKey">
            <el-input v-model.trim="form.apiKey" :placeholder="t('请填写模型的API KEY')" type="password"
                      show-password></el-input>
          </el-form-item>
          <el-form-item>
            <el-button @click="saveModel">{{ t('保存') }}</el-button>
          </el-form-item>
        </el-form>
      </el-tab-pane>
      <el-tab-pane :label="t('离线模型')" lazy>
        <el-row :gutter="10" class="mt10">
          <el-col :span="12" v-for="item in offlineModels">
            <div class="bg-card br5 mb5">
              <div class="flex-space-between">
                <el-text>{{ item.name }}</el-text>
                <div style="margin-top: -6px">
                  <el-text v-if="item.taskType===1" size="small">LLM</el-text>
                  <el-text v-if="item.taskType===2" size="small">VL</el-text>
                </div>
              </div>
              <div>
                <el-text type="info" size="small">{{ item.description }}</el-text>
              </div>
              <div class="mt10">
                <el-button v-if="item.status === -1"
                           @click="installOfflineModel(item)"
                           type="primary" text size="small" icon="download">
                  {{ t('安装') }}
                </el-button>
                <el-button v-if="[0,1].indexOf(item.status)>-1" type="info" text size="small">{{ t('已安装') }}
                </el-button>
                <el-button v-if="item.status===3" type="primary" text size="small" loading>{{ t('安装中') }}</el-button>
                <el-button v-if="item.status===4" type="primary" text size="small" loading>{{ t('启动中') }}</el-button>
              </div>
            </div>
          </el-col>
        </el-row>

      </el-tab-pane>
    </el-tabs>
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

:deep(.el-tabs__header) {
  position: absolute;
  top: 10px;
  right: 50px;
}

</style>