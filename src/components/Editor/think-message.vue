<template>
  <!-- 可折叠容器 -->
  <div class="think-container" :class="{'hide-content': !isShowThinkContent}">
    <!-- 控制面板 -->
    <div class="think-header" @click="isShowThinkContent=!isShowThinkContent">
      <div class="title flex-v">
        <div class="mt5">
          <el-icon class="mr5" size="14" :class="{'loading':thinking}">
            <svg-icon icon-class="think"></svg-icon>
          </el-icon>
        </div>
        <span>
          <template v-if="!thinking">已</template>深度思考
        </span>
      </div>
      <span class="status-badge" :class="{
        'status-active': isShowThinkContent,
        'status-inactive': !isShowThinkContent
      }">
        <el-icon v-if="!isShowThinkContent"><ArrowRight></ArrowRight></el-icon>
        <el-icon v-if="isShowThinkContent"><ArrowDown></ArrowDown></el-icon>
      </span>
    </div>

    <!-- 可动画内容区域 -->
    <div class="content">
      <div v-show="isShowThinkContent" class="think-content">
        <v-md-preview
            :text="value"
            height="400px"
            class="markdown-renderer">
        </v-md-preview>
      </div>
    </div>
  </div>
</template>

<script>


import SvgIcon from "@components/SvgIcon/index.vue";

export default {
  name: "think-message",
  components: {SvgIcon},
  props: {
    value: String,
    expand: {
      type: Boolean,
      default: false
    },
    thinking: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      isShowThinkContent: false
    }
  },
  created() {
    this.init()
  },
  methods: {
    init() {
      this.isShowThinkContent = this.expand
    }
  }
}
</script>


<style scoped lang="scss">
.think-container {
  border: 1px solid #e8e8e8;
  margin: -5px 0 10px 18px;
  background-color: var(--el-color-primary-light-12);
  border-left: 6px solid var(--el-color-primary-light-9);
  border-radius: 8px;
  box-shadow: 0 2px 6px rgba(255, 140, 0, 0.1);
  word-break: break-word;
}

.think-header {
  display: flex;
  align-items: center;
  padding: 6px 16px;
  cursor: pointer;
  position: relative;
  font-size: 13px;

  .title {
    color: #333;
  }

  .status-badge {
    margin-left: auto;
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 12px;
    transition: all 0.3s ease;
    color: #333;
  }
}

.hide-content {
  width: 200px;
}

:deep(.content) {
  .v-md-editor-preview {
    .github-markdown-body {
      padding: 10px 20px;
      font-size: 13px;
      color: #787878;
    }
  }
}


</style>