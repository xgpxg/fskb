<template>
  <div>
    <el-table-plus
      :data.sync="data"
      :page-size="10"
      :current-page.sync="currPage"
      data-url="/api/auth/userManage/permissions"
      :columns="columns"
      :selection="true"
      @select="select"
      :optionColumn="optionColumn"
      :max-height="'500px'"
    >
    </el-table-plus>
  </div>

</template>

<script>
  import ElTablePlus from "./el-table-plus";

  export default {
    name: "demo",
    components: {ElTablePlus},
    data() {
      return {
        data: {},
        columns: [
          {
            label: '权限ID', prop: 'purviewId', align: 'center', columns: [
              {label: '权限ID', prop: 'purviewId'},
              {label: '权限名称', prop: 'purviewName'},
            ]
          },
          {
            label: '权限名称', prop: 'purviewName'
          },
          {
            label: '权限编码', prop: 'purviewCode'
          }
        ],
        currPage: 1,
        optionColumn: {
          options: [
            {
              type: 'text',
              text: '新增',
              handler: (row) => {
                alert('点击了新增')
              }
            }, {
              type: 'text',
              text: '删除',
              handler: (row) => {
                alert('点击了删除')
              }
            }
          ],
          width: '100px',
        }
      }
    },
    methods: {
      loadData(resolver) {
        let reqParam = {
          //key为后台封装的对应的当前页key
          pageNum: this.currPage,
        };
        this.R.get('/api/auth/userManage/permissions', reqParam).then(res => {
          resolver(res.data)
        })
      },
      pageChange(page) {
        alert(page)
      },
      select(selection, row) {
        alert(JSON.stringify(selection))
      }
    }
  }
</script>

<style scoped>

</style>
