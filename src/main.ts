import {createApp} from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import 'virtual:svg-icons-register';
import router from './router'
import store from './store'
import PubSub from 'pubsub-js'

import VMdPreview from '@kangc/v-md-editor/lib/preview';
import '@kangc/v-md-editor/lib/style/preview.css';
import githubTheme from '@kangc/v-md-editor/lib/theme/github.js';
import '@kangc/v-md-editor/lib/theme/style/github.css';
import createKatexPlugin from '@kangc/v-md-editor/lib/plugins/katex/cdn';
import 'katex/dist/katex.css';
import createMermaidPlugin from '@kangc/v-md-editor/lib/plugins/mermaid/cdn';
import '@kangc/v-md-editor/lib/plugins/mermaid/mermaid.css';
import hljs from 'highlight.js';

VMdPreview.use(githubTheme, {
    Hljs: hljs,
});
VMdPreview.use(createKatexPlugin());
VMdPreview.use(createMermaidPlugin());

import VMdEditor from '@kangc/v-md-editor';
import '@kangc/v-md-editor/lib/style/base-editor.css';
import '@kangc/v-md-editor/lib/theme/style/github.css';

VMdEditor.use(githubTheme, {
    Hljs: hljs,
});



import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css'

import '@/styles/index.scss'

import i18n from './i18n/i18n.ts'

const app = createApp(App)

app.use(store)
app.use(router)
app.use(ElementPlus, {locale: zhCn,})
app.use(PubSub)
app.use(VMdPreview);
app.use(VMdEditor);
app.use(i18n);

app.mount("#app")

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}