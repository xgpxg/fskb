<!--自适应头像：当头像为空是，默认取text的第一个字符作为头像，背景色由第一个字符hash得到-->
<template>
  <el-avatar v-if="src" :src="src"
             style="background: transparent;" :size="size" shape="square"
             :style="{background:transparent?'transparent' : U.randomColor(text?text:''),width:size,height:size}"></el-avatar>
  <el-avatar v-else :style="{background:U.randomColor(text?text:''),width:size,height:size}" :size="size"
             shape="square">
    <div>
      {{ text?.substring(0, 1) }}
    </div>
  </el-avatar>
</template>

<script>


import {U} from "../../utils/util.js";
import {convertImageSrc} from "@/utils/commands.js";

export default {
  name: "auto-avatar",
  computed: {
    U() {
      return U
    }
  },
  components: {},
  props: {
    src: String,
    text: String,
    size: String,
    transparent: {
      type: Boolean,
      default: false
    },
  },
  data() {
    return {}
  },
  created() {
    this.init()
  },
  methods: {
    convertImageSrc,
    init() {

    }
  }
}
</script>


<style scoped lang="scss">

</style>