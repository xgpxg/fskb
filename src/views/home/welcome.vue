<script setup lang="ts">
import {ref} from 'vue'
import KbAdd from "../knowledge/kb-add.vue";
import {ElMessage} from "element-plus";
import DropArea from "@/views/home/drop-area.vue";
import RecentKbList from "@/views/home/recent-kb-list.vue";
import {useI18n} from "vue-i18n";

const {t} = useI18n()

const kbAddRef = ref(null);
const showAddKb = () => {
  kbAddRef.value.show()
}


// 拖拽文件处理
const handleDragOver = (e) => {
  e.preventDefault()
}

</script>


<template>
  <div class="welcome-container">
    <div class="flex">
      <el-image src="/images/xfs.png" style="height: 80px"></el-image>
      <div class="ml10">
        <h1>{{ t('小飞树知识库') }}</h1>
        <p class="subtitle">{{ t('构建你的专属私有知识库') }}</p>
      </div>
    </div>

    <div class="actions">
      <el-button @click="showAddKb" type="primary" size="large" icon="plus">{{ t('新建知识库') }}</el-button>
      <el-popover trigger="click" width="300">
        <template #reference>
          <el-button @click="" type="success" size="large" plain icon="VideoPlay">{{ t('使用教程') }}</el-button>
        </template>
        <div class="flex-v">
          ✊ {{ t('教程还在努力制作中，敬请期待')}}
        </div>
      </el-popover>
    </div>

    <div class="fill-width">
      <drop-area class="drop-area"></drop-area>
    </div>

    <div class="fill-width">
      <recent-kb-list></recent-kb-list>
    </div>
  </div>
  <kb-add ref="kbAddRef"></kb-add>

</template>


<style scoped lang="scss">
.welcome-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: calc(100vh - 40px);
  margin-left: 1px;
  text-align: center;
  background-color: #f8f9fa;

  h1 {
    font-size: 2rem;
    margin-bottom: 0.5rem;
  }

  .subtitle {
    font-size: 1.1rem;
    color: #666;
    margin-bottom: 2rem;
    margin-top: 10px;
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;

    .btn {
      padding: 0.6rem 1.2rem;
      border-radius: 6px;
      font-size: 1rem;
      cursor: pointer;
      transition: all 0.2s ease-in-out;
    }

    .primary {
      background-color: #0d6efd;
      color: white;
      border: none;

      &:hover {
        background-color: #0b5ed7;
      }
    }

    .secondary {
      background-color: transparent;
      color: #0d6efd;
      border: 2px solid #0d6efd;

      &:hover {
        background-color: #e0e8f0;
      }
    }

    .tertiary {
      background-color: #ffc107;
      color: black;
      border: none;

      &:hover {
        background-color: #e0a800;
      }
    }
  }


  .drop-area {
    width: 100%;
    max-width: 550px;
    margin-left: auto;
    margin-right: auto;
    margin-top: 30px;
  }

}


</style>