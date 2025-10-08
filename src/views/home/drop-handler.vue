<script setup lang="ts">
import {getCurrentWebview} from "@tauri-apps/api/webview";
import {computed, onBeforeUnmount, onMounted, ref, watch} from "vue";
import {sep} from "@tauri-apps/api/path";
import SvgIcon from "@components/SvgIcon/index.vue";
import {call, convertImageSrc} from "@/utils/commands.ts";
import {ElMessage} from "element-plus";
import {useRoute} from "vue-router";
import {invoke} from "@tauri-apps/api/core";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const isShow = ref(false)
const files = ref([])
const separator = sep()
const kbs = ref([])
const route = useRoute()
const currKnowledgeBaseId = computed(() => {
  const id = route.params.knowledgeBaseId;
  return parseInt(Array.isArray(id) ? id[0] : id);
})
let unlisten = null
const rules = {
  kbId: {required: true, message: '请选择知识库'}
}
const form = ref({
  kbId: null
})
const formRef = ref()

const listAllKb = async () => {
  kbs.value = await call('list_all_kb')
  form.value.kbId = kbs.value.find(kb => kb.id === currKnowledgeBaseId.value)?.id
}


onMounted(async () => {
  await listAllKb()

  unlisten = await getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'over') {
      //console.log('User hovering', event.payload.position);
      // const elements = document.elementsFromPoint(event.payload.position.x, event.payload.position.y);
      // elements.forEach(el => {
      //   el.classList.remove('drop-active');
      // })
      // elements[0].classList.add('drop-active');
    } else if (event.payload.type === 'drop') {
      //console.log('User dropped', event.payload.paths);
      // const elements = document.elementsFromPoint(event.payload.position.x, event.payload.position.y);
      // elements[0].classList.add('drop-active');
      // console.log(elements)
      if (event.payload.paths.length === 0) {
        return
      }
      event.payload.paths.forEach(path => {
        files.value.push(convertToFileInfo(path))
      })
      isShow.value = true
    } else {
      console.log('File drop cancelled');
    }
  });

  // 监听从欢迎页面点击导入的文件
  PubSub.unsubscribe('kb/drop/files')
  PubSub.subscribe('kb/drop/files', (msg: string, data: any) => {
    data.files.forEach(file => {
      files.value.push(convertToFileInfo(file))
      isShow.value = true
    })
  })
})

const convertToFileInfo = (path: string) => {
  let suffix = path.substring(path.lastIndexOf('.') + 1)
  let isImage = ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp', 'svg'].includes(suffix)
  let isDoc = ['pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'txt', 'md'].includes(suffix)
  return {
    name: path.substring(path.lastIndexOf(separator) + 1),
    path: path,
    icon: suffix,
    isImage: isImage,
    isDoc: isDoc,
  }
}


onBeforeUnmount(() => {
  if (unlisten) {
    unlisten()
  }
})

// 初始化，每次打开弹窗时都执行
const init = async () => {
  await listAllKb()
}

// 监听isShow变化，触发初始化
watch(isShow, (value) => {
  init()
})

// 弹窗关闭时重置数据
const reset = () => {
  files.value = []
  isShow.value = false
  form.value = {
    kbId: null
  }
}

const removeFile = (index: number) => {
  files.value.splice(index, 1)
  if (files.value.length === 0) {
    reset()
  }
}

const addKbFiles = async () => {
  await formRef.value.validate()

  await call('add_kb_file', {
    kbId: form.value.kbId,
    files: files.value.map(file => file.path)
  })
  ElMessage.success({
    message: `已添加${files.value.length}个文件到知识库`,
    plain: true
  })
  reset()
}

</script>

<template>
  <el-dialog v-model="isShow" :title="t('添加到知识库')" width="520px" :with-header="false" destroy-on-close @close="reset"
             :close-on-click-modal="false" draggable append-to-body>
    <div class="bg-card br5 ">
      <div class="flex-space-between mt5" v-for="(item, index) in files" :key="index">
        <div class="flex">
          <div style="width: 20px">
            <el-image v-if="item.isImage" :src="convertImageSrc(item.path)"></el-image>
            <svg-icon v-else-if="item.isDoc" :icon-class="item.icon" size="20"></svg-icon>
            <svg-icon v-else icon-class="question" size="20"></svg-icon>
          </div>
          <div class="ml10">{{ item.name }}</div>
        </div>
        <div>
          <div class="ml10">
            <el-button @click="removeFile(index)" text type="text" icon="delete"></el-button>
          </div>
        </div>
      </div>
    </div>
    <div class=" mt10">
      <el-form ref="formRef" :model="form" :rules="rules">
        <el-form-item prop="kbId">
          <el-select v-model="form.kbId" :placeholder="t('选择知识库')" filterable>
            <el-option v-for="(kb, index) in kbs" :key="index" :label="kb.name" :value="kb.id"></el-option>
          </el-select>
        </el-form-item>
      </el-form>
    </div>
    <template #footer>
      <el-button @click="isShow = false">{{t('取消')}}</el-button>
      <el-button type="primary" @click="addKbFiles">{{ t('确定')}}</el-button>
    </template>
  </el-dialog>
</template>

<style scoped lang="scss">

</style>