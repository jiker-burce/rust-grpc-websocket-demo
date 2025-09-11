import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { userApi } from '@/services/api'

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
      const response = await userApi.login(email, password)
      console.log('登录响应:', response.data)
      if (response.data.success) {
        setUser(response.data.data.user)
        setToken(response.data.data.token)
        console.log('用户信息已设置:', user.value)
        console.log('Token已设置:', token.value)
        return { success: true, message: response.data.message }
      } else {
        return { success: false, message: response.data.message }
      }
    } catch (error) {
      console.error('登录错误:', error)
      return { 
        success: false, 
        message: error.response?.data?.message || '登录失败，请重试' 
      }
    } finally {
      loading.value = false
    }
  }

  const register = async (username, email, password) => {
    loading.value = true
    try {
      const response = await userApi.register(username, email, password)
      if (response.data.success) {
        return { success: true, message: response.data.message }
      } else {
        return { success: false, message: response.data.message }
      }
    } catch (error) {
      return { 
        success: false, 
        message: error.response?.data?.message || '注册失败，请重试' 
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
      const response = await userApi.updateProfile(user.value.id, username, avatar)
      if (response.data.success) {
        setUser(response.data.user)
        return { success: true, message: response.data.message }
      } else {
        return { success: false, message: response.data.message }
      }
    } catch (error) {
      return { 
        success: false, 
        message: error.response?.data?.message || '更新失败，请重试' 
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
