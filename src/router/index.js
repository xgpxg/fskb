import {createRouter, createWebHistory, createWebHashHistory} from 'vue-router'
import Layout from '@/layout/index.vue'

const routes = [
    {
        path: '/init',
        component: () => import('@/views/init/index.vue'),
        name: 'Init',
        meta: {title: '初始化', icon: 'init', affix: true,}
    },
    {
        path: '/',
        component: Layout,
        redirect: 'welcome',
        children: [
            {
                path: 'welcome',
                component: () => import('@/views/home/index.vue'),
                name: 'Welcome',
                meta: {title: '首页', icon: 'welcome', affix: true,}
            },
            {
                path: 'search',
                component: () => import('@/views/search/search.vue'),
                name: 'Search',
                meta: {title: '搜索', icon: 'search', affix: true,}
            },
            {
                path: 'chat/:knowledgeBaseId',
                component: () => import('@/views/chat/chat.vue'),
                name: 'Chat',
                meta: {title: '问答', icon: 'chat', affix: true,}
            },
        ]
    },
]


const router = createRouter({
    //history: createWebHashHistory(),  // hash 模式
    history: createWebHistory(),  // history 模式
    routes: routes
})

// 全局路由守卫
router.beforeEach((to, from, next) => {
    // console.log(to, from)
    if (to.meta.title) {
        document.title = `${to.meta.title}`;
    }
    next()
})

export default router
