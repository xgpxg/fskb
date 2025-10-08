<template>
    <el-table-column
            v-if="column.columns"
            :label="column.label"
            :prop="column.prop"
            :column="column.columns"
            :width="column.width"
            :min-width="column.minWidth"
            :fixed="column.fixed"
            :align="column.align"
            :show-overflow-tooltip="column.showOverflowTooltip"
            :render-header="column.width==='auto'?renderHeader:undefined">
        <el-table-column-plus v-for="(col,index) in column.columns"
                              :column="col"
                              :key="index">
            <!--复制所有插槽-->
            <template v-for="slot in Object.keys($scopedSlots)"
                      slot-scope="scope" :slot="slot">
                <slot :name="slot" v-bind="scope"/>
            </template>
        </el-table-column-plus>
    </el-table-column>
    <el-table-column
            v-else-if="!column.hidden || !column.hidden()"
            :label="column.label"
            :prop="column.prop"
            :width="column.width"
            :min-width="column.minWidth"
            :fixed="column.fixed"
            :align="column.align"
            :show-overflow-tooltip="column.showOverflowTooltip"
            :render-header="column.width==='auto'?renderHeader:undefined">
        <template slot-scope="scope">
            <span v-for="slot in Object.keys($scopedSlots)" :key="slot">
                <slot v-if="slot===column.prop" :name="slot" :row="scope.row" :column="scope.column" :$index="scope.$index"/>
            </span>
            <span v-if="Object.keys($scopedSlots).indexOf(column.prop)<0">{{scope.row[column.prop]}}</span>
        </template>
    </el-table-column>
</template>

<script>
  export default {
    name: "el-table-column-plus",
    props: {
      column: {
        type: Object,
        required: false,
        default: () => {
        }
      }
    },
    methods: {
      /**
       * 自动列宽
       * @param h
       * @param column
       * @param $index
       * @returns {*}
       */
      renderHeader(h, {column, $index}) {
        let number = column.label.length;
        //按比例算下
        let size = 15 / (number / (number + 2));
        column.minWidth = (number * size);
        return h('div', {}, [column.label])
      },
    }
  }
</script>

<style scoped>
</style>
