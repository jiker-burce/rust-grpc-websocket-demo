import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useUserStore } from './user'
import { useWebSocketStore } from './websocket'
import grpcClient from '../services/grpc-client'
import dayjs from 'dayjs'

// 格式化时间戳为人类可读格式
const formatTimestamp = (timestamp) => {
  if (!timestamp) return dayjs().format('MM-DD HH:mm:ss')
  
  // 判断时间戳是秒还是毫秒
  // 如果时间戳小于 10000000000，认为是秒，否则认为是毫秒
  const timestampMs = timestamp < 10000000000 ? timestamp * 1000 : timestamp
  const messageTime = dayjs(timestampMs)
  const now = dayjs()
  
  // 如果是今天，只显示时间
  if (messageTime.isSame(now, 'day')) {
    return messageTime.format('HH:mm:ss')
  }
  // 如果是昨天，显示昨天 + 时间
  else if (messageTime.isSame(now.subtract(1, 'day'), 'day')) {
    return `昨天 ${messageTime.format('HH:mm:ss')}`
  }
  // 如果是今年，显示月日 + 时间
  else if (messageTime.isSame(now, 'year')) {
    return messageTime.format('MM-DD HH:mm:ss')
  }
  // 其他情况显示完整日期 + 时间
  else {
    return messageTime.format('YYYY-MM-DD HH:mm:ss')
  }
}

/**
 * 混合架构聊天store
 * 结合WebSocket实时通信和gRPC请求响应
 */
export const useChatHybridStore = defineStore('chatHybrid', () => {
  const messages = ref([])
  const currentRoom = ref('general')
  const onlineUsers = ref([])
  const loading = ref(false)
  const error = ref(null)

  const userStore = useUserStore()
  const wsStore = useWebSocketStore()

  /**
   * 加载历史消息 - 使用gRPC
   */
  const loadHistoryMessages = async (roomId = currentRoom.value) => {
    try {
      loading.value = true
      error.value = null
      
      console.log('通过gRPC加载历史消息:', roomId)
      const historyMessages = await grpcClient.getMessages(roomId, 100, 0)
      
      // 更新消息列表，格式化时间戳
      messages.value = historyMessages.map(msg => ({
        ...msg,
        timestamp: formatTimestamp(msg.timestamp)
      }))
      console.log('历史消息加载完成:', historyMessages.length, '条')
      
    } catch (err) {
      console.error('加载历史消息失败:', err)
      error.value = '加载历史消息失败'
    } finally {
      loading.value = false
    }
  }

  /**
   * 发送消息 - 使用WebSocket实时通信
   */
  const sendMessage = async (content, messageType = 'text') => {
    if (!userStore.user) {
      console.error('用户未登录')
      return
    }

    try {
      console.log('通过WebSocket发送实时消息:', { content, messageType })
      
      // 使用WebSocket发送实时消息
      await wsStore.sendChatMessage(content, messageType)
      
      // 同时通过gRPC存储消息（可选，用于持久化）
      try {
        await grpcClient.sendMessage(
          userStore.user.id,
          currentRoom.value,
          content,
          messageType === 'text' ? 0 : 1
        )
        console.log('消息已通过gRPC存储')
      } catch (grpcError) {
        console.warn('gRPC存储消息失败，但WebSocket发送成功:', grpcError)
      }
      
    } catch (err) {
      console.error('发送消息失败:', err)
      error.value = '发送消息失败'
    }
  }

  /**
   * 加入房间 - 使用WebSocket实时通信
   */
  const joinRoom = async (roomId) => {
    try {
      console.log('通过WebSocket加入房间:', roomId)
      
      // 1. 通过WebSocket加入房间（实时通信）
      await wsStore.joinRoom(roomId)
      
      // 2. 更新当前房间
      currentRoom.value = roomId
      
      // 3. 加载历史消息（gRPC数据查询）
      await loadHistoryMessages(roomId)
      
      // 4. 获取在线用户（WebSocket实时通信）
      await loadOnlineUsers(roomId)
      
      console.log('成功加入房间:', roomId)
      
    } catch (err) {
      console.error('WebSocket加入房间失败:', err)
      error.value = '加入房间失败'
    }
  }

  /**
   * 离开房间 - 使用WebSocket实时通信
   */
  const leaveRoom = async (roomId = currentRoom.value) => {
    try {
      console.log('通过WebSocket离开房间:', roomId)
      
      // 1. 通过WebSocket离开房间（实时通信）
      await wsStore.leaveRoom(roomId)
      
      // 2. 清空当前房间数据
      if (currentRoom.value === roomId) {
        currentRoom.value = 'general'
        messages.value = []
        onlineUsers.value = []
      }
      
      console.log('成功离开房间:', roomId)
      
    } catch (err) {
      console.error('离开房间失败:', err)
      error.value = '离开房间失败'
    }
  }

  /**
   * 加载在线用户 - 使用WebSocket实时通信
   */
  const loadOnlineUsers = async (roomId = currentRoom.value) => {
    try {
      console.log('通过WebSocket加载在线用户:', roomId)
      await wsStore.getOnlineUsers(roomId)
      console.log('在线用户请求已发送')
      
    } catch (err) {
      console.error('WebSocket加载在线用户失败:', err)
      error.value = '加载在线用户失败'
    }
  }

  /**
   * 添加消息到列表（由WebSocket实时推送调用）
   */
  const addMessage = (message) => {
    console.log('添加实时消息到列表:', message)
    console.log('当前消息列表长度:', messages.value.length)
    
    // 格式化消息时间戳
    const formattedMessage = {
      ...message,
      timestamp: formatTimestamp(message.timestamp)
    }
    
    messages.value.push(formattedMessage)
    console.log('添加后消息列表长度:', messages.value.length)
    console.log('最新消息:', messages.value[messages.value.length - 1])
  }

  /**
   * 移除临时消息
   */
  const removeTempMessage = (content) => {
    const index = messages.value.findIndex(msg => 
      msg.content === content && msg.temp
    )
    if (index > -1) {
      messages.value.splice(index, 1)
    }
  }

  /**
   * 清空消息
   */
  const clearMessages = () => {
    messages.value = []
  }

  /**
   * 更新在线用户列表
   */
  const updateOnlineUsers = (roomId, users) => {
    console.log('更新在线用户列表:', { roomId, users })
    if (currentRoom.value === roomId) {
      onlineUsers.value = users
      console.log('当前房间在线用户已更新:', onlineUsers.value.length, '个用户')
    }
  }

  /**
   * 清空错误
   */
  const clearError = () => {
    error.value = null
  }

  return {
    // 状态
    messages,
    currentRoom,
    onlineUsers,
    loading,
    error,
    
    // 方法
    loadHistoryMessages,
    sendMessage,
    joinRoom,
    leaveRoom,
    loadOnlineUsers,
    addMessage,
    removeTempMessage,
    clearMessages,
    updateOnlineUsers,
    clearError
  }
})
