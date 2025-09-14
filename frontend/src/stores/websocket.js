import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useUserStore } from './user'
import { useChatHybridStore } from './chat-hybrid'

// WebSocket消息处理函数 - 专注实时通信
const handleRealtimeMessage = (message) => {
  switch (message.type) {
    case 'new_message':
      // 新消息推送 - 实时通信
      return {
        type: 'new_message',
        message: message.message
      }
    
    case 'user_joined':
      // 用户加入房间通知 - 实时状态同步
      return {
        type: 'user_joined',
        user_id: message.user_id,
        username: message.username,
        room_id: message.room_id
      }
    
    case 'user_left':
      // 用户离开房间通知 - 实时状态同步
      return {
        type: 'user_left',
        user_id: message.user_id,
        username: message.username,
        room_id: message.room_id
      }
    
    case 'ping':
      // 心跳响应 - 连接保活
      return {
        type: 'pong',
        timestamp: message.timestamp
      }
    
    default:
      console.warn('Unknown WebSocket message type:', message.type)
      return message
  }
}

export const useWebSocketStore = defineStore('websocket', () => {
  const messageStream = ref(null)
  const connected = ref(false)
  const connecting = ref(false)
  const currentUserId = ref(null)

  const connect = async () => {
    if (connecting.value || connected.value) return

    const userStore = useUserStore()
    if (!userStore.isAuthenticated) return

    connecting.value = true

    try {
      console.log('Connecting to WebSocket service...')
      
      // 使用后端数据库中的真实用户ID
      const userId = userStore.user.id
      currentUserId.value = userId
      console.log('使用数据库用户ID:', userId)
      
      // 使用WebSocket进行实时通信
      const wsUrl = `ws://localhost:8301/ws`
      const socket = new WebSocket(wsUrl)
      messageStream.value = socket
      
      socket.onopen = () => {
        console.log('WebSocket connected')
        connected.value = true
        connecting.value = false
        
        // 加入默认房间
        joinRoom('general')
        
        // 加载历史消息
        loadHistoryMessages('general')
      }
      
                 socket.onmessage = (event) => {
                   try {
                     // 解析WebSocket消息
                     const message = JSON.parse(event.data)
                     console.log('收到WebSocket消息:', message)
                     
                     // 直接处理JSON消息
                     handleMessage(message)
                   } catch (error) {
                     console.error('解析WebSocket消息失败:', error)
                   }
                 }
      
      socket.onerror = (error) => {
        console.error('WebSocket error:', error)
        connected.value = false
        connecting.value = false
      }
      
      socket.onclose = () => {
        console.log('WebSocket disconnected')
        connected.value = false
        connecting.value = false
      }
      
    } catch (error) {
      connecting.value = false
      console.error('Failed to connect WebSocket service:', error)
    }
  }

  const disconnect = () => {
    if (messageStream.value) {
      if (messageStream.value.close) {
        // WebSocket
        messageStream.value.close()
      } else if (messageStream.value.cancel) {
        // gRPC stream
        messageStream.value.cancel()
      }
      messageStream.value = null
    }
    connected.value = false
    connecting.value = false
    currentUserId.value = null
  }

  // WebSocket不再处理历史消息查询，这些由gRPC客户端处理
  const loadHistoryMessages = async (roomId) => {
    console.log('历史消息查询已移至gRPC客户端处理')
  }

  // 发送消息（WebSocket实时通信）
  const sendMessage = async (message) => {
    if (connected.value && messageStream.value) {
      try {
        console.log('通过WebSocket发送实时消息...')
        
        // 创建SendMessageRequest
        const request = new SendMessageRequest()
        request.setUserId(message.user_id)
        request.setContent(message.content)
        request.setRoomId(message.room_id)
        request.setMessageType(MessageType[message.message_type?.toUpperCase()] || MessageType.TEXT)
        request.setTimestamp(Date.now())
        
        // 序列化为protobuf二进制数据发送
        const binaryData = request.serializeBinary()
        
        // 创建gRPC消息帧（简化版）
        const grpcMessage = {
          type: 'send_message',
          data: Array.from(binaryData) // 转换为数组以便JSON传输
        }
        
        messageStream.value.send(JSON.stringify(grpcMessage))
        console.log('实时消息已通过WebSocket发送')
      } catch (error) {
        console.error('WebSocket发送消息失败:', error)
      }
    }
  }

  // 加入房间（WebSocket实时通信）
  const joinRoom = async (roomId) => {
    if (connected.value && messageStream.value && currentUserId.value) {
      try {
        console.log(`通过WebSocket加入房间 ${roomId}...`)
        
        // 发送包含房间ID和用户ID的JSON消息
        const message = {
          type: 'join_room',
          room_id: roomId,
          user_id: currentUserId.value
        }
        
        messageStream.value.send(JSON.stringify(message))
        console.log(`已加入房间 ${roomId}`)
      } catch (error) {
        console.error('WebSocket加入房间失败:', error)
      }
    }
  }

  // 离开房间（WebSocket实时通信）
  const leaveRoom = async (roomId) => {
    if (connected.value && messageStream.value && currentUserId.value) {
      try {
        console.log(`通过WebSocket离开房间 ${roomId}...`)
        
        // 发送包含房间ID和用户ID的JSON消息
        const message = {
          type: 'leave_room',
          room_id: roomId,
          user_id: currentUserId.value
        }
        
        messageStream.value.send(JSON.stringify(message))
        console.log(`已离开房间 ${roomId}`)
      } catch (error) {
        console.error('WebSocket离开房间失败:', error)
      }
    }
  }

  // 获取在线用户（WebSocket实时通信）
  const getOnlineUsers = async (roomId) => {
    if (connected.value && messageStream.value) {
      try {
        console.log(`通过WebSocket获取房间 ${roomId} 的在线用户...`)
        
        // 发送包含房间ID的JSON消息
        const message = {
          type: 'get_online_users',
          room_id: roomId
        }
        
        messageStream.value.send(JSON.stringify(message))
        console.log('已请求在线用户列表')
      } catch (error) {
        console.error('WebSocket获取在线用户失败:', error)
      }
    }
  }

  // 发送聊天消息（WebSocket实时通信）
  const sendChatMessage = async (content, messageType = 'text') => {
    const chatStore = useChatHybridStore()
    
    if (currentUserId.value) {
      console.log('通过WebSocket发送实时聊天消息:', {
        room_id: chatStore.currentRoom,
        user_id: currentUserId.value,
        content,
        message_type: messageType
      })
      
      try {
        // 发送包含消息内容的JSON消息
        const message = {
          type: 'send_message',
          user_id: currentUserId.value,
          content,
          room_id: chatStore.currentRoom,
          message_type: messageType
        }
        
        if (connected.value && messageStream.value) {
          messageStream.value.send(JSON.stringify(message))
          console.log('实时消息已发送')
        }
      } catch (error) {
        console.error('WebSocket发送聊天消息失败:', error)
      }
    }
  }

  // 处理WebSocket实时消息
  const handleMessage = (message) => {
    const chatStore = useChatHybridStore()
    
    console.log('收到WebSocket实时消息:', message)
    console.log('消息类型:', message.type)
    
    // 处理实时消息
    switch (message.type) {
      case 'new_message':
        console.log('处理新消息推送:', message.message)
        console.log('消息详情:', JSON.stringify(message.message, null, 2))
        chatStore.addMessage(message.message)
        console.log('消息已添加到聊天列表')
        break
        
      case 'user_joined':
        console.log('用户加入房间:', message.username)
        // 可以在这里更新在线用户列表
        break
        
      case 'user_left':
        console.log('用户离开房间:', message.username)
        // 可以在这里更新在线用户列表
        break
        
      case 'online_users_list':
        console.log('收到在线用户列表:', message.users)
        console.log('房间ID:', message.room_id)
        // 更新聊天store中的在线用户列表
        chatStore.updateOnlineUsers(message.room_id, message.users)
        break
        
      case 'pong':
        console.log('收到心跳响应:', message.timestamp)
        break
        
      default:
        console.log('处理其他WebSocket消息:', message)
    }
  }

  return {
    messageStream,
    connected,
    connecting,
    connect,
    disconnect,
    sendMessage,
    joinRoom,
    leaveRoom,
    getOnlineUsers,
    sendChatMessage,
    handleMessage
  }
})
