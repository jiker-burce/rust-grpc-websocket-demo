import axios from 'axios'
import { useUserStore } from '@/stores/user'

// 创建axios实例
const api = axios.create({
  baseURL: '/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器
api.interceptors.request.use(
  (config) => {
    const userStore = useUserStore()
    if (userStore.token) {
      config.headers.Authorization = `Bearer ${userStore.token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
api.interceptors.response.use(
  (response) => {
    return response
  },
  (error) => {
    if (error.response?.status === 401) {
      // Token过期，清除用户信息
      const userStore = useUserStore()
      userStore.logout()
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

// 用户相关API
export const userApi = {
  // 用户注册
  register: (username, email, password) => {
    return api.post('/users/register', {
      username,
      email,
      password,
    })
  },

  // 用户登录
  login: (email, password) => {
    return api.post('/users/login', {
      email,
      password,
    })
  },

  // 获取用户信息
  getUser: (userId) => {
    return api.get(`/users/${userId}`)
  },

  // 更新用户信息
  updateProfile: (userId, username, avatar) => {
    return api.put(`/users/${userId}`, {
      username,
      avatar,
    })
  },
}

// 聊天相关API
export const chatApi = {
  // 发送消息
  sendMessage: (messageData) => {
    return api.post('/chat/messages', messageData)
  },

  // 获取消息列表
  getMessages: (roomId, limit = 50, beforeTimestamp = null) => {
    const params = { limit }
    if (beforeTimestamp) {
      params.before_timestamp = beforeTimestamp
    }
    return api.get(`/chat/rooms/${roomId}/messages`, { params })
  },

  // 获取在线用户
  getOnlineUsers: (roomId) => {
    return api.get(`/chat/rooms/${roomId}/users`)
  },

  // 加入房间
  joinRoom: (roomId) => {
    return api.post(`/chat/rooms/${roomId}/join`)
  },

  // 离开房间
  leaveRoom: (roomId) => {
    return api.post(`/chat/rooms/${roomId}/leave`)
  },

  // 获取房间列表
  getRooms: () => {
    return api.get('/chat/rooms')
  },
}

export default api
