<template>
  <section class="app-main">
    <div class="main">
      <router-view :key="key"></router-view>
    </div>
  </section>
</template>

<script>
export default {
  name: 'AppMain',
  computed: {
    key() {
      let name = this.$route['name']
      if (name === 'Chat') {
        return this.$route.path
      }
    }
  },
  methods: {}
}
</script>

<style scoped lang="scss">
.app-main {
  height: calc(100vh - 40px);
  position: relative;
  // background: #2c2c2c;
}

.fixed-header + .app-main {
  padding-top: 50px;
}
</style>

<style lang="scss">
// fix css style bug in open el-dialog
.el-popup-parent--hidden {
  .fixed-header {
    padding-right: 15px;
  }
}
</style>
