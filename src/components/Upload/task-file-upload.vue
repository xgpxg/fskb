<!--异步任务文件上传组件-->
<template>
  <div class="upload-image">
    <el-upload :http-request="upload" class="avatar-uploader" :disabled="uploading"
               :show-file-list="false" v-bind="$attrs" v-on="$listeners">
      <slot name="default">
        <el-button icon="upload" :loading="uploading">
          上传
        </el-button>
      </slot>
    </el-upload>
  </div>
</template>

<script>
import axios from 'axios'

const API = import.meta.env.VITE_API
export default {
  name: "task-file-upload",
  components: {},
  props: {
    //上传后的文件地址
    value: String,
    //应用的AccessToken
    accessToken: String
  },
  data() {
    return {
      uploading: false,
      fileUrl: null,
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
      let param = new FormData()
      param.append('file', options.file)
      this.uploading = true
      axios({
        method: 'post',
        url: API + '/openapi/upload/addTaskFile',
        data: {'file': options.file},
        headers: {
          'Content-Type': 'multipart/form-data',
          'AccessToken': this.accessToken
        }
      }).then(res => {
        this.fileUrl = res.data.data
        this.$emit('update:value', this.fileUrl)
        this.uploading = false
      }).catch(error => {
        if (error.response) {
          this.response = error.response.data
          //如果异常为token已失效，则刷新页面(小概率事件)
          if (this.response.code === 10003) {
            this.$message.error('AccessToken已失效，请刷新页面重试');
          }
        } else {
          this.response = '调用失败：' + error.message
        }
        this.uploading = false
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
    font-size: 28px;
    color: #8c939d;
    text-align: center;
  }

}

</style>