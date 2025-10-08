<template>
  <div class="info-tip" :style="{fontSize:size==='small'?'12px':''}">
    <svg-icon icon-class="info" class="mr5" :color="getColor(type)"></svg-icon>
    <span :style="{color:getColor(type)}">
         <slot>
    </slot>
    </span>
  </div>
</template>

<script>
import SvgIcon from "../SvgIcon/index.vue";

export default {
  name: "TextTip",
  components: {SvgIcon},
  props: {
    /**
     * 类型：info、warning、success、danger
     */
    type: {
      type: String,
      default: 'info'
    },
    size: String
  },
  methods: {
    getColor(type) {
      switch (type) {
        case 'info':
          return '#696969';
        case 'success':
          return '#4a9d41';
        case 'warning':
          return '#ef932f';
        case 'danger':
          return '#e74343';
      }
    },
  }
}
</script>

<style scoped>
.info-tip {

}
</style>