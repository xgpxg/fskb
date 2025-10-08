<script setup lang="ts">
import {h, onMounted, reactive, ref} from "vue";
import SvgIcon from "@components/SvgIcon/index.vue";
import ContextMenu, {MenuOptions} from '@imengyu/vue3-context-menu'
import {openPath, revealItemInDir} from "@tauri-apps/plugin-opener";
import {call} from "@/utils/commands.ts";
import {U} from "@/utils/util";

const props = defineProps({
  kbId: {
    type: Number,
    required: true
  }
})
const records = ref([])

onMounted(() => {
  loadKbImportRecordList()
})

const loadKbImportRecordList = async () => {
  const data = await call('kb_import_record_list', {
    req: {
      kbId: props.kbId,
      page: {
        pageSize: 10000,
        pageNum: 1,
      }
    }
  })
  records.value = data.list
}

const deleteRecord = async (record: any) => {
  await call('delete_kb_import_record', {
    id: record.id
  })
  await loadKbImportRecordList()
}


const onContextMenu = (e: MouseEvent, item) => {
  ContextMenu.showContextMenu({
    x: e.x, y: e.y,
    theme: '',
    items: [
      {
        label: '引用',
        icon: h(SvgIcon, {iconClass: 'ref'}),
        onClick: () => {
          PubSub.publish('chat/message/command/input', {
            // 指令：引用文件
            command: 'QuoteFile',
            data: {
              filePath: item.filePath
            }
          })
        },
      },
      {
        label: '提取文本(OCR)',
        icon: h(SvgIcon, {iconClass: 'text2'}),
        onClick: () => {
          PubSub.publish('chat/message/command', {
            // 指令：获取文本
            command: 'GetText',
            data: {
              filePath: item.filePath
            }
          })
        },
      },
      {
        label: '总结内容',
        icon: h(SvgIcon, {iconClass: 'summarize'}),
        onClick: () => alert('Nothing yet'),
        hidden: !U.isDev()
      },
      {
        label: '翻译',
        icon: h(SvgIcon, {iconClass: 'translate'}),
        divided: true,
        onClick: () => alert('Nothing yet'),
        hidden: !U.isDev()
      },
      {
        label: '打开文件位置',
        onClick: () => revealItemInDir(item.filePath),
      },
      {
        label: '删除',
        onClick: () => deleteRecord(item),
      },
    ]
  });
}

const getIcon = (filename: string) => {
  let ext = filename.substring(filename.lastIndexOf('.') + 1);
  if (['png', 'jpg', 'jpeg', 'gif'].indexOf(ext) > -1) {
    return 'image'
  }
  if (['pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'txt', 'md', 'zip'].indexOf(ext) > -1) {
    return ext
  }
  return 'file'
}
</script>

<template>
  <div class="kb-record-list">
    <template v-for="item in records" v-if="records.length>0">
      <div class="item" @contextmenu="onContextMenu($event,item)" @dblclick="openPath(item.filePath)">
        <div class="title flex-v">
          <div>
            <svg-icon :icon-class="getIcon(item.fileName)"></svg-icon>
          </div>
          <div class="ml5 ellipsis" :title="item.fileName">{{ item.fileName }}</div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped lang="scss">
.kb-record-list {
  max-height: 300px;

  &::-webkit-scrollbar {
    width: 0 !important;
    height: 0;
  }
}

.item {
  background: #fafafa;
  padding: 10px 10px 10px 20px;
  font-size: 14px;
  user-select: none;
  color: var(--el-text-color-primary);

  &:hover {
    background: #eaeaea;
  }
}
</style>