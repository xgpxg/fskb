<script setup lang="ts">
import {computed, h, onMounted, ref, watch} from "vue";
import NoteDetail from "@/views/note/note-detail.vue";
import {call} from "@/utils/commands.ts";
import {useRouter} from "vue-router";
import {ElMessage} from "element-plus";
import ContextMenu from "@imengyu/vue3-context-menu";
import SvgIcon from "@components/SvgIcon/index.vue";
import {U} from "@/utils/util";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const router = useRouter()
const route = router.currentRoute

const knowledgeBaseId = computed(() => {
  const id = route.value.params.knowledgeBaseId;
  return parseInt(Array.isArray(id) ? id[0] : id);
})

const notes = ref([])


const noteDetailRef = ref(null)
const currentNote = ref(null)
const showNoteDetail = (item) => {
  currentNote.value = item
  noteDetailRef.value.show()
}

onMounted(async () => {
  await loadAllNotes()
})

watch(knowledgeBaseId, () => {
  loadAllNotes()
})

const searchForm = ref({
  filterText: ''
})
const loadAllNotes = async () => {
  notes.value = await call('list_all_notes', {
    kbId: knowledgeBaseId.value,
    filterText: searchForm.value.filterText
  })

  // 生成标题和摘要
  let needGenTitleAndSummary = notes.value.filter(note => (note.title === null || note.summary === null) && note.content)
  for (let note of needGenTitleAndSummary) {
    note.title = '正在生成标题...'
    note.summary = '正在总结...'
    await genNoteTitleAndSummary(note)
  }

  // 重新加载
  notes.value = await call('list_all_notes', {
    kbId: knowledgeBaseId.value,
    filterText: searchForm.value.filterText
  })
}

PubSub.unsubscribe('note/list/reload')
PubSub.subscribe('note/list/reload', async () => {
  await loadAllNotes()
})

const genNoteTitleAndSummary = async (note: any) => {
  await call('gen_note_title_and_summary', {
    id: note.id
  })
}

const deleteNote = async (note: any) => {
  await call('delete_note', {
    id: note.id
  })
  notes.value = await call('list_all_notes', {
    kbId: knowledgeBaseId.value
  })
}

const onContextMenu = (e: MouseEvent, item: any) => {
  ContextMenu.showContextMenu({
    x: e.x, y: e.y,
    theme: '',
    items: [
      {
        label: t('复制内容'),
        icon: h(SvgIcon, {iconClass: 'copy'}),
        onClick: () => {
          U.copyText(item.content)
        },
      },
      {
        label: '生成分享卡片',
        icon: h(SvgIcon, {iconClass: 'text2'}),
        onClick: () => {

        },
        hidden: !U.isDev()
      },
      {
        label: '导出为',
        icon: h(SvgIcon, {iconClass: 'summarize'}),
        onClick: () => alert('Click Simple item'),
        hidden: !U.isDev(),
        children: [
          {
            label: 'Markdown',
            icon: h(SvgIcon, {iconClass: 'summarize'}),
            onClick: () => {

            },
          },
          {
            label: 'PDF',
            icon: h(SvgIcon, {iconClass: 'summarize'}),
            onClick: () => {

            },
          },
          {
            label: 'Word',
            icon: h(SvgIcon, {iconClass: 'summarize'}),
            onClick: () => {

            },
          },
        ],
        divided: true,
      },
      {
        label: t('删除'),
        onClick: () => {
          deleteNote(item)
          if (item.id === currentNote.value.id) {
            noteDetailRef.value.close()
          }
        },
      },
    ]
  });
}

</script>

<template>
  <div class="title-block">
    <div class="flex-space-between">
      <div>{{ t('笔记本') }}</div>
      <!--      <div>
              <el-text size="small" type="info" class="cursor-pointer">
                时间
                <el-icon>
                  <SortDown/>
                </el-icon>
              </el-text>
            </div>-->
      <div>
        <el-dropdown v-if="U.isDev()">
          <span class="el-dropdown-link">
            <el-text size="small" type="info" class="cursor-pointer">
                <el-icon><MoreFilled/></el-icon>
              </el-text>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item>导出所有笔记</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>
  </div>
  <div>
    <el-input
        v-model="searchForm.filterText"
        prefix-icon="search"
        clearable
        class="mb10"
        size="default"
        :placeholder="notes.length>0?`${t('搜索笔记')}（共${notes.length}篇）`:'搜索笔记'"
        @input="loadAllNotes">
    </el-input>
  </div>
  <div>
    <div class="note-list">
      <template v-for="item in notes">
        <div class="item flex-space-between" @click="showNoteDetail(item)" @contextmenu="onContextMenu($event,item)">
          <div style="width: 90%">
            <div class="title">
              <el-text truncated>{{ item.title }}</el-text>
            </div>
            <div class="summary">
              <el-text size="small" type="info" truncated>
                {{ item.summary }}
              </el-text>
            </div>
          </div>
          <div>
            <el-text>
              <el-icon>
                <ArrowRight></ArrowRight>
              </el-icon>
            </el-text>
          </div>
        </div>
      </template>
      <template v-if="notes.length===0">
        <div class="flex-column-v mt50">
          <svg-icon icon-class="empty2" size="100"></svg-icon>
          <el-text type="info" size="small">
            {{ t('暂无笔记') }}
          </el-text>
        </div>
      </template>
    </div>
  </div>
  <note-detail ref="noteDetailRef" :note="currentNote"></note-detail>
</template>

<style scoped lang="scss">
.note-list {
  //overflow-y: auto;
  //height: calc(100vh - 415px);

  &::-webkit-scrollbar {
    width: 0 !important;
    height: 0;
  }
}

.item {
  background: #fafafa;
  border-radius: 5px;
  padding: 10px;
  transition: all 0.3s ease;
  cursor: pointer;
  margin-bottom: 10px;
}
</style>