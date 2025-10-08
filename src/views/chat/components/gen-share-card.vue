<script setup lang="ts">
import {ref, reactive, onMounted, watch} from 'vue'
import {call} from "@/utils/commands.ts";
import html2canvas from 'html2canvas';
import {ElMessage} from "element-plus";
import {save} from "@tauri-apps/plugin-dialog";
import {writeFile} from "@tauri-apps/plugin-fs";
import {invoke} from "@tauri-apps/api/core";
import SvgIcon from "@components/SvgIcon/index.vue";
import {openPath} from "@tauri-apps/plugin-opener";

const isShow = ref(false)

PubSub.unsubscribe('chat/message/command/share/card')
PubSub.subscribe('chat/message/command/share/card', (event: string, message: any) => {
  isShow.value = true
  form.value.content = message.text
})

// 定义卡片样式类型
interface CardStyle {
  id: string
  name: string
  backgroundColor: string
  textColor: string
  borderColor: string
  borderRadius: string
  fontFamily: string
  fontSize: string
  boxShadow: string
  headerColor?: string
  footerColor?: string
  padding?: string
}

// 内置卡片样式
const builtInStyles = reactive<CardStyle[]>([
  {
    id: 'classic',
    name: '经典白',
    backgroundColor: '#ffffff',
    textColor: '#333333',
    borderColor: '#e0e0e0',
    borderRadius: '8px',
    fontFamily: 'Arial, sans-serif',
    fontSize: '16px',
    boxShadow: '0 2px 10px rgba(0, 0, 0, 0.1)'
  },
  {
    id: 'modern',
    name: '现代灰',
    backgroundColor: '#f8f9fa',
    textColor: '#212529',
    borderColor: '#dee2e6',
    borderRadius: '8px',
    fontFamily: '"Helvetica Neue", Helvetica, Arial, sans-serif',
    fontSize: '15px',
    boxShadow: '0 4px 15px rgba(0, 0, 0, 0.08)'
  },
  {
    id: 'dark',
    name: '暗夜黑',
    backgroundColor: '#212529',
    textColor: '#f8f9fa',
    borderColor: '#495057',
    borderRadius: '8px',
    fontFamily: 'monospace',
    fontSize: '16px',
    boxShadow: '0 4px 12px rgba(0, 0, 0, 0.3)'
  },
  {
    id: 'vibrant',
    name: '活力蓝',
    backgroundColor: '#e3f2fd',
    textColor: '#1976d2',
    borderColor: '#90caf9',
    borderRadius: '8px',
    fontFamily: '"Comic Sans MS", cursive, sans-serif',
    fontSize: '17px',
    boxShadow: '0 4px 12px rgba(25, 118, 210, 0.2)'
  },
  {
    id: 'warm',
    name: '温暖橙',
    backgroundColor: '#fff3e0',
    textColor: '#e65100',
    borderColor: '#ffcc80',
    borderRadius: '8px',
    fontFamily: '"Segoe UI", Tahoma, Geneva, Verdana, sans-serif',
    fontSize: '16px',
    boxShadow: '0 4px 12px rgba(230, 81, 0, 0.2)'
  },
  {
    id: 'fresh',
    name: '清新绿',
    backgroundColor: '#e8f5e9',
    textColor: '#1b5e20',
    borderColor: '#a5d6a7',
    borderRadius: '6px',
    fontFamily: '"Courier New", Courier, monospace',
    fontSize: '15px',
    boxShadow: '0 4px 12px rgba(27, 94, 32, 0.2)',
    padding: '10px'
  },

])

// 当前选中的样式
const currentStyle = ref<CardStyle>(builtInStyles[5])
const form = ref((() => {
  let old = localStorage.getItem('share_card_gen_config')
  if (old) {
    return JSON.parse(old)
  }
  return {
    // 内容
    content: '',
    // 风格
    style: [],
    // 布局
    layout: '',
    //标题
    title: '',
    // 模型
    modelName: '',
    // 提示词
    prompt: ''
  }
})())

const options = ref({
  styles: JSON.parse(localStorage.getItem('share_card_gen_options') || '{}').styles
      || [
        '自适应宽度', '响应式设计', '高对比度', '扁平化设计', '材质设计', '霓虹', '手绘', '商务', '儿童',
        '艺术', '极客', '优雅', '现代', '经典', '简约', '鲜艳', '暗黑', '明亮', '复古', '未来', '极简',
        '创意', '抽象', '立体', '渐变', '毛玻璃', '卡片', '插画', '涂鸦', '梦幻', '工业', '田园', '海洋',
        '森林', '沙漠', '都市', '学院', '时尚', '奢华', '朴素'
      ],
  layouts: JSON.parse(localStorage.getItem('share_card_gen_options') || '{}').layouts
      || [
        '方形', '竖版', '横版', '圆形', '椭圆形', '圆角矩形', '胶囊形', '卡片形', '海报形', '便签形', '相框形',
        '对话框形', '列表形', '网格形', '响应式', '移动端', '平板端', '桌面端', '自适应'
      ],
  prompts: JSON.parse(localStorage.getItem('share_card_gen_options') || '{}').prompts
      || []
})
const result = ref(null)
const models = ref([])
const isGenerating = ref(false)
const zoom = ref(1)
const lastSavedPath = ref(null)
onMounted(() => {
  loadAllAvailableModels()
})
const loadAllAvailableModels = async () => {
  models.value = await call('all_available_models', {taskType: 1})
}

watch(form, (value) => {
  let v = {...value, content: ''}
  localStorage.setItem('share_card_gen_config', JSON.stringify(v))
}, {deep: true})
watch(options, (value) => {
  localStorage.setItem('share_card_gen_options', JSON.stringify(value))
}, {deep: true})
const appendStyles = (value: string[]) => {
  value.forEach(item => {
    if (item && !options.value.styles.includes(item)) {
      options.value.styles.unshift(item);
    }
  });
}
const appendLayouts = (value: string) => {
  if (value && !options.value.layouts.includes(value)) {
    options.value.layouts.unshift(value);
  }
}
// 生成卡片
const generateAiStyle = () => {
  if (!form.value.content) {
    return
  }
  isGenerating.value = true
  call('gen_share_card', {
    ...form.value
  }).then((res: any) => {
    result.value = res
    attachShadowDOM(result.value, document.getElementById('share-card-result'))
    isGenerating.value = false
  })

}

const attachShadowDOM = (htmlContent: string, container: HTMLElement) => {
  // 检查是否已经存在 shadow root
  let shadow = container.shadowRoot;

  // 如果不存在 shadow root，则创建一个新的
  if (!shadow) {
    shadow = container.attachShadow({mode: 'open'});
  }

  // 清空 shadow root 中的内容
  shadow.innerHTML = '';

  // 创建包装元素
  const wrapper = document.createElement('div');
  wrapper.className = 'share-card-content';
  wrapper.innerHTML = htmlContent;


  // 设置所有子元素为可编辑
  const allElements = wrapper.querySelectorAll('*');
  allElements.forEach(element => {
    element.setAttribute('contenteditable', 'true');
  });

  // 添加样式
  const style = document.createElement('style');
  style.textContent = `
    .share-card-content {
      max-width: 100%;
      margin: 0 auto;
      display: flex;
      justify-content: center;
    }
  `;

  shadow.appendChild(style);
  shadow.appendChild(wrapper);
};

// 保存图片
const saveAsImage = async () => {
  // 卡片结果容器
  let element = document.getElementById('share-card-result');
  if (!element) {
    return
  }
  // 重置缩放，否则样式会错乱
  zoom.value = 1

  // 延时200毫秒
  await new Promise(resolve => setTimeout(resolve, 200));

  const ele = element.shadowRoot?.querySelector('.share-card-content') || element;

  try {
    // 获取实际尺寸
    const rect = ele.getBoundingClientRect();
    const actualWidth = rect.width / zoom.value;
    const actualHeight = rect.height / zoom.value;

    const canvas = await html2canvas(ele as HTMLElement, {
      backgroundColor: null,
      scale: 3,
      useCORS: true,
      scrollX: 0,
      scrollY: 0,
      height: actualHeight,
      width: actualWidth,
    });

    const image = canvas.toDataURL('image/png');
    const fileName = Date.now() + '.png'
    const base64Data = image.split(',')[1];
    const buffer = Uint8Array.from(atob(base64Data), c => c.charCodeAt(0));
    const path = await call('save_file_to_file_dir', {
      fileName: fileName,
      bytes: buffer
    });
    await openPath(path)
    lastSavedPath.value = path
  } catch (error) {
    console.error('转换图片失败:', error);
  }
};

// 下载图片
const downloadImage = (dataUrl: string, filename: string) => {
  const link = document.createElement('a');
  link.href = dataUrl;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
};

const copyToClipboard = async () => {
  // 卡片结果容器
  let element = document.getElementById('share-card-result');
  if (!element) {
    return
  }
  zoom.value = 1

  // 延时200毫秒
  await new Promise(resolve => setTimeout(resolve, 200));

  const ele = element.shadowRoot?.querySelector('.share-card-content') || element;

  try {

    // 获取实际尺寸
    const rect = ele.getBoundingClientRect();
    const actualWidth = rect.width / zoom.value;
    const actualHeight = rect.height / zoom.value;

    const canvas = await html2canvas(ele as HTMLElement, {
      backgroundColor: null,
      scale: 3,
      useCORS: true,
      scrollX: 0,
      scrollY: 0,
      height: actualHeight,
      width: actualWidth,
    });

    canvas.toBlob(async (blob) => {
      if (blob) {
        try {
          await navigator.clipboard.write([
            new ClipboardItem({'image/png': blob})
          ]);
          ElMessage.success('已复制到剪贴板');
        } catch (err) {
          ElMessage.error('复制图片失败:' + err);
        }
      }
    });
  } catch (error) {
    ElMessage.error('转换图片失败');
  }
};

const changeZoom = (event: WheelEvent) => {
  if (event.ctrlKey && event.deltaY > 0) {
    zoom.value -= 0.05
  }
  if (event.ctrlKey && event.deltaY < 0) {
    zoom.value += 0.05
  }
}


</script>

<template>
  <el-drawer v-model="isShow" title="制作分享卡片" size="calc(100vw - 250px)">
    <div class="share-card-generator">
      <div class="main-content">
        <div class="input-section">
          <el-input
              v-model="form.content"
              type="textarea"
              placeholder="请输入卡片内容..."
              class="text-input"
              resize="none"
          ></el-input>
          <div class="">
            <div class="flex-space-between">
              <h4>样式设置</h4>
            </div>
            <el-form class="mt10">
              <el-form-item label="风格">
                <el-select v-model="form.style"
                           placeholder="选择风格"
                           multiple
                           allow-create
                           filterable
                           collapse-tags
                           :max-collapse-tags="2"
                           clearable
                           @change="appendStyles">
                  <el-option v-for="(item,index) in options.styles" :label="item" :value="item">
                    <div class="select-option">
                      <span>{{ item }}</span>
                      <el-button
                          class="delete-option-btn"
                          type="danger"
                          size="small"
                          link
                          icon="Delete"
                          @click.stop="options.styles.splice(index,1);form.style =form.style.filter(v => v !== item) ">
                      </el-button>
                    </div>
                  </el-option>
                </el-select>
              </el-form-item>
              <el-form-item label="布局">
                <el-select v-model="form.layout"
                           placeholder="选择布局"
                           allow-create
                           filterable
                           clearable
                           @change="appendLayouts">
                  <el-option v-for="(item,index) in options.layouts" :label="item" :value="item">
                    <div class="select-option">
                      <span>{{ item }}</span>
                      <el-button
                          class="delete-option-btn"
                          type="danger"
                          size="small"
                          link
                          icon="Delete"
                          @click.stop="options.layouts.splice(index,1);form.layout = form.layout===item?'':form.layout">
                      </el-button>
                    </div>
                  </el-option>
                </el-select>
              </el-form-item>
              <el-form-item label="标题">
                <div class="flex fill-width">
                  <el-input
                      v-model="form.title"
                      placeholder="卡片标题"
                      clearable
                  ></el-input>
                </div>
              </el-form-item>
              <el-form-item label="模型">
                <el-select v-model="form.modelName" placeholder="选择生成模型" clearable>
                  <el-option v-for="item in models" :label="item.name" :value="item.name"></el-option>
                </el-select>
              </el-form-item>
              <el-form-item label="要求">
                <div class="flex fill-width">
                  <el-input v-model="form.prompt"
                            placeholder="简要描述你的额外要求"
                            class="mr10">
                  </el-input>
                  <el-button
                      type="primary"
                      class="generate-btn"
                      @click="generateAiStyle"
                      :loading="isGenerating"
                  >
                    生成{{ isGenerating ? '中' : '' }}
                  </el-button>
                </div>
              </el-form-item>
            </el-form>
          </div>
        </div>

        <div class="preview-section">
          <el-scrollbar max-height="calc(100vh - 174px)">
            <div id="share-card-result"
                 :style="{zoom:zoom}"
                 @wheel="changeZoom"
                 v-loading="isGenerating"
                 element-loading-text="生成中...">
              <div>
                <div class="card-preview">
                  <div
                      class=""
                      :style="{
                backgroundColor: currentStyle.backgroundColor,
                color: currentStyle.textColor,
                border: `1px solid ${currentStyle.borderColor}`,
                borderRadius: currentStyle.borderRadius,
                fontFamily: currentStyle.fontFamily,
                fontSize: currentStyle.fontSize,
                boxShadow: currentStyle.boxShadow,
                padding: currentStyle.padding
              }"
                  >
                    <div class="card-header" :style="{ color: currentStyle.headerColor || currentStyle.textColor }">
                      {{ form.title }}
                    </div>

                    <div class="card-content">
                      <div v-if="form.content" class="content-text">{{ form.content }}</div>
                      <div v-else class="placeholder-text">
                        输入卡片内容生成卡片
                      </div>
                    </div>

                    <div class="card-footer" :style="{ color: currentStyle.footerColor || currentStyle.textColor }">
                      <div class="logo">ShareCard</div>
                      <div class="date">{{ new Date().toLocaleDateString() }}</div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

          </el-scrollbar>
          <div class="action-buttons animate__animated animate__fadeInUp">
            <div class="flex-center mt10">
              <el-button
                  type="primary"
                  link
                  icon="zoom-in"
                  @click="zoom=zoom*1.1"
              >
                放大
              </el-button>
              <el-button
                  type="primary"
                  link
                  icon="zoom-out"
                  @click="zoom=zoom*0.9"
              >
                缩小
              </el-button>
              <el-button
                  type="primary"
                  link
                  @click="zoom = 1"
                  icon="refresh"
              >
                重置缩放
              </el-button>
              <el-button type="primary" link @click="copyToClipboard" icon="CopyDocument">复制图片</el-button>
              <el-button type="primary" link @click="saveAsImage" icon="download">保存图片</el-button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </el-drawer>
</template>

<style scoped lang="scss">
.share-card-generator {
  padding: 10px;

  .main-content {
    display: flex;
    gap: 15px;
  }

  .input-section {
    flex: 1;

    .text-input {
      margin-bottom: 15px;
    }
  }

  .preview-section {
    flex: 1.5;
    max-width: 50%;

    .text-input {
      margin-bottom: 15px;
    }
  }

  .card-preview {
    transition: all 0.3s ease;
    display: flex;
    flex-direction: column;
    min-height: 280px;

    .card-header {
      padding-bottom: 8px;
      border-bottom: 1px solid currentColor;
      margin-bottom: 10px;
      font-weight: bold;
      font-size: 14px;
    }

    .card-content {
      flex: 1;
      display: flex;
      align-items: center;
      justify-content: center;
      text-align: center;
      overflow: auto;

      .content-text {
        line-height: 1.5;
        font-size: 13px;
        white-space: pre-wrap;
      }

      .placeholder-text {
        color: #999;
        font-style: italic;
        font-size: 12px;
        height: 50vh;
        display: flex;
        align-items: center;
      }
    }

    .card-footer {
      display: flex;
      justify-content: space-between;
      margin-top: 10px;
      padding-top: 8px;
      border-top: 1px dashed currentColor;
      font-size: 10px;
    }
  }

  .action-buttons {
    position: absolute;
    bottom: 78px;
  }
}

.share-card-result {
  max-width: 280px;
  overflow-x: auto;
}

:deep(.el-form-item ) {
  margin-bottom: 10px;
}

:deep(.el-scrollbar) {
  height: unset;
}

:deep(.el-textarea__inner) {
  background: #fafafa !important;
  border: none !important;
  height: calc(100vh - 380px);

}

:deep(.el-textarea) {
  ::-webkit-scrollbar {
    width: 0;
    height: 0;
  }
}

.select-option {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.delete-option-btn {
  opacity: 0;
  transition: opacity 0.2s;
  margin-right: 10px;
}

.select-option:hover .delete-option-btn {
  opacity: 1;
}


</style>
