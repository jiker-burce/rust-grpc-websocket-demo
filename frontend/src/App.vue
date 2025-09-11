<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
import { useUserStore } from '@/stores/user'
import { useWebSocketStore } from '@/stores/websocket'

const userStore = useUserStore()
const wsStore = useWebSocketStore()

onMounted(async () => {
  // 检查本地存储的用户信息
  const token = localStorage.getItem('token')
  const userInfo = localStorage.getItem('userInfo')
  
  if (token && userInfo) {
    try {
      userStore.setUser(JSON.parse(userInfo))
      userStore.setToken(token)
      
      // 连接WebSocket
      await wsStore.connect()
    } catch (error) {
      console.error('Failed to restore user session:', error)
      localStorage.removeItem('token')
      localStorage.removeItem('userInfo')
    }
  }
})
</script>

<style>
#app {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  height: 100vh;
  margin: 0;
  padding: 0;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  background-color: #f5f5f5;
}
</style>
