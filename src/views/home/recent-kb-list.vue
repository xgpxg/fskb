<script setup lang="ts">
import {ref} from "vue";
import {useRouter} from "vue-router";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
const router = useRouter()
const recentKbs = ref<[{
  id: string,
  name: string,
  description: string,
}]>(JSON.parse(localStorage.getItem('RECENT_KBS') || '[]'))

const toChat = (item: object) => {
  router.push({name: 'Chat', params: {knowledgeBaseId: item.id}})
}
PubSub.unsubscribe('kb/recent/reload')
PubSub.subscribe('kb/recent/reload', (message: string, data: any) => {
  recentKbs.value = JSON.parse(localStorage.getItem('RECENT_KBS') || '[]')
})
</script>

<template>
  <div class="recent-section" v-if="recentKbs.length > 0">
    <el-text type="info">{{ t('最近使用的知识库') }}</el-text>
    <div class="content">
      <el-row :gutter="20" class="mt10">
        <el-col :span="8" v-for="kb in recentKbs">
          <div class="card flex-space-between br5" @click="toChat(kb)">
            <div class="">
              <el-text class="name">{{ kb.name }}</el-text>
              <div class="description">
                <el-text truncated type="info" size="small" v-if="kb.description">{{ kb.description }}</el-text>
                <el-text truncated type="info" size="small" v-else>{{ t('暂无描述') }}</el-text>
              </div>
            </div>
            <div>
              <el-text type="info" size="large">
                <el-icon>
                  <ArrowRight></ArrowRight>
                </el-icon>
              </el-text>
            </div>
          </div>
        </el-col>
      </el-row>
    </div>
  </div>

</template>

<style scoped lang="scss">
.recent-section {
  margin-top: 3rem;
  max-width: 550px;
  width: 100%;
  margin-left: auto;
  margin-right: auto;

  .content {
    text-align: left;

    .card {
      background: #ffffff;
      padding: 10px 20px;
    }
  }

  .name {
    position: relative;
    padding-left: 12px;
    font-size: 14px;
    max-width: 110px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: inline-block;

    &::before {
      content: '';
      position: absolute;
      left: 0;
      top: 2px;
      width: 4px;
      height: 14px;
      background: var(--el-color-primary-light-3);
      border-radius: 2px;
    }
  }

  .description {
    max-width: 110px;
  }

}

</style>