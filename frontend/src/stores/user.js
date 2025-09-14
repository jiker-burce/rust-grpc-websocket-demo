import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { grpcClient } from '@/services/grpc'

export const useUserStore = defineStore('user', () => {
  const user = ref(null)
  const token = ref(null)
  const loading = ref(false)

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  const setUser = (userData) => {
    user.value = userData
    localStorage.setItem('userInfo', JSON.stringify(userData))
  }

  const setToken = (tokenValue) => {
    token.value = tokenValue
    localStorage.setItem('token', tokenValue)
  }

  const clearUser = () => {
    user.value = null
    token.value = null
    localStorage.removeItem('userInfo')
    localStorage.removeItem('token')
  }

  const login = async (email, password) => {
    loading.value = true
    try {
      const response = await grpcClient.login(email, password)
      console.log('gRPC登录响应:', response)
      if (response.success) {
        setUser(response.user)
        setToken(response.token)
        console.log('用户信息已设置:', user.value)
        console.log('Token已设置:', token.value)
        return { success: true, message: response.message }
      } else {
        return { success: false, message: response.message }
      }
    } catch (error) {
      console.error('gRPC登录错误:', error)
      return { 
        success: false, 
        message: error.message || '登录失败，请重试' 
      }
    } finally {
      loading.value = false
    }
  }

  const register = async (username, email, password) => {
    loading.value = true
    try {
      const response = await grpcClient.register(username, email, password)
      if (response.success) {
        return { success: true, message: response.message }
      } else {
        return { success: false, message: response.message }
      }
    } catch (error) {
      return { 
        success: false, 
        message: error.message || '注册失败，请重试' 
      }
    } finally {
      loading.value = false
    }
  }

  const logout = () => {
    clearUser()
  }

  const updateProfile = async (username, avatar) => {
    loading.value = true
    try {
      const response = await grpcClient.updateUser(user.value.id, username, avatar)
      if (response.success) {
        setUser(response.user)
        return { success: true, message: response.message }
      } else {
        return { success: false, message: response.message }
      }
    } catch (error) {
      return { 
        success: false, 
        message: error.message || '更新失败，请重试' 
      }
    } finally {
      loading.value = false
    }
  }

  return {
    user,
    token,
    loading,
    isAuthenticated,
    setUser,
    setToken,
    clearUser,
    login,
    register,
    logout,
    updateProfile
  }
})
