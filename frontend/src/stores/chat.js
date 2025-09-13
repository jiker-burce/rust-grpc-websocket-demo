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
    messages.value = messageList.map(msg => ({
      ...msg,
      timestamp: dayjs(msg.timestamp * 1000).format('HH:mm:ss')
    }))
  }

  const setOnlineUsers = (users) => {
    onlineUsers.value = users
  }

  const setCurrentRoom = (roomId) => {
    currentRoom.value = roomId
    messages.value = []
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

  const loadMessages = async (limit = 50) => {
    loading.value = true
    try {
      const response = await chatApi.getMessages(currentRoom.value, limit)
      setMessages(response.data)
      return { success: true }
    } catch (error) {
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
      const response = await chatApi.joinRoom(roomId)
      if (response.data.success) {
        setCurrentRoom(roomId)
        await loadMessages()
        await getOnlineUsers()
        return { success: true }
      } else {
        return { success: false, message: response.data.message }
      }
    } catch (error) {
      return { 
        success: false, 
        message: error.response?.data?.message || '加入房间失败' 
      }
    }
  }

  const leaveRoom = async (roomId) => {
    try {
      const response = await chatApi.leaveRoom(roomId)
      return { success: response.data.success, message: response.data.message }
    } catch (error) {
      return { 
        success: false, 
        message: error.response?.data?.message || '离开房间失败' 
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
