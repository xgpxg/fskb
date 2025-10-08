<script setup lang="ts">

import SvgIcon from "../../components/SvgIcon/index.vue";
import KbDetail from "../knowledge/kb-detail.vue";
import {ref} from "vue";
import {useRouter} from "vue-router";

const props = defineProps({
  knowledgeBase: {
    type: Object,
    required: true
  }
})
const router = useRouter()
const kbDetailRef = ref(null)
const showKbDetail = () => {
  kbDetailRef.value.show()
}
</script>

<template>
  <div class="header">
    <div class="back" @click="router.push({name:'Welcome'})">
      <svg-icon icon-class="back" @click=""></svg-icon>
    </div>
    <div>
      <el-text size="large" truncated>{{ knowledgeBase.name }}</el-text>
    </div>
    <div class="actions">
      <svg-icon icon-class="more" @click="showKbDetail"></svg-icon>
    </div>
  </div>
  <kb-detail ref="kbDetailRef" :key="'kb_'+knowledgeBase.id"></kb-detail>
</template>

<style scoped lang="scss">
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 50px;
}

.back {
  margin-left: 20px;
}

.actions {
  margin-right: 20px;
}
</style>