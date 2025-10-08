<template>
  <div style="z-index: 1000000">
    <el-button icon="plus" class="mb10" @click="add()" type="success" plain
               size="small">添加
    </el-button>
    <el-table class="table draggable" :data="value" row-key="id"
              default-expand-all>
      <el-table-column label="参数名" prop="name" width="200">
        <template #default="{row}">
          <el-input v-model="row.name" maxlength="20"></el-input>
        </template>
      </el-table-column>
      <el-table-column label="描述" prop="description" width="300">
        <template #default="{row}">
          <el-input v-model="row.description" maxlength="500"></el-input>
        </template>
      </el-table-column>
      <el-table-column label="数据类型" prop="dataType" width="160">
        <template #default="{row}">
          <el-select v-model="row.dataType" append-to=".el-overlay-dialog">
            <el-option value="string" label="string">string</el-option>
            <el-option value="number" label="number">number</el-option>
            <el-option value="bool" label="bool">bool</el-option>
            <el-option value="object" label="object">object</el-option>
            <el-option value="array" label="array">array</el-option>
            <el-option value="enum" label="enum">enum</el-option>
          </el-select>
        </template>
      </el-table-column>
      <el-table-column label="位置" prop="position" width="120">
        <template #default="{row}">
          <el-select v-model="row.position" append-to=".el-overlay-dialog">
            <el-option value="url" label="url">url</el-option>
            <el-option value="body" label="body">body</el-option>
            <el-option value="header" label="header">header</el-option>
          </el-select>
        </template>
      </el-table-column>
      <el-table-column label="必须" prop="required" width="120">
        <template #default="{row}">
          <el-select v-model="row.required" append-to=".el-overlay-dialog">
            <el-option :value="1" label="是">是</el-option>
            <el-option :value="0" label="否">否</el-option>
          </el-select>
        </template>
      </el-table-column>
      <el-table-column label="默认值" prop="required" width="150">
        <template #default="{row}">
          <el-input v-model="row.defaultValue"
                    @blur="e=>handleNumberOnBlur(e,row)"></el-input>
        </template>
      </el-table-column>
      <el-table-column label="操作" width="140" fixed="right">
        <template #default="{row}">
          <el-button @click="showExtConfig(row)" size="small" text circle
                     icon="setting" :disabled="row.dataType!=='enum'"></el-button>
          <el-button @click="del(row)" size="small" text circle
                     icon="delete"></el-button>
          <el-button @click="add(row)" size="small" text circle
                     icon="plus"></el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
  <el-dialog title="参数扩展配置" v-model="showExtConfigDraw" :modal="true" width="700" draggable>
    <el-form :model="oldRow" label-width="80px">
      <el-form-item label="枚举配置" v-if="oldRow.dataType==='enum'">
        <el-button @click="addEnum(oldRow.enums,oldRow)" icon="plus">添加</el-button>
        <el-table
            :data="oldRow.enums"
            class="fill-width mt10">
          <el-table-column
              prop="name"
              label="名称">
            <template #default="{row}">
              <el-input v-model="row.name"></el-input>
            </template>
          </el-table-column>
          <el-table-column
              prop="value"
              label="值">
            <template #default="{row}">
              <el-input v-model="row.value" @blur="onEnumValueChange(row)"></el-input>
            </template>
          </el-table-column>
          <el-table-column
              prop=""
              label="操作" width="100">
            <template #default="{row}">
              <el-button type="text" @click="delEnum(row,oldRow.enums)">删除
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="cancelExtConfig">取消</el-button>
      <el-button type="primary" @click="confirmExtConfig">确认</el-button>
    </template>
  </el-dialog>
</template>

<script>
import SvgIcon from "@components/SvgIcon/index.vue";

export default {
  name: "api-param-table",
  components: {SvgIcon},
  props: {
    value: Array,
    type: String,
    schema: Object,
  },
  data() {
    return {
      data: [],
      currRow: null,
      oldRow: null,
      showExtConfigDraw: false,
    }
  },
  watch: {
    value(newVal) {
      this.data = newVal || []
    },
    data: {
      handler(newVal) {
        if (newVal) {
          this.$emit('update:value', newVal)
          this.$emit('update:schema', this.convertToJSONSchema(newVal))
        }
      },
      deep: true
    }
  },
  created() {
    this.init()
  },
  methods: {
    init() {
      this.data = this.value || []
    },
    add(parent) {
      let row = {
        id: this.U.randomString(5),
        //参数名
        name: null,
        //参数描述
        description: null,
        //参数位置：url、body、header
        position: 'url',
        //数据类型：string,number,object,array[string],array[number],array[object],file
        dataType: 'string',
        //是否必须：0否 1是
        required: 1,
        //默认值
        defaultValue: null,
        //参数层级，顶层为0，用于展示时层级缩进
        level: 0,
        //枚举值
        enums: [],
      }
      if (!parent) {
        this.data.push(row)
      } else {
        row.level = parent.level + 1
        if (parent.children) {
          parent.children.push(row)
        } else {
          parent.children = []
          parent.children.push(row)
        }
      }
    },
    del(row, parent) {
      parent = parent || this.data
      let index = parent.indexOf(row)
      if (index > -1) {
        parent.splice(index, 1)
      } else {
        for (let v of parent) {
          if (v.children) {
            this.del(row, v.children)
          }
        }
      }
    },
    showExtConfig(row) {
      this.currRow = row
      this.oldRow = this.U.copy(row)
      this.showExtConfigDraw = true
    },
    confirmExtConfig() {
      this.showExtConfigDraw = false
      Object.assign(this.currRow, this.oldRow)
      this.oldRow = {}
    },
    cancelExtConfig() {
      this.showExtConfigDraw = false
      this.oldRow = {}
    },
    addEnum(enums, oldRow) {
      if (!oldRow.enums) {
        oldRow.enums = []
      }
      enums.push({
        name: '',
        value: ''
      })
    },
    delEnum(row, enums) {
      enums.splice(enums.indexOf(row), 1)
    },
    /**
     * 数字类型的输入框失去焦点后将字符串转为数字
     * @param ele 输入框
     * @param row 当前行
     */
    handleNumberOnBlur(ele, row) {
      if (row.dataType !== 'number') {
        return
      }
      if (!row.defaultValue) {
        return
      }
      if (isNaN(Number(row.defaultValue))) {
        row.defaultValue = null
        return
      }
      row.defaultValue = Number(row.defaultValue)
    },
    onEnumValueChange(row) {
      if (row.value === '') {
        row.value = null
      }
    },
    convertToJSONSchema(params) {
      // 构建属性定义
      const properties = {};
      const required = [];

      // 处理单个参数
      function processParameter(param) {
        const schemaProperty = {
          type: convertDataType(param.dataType),
          description: param.description || undefined,
          default: getDefaultValue(param),
        };

        // 处理枚举类型
        if (param.enums && param.enums.length > 0) {
          schemaProperty.enum = param.enums.map(e => e.value);
        }

        // 处理嵌套结构
        if (param.children && param.children.length > 0) {
          const childProperties = {};
          const childRequired = [];

          param.children.forEach(child => {
            childProperties[child.name] = processParameter(child);
            if (child.required === 1) {
              childRequired.push(child.name);
            }
          });

          schemaProperty.type = 'object';
          schemaProperty.properties = childProperties;
          if (childRequired.length > 0) {
            schemaProperty.required = childRequired;
          }
        }

        return schemaProperty;
      }

      // 类型转换映射
      function convertDataType(dataType) {
        const typeMap = {
          'string': 'string',
          'number': 'number',
          'integer': 'integer',
          'bool': 'boolean',
          'object': 'object',
          'array': 'array',
          'enum': 'string'
        };
        return typeMap[dataType] || 'string';
      }

      // 获取默认值
      function getDefaultValue(param) {
        if (param.defaultValue === null || param.defaultValue === undefined) {
          return undefined;
        }

        switch (param.dataType) {
          case 'number':
            return Number(param.defaultValue);
          case 'bool':
            return param.defaultValue === 'true' || param.defaultValue === true;
          case 'integer':
            return parseInt(param.defaultValue);
          default:
            return param.defaultValue.toString();
        }
      }

      // 处理所有顶级参数
      params.forEach(param => {
        if (param.level === 0) {
          properties[param.name] = processParameter(param);
          if (param.required === 1) {
            required.push(param.name);
          }
        }
      });

      // 构建完整schema
      return {
        //$schema: "https://json-schema.org/draft/2020-12/schema",
        //$id: "https://example.com/schemas/api-params.json",
        //title: "API请求参数",
        //description: "自动生成的API参数校验Schema",
        type: "object",
        properties,
        required: required.length > 0 ? required : undefined
      };
    }
  }
}
</script>


<style scoped lang="scss">
:deep(.el-table) {
  .cell {
    display: flex;
    align-items: center;
  }
}
</style>