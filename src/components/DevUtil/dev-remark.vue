<template>
  <div v-if="U.isDev() && !hidden">
    <div class="dev-remark">
      <slot></slot>
    </div>
  </div>
</template>

<script>


export default {
  name: "dev-remark",
  components: {},
  props: {
    hidden: Boolean
  },
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
.dev-remark {
  border: #e5e5e5 1px solid;
  padding: 10px;
  background: #fff7da;
  line-height: 25px;
}
</style>