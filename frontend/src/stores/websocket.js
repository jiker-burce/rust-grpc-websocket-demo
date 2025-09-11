import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useUserStore } from './user'
import { useChatStore } from './chat'

export const useWebSocketStore = defineStore('websocket', () => {
  const socket = ref(null)
  const connected = ref(false)
  const connecting = ref(false)

  const connect = async () => {
    if (connecting.value || connected.value) return

    const userStore = useUserStore()
    if (!userStore.isAuthenticated) return

    connecting.value = true

    try {
      // 使用原生WebSocket连接
      const wsUrl = `ws://localhost:8301`
      socket.value = new WebSocket(wsUrl)

      socket.value.onopen = () => {
        connected.value = true
        connecting.value = false
        console.log('WebSocket connected')
        
        // 加入默认房间
        joinRoom('general')
      }

      socket.value.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data)
          handleMessage(message)
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error)
        }
      }

      socket.value.onclose = () => {
        connected.value = false
        connecting.value = false
        console.log('WebSocket disconnected')
        
        // 尝试重连
        setTimeout(() => {
          if (!connected.value) {
            connect()
          }
        }, 3000)
      }

      socket.value.onerror = (error) => {
        connected.value = false
        connecting.value = false
        console.error('WebSocket error:', error)
      }
    } catch (error) {
      connecting.value = false
      console.error('Failed to connect WebSocket:', error)
    }
  }

  const disconnect = () => {
    if (socket.value) {
      socket.value.close()
      socket.value = null
    }
    connected.value = false
    connecting.value = false
  }

  const sendMessage = (message) => {
    if (socket.value && connected.value) {
      socket.value.send(JSON.stringify(message))
    }
  }

  const joinRoom = (roomId) => {
    const userStore = useUserStore()
    if (userStore.user) {
      sendMessage({
        type: 'join_room',
        room_id: roomId,
        user_id: userStore.user.id
      })
    }
  }

  const leaveRoom = (roomId) => {
    const userStore = useUserStore()
    if (userStore.user) {
      sendMessage({
        type: 'leave_room',
        room_id: roomId,
        user_id: userStore.user.id
      })
    }
  }

  const sendChatMessage = (content, messageType = 'text') => {
    const userStore = useUserStore()
    const chatStore = useChatStore()
    
    if (userStore.user) {
      console.log('发送聊天消息:', {
        type: 'chat_message',
        room_id: chatStore.currentRoom,
        user_id: userStore.user.id,
        content,
        message_type: messageType
      })
      
      sendMessage({
        type: 'chat_message',
        room_id: chatStore.currentRoom,
        user_id: userStore.user.id,
        content,
        message_type: messageType
      })
    }
  }

  const handleMessage = (message) => {
    const chatStore = useChatStore()
    
    console.log('收到WebSocket消息:', message)
    
    switch (message.type) {
      case 'chat_message':
        console.log('处理聊天消息:', message)
        // 移除临时消息（如果有的话）
        chatStore.removeTempMessage(message.content)
        // 添加正式消息
        chatStore.addMessage(message)
        break
      case 'user_online':
        console.log('用户上线:', message)
        // 更新在线用户列表
        chatStore.getOnlineUsers()
        break
      case 'user_offline':
        console.log('用户下线:', message)
        // 更新在线用户列表
        chatStore.getOnlineUsers()
        break
      case 'success':
        console.log('Success:', message.message)
        break
      case 'error':
        console.error('Error:', message.message)
        break
      default:
        console.log('Unknown message type:', message.type)
    }
  }

  return {
    socket,
    connected,
    connecting,
    connect,
    disconnect,
    sendMessage,
    joinRoom,
    leaveRoom,
    sendChatMessage,
    handleMessage
  }
})
