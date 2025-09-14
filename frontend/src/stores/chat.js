import { defineStore } from 'pinia'
import { ref } from 'vue'
import { grpcClient } from '@/services/grpc'
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

export const useChatStore = defineStore('chat', () => {
  const messages = ref([])
  const onlineUsers = ref([])
  const currentRoom = ref('general')
  const loading = ref(false)

  const addMessage = (message) => {
    console.log('添加消息到聊天列表:', message)
    const formattedMessage = {
      ...message,
      id: message.id || `temp_${Date.now()}_${Math.random()}`, // 确保有ID字段
      timestamp: message.timestamp ? formatTimestamp(message.timestamp) : dayjs().format('MM-DD HH:mm:ss')
    }
    console.log('格式化后的消息:', formattedMessage)
    messages.value.push(formattedMessage)
    console.log('当前消息列表:', messages.value)
  }

  const removeTempMessage = (content) => {
    console.log('移除临时消息:', content)
    const index = messages.value.findIndex(msg => 
      msg.content === content && msg.is_temp === true
    )
    if (index !== -1) {
      messages.value.splice(index, 1)
      console.log('临时消息已移除')
    }
  }

  const setMessages = (messageList) => {
    console.log('从API获取的消息列表（已按时间正序排列）:', messageList.map(msg => ({
      content: msg.content,
      timestamp: msg.timestamp,
      username: msg.username
    })))
    
    // 直接使用API返回的消息列表，API已经按时间正序排列（最新的在底部）
    messages.value = messageList.map(msg => ({
      ...msg,
      timestamp: formatTimestamp(msg.timestamp)
    }))
    
    console.log('最终显示的消息列表:', messages.value.map(msg => ({
      content: msg.content,
      timestamp: msg.timestamp,
      username: msg.username
    })))
  }

  const setOnlineUsers = (users) => {
    onlineUsers.value = users
  }

  const setCurrentRoom = (roomId) => {
    currentRoom.value = roomId
    // 不要立即清空消息，让loadMessages来处理
  }

  const sendMessage = async (content, messageType = 'text') => {
    try {
      // 这里不再直接发送消息，而是通过WebSocket store发送
      // 因为消息会通过gRPC流返回
      return { success: true }
    } catch (error) {
      return { 
        success: false, 
        message: error.message || '发送失败，请重试' 
      }
    }
  }

  const loadMessages = async () => {
    loading.value = true
    try {
      console.log('正在通过gRPC加载房间消息:', currentRoom.value)
      // 通过gRPC流式传输获取消息
      // 消息会通过WebSocket store的流监听器自动添加到messages
      return { success: true }
    } catch (error) {
      console.error('加载消息失败:', error)
      return { 
        success: false, 
        message: error.message || '加载消息失败' 
      }
    } finally {
      loading.value = false
    }
  }

  const getOnlineUsers = async () => {
    try {
      console.log('通过WebSocket获取在线用户，房间ID:', currentRoom.value)
      
      // 使用WebSocket store来获取在线用户
      const websocketStore = useWebSocketStore()
      await websocketStore.getOnlineUsers(currentRoom.value)
      
      return { success: true }
    } catch (error) {
      console.error('获取在线用户失败:', error)
      return { 
        success: false, 
        message: error.message || '获取在线用户失败' 
      }
    }
  }

  const joinRoom = async (roomId) => {
    try {
      // 先清空消息，再设置房间
      messages.value = []
      setCurrentRoom(roomId)
      await loadMessages()
      await getOnlineUsers()
      return { success: true }
    } catch (error) {
      return { 
        success: false, 
        message: error.message || '加入房间失败' 
      }
    }
  }

  const leaveRoom = async (roomId) => {
    try {
      // 直接返回成功，不依赖HTTP API
      return { success: true, message: '成功离开房间' }
    } catch (error) {
      return { 
        success: false, 
        message: error.message || '离开房间失败' 
      }
    }
  }

  return {
    messages,
    onlineUsers,
    currentRoom,
    loading,
    addMessage,
    removeTempMessage,
    setMessages,
    setOnlineUsers,
    setCurrentRoom,
    sendMessage,
    loadMessages,
    getOnlineUsers,
    joinRoom,
    leaveRoom
  }
})
