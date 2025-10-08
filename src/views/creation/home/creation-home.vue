<script setup lang="ts">
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'

// 状态管理
const activeTab = ref('image')
const prompt = ref('')
const generatedContent = ref<any>(null)
const isGenerating = ref(false)
const resources = ref([])

// 表单数据
const deployForm = reactive({
  name: '',
  description: '',
  server: '',
  username: '',
  password: ''
})

// 生成内容的方法
const generateContent = async () => {
  if (!prompt.value.trim()) {
    ElMessage.warning('请输入提示词')
    return
  }

  isGenerating.value = true
  try {
    // 模拟生成过程
    setTimeout(() => {
      if (activeTab.value === 'image') {
        // 模拟生成图片
        generatedContent.value = {
          type: 'image',
          url: 'https://images.unsplash.com/photo-1506744038136-46273834b3fb?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=2070&q=80',
          prompt: prompt.value
        }
      } else if (activeTab.value === 'game') {
        // 模拟生成游戏代码
        generatedContent.value = {
          type: 'game',
          code: `<!DOCTYPE html>
<html>
<head>
  <title>Generated Game</title>
  <style>
    body { margin: 0; overflow: hidden; }
    canvas { display: block; background: #1a1a2e; }
  </style>
</head>
<body>
  <canvas id="gameCanvas"></canvas>
  <script>
    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    
    // Simple game example
    const player = { x: 100, y: 100, size: 20, speed: 5 };
    
    function drawPlayer() {
      ctx.fillStyle = '#4cc9f0';
      ctx.fillRect(player.x, player.y, player.size, player.size);
    }
    
    function update() {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      drawPlayer();
      requestAnimationFrame(update);
    }
    
    document.addEventListener('keydown', (e) => {
      if (e.key === 'ArrowUp') player.y -= player.speed;
      if (e.key === 'ArrowDown') player.y += player.speed;
      if (e.key === 'ArrowLeft') player.x -= player.speed;
      if (e.key === 'ArrowRight') player.x += player.speed;
    });
    
    update();
  </script>
</body>
</html>`,
          prompt: prompt.value
        }
      }
      isGenerating.value = false
      ElMessage.success('内容生成成功')
    }, 1500)
  } catch (error) {
    isGenerating.value = false
    ElMessage.error('生成失败: ' + error.message)
  }
}

// 添加到资源库
const addToResources = () => {
  if (!generatedContent.value) {
    ElMessage.warning('没有可添加的资源')
    return
  }

  const resource = {
    id: Date.now(),
    type: generatedContent.value.type,
    url: generatedContent.value.url || null,
    code: generatedContent.value.code || null,
    prompt: generatedContent.value.prompt,
    createdAt: new Date()
  }

  resources.value.push(resource)
  ElMessage.success('已添加到资源库')
}

// 部署应用
const deployApplication = () => {
  if (!deployForm.name.trim()) {
    ElMessage.warning('请输入应用名称')
    return
  }

  if (!generatedContent.value) {
    ElMessage.warning('没有可部署的内容')
    return
  }

  ElMessage.success(`应用 "${deployForm.name}" 部署成功！`)
  // 重置表单
  deployForm.name = ''
  deployForm.description = ''
  deployForm.server = ''
  deployForm.username = ''
  deployForm.password = ''
}

// 使用资源
const useResource = (resource) => {
  if (resource.type === 'image') {
    generatedContent.value = {
      type: 'image',
      url: resource.url,
      prompt: resource.prompt
    }
  } else if (resource.type === 'game') {
    generatedContent.value = {
      type: 'game',
      code: resource.code,
      prompt: resource.prompt
    }
  }
  ElMessage.success(`已使用资源: ${resource.prompt.substring(0, 20)}...`)
}
</script>

<template>
  <div class="creation-home">
    <div class="header">
      <h1>创意工坊</h1>
      <p>通过AI生成图片、游戏等内容，并进行部署发布</p>
    </div>

    <el-tabs v-model="activeTab" class="main-tabs">
      <el-tab-pane label="图片生成" name="image">
        <div class="content-area">
          <div class="input-section">
            <el-input
              v-model="prompt"
              type="textarea"
              placeholder="请输入图片描述，例如：一只在海边看日落的猫"
              :rows="4"
              class="prompt-input"
            />
            <div class="actions">
              <el-button 
                type="primary" 
                @click="generateContent" 
                :loading="isGenerating"
                size="large"
              >
                {{ isGenerating ? '生成中...' : '生成图片' }}
              </el-button>
              <el-button 
                v-if="generatedContent && generatedContent.type === 'image'" 
                @click="addToResources"
                type="success"
              >
                添加到资源库
              </el-button>
            </div>
          </div>

          <div class="preview-section" v-if="generatedContent && generatedContent.type === 'image'">
            <h3>生成结果</h3>
            <el-image 
              :src="generatedContent.url" 
              class="generated-image"
              fit="contain"
              :preview-src-list="[generatedContent.url]"
            />
            <p class="prompt-display">"{{ generatedContent.prompt }}"</p>
          </div>
        </div>
      </el-tab-pane>

      <el-tab-pane label="游戏生成" name="game">
        <div class="content-area">
          <div class="input-section">
            <el-input
              v-model="prompt"
              type="textarea"
              placeholder="请输入游戏描述，例如：一个简单的太空射击游戏，玩家控制飞船躲避陨石"
              :rows="4"
              class="prompt-input"
            />
            <div class="actions">
              <el-button 
                type="primary" 
                @click="generateContent" 
                :loading="isGenerating"
                size="large"
              >
                {{ isGenerating ? '生成中...' : '生成游戏' }}
              </el-button>
              <el-button 
                v-if="generatedContent && generatedContent.type === 'game'" 
                @click="addToResources"
                type="success"
              >
                添加到资源库
              </el-button>
            </div>
          </div>

          <div class="preview-section" v-if="generatedContent && generatedContent.type === 'game'">
            <h3>生成结果</h3>
            <div class="code-preview">
              <pre>{{ generatedContent.code.substring(0, 500) }}{{ generatedContent.code.length > 500 ? '...' : '' }}</pre>
            </div>
            <p class="prompt-display">"{{ generatedContent.prompt }}"</p>
          </div>
        </div>
      </el-tab-pane>

      <el-tab-pane label="资源库" name="resources">
        <div class="resources-section">
          <h3>我的资源</h3>
          <div v-if="resources.length === 0" class="empty-resources">
            <p>暂无资源，请先生成图片或游戏</p>
          </div>
          <div v-else class="resources-grid">
            <div 
              v-for="resource in resources" 
              :key="resource.id"
              class="resource-card"
              @click="useResource(resource)"
            >
              <div v-if="resource.type === 'image'" class="resource-image">
                <el-image 
                  :src="resource.url" 
                  fit="cover"
                  class="thumbnail"
                />
              </div>
              <div v-else class="resource-code">
                <div class="code-icon">🎮</div>
              </div>
              <div class="resource-info">
                <p class="resource-prompt">{{ resource.prompt.substring(0, 30) }}{{ resource.prompt.length > 30 ? '...' : '' }}</p>
                <p class="resource-date">{{ new Date(resource.createdAt).toLocaleString() }}</p>
              </div>
            </div>
          </div>
        </div>
      </el-tab-pane>

      <el-tab-pane label="部署发布" name="deploy">
        <div class="deploy-section">
          <h3>部署应用</h3>
          <el-form :model="deployForm" label-position="top" class="deploy-form">
            <el-form-item label="应用名称">
              <el-input v-model="deployForm.name" placeholder="请输入应用名称" />
            </el-form-item>
            <el-form-item label="应用描述">
              <el-input 
                v-model="deployForm.description" 
                type="textarea" 
                :rows="3" 
                placeholder="请输入应用描述" 
              />
            </el-form-item>
            <el-form-item label="服务器地址">
              <el-input v-model="deployForm.server" placeholder="请输入服务器地址，例如：http://example.com" />
            </el-form-item>
            <el-form-item label="用户名">
              <el-input v-model="deployForm.username" placeholder="请输入用户名" />
            </el-form-item>
            <el-form-item label="密码">
              <el-input 
                v-model="deployForm.password" 
                type="password" 
                placeholder="请输入密码" 
                show-password
              />
            </el-form-item>
            <el-form-item>
              <el-button 
                type="primary" 
                @click="deployApplication"
                size="large"
              >
                部署应用
              </el-button>
            </el-form-item>
          </el-form>
        </div>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped lang="scss">
.creation-home {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;

  .header {
    text-align: center;
    margin-bottom: 30px;

    h1 {
      font-size: 2.5rem;
      margin-bottom: 10px;
      background: linear-gradient(135deg, #409eff, #66b1ff);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }

    p {
      font-size: 1.1rem;
      color: #666;
    }
  }

  .main-tabs {
    :deep(.el-tabs__header) {
      margin-bottom: 30px;
      
      .el-tabs__nav-wrap::after {
        height: 1px;
      }
      
      .el-tabs__item {
        font-size: 1.1rem;
        font-weight: 500;
        padding: 0 30px;
      }
    }
  }

  .content-area {
    display: flex;
    flex-direction: column;
    gap: 30px;
  }

  .input-section {
    .prompt-input {
      margin-bottom: 20px;
      
      :deep(.el-textarea__inner) {
        font-size: 16px;
        border-radius: 8px;
        padding: 16px;
        border: 1px solid #dcdfe6;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        
        &:focus {
          border-color: #409eff;
          box-shadow: 0 2px 12px rgba(64, 158, 255, 0.2);
        }
      }
    }

    .actions {
      display: flex;
      gap: 15px;
      justify-content: center;
    }
  }

  .preview-section {
    h3 {
      margin-bottom: 15px;
      font-size: 1.3rem;
    }

    .generated-image {
      max-width: 100%;
      max-height: 500px;
      border-radius: 8px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
      margin-bottom: 15px;
    }

    .code-preview {
      background: #1e1e1e;
      border-radius: 8px;
      padding: 20px;
      max-height: 300px;
      overflow: auto;
      margin-bottom: 15px;
      
      pre {
        color: #d4d4d4;
        font-family: 'Courier New', monospace;
        line-height: 1.5;
        margin: 0;
      }
    }

    .prompt-display {
      font-style: italic;
      color: #666;
      padding: 15px;
      background: #f5f7fa;
      border-radius: 8px;
      border-left: 4px solid #409eff;
    }
  }

  .resources-section {
    h3 {
      margin-bottom: 20px;
      font-size: 1.3rem;
    }

    .empty-resources {
      text-align: center;
      padding: 50px;
      color: #999;
    }

    .resources-grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
      gap: 20px;
    }

    .resource-card {
      border: 1px solid #ebeef5;
      border-radius: 8px;
      overflow: hidden;
      cursor: pointer;
      transition: all 0.3s ease;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);

      &:hover {
        transform: translateY(-5px);
        box-shadow: 0 6px 16px rgba(0, 0, 0, 0.1);
        border-color: #409eff;
      }

      .resource-image,
      .resource-code {
        height: 150px;
        display: flex;
        align-items: center;
        justify-content: center;
      }

      .resource-image {
        .thumbnail {
          width: 100%;
          height: 100%;
        }
      }

      .resource-code {
        background: linear-gradient(135deg, #1a3a6d, #2c5282);
        
        .code-icon {
          font-size: 3rem;
        }
      }

      .resource-info {
        padding: 15px;

        .resource-prompt {
          font-weight: 500;
          margin-bottom: 8px;
          display: -webkit-box;
          -webkit-line-clamp: 2;
          -webkit-box-orient: vertical;
          overflow: hidden;
        }

        .resource-date {
          font-size: 0.85rem;
          color: #999;
        }
      }
    }
  }

  .deploy-section {
    max-width: 600px;
    margin: 0 auto;

    h3 {
      margin-bottom: 20px;
      font-size: 1.3rem;
    }

    .deploy-form {
      :deep(.el-form-item__label) {
        font-weight: 500;
      }

      :deep(.el-input__inner),
      :deep(.el-textarea__inner) {
        border-radius: 8px;
      }
    }
  }
}
</style>