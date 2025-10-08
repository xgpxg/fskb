<!--短信验证码发送组件-->
<template>
  <el-button @click="getSmsCode"
             :disabled="innerDisabled" :size="size">{{ msg }}
  </el-button>
</template>

<script>
export default {
  name: "SmsCode",
  props: {
    phone: {
      type: String
    },
    disabled: {
      type: Boolean
    },
    size: {
      type: String,
      default: () => {
        return ''
      }
    }
  },
  data() {
    return {
      innerDisabled: false,
      msg: '获取验证码',
      timer: null
    }
  },
  methods: {
    getSmsCode() {
      let that = this;
      if (!this.phone) {
        return false;
      }
      // 校验邮箱
      if (this.phone.indexOf('@') >= 0) {
        if (!this.U.checkEmail(this.phone)) {
          this.$message({
            type: 'error',
            message: '邮箱格式不正确'
          });
          return false;
        }
      }
      // 校验手机号
      else {
        if (!this.U.checkPhone(this.phone)) {
          this.$message({
            type: 'error',
            message: '手机号格式不正确'
          });
          return false;
        }
      }

      let reqParam = {
        phone: that.phone,
      };
      that.innerDisabled = true;
      that.R.postJson('/user/common/getVerifyCode', reqParam).then(res => {
        if (res.code === 0) {
          let oldDateObj = new Date();
          let newDateObj = new Date();
          newDateObj.setTime(oldDateObj.getTime() + (2 * 60 * 1000));
          this.timer = setInterval(() => {
            let timeInterval = that.U.getTimeInterval(new Date(), newDateObj);
            that.msg = `${timeInterval.minutes}分${timeInterval.seconds}秒后重发`;
            if (timeInterval.allSeconds <= 0) {
              that.msg = '获取验证码';
              that.innerDisabled = false;
              clearInterval(this.timer);
            }
          }, 1000);
          this.$message.success('验证码已发送')
        } else {
          that.innerDisabled = false;
          clearInterval(this.timer)
        }
      }).catch(e => {
        this.innerDisabled = false
      })
    }
  }
}
</script>

<style scoped>

</style>
