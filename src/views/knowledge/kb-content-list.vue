<script setup lang="ts">
import {onMounted, onUnmounted, ref, watch} from "vue";
import {open} from '@tauri-apps/plugin-dialog'
import {call} from "../../utils/commands.ts";
import {U} from "@/utils/util.js"
import {ElMessage} from "element-plus";
import {openPath, revealItemInDir} from "@tauri-apps/plugin-opener";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const props = defineProps({
  knowledgeBase: {
    type: Object,
    required: true
  }
})

const selectedFiles = ref([])
const importRecordList = ref([])
const page = ref({pageNum: 1, pageSize: 5})
const timer = ref(null)
const loadKbImportRecordList = async () => {
  const data = await call('kb_import_record_list', {
    req: {
      kbId: props.knowledgeBase.id,
      page: page.value
    }
  })
  importRecordList.value = data.list
  page.value.total = data.total

  let parsingRecord = importRecordList.value.find(item => item.status === 2)
  if (parsingRecord && !timer.value) {
    timer.value = setInterval(async () => {
      await loadKbImportRecordList()
      let parsingRecord = importRecordList.value.find(item => item.status === 2)
      if (!parsingRecord) {
        clearInterval(timer.value)
      }
    }, 1000)
  }
}

onMounted(async () => {
  await loadKbImportRecordList()
})
const openFileDialog = async () => {
  const files = await open({
    multiple: true, // 允许选择多个文件
    filters: [
      {
        name: '文档和图片',
        extensions: ['doc', 'docx', 'pdf', 'txt', 'md', 'ppt', 'pptx', 'xls', 'xlsx', 'png', 'jpg', 'jpeg', 'webp', 'bmp'],
      },
    ],
  })

  if (Array.isArray(files)) {
    selectedFiles.value = files
  } else if (files !== null) {
    selectedFiles.value = [files]
  }

  if (selectedFiles.value.length === 0) {
    return
  }
  await call('add_kb_file', {
    kbId: props.knowledgeBase.id,
    files: selectedFiles.value
  })

  await loadKbImportRecordList()
}

const deleteRecord = async (record: object) => {
  await call('delete_kb_import_record', {
    id: record.id
  })
  await loadKbImportRecordList()
}


onUnmounted(() => {
  if (timer.value) {
    clearInterval(timer.value)
  }
})
</script>

<template>
  <el-form>
    <el-form-item :label="t('导入')">
      <div class="import-area" @click="openFileDialog">
        <el-text>{{ t('将文件拖到此处，或点击导入') }}</el-text>
      </div>
    </el-form-item>
    <el-form-item :label="t('文件')">
      <el-table :data="importRecordList">
        <el-table-column :label="t('文件名')" prop="fileName" min-width="160" show-overflow-tooltip>
          <template #default="{row}">
            <el-text @click="openPath(row.filePath)">
              {{ row.fileName }}
            </el-text>
          </template>
        </el-table-column>
        <el-table-column :label="t('大小')" prop="fileSize">
          <template #default="{row}">
            {{ U.renderSize(row.fileSize, 1) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('状态')" prop="status">
          <template #default="{row}">
            <el-text v-if="row.status === 0" type="info" disable-transitions>{{ t('待解析') }}</el-text>
            <el-text v-if="row.status === 1" type="success" disable-transitions>{{ t('成功') }}</el-text>
            <el-text v-if="row.status === 2" type="primary" disable-transitions>{{ t('解析中') }}</el-text>
            <el-tooltip :content="row.statusMsg">
              <el-text v-if="row.status === 3" type="danger" disable-transitions>{{ t('失败') }}</el-text>
            </el-tooltip>
          </template>
        </el-table-column>
        <el-table-column :label="t('操作')" width="90">
          <template #default="{row}">
            <el-popconfirm @confirm="deleteRecord(row)" :title="t('确认从知识库中删除该文件')" width="230">
              <template #reference>
                <div class="flex-v">
                  <div>
                    <el-text type="danger" :underline="false">{{ t('删除') }}</el-text>
                  </div>
                </div>
              </template>
            </el-popconfirm>
          </template>
        </el-table-column>
        <template #empty>
          {{ t('暂无数据') }}
        </template>
      </el-table>
      <el-pagination
          class="mt20"
          background
          :page-size="page.pageSize"
          :current-page="page.pageNum"
          :total="page.total"
          hide-on-single-page
          :pager-count="5"
          @current-change="(pageNum) => {page.pageNum = pageNum; loadKbImportRecordList()}"
      ></el-pagination>
    </el-form-item>
  </el-form>
</template>

<style scoped lang="scss">
.import-area {
  width: 100%;
  text-align: center;
  line-height: 70px;
  color: #ccc;
  cursor: pointer;
  border-radius: 10px;
  border: 2px dashed #c0c4cc;

  &:hover {
    border-color: var(--el-color-primary);
    background-color: var(--el-color-primary-light-11);
  }
}
</style>