<template>
  <div class="info-tip" :style="{fontSize:size==='small'?'13px':''}">
    <slot>
    </slot>
  </div>
</template>

<script>
import SvgIcon from "../SvgIcon/index.vue";

export default {
  name: "InfoTip",
  components: {SvgIcon},
  props: {
    size: String
  }
}
</script>

<style scoped>
.info-tip {
  background: var(--el-color-primary-light-11);
  border-radius: 5px;
  padding: 5px 10px;
  line-height: 20px;
  font-size: 14px;
  color: var(--el-color-info);
}
</style>