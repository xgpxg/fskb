<template>
  <div>
    <el-row :gutter="10">
      <el-col :span="enableHeader?3:0" >
        <el-affix :offset="offsetTop" v-if="enableHeader">
          <div class=" mt10" v-if="titles.length>0">
            <div class="color-secondary">
              目录
            </div>
            <div
                v-for="anchor in titles"
                :style="{ padding: `10px 0 10px ${anchor.indent * 20}px`}"
                @click="handleAnchorClick(anchor)">
              <el-link :underline="false">
                <div class="ellipsis" style="width: 10vw"
                     :title="anchor.title">
                  {{ anchor.title }}
                </div>
              </el-link>
            </div>
          </div>
        </el-affix>
      </el-col>
      <el-col :span="enableHeader?21:24">
        <v-md-preview ref="editor"
                      :text="value"
                      height="400px"
                      v-bind="$attrs" v-on="$listeners">
        </v-md-preview>
      </el-col>
    </el-row>
  </div>
</template>

<script>


export default {
  name: "document-md-preview",
  components: {},
  props: {
    value: String,
    enableHeader: Boolean
  },
  data() {
    return {
      titles: [],
      offsetTop: 60
    }
  }
  ,
  watch: {
    values(newVal) {
      this.$emit('update:value', newVal)
    }
  },
  mounted() {
    this.init()
  },
  methods: {
    init() {
      if (this.enableHeader) {
        this.buildHeaders()
        this.resetOffsetTop()
      }
    },
    buildHeaders() {
      const anchors = this.$refs.editor.$el.querySelectorAll('h1,h2,h3,h4,h5,h6');
      const titles = Array.from(anchors).filter((title) => !!title.innerText.trim());

      if (!titles.length) {
        this.titles = [];
        return;
      }

      const hTags = Array.from(new Set(titles.map((title) => title.tagName))).sort();

      this.titles = titles.map((el) => ({
        title: el.innerText,
        lineIndex: el.getAttribute('data-v-md-line'),
        indent: hTags.indexOf(el.tagName),
      }));
    },
    handleAnchorClick(anchor) {
      const {editor} = this.$refs;
      const {lineIndex} = anchor;

      const heading = editor.$el.querySelector(`[data-v-md-line="${lineIndex}"]`);

      if (heading) {
        heading.scrollIntoView({
          behavior: 'smooth'
        });
      }
    },
    resetOffsetTop() {
      this.offsetTop = 0
      setTimeout(() => {
        this.offsetTop = 60
      }, 200)
    },
  }
}
</script>


<style scoped lang="scss">
:deep(.v-md-editor-preview) {
  .github-markdown-body {
    padding: 10px 20px;
    font-size: 16px;
    line-height: 1.8;
    color: #333;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", "Liberation Sans", "Helvetica Neue", Helvetica, Tahoma, Arial, "PingFang SC", "Hiragino Sans GB", "Heiti SC", "Microsoft YaHei", "WenQuanYi Micro Hei", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"
  }
}
</style>