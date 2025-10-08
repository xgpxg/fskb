<template>
  <div style="height: 300px" class="flex-center">
    <slot></slot>
  </div>
</template>
<script>
import {defineComponent} from 'vue'

export default defineComponent({
  name: "TableEmpty"
})
</script>



<style scoped lang="scss">

</style>