import { defineStore } from 'pinia'
import { ref } from 'vue'
import { chatApi } from '@/services/api'
import dayjs from 'dayjs'

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
      timestamp: message.timestamp ? dayjs(message.timestamp * 1000).format('HH:mm:ss') : dayjs().format('HH:mm:ss')
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
      timestamp: dayjs(msg.timestamp * 1000).format('HH:mm:ss')
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
      const response = await chatApi.sendMessage({
        content,
        room_id: currentRoom.value,
        message_type: messageType
      })
      
      if (response.data.success) {
        addMessage(response.data.chat_message)
        return { success: true }
      } else {
        return { success: false, message: response.data.message }
      }
    } catch (error) {
      return { 
        success: false, 
        message: error.response?.data?.message || '发送失败，请重试' 
      }
    }
  }

  const loadMessages = async () => {
    loading.value = true
    try {
      console.log('正在加载房间消息:', currentRoom.value, '无数量限制')
      const response = await chatApi.getMessages(currentRoom.value)
      console.log('收到消息响应:', response.data)
      setMessages(response.data)
      console.log('设置后的消息列表:', messages.value)
      return { success: true }
    } catch (error) {
      console.error('加载消息失败:', error)
      return { 
        success: false, 
        message: error.response?.data?.message || '加载消息失败' 
      }
    } finally {
      loading.value = false
    }
  }

  const getOnlineUsers = async () => {
    try {
      console.log('获取在线用户，房间ID:', currentRoom.value)
      const response = await chatApi.getOnlineUsers(currentRoom.value)
      console.log('在线用户API响应:', response.data)
      setOnlineUsers(response.data.users)
      console.log('设置后的在线用户:', onlineUsers.value)
      return { success: true }
    } catch (error) {
      console.error('获取在线用户失败:', error)
      return { 
        success: false, 
        message: error.response?.data?.message || '获取在线用户失败' 
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
