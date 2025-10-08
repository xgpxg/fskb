<!--表格扩展，支持分页、复杂表头、自动跨行等-->
<template>
  <div :style="{'max-width':maxWidth}">
    <el-table ref="table"
              :data="datas"
              :row-key="rowKey"
              @select="onSelect"
              @select-all="onSelectAll"
              @selection-change="onSelectionChange"
              @row-click="onRowClick"
              @row-dblclick="onRowDblclick"
              :height="height"
              :max-height="maxHeight"
              v-loading="showLoading && isLoading"
              :header-cell-class-name="cellClass"
              :span-method="spanMethod"
              :border="border"
              :size="size"
              :row-style="getRowStyle"
              :cell-style="getCellStyle"
              :header-cell-style="{background:'#ffffff',color:'#606266',fontWeight:500}"
              :show-summary="showSummary"
              :highlight-current-row="!selection"
              :tree-props="treeProps"
              @current-change="onCurrentChange"
              :load="treeLoader"
              :lazy="lazy"
              v-bind="$attrs" v-on="$listeners">
      <!--复选框-->
      <el-table-column v-if="selection"
                       type="selection"
                       align="center"
                       width="55"
                       :selectable="selectable">
      </el-table-column>
      <el-table-column v-if="isShowIndex" align="center"
                       type="index"
                       width="50"
                       label="序号">
      </el-table-column>
      <el-table-column-plus v-for="(column , index) in columns"
                            :column="column"
                            :key="index"
                            v-if="(typeof column.hide === 'function'?!column.hide(row):!column.hide)"
      >
        <!--复制所有插槽-->
        <template v-for="slot in Object.keys($scopedSlots)"
                  slot-scope="scope" :slot="slot">
          <slot :name="slot" v-bind="scope"/>
        </template>
      </el-table-column-plus>

      <!--操作栏-->
      <el-table-column
        v-if="optionColumn && optionColumn.options && optionColumn.options.length>0"
        :label="optionColumn.label?optionColumn.label:'操作'"
        :width="optionColumn.width"
        :fixed="optionColumn.fixed"
        align="center">
        <template slot-scope="{row,$index}">
          <template v-for="(option,index) in optionColumn.options">
            <el-button :key="index" :type="option.type"
                       :size="option.size?option.size:'medium'"
                       @click="option.handler(row,$index)"
                       :disabled="typeof option.disabled === 'function'?option.disabled(row):option.disabled"
                       v-if="typeof option.hide === 'function'?!option.hide(row):!option.hide">
              {{ option.text }}
            </el-button>
          </template>
        </template>
      </el-table-column>
    </el-table>

    <!--分页-->
    <div style="margin-top:10px;float: right;margin-right: 10px"
         v-if="pagination">
      <el-pagination
        background
        :layout="layout"
        :total="total"
        :page-size="pageSize"
        @current-change="onPageChange"
        @size-change="onSizeChange"
        :current-page="currPage_"
        :page-sizes="pageSizes"
        :style="paginationStyle">
      </el-pagination>
    </div>
  </div>

</template>

<script>
import ElTableColumnPlus from "./el-table-column-plus.vue";
export default {
  name: "el-table-plus",
  components: {ElTableColumnPlus},
  props: {
    /**
     * 表格数据(含分页)
     */
    data: {
      type: [Object, Array],
      required: false,
      default: () => {
        {
          return {data: {list: []}}
        }
      }
    },
    /**
     * 表格列配置
     */
    columns: {
      type: Array,
      required: false,
      default: () => [{}]
    },
    /**
     * 是否分页
     */
    pagination: {
      Type: Boolean,
      required: false,
      default: true
    },
    /**
     * 分页大小，pagination=true时生效
     */
    pageSize: {
      type: Number,
      required: false,
      default: 10
    },
    /**
     * 分页样式(原生属性)
     */
    layout: {
      type: String,
      required: false,
      default: 'total, sizes, prev, pager, next, jumper'
    },
    /**
     *
     */
    paginationStyle: {
      type: Object,
      required: false
    },
    pageSizes: {
      type: Array,
      default: () => [5, 10, 20, 30, 40, 50, 100]
    },
    /**
     * 加载数据的url
     */
    dataUrl: {
      type: String,
      required: false
    },
    /**
     * 加载数据(优先级高于dataUrl)
     */
    loadData: {
      type: Function,
      required: false
    },
    /**
     * 是否开启多选
     */
    selection: {
      type: Boolean,
      default: false
    },
    /**
     * 是否可选中行，参数:当前行
     */
    selectable: {
      type: Function,
      required: false
    },
    /**
     * 是否显示全选
     */
    showSelectAll: {
      type: Boolean,
      default: true
    },
    /**
     * 当前页
     */
    currentPage: {
      type: Number,
      default: 1
    },
    /**
     * 操作栏
     */
    optionColumn: {
      type: Object,
      default: null
    },
    /**
     * 是否展示加载遮罩层
     */
    showLoading: {
      type: Boolean,
      default: true
    },
    /**
     * 原生属性
     */
    maxHeight: {
      type: String,
      default: null
    },
    height: {
      type: String,
      default: null
    },
    /**
     * 表格最大宽度
     */
    maxWidth: {
      type: String,
      default: null
    },
    /**
     * 自动跨行的列prop(enableRowspan=true时有效)
     */
    autoRowspanKey: {
      type: Array,
      default: () => []
    },
    /**
     * 是否允许跨行
     */
    enableAutoRowspan: {
      type: Boolean,
      default: false
    },
    /**
     * 表格尺寸(原生) 可选值：medium / small / mini
     */
    size: {
      type: String
    },
    /**
     * 是否是紧凑型表格(对行高和单元格边距进行了调整)
     */
    isCompact: {
      type: Boolean,
      default: false
    },
    rowStyle: {
      type: [Object, Function]
    },
    cellStyle: {
      type: [Object, Function]
    },
    /**
     * 指定状态列, 将编码替换为名称
     */
    statusColumns: {
      type: Array
    },
    searchData: {
      type: Object,
      default: () => {
      }
    },
    showSummary: {
      type: Boolean,
      default: false
    },
    rowKey: {
      type: String
    },
    border: {
      type: Boolean,
      default: false
    },
    /**
     * 根据屏幕高度自动计算分页大小
     */
    autoPageSize: {
      type: Boolean,
      default: false
    },
    treeProps: {
      type: Object,
      default() {
        return {
          hasChildren: 'hasChildren',
          children: 'children'
        };
      }
    },
    lazy: {
      type: Boolean,
      default: null
    },
    load: {
      type: Function,
      default: null
    },
    isShowIndex: {
      type: Boolean,
      default: false
    }
  },
  computed: {},
  data() {
    return {
      datas: [],
      isLoading: false,
      total: 0,
      rowspan: {},
      currPage_: this.currPage,
      pageSize_: this.pageSize,
      show: true,
      treeMap: new Map(),
    }
  },
  watch: {
    data: {
      handler(newVal, oldVal) {
        this.datas = this.pagination ? newVal.list : newVal;
        this.total = this.data.total;
      },
      immediate: true
    },
    /* currentPage: {
       handler(newVal, oldVal) {
         this.init();
       },
     },*/
    currPage_: {
      handler(newVal, oldVal) {
        this.init();
      },
    },
    pageSize_: {
      handler(newVal, oldVal) {
        this.init();
      },
    },
    /**
     * 搜索参数
     */
    searchData: {
      handler(newVal, oldVal) {
        this.currPage_ = 1;
        this.init();
      },
      deep: true
    }
  },
  beforeCreate() {
    //英文1个长度 中文2个长度
    String.prototype.gblen = function () {
      let len = 0;
      for (let i = 0; i < this.length; i++) {
        if (this.charCodeAt(i) > 127 || this.charCodeAt(i) === 94) {
          len += 2;
        } else {
          len++;
        }
      }
      return len;
    }
  },
  /*created() {
    this.init();
  },*/
  mounted() {
    if (this.autoPageSize) {
      this.setAutoPageSize();
    } else {
      this.init();
    }
  },
  methods: {
    init() {
      let that = this;
      that.isLoading = true;
      if (this.loadData) {
        this.loadData((data) => {
          this.convertStatusColumn(data.list, () => {
            if (that.pagination) {
              that.datas = data.list;
              that.total = data.total;
            } else {
              that.datas = data.list;
            }
            that.isLoading = false;
            that.$emit('update:data', data);
            this.modifyColumnWidth(that.datas);
          });
        }, that.pagination ? {
          pageNum: this.currPage_,
          pageSize: this.pageSize_
        } : {}, this.searchData);
      } else if (this.dataUrl) {
        let pageInfo = {
          pageNum: this.currentPage,
          pageSize: this.pageSize
        };
        this.U.get(this.dataUrl, {...pageInfo, ...this.searchData}).then(res => {
          that.datas = res.data.data;
          that.total = res.data.data.total;
          that.isLoading = false;
          that.$emit('update:data', res.data);
          this.modifyColumnWidth(that.datas);
        })
      } else {
        this.datas = this.pagination ? this.data.list : this.data;
        that.isLoading = false;
        this.modifyColumnWidth(that.datas);
      }
    },
    refresh() {
      this.init();
    },
    /**
     * 根据列内容自适应列宽 column配置中width=auto-content
     */
    modifyColumnWidth(data) {
      try {
        let maxArr = [];
        this.columns.forEach((column) => {
          let prop = column.prop;
          let max = Math.max(...data.map(d => {
            return d[prop] ? d[prop].gblen() : 12
          }));
          maxArr.push(max);
        });
        for (let i = 0; i < this.columns.length; i++) {
          if (this.columns[i].width === 'auto-content') {
            this.columns[i].width = maxArr[i] * 12;
          }
        }
      } catch (e) {
        //console.error(e)
      }

    },
    setAutoPageSize() {
      let pageSize = this.pageSize;
      try {
        let screenHeight = document.body.clientHeight;
        let zoom = this.detectZoom();
        let top = this.$refs['table'].$el.offsetTop;
        let colHeight = this.isCompact ? 28 : 35;
        let scale = 100;
        scale = zoom <= 67 ? 30 : scale;
        scale = 67 < zoom && zoom <= 75 ? 40 : scale;
        scale = 75 < zoom && zoom <= 80 ? 80 : scale;
        scale = 80 < zoom && zoom <= 90 ? 80 : scale;
        scale = 90 < zoom && zoom <= 100 ? 80 : scale;
        scale = 100 < zoom && zoom <= 110 ? 80 : scale;
        scale = 110 < zoom && zoom <= 120 ? 75 : scale;
        scale = 120 < zoom && zoom <= 125 ? 75 : scale;
        scale = 125 < zoom && zoom <= 150 ? 90 : scale;
        scale = 150 < zoom && zoom <= 175 ? 100 : scale;
        scale = 175 < zoom && zoom <= 175 ? 100 : scale;
        scale = 200 < zoom && zoom ? 100 : scale;
        pageSize = (screenHeight - top) / (colHeight * (zoom / scale * 1.01));
        pageSize = pageSize - 1;
        pageSize = parseInt(pageSize + '');
        if (zoom <= 100) {
          pageSize = pageSize - 2 * Math.sqrt(Math.log(zoom));
          pageSize = parseInt(pageSize + '')
        }
      } catch (e) {
        console.error(e);
        pageSize = this.pageSize;
      }
      let div = pageSize / 10;
      if (div <= 1) {
        pageSize = 10;
      } else if (div <= 2) {
        pageSize = 10;
      } else if (div <= 3) {
        pageSize = 20;
      } else if (div <= 4) {
        pageSize = 30;
      } else if (div <= 5) {
        pageSize = 40;
      } else if (div <= 6) {
        pageSize = 50;
      } else if (div <= 100) {
        pageSize = 100;
      }
      //Note:修改分页 触发数据加载
      if (this.pageSize === pageSize) {
        this.init();
      } else {
        this.pageSize_ = pageSize;
      }
    },
    /**
     * 页码改变事件
     * @param page
     */
    onPageChange(page) {
      this.currPage_ = page;
      this.$emit('update:current-page', page);
      this.$emit('current-page', page);
    },
    /**
     * 页面大小改变事件
     */
    onSizeChange(val) {
      this.pageSize_ = val;
      this.$emit('update:page-size', val);
      this.$emit('page-size', val);
    },
    /**
     * 合并行
     * TODO 需性能优化
     * @param row
     * @param column
     * @param rowIndex
     * @param columnIndex
     * @returns {*[]|number[]}
     */
    spanMethod({row, column, rowIndex, columnIndex}) {
      if (!this.enableAutoRowspan) {
        return undefined;
      }
      //复制一份
      let autoRowspanKey = JSON.parse(JSON.stringify(this.autoRowspanKey));
      //列配置树转为list
      let columnsList = this.treeToList(this.columns);
      //列配置中是否存在跨行配置
      let hasRowspan = columnsList.filter(item => item.autoRowspan).length > 0;
      if (!autoRowspanKey && !hasRowspan) {
        return [1, 1];
      }
      //清理autoRowSpanKey，只保留columns配置中存在的列prop
      autoRowspanKey = autoRowspanKey.filter(item => columnsList.map(v => v.prop).includes(item));

      //获取columns上配置的rowspan的prop，合并到autoRowSpanKey
      for (let i in columnsList) {
        if (columnsList.hasOwnProperty(i)) {
          let prop = columnsList[i].prop;

          if (columnsList[i].autoRowspan && autoRowspanKey.indexOf(prop) < 0) {
            autoRowspanKey.push(columnsList[i].prop);
          }
        }
      }
      //计算需要跨行列的prop对应列索引
      let keyIndexList = [];
      let ks = columnsList.map(item => item.prop);
      for (let key of autoRowspanKey) {
        for (let i in ks) {
          if (ks.hasOwnProperty(i)) {
            if (ks[i] === key) {
              //+2 即加上索引列和复选框列 +1即加上索引列 不然会导致列索引计算错误
              let n = this.isShowIndex?1:0;
              keyIndexList.push(this.selection ? parseInt(i) + (n+1) : parseInt(i) + n);
            }
          }
        }
      }
      //计算需要跨的行数
      let span = undefined;
      for (let i in keyIndexList) {
        if (columnIndex === keyIndexList[i]) {
          let rowSpan = 1;
          if (this.rowspan[columnIndex] === undefined) {
            rowSpan = this.getRowSpan(row, column, rowIndex, columnIndex, autoRowspanKey[i]);
          }
          if (rowSpan > 1) {

            if (this.rowspan[columnIndex] === undefined) {
              this.rowspan[columnIndex] = rowSpan;
              span = [this.rowspan[columnIndex], 1];
            }
          } else if (this.rowspan && this.rowspan[columnIndex] > 1) {
            this.rowspan[columnIndex]--;

            if (this.rowspan[columnIndex] <= 1) {
              this.rowspan[columnIndex] = undefined;
            }
            span = [0, 0]
          }
        }
      }
      //保留原生属性
      this.$emit('span-method', {row, column, rowIndex, columnIndex});
      return span;
    },
    treeToList(data) {
      const result = [];
      data.forEach(item => {
        const loop = data => {
          if (!data.columns) {
            result.push({
              prop: data.prop,
              autoRowspan: data.autoRowspan
            });
          }
          let child = data.columns
          if (child) {
            for (let i = 0; i < child.length; i++) {
              loop(child[i])
            }
          }
        };
        loop(item);
      });
      return result;
    },
    getRowSpan(row, column, rowIndex, columnIndex, key) {
      let span = 1;
      while (rowIndex + 1 < this.datas.length && row[key] === this.datas[rowIndex + 1][key]) {
        rowIndex++;
        span++;
      }
      return span;
    },
    /**
     * 当showSelectAll=false用于隐藏全选
     * @param row
     * @returns {string}
     */
    cellClass(row) {
      if (!this.showSelectAll && row.columnIndex === 0) {
        return 'hide-select-all'
      }
    },
    /**
     * 表格行样式
     */
    getRowStyle({row, rowIndex}) {
      if (this.rowStyle) {
        return this.rowStyle
      } else if (this.isCompact) {
        return {height: '0'}
      }
    },
    /**
     * 单元格样式
     */
    getCellStyle({row, column, rowIndex, columnIndex}) {
      if (this.cellStyle) {
        return this.cellStyle
      } else if (this.isCompact) {
        return {padding: '3px'}
      }
    },
    /**
     * 转换字典编码列
     *
     * 列配置prop以下划线结尾,且dict不为空时将自动进行转换
     * 如：{prop: 'status_', label: '状态',dict:'COMMON_STATUS', width: '80', align: 'center'}
     */
    convertStatusColumn(datas, callback) {
      if (this.columns.filter(v => !!v.dict).length === 0) {
        callback();
      }
      //Promise数组
      let ps = [];
      this.columns.forEach(column => {
        if (column.dict) {
          let p = new Promise((resolve, reject) => {
            this.R.getDict(column.dict, false).then(res => {
              datas.forEach(data => {
                let value = res.data.filter(v => v.itemCode === data[column.prop.substring(0, column.prop.length - 1)]);
                if (value && value.length > 0) {
                  data[column.prop] = value[0].itemText;
                }
              })
              resolve();
            })
          })
          ps.push(p);
        }
      })
      //所有字典加载完成一起返回
      Promise.all(ps).then(res => {
        callback();
      })
    },
    reset() {
      this.currPage_ = 1;
    },
    getDynamicPageSize() {
      if (this.autoPageSize) {
        let screenHeight = document.body.clientHeight;
        let top = this.$refs['table'].$el.getBoundingClientRect().top;
        let pageSize = (screenHeight - top) / 35;
        return parseInt(pageSize + '');
      }
    },
    detectZoom() {
      let ratio = 0,
        screen = window.screen,
        ua = navigator.userAgent.toLowerCase();
      if (window.devicePixelRatio !== undefined) {
        ratio = window.devicePixelRatio;
      } else if (~ua.indexOf('msie')) {
        if (screen.deviceXDPI && screen.logicalXDPI) {
          ratio = screen.deviceXDPI / screen.logicalXDPI;
        }
      } else if (window.outerWidth !== undefined && window.innerWidth !== undefined) {
        ratio = window.outerWidth / window.innerWidth;
      }
      if (ratio) {
        ratio = Math.round(ratio * 100);
      }
      return ratio;
    },
    treeLoader(tree, treeNode, resolve) {
      this.treeMap.set(tree[this.rowKey].toString(), {tree, treeNode, resolve})
      let that = this;
      this.load(tree, treeNode, (data) => {
        if (data.length === 0) {
          tree.hasChildren = false;
          let lazyTreeNodeMap = that.$refs['table'].store.states.lazyTreeNodeMap;
          that.$set(lazyTreeNodeMap, tree.purviewId, []);
        } else {

        }
        this.$refs['table'].toggleRowExpansion(tree, true);

        this.convertStatusColumn(data, () => {
          resolve(data)
        })

      });
    },
    /**
     * 局部刷新表格树
     * @param parentId 要刷新的rowKey，一般是上级ID
     */
    reloadTree(parentId) {
      if (!parentId || parentId === -1) {
        this.refresh();
        return;
      }

      parentId = parentId.toString();
      let v = this.treeMap.get(parentId);
      if (!v) {
        let v = this.treeMap.get(this.findParent(parentId));
        if (!v) {
          this.refresh();
        } else {
          let {tree, treeNode, resolve} = v
          this.treeLoader(tree, treeNode, resolve);

        }
      } else {
        let {tree, treeNode, resolve} = v
        this.treeLoader(tree, treeNode, resolve);
      }
    },
    findParent(key) {
      let lazyTreeNodeMap = this.$refs['table'].store.states.lazyTreeNodeMap;
      let result;
      Object.keys(lazyTreeNodeMap).forEach(k => {
        let list = lazyTreeNodeMap[k];
        if (list.length > 0) {
          list.forEach(v => {
            if (v[this.rowKey].toString() === key) {
              result = k;
            }
          })
        }
      })
      return result;
    },
    ///////////原生事件//////////////
    onSelect(selection, row) {
      this.$emit('select', selection, row);
    },
    onSelectAll(selection) {
      this.$emit('select-all', selection);
    },
    onSelectionChange(selection) {
      this.$emit('selection-change', selection);
    },
    onRowClick(row) {
      this.$emit('rowClick', row);
    },
    onRowDblclick(row) {
      this.$emit('rowDblclick', row);
    },
    onCurrentChange(row) {
      this.$emit('current-change', row);
    },
    //其他原生事件或属性如果有用到再按需添加


    getTableRef() {
      return this.$refs['table'];
    }
  }

}
</script>

<style scoped>
/deep/ .el-table .hide-select-all .cell .el-checkbox__inner {
  display: none;
}
.el-table{
  border-left: 1px solid #e6ebf5;
  border-right: 1px solid #e6ebf5;
  border-top: 1px solid #e6ebf5;
}

</style>
