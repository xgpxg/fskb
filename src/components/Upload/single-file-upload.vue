<!--单文件上传组件-->
<template>
  <div class="upload-file">
    <el-upload ref="uploader" :http-request="upload"
               :show-file-list="false"
               :auto-upload="true">
      <el-button icon="upload" :loading="uploading">{{ uploadText }}</el-button>
      <span v-if="file"
            class="ml10 color-secondary">
        {{ uploading ? '上传中' : '已上传' }}：{{ file.name }}</span>
    </el-upload>
  </div>
</template>
<script>
export default {
  name: "single-file-upload",
  components: {},
  props: {
    /**
     * 业务类型：1系统文件 2用户文件 3临时文件 4数据资源文件
     */
    businessType: Number,
    /**
     * 预留字段，暂未使用
     */
    businessId: Number,
    /**
     * 是否启用CDN，非必填
     */
    enableCdn: Boolean,
    /**
     * 是否私有的，私有文件将上传到/private下，非必填
     */
    isPrivate: Boolean,
    /**
     * 文件路径，非必填
     */
    path: String,
    /**
     * 绑定文件地址，非必填
     */
    value: String,
    /**
     * 上传按钮显示文本
     */
    uploadText: {
      type: String,
      default() {
        return '上传文件'
      }
    }
  },
  data() {
    return {
      /**
       * 上传成功后的文件地址或文件ID（当businessType不为空时返回文件ID）
       */
      fileUrl: null,
      /**
       * 上传状态
       */
      uploading: false,
      /**
       * 文件对象
       */
      file: null
    }
  },
  watch: {
    value(newVal) {
      this.fileUrl = newVal
    }
  },
  created() {
    this.init()
  },
  methods: {
    init() {
      this.fileUrl = this.value
    },
    upload(options) {
      if (!this.businessType) {
        this.$message.error('业务类型不能为空')
      }
      this.file = options.file
      this.uploading = true
      this.R.upload('/console/common/upload', options.file, {
        businessType: this.businessType,
        enableCdn: this.enableCdn ? 1 : 0,
        isPrivate: this.isPrivate ? 1 : 0
      }).then(res => {
        this.fileUrl = res.data
        this.$emit('update:value', this.fileUrl)
        this.uploading = false
      })
    }
  }
}
</script>


<style scoped lang="scss">
.upload-file {

}

</style>