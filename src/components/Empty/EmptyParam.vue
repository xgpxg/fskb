<template>
  <div>
    <div class="flex-column-v">
      <svg-icon icon-class="empty2" size="50"></svg-icon>
      <span class="color-secondary small-font">没有参数</span>
    </div>
  </div>
</template>

<script>


import SvgIcon from "@components/SvgIcon/index.vue";

export default {
  name: "empty-param",
  components: {SvgIcon},
  data() {
    return {}
  },
  created() {
    this.init()
  },
  methods: {
    init() {

    }
  }
}
</script>


<style scoped lang="scss">

</style>