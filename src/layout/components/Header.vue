<script setup lang="ts">
import {getCurrentWindow} from '@tauri-apps/api/window';
import SvgIcon from "@components/SvgIcon/index.vue";
import store from '@/store'
import {computed, onMounted, ref} from "vue";
import multiavatar from "@multiavatar/multiavatar/esm";
import {U} from '@/utils/util'
import UserSettings from "@/views/user/user-settings.vue";
import {v4 as uuidv4} from 'uuid';

const appWindow = getCurrentWindow();
const user = computed(() => store.state.user)
const userSettingsRef = ref()
const genUserAvatar = (nickname: string) => {
  return 'data:image/svg+xml;utf8,' + multiavatar(nickname, true).replaceAll('#', '%23')
}

const applyTheme = (theme: string) => {
  const html = document.documentElement
  html.classList.add(theme)
  document.body.removeAttribute('class')
}

onMounted(() => {
  let id = localStorage.getItem('__FS_USER_LOCAL_ID')
  if (!id) {
    localStorage.setItem('__FS_USER_LOCAL_ID', uuidv4())
  }
  id = localStorage.getItem('__FS_USER_LOCAL_ID')
  store.commit('user/setAvatar', genUserAvatar(id))

  //在整个应用中禁用右键菜单
  document.addEventListener('contextmenu', (event: MouseEvent) => {
    event.preventDefault();
    return false;
  });

  // 生产环境下禁用F5
  if (!U.isDev()) {
    document.addEventListener('keydown', (event: KeyboardEvent) => {
      if (event.key === 'F5' && !event.ctrlKey) {
        event.preventDefault();
        return false;
      }
    });
  }

  //设置主题
  const theme = localStorage.getItem('theme')
  applyTheme(theme || 'default')
})

</script>

<template>
  <div class="title-bar">
    <div class="drag-region" data-tauri-drag-region>
      <div class="fill-width flex" data-tauri-drag-region>
        <div class="avatar" data-tauri-drag-region>
          <el-avatar :src="user.avatar" size="small" @click="userSettingsRef.show()"
                     style="background: transparent"></el-avatar>
        </div>
        <div class="menu fill-width" data-tauri-drag-region>
          <div class="main-menu" v-if="U.isDev()">

          </div>
        </div>
      </div>

    </div>
    <div class="controls">
      <svg-icon icon-class="minsize" id="title-bar-minimize" class="control" @click="appWindow.minimize()">
      </svg-icon>
      <svg-icon icon-class="maxsize" id="title-bar-maximize" class="control" size="14"
                @click="appWindow.toggleMaximize()">
      </svg-icon>
      <svg-icon icon-class="close" id="title-bar-close" class="control" @click="appWindow.close()">
      </svg-icon>
    </div>
  </div>
  <user-settings ref="userSettingsRef"></user-settings>
</template>

<style scoped lang="scss">
.title-bar {
  height: 40px;
  background: linear-gradient(90deg, var(--el-color-primary-light-3), var(--el-color-primary));
  backdrop-filter: blur(10px);
  user-select: none;
  display: grid;
  grid-template-columns: auto max-content;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;

  .drag-region {
    height: 40px;
    width: calc(100vw - 200px);
  }

  .controls {
    display: flex;
    color: #ffffff;
    align-items: center; /* 垂直居中 */
    .control {
      padding: 0 20px;
      height: 40px;

      &:hover {
        background: var(--el-color-primary-light-3);
        //color: var(--el-color-primary);
      }
    }
  }

  .avatar {
    display: flex;
    align-items: center; /* 垂直居中 */
    height: 40px;
    padding: 0 20px;
  }

  .menu {
    display: flex;
    align-items: center;

    .main-menu {
      margin-left: 200px;
    }

    .main-menu-btn {
      background: transparent;
      border: unset;
      color: #ffffff;
    }
  }
}


</style>