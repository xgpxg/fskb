<template>
  <div>
    <el-container>
      <el-header height="40px">
        <Header></Header>
      </el-header>
      <el-container>
        <el-aside :width="menuWidth + 'px'" v-if="U.isPc()">
          <Menu :collapse="collapse" :width="menuWidth"></Menu>
        </el-aside>
        <el-container>
          <el-main class="chat-area">
            <AppMain></AppMain>
          </el-main>
          <el-aside width="25vw" v-if="$route.params.knowledgeBaseId">
            <option-area></option-area>
          </el-aside>
        </el-container>
      </el-container>
    </el-container>
  </div>
  <Language></Language>
  <!--  <Intro></Intro>-->
</template>


<script>

import Menu from "@/layout/components/Menu.vue";
import AppMain from "./components/AppMain.vue";
import {U} from "../utils/util.js";
import Header from "@/layout/components/Header.vue";
import Language from "@/layout/components/Language.vue";
import Note from "@/views/note/note.vue";
import OptionArea from "@/views/option/option-area.vue";
//import Intro from "@/layout/components/Intro.vue";

export default {
  computed: {
    U() {
      return U
    }
  },
  components: {OptionArea, Language, Header, AppMain, Menu},
  data() {
    return {
      defaultMenuWidth: 250,
      menuWidth: 0,
      headerHeight: '50px',
      top: 0,
      showLogin: false,
      collapse: false,
    }
  },
  provide() {
    return {
      scrollTop: this.scrollTop,
      scrollTo: this.scrollTo,
      leftMenuWidth: this.getMenuWidth,
      hideMenu: this.hideMenu,
      showMenu: this.showMenu
    }
  },
  created() {
    this.menuWidth = this.defaultMenuWidth
  },
  mounted() {
    PubSub.subscribe('NEED_LOGIN', (msg, data) => {
      this.showLogin = true
    })
  },
  methods: {
    getKey(route) {
      return route.fullPath;
    },
    switchCollapse(isCollapse) {
      this.collapse = isCollapse
      if (isCollapse) {
        this.menuWidth = 65
      } else {
        this.menuWidth = 130
      }
    },
    /**
     * 滚动事件
     * @param scrollTop
     */
    scroll({scrollTop}) {
      this.top = scrollTop
    },
    /**
     * 设置滚动距离顶部的位置
     * @param top
     */
    scrollTo(top) {
      this.$refs['scrollbar'].setScrollTop(top)
    },
    /**
     * 获取滚动距离顶部的位置
     * @returns {number}
     */
    scrollTop() {
      return this.top
    },
    getMenuWidth() {
      return this.menuWidth
    },
    hideMenu() {
      this.menuWidth = 0
    },
    showMenu() {
      this.menuWidth = this.defaultMenuWidth
    }
  }
}
</script>

<style scoped lang="scss">
.el-header {
  padding: 0;
}

.el-main {
}

.chat-area{
  overflow-y: hidden;
  border-right: rgb(225, 229, 248) solid 1px;
}
</style>