<!--单图上传组件-->
<template>
  <div class="upload-image">
    <el-upload :http-request="upload" class="avatar-uploader"
               :show-file-list="false"
               accept="image/*">
      <img v-if="imageUrl" :src="imageUrl" class="avatar"
           :style="{width:size+'px',height:size+'px'}"/>
      <el-icon v-else class="avatar-uploader-icon"
               :style="{width:size+'px',height:size+'px'}">
        <Plus/>
      </el-icon>
    </el-upload>
  </div>
</template>

<script>


export default {
  name: "single-image-upload",
  components: {},
  props: {
    size: {
      type: Number,
      default: () => {
        return 100
      }
    },
    value: String
  },
  data() {
    return {
      imageUrl: null
    }
  },
  watch: {
    value(newVal) {
      this.imageUrl = newVal
    }
  },
  created() {
    this.init()
  },
  methods: {
    init() {
      this.imageUrl = this.value
    },
    upload(options) {
      this.R.upload('/file/upload', options.file, {file_name: options.file.name}).then(res => {
        this.imageUrl = res.data
        debugger
        this.$emit('update:value', this.imageUrl)
      })
    }
  }
}
</script>


<style scoped lang="scss">
.upload-image {
  :deep(.avatar-uploader) {
    .avatar {
      display: block;
    }

    .el-upload {
      border: 1px dashed var(--el-border-color);
      border-radius: 6px;
      cursor: pointer;
      position: relative;
      overflow: hidden;
      transition: var(--el-transition-duration-fast);
    }

    .el-upload:hover {
      border-color: var(--el-color-primary);
    }


  }

  .el-icon.avatar-uploader-icon {
    font-size: 20px;
    color: #8c939d;
    text-align: center;
  }

}

</style>