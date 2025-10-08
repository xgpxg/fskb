<template>
  <div class="menu" :style="{ width: width+'px'}">
    <div>
      <div class="pd10">
        <search-input/>
      </div>
      <div>
        <kb-list/>
      </div>
    </div>
    <div class="setting-wrapper">
      <setting-bar/>
    </div>
  </div>
</template>

<script>
import SvgIcon from "../../components/SvgIcon/index.vue";
import DevRemark from "@components/DevUtil/dev-remark.vue";
import AutoAvatar from "../../components/avatar/AutoAvatar.vue";
import {U} from "../../utils/util.js";
import SearchInput from "../../views/search/search-input.vue";
import KbList from "../../views/knowledge/kb-list.vue";
import SettingBar from "../../views/system/setting-bar.vue";

export default {
  components: {
    SettingBar,
    KbList,
    SearchInput,
    AutoAvatar,
    DevRemark,
    SvgIcon
  },
  props: {
    collapse: Boolean,
    width: Number,
  },
  provide() {
    return {
      loadBalance: this.loadBalance,
    }
  },
  data() {
    return {
      isCollapse: true,
    }
  },
  computed: {
    U() {
      return U
    },
    user() {
      return this.$store.state.user
    }
  },
  watch: {
    collapse: {
      handler(newVal) {
        this.isCollapse = newVal
      },
      immediate: true
    }
  },
  created() {
  },
  mounted() {
  },
  beforeUnmount() {

  },
  methods: {
    showUserInfoDialog() {
      this.$refs['user-info-dialog'].show()
    }
  }
}
</script>
<style scoped lang="scss">
.menu {
  height: calc(100vh - 40px);
  background-color: #F8F9FA;
  padding: 0;
  //border-radius: 20px;
  //box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
  //margin-left: 20px;
  //margin-top: 10px;
  border-right: rgb(225, 229, 248) solid 1px;
  position: fixed;
  z-index: 0;

  display: flex;
  flex-direction: column;
  justify-content: space-between;

  :deep(.el-menu) {
    border-right: unset;
    background: transparent;

    --el-menu-item-height: 40px;
  }

  :deep(.el-menu-item) {
    margin: 0 10px;
    font-size: 16px;
    border-radius: 10px;

    &.is-active {
      font-weight: 600;
      color: var(--el-color-primary);
      background: #F3EBFF;

    }

    &:hover {
      font-weight: bold;
      color: var(--el-color-primary);
      background: #F3EBFF;
      border-radius: 10px;
    }

    .el-icon {
      > :hover {
        font-size: 1.5em;
        transition: all 0.3s;
      }
    }
  }

  :deep(.el-badge__content) {
    border: none;
    scale: 0.8;
  }


}


.collapse {
  position: fixed;
  bottom: 0;
  height: 50px;
  display: flex;
  justify-content: center;
  align-items: center;
}


.bottom {
  position: relative;

  .content {
    position: fixed;
    padding: 10px 20px;
    bottom: 60px;
  }
}

.filing {
  .content {
    position: fixed;
    padding: 0 10px;
    bottom: 10px;
  }
}

.no-header-dialog {
  :deep(.el-dialog__header) {
    display: none;
  }
}


.logo {
  position: relative;

  .logo-image {
    width: 30px;
  }

  .logo-text {
    font-size: 1.2em;
    font-family: 'Arial Black', sans-serif;
    display: flex;
    align-items: center;
    position: relative;
    margin-left: 6px;
  }

  .letter-m {
    color: var(--el-color-primary);
  }

  .letters-rag {
    color: #4c4c4c; // 黑色
  }

  .setting-wrapper {
    flex-shrink: 0;
  }

}


</style>
