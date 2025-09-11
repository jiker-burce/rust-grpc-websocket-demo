import { createRouter, createWebHistory } from 'vue-router'
import { useUserStore } from '@/stores/user'

const routes = [
  {
    path: '/',
    redirect: '/login'
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/Login.vue'),
    meta: { requiresGuest: true }
  },
  {
    path: '/register',
    name: 'Register',
    component: () => import('@/views/Register.vue'),
    meta: { requiresGuest: true }
  },
  {
    path: '/chat',
    name: 'Chat',
    component: () => import('@/views/Chat.vue'),
    meta: { requiresAuth: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const userStore = useUserStore()
  const isAuthenticated = userStore.isAuthenticated
  
  console.log('路由守卫:', {
    to: to.path,
    from: from.path,
    isAuthenticated,
    user: userStore.user,
    token: userStore.token
  })

  if (to.meta.requiresAuth && !isAuthenticated) {
    console.log('需要认证，重定向到登录页')
    next('/login')
  } else if (to.meta.requiresGuest && isAuthenticated) {
    console.log('已认证，重定向到聊天页')
    next('/chat')
  } else {
    console.log('正常导航')
    next()
  }
})

export default router
