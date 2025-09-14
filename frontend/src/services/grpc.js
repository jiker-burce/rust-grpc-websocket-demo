// 真正的gRPC-Web客户端实现
// 使用从proto文件生成的TypeScript代码

import { grpc } from 'grpc-web'
import { 
  UserServiceClient,
  ChatServiceClient 
} from './grpc-generated/ChatServiceClientPb'
import {
  RegisterRequest,
  RegisterResponse,
  LoginRequest,
  LoginResponse,
  GetUserRequest,
  GetUserResponse,
  UpdateUserRequest,
  UpdateUserResponse,
  SendMessageRequest,
  SendMessageResponse,
  GetMessagesRequest,
  GetMessagesResponse,
  GetOnlineUsersRequest,
  GetOnlineUsersResponse,
  JoinRoomRequest,
  JoinRoomResponse,
  LeaveRoomRequest,
  LeaveRoomResponse,
  MessageType
} from './grpc-generated/chat_pb_wrapper.js'

// 创建gRPC客户端
const userServiceClient = new UserServiceClient('http://localhost:3002/grpc', null, {
  'format': 'text'
})
const chatServiceClient = new ChatServiceClient('http://localhost:3002/grpc', null, {
  'format': 'text'
})

// gRPC客户端类
export class GrpcClient {
  constructor() {
    console.log('GrpcClient initialized with real gRPC-Web')
  }

  // 用户相关方法
  async register(username, email, password) {
    try {
      const request = new RegisterRequest()
      request.setUsername(username)
      request.setEmail(email)
      request.setPassword(password)

      const response = await new Promise((resolve, reject) => {
        userServiceClient.register(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      return {
        success: response.getSuccess(),
        message: response.getMessage(),
        user: response.getUser() ? {
          id: response.getUser().getId(),
          username: response.getUser().getUsername(),
          email: response.getUser().getEmail(),
          avatar: response.getUser().getAvatar(),
          is_online: response.getUser().getIsOnline(),
          created_at: response.getUser().getCreatedAt(),
          updated_at: response.getUser().getUpdatedAt()
        } : null
      }
    } catch (error) {
      console.error('gRPC register error:', error)
      throw new Error('注册失败: ' + error.message)
    }
  }

  async login(email, password) {
    try {
      const request = new LoginRequest()
      request.setEmail(email)
      request.setPassword(password)

      const response = await new Promise((resolve, reject) => {
        userServiceClient.login(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      return {
        success: response.getSuccess(),
        message: response.getMessage(),
        token: response.getToken(),
        user: response.getUser() ? {
          id: response.getUser().getId(),
          username: response.getUser().getUsername(),
          email: response.getUser().getEmail(),
          avatar: response.getUser().getAvatar(),
          is_online: response.getUser().getIsOnline(),
          created_at: response.getUser().getCreatedAt(),
          updated_at: response.getUser().getUpdatedAt()
        } : null
      }
    } catch (error) {
      console.error('gRPC login error:', error)
      throw new Error('登录失败: ' + error.message)
    }
  }

  async getUser(userId) {
    try {
      const request = new GetUserRequest()
      request.setUserId(userId)

      const response = await new Promise((resolve, reject) => {
        userServiceClient.getUser(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      return {
        success: response.getSuccess(),
        user: response.getUser() ? {
          id: response.getUser().getId(),
          username: response.getUser().getUsername(),
          email: response.getUser().getEmail(),
          avatar: response.getUser().getAvatar(),
          is_online: response.getUser().getIsOnline(),
          created_at: response.getUser().getCreatedAt(),
          updated_at: response.getUser().getUpdatedAt()
        } : null
      }
    } catch (error) {
      console.error('gRPC getUser error:', error)
      throw new Error('获取用户信息失败: ' + error.message)
    }
  }

  async updateUser(userId, username, avatar) {
    try {
      const request = new UpdateUserRequest()
      request.setUserId(userId)
      request.setUsername(username)
      request.setAvatar(avatar || '')

      const response = await new Promise((resolve, reject) => {
        userServiceClient.updateUser(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      return {
        success: response.getSuccess(),
        message: response.getMessage(),
        user: response.getUser() ? {
          id: response.getUser().getId(),
          username: response.getUser().getUsername(),
          email: response.getUser().getEmail(),
          avatar: response.getUser().getAvatar(),
          is_online: response.getUser().getIsOnline(),
          created_at: response.getUser().getCreatedAt(),
          updated_at: response.getUser().getUpdatedAt()
        } : null
      }
    } catch (error) {
      console.error('gRPC updateUser error:', error)
      throw new Error('更新用户信息失败: ' + error.message)
    }
  }

  // 聊天相关方法
  async sendMessage(userId, content, roomId, messageType = 'text') {
    try {
      const request = new SendMessageRequest()
      request.setUserId(userId)
      request.setContent(content)
      request.setRoomId(roomId)
      
      // 转换消息类型
      let type = MessageType.TEXT
      switch (messageType) {
        case 'image':
          type = MessageType.IMAGE
          break
        case 'file':
          type = MessageType.FILE
          break
        case 'system':
          type = MessageType.SYSTEM
          break
        default:
          type = MessageType.TEXT
      }
      request.setMessageType(type)

      const response = await new Promise((resolve, reject) => {
        chatServiceClient.sendMessage(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      return {
        success: response.getSuccess(),
        message: response.getMessage(),
        chat_message: response.getChatMessage() ? {
          id: response.getChatMessage().getId(),
          user_id: response.getChatMessage().getUserId(),
          username: response.getChatMessage().getUsername(),
          content: response.getChatMessage().getContent(),
          room_id: response.getChatMessage().getRoomId(),
          message_type: response.getChatMessage().getMessageType(),
          timestamp: response.getChatMessage().getTimestamp()
        } : null
      }
    } catch (error) {
      console.error('gRPC sendMessage error:', error)
      throw new Error('发送消息失败: ' + error.message)
    }
  }

  // 获取历史消息（一次性调用，不使用轮询）
  async getMessages(roomId, limit = 100, beforeTimestamp = 0) {
    console.log('gRPC getMessages called with:', { roomId, limit, beforeTimestamp })
    
    try {
      const request = new GetMessagesRequest()
      request.setRoomId(roomId)
      request.setLimit(limit)
      request.setBeforeTimestamp(beforeTimestamp)

      const response = await new Promise((resolve, reject) => {
        chatServiceClient.getMessages(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      if (response && response.getMessagesList) {
        const messages = response.getMessagesList()
        console.log(`gRPC loaded ${messages.length} messages`)
        
        // 转换为标准格式
        return messages.map(message => ({
          id: message.getId(),
          user_id: message.getUserId(),
          username: message.getUsername(),
          content: message.getContent(),
          room_id: message.getRoomId(),
          message_type: message.getMessageType(),
          timestamp: message.getTimestamp()
        }))
      }
      
      return []
    } catch (error) {
      console.error('gRPC getMessages error:', error)
      throw new Error('获取消息失败: ' + error.message)
    }
  }

  async getOnlineUsers(roomId) {
    try {
      const request = new GetOnlineUsersRequest()
      request.setRoomId(roomId)

      const response = await new Promise((resolve, reject) => {
        chatServiceClient.getOnlineUsers(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      return {
        users: response.getUsersList().map(user => ({
          id: user.getId(),
          username: user.getUsername(),
          email: user.getEmail(),
          avatar: user.getAvatar(),
          is_online: user.getIsOnline(),
          created_at: user.getCreatedAt(),
          updated_at: user.getUpdatedAt()
        }))
      }
    } catch (error) {
      console.error('gRPC getOnlineUsers error:', error)
      throw new Error('获取在线用户失败: ' + error.message)
    }
  }

  async joinRoom(userId, roomId) {
    try {
      const request = new JoinRoomRequest()
      request.setUserId(userId)
      request.setRoomId(roomId)

      const response = await new Promise((resolve, reject) => {
        chatServiceClient.joinRoom(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      return {
        success: response.getSuccess(),
        message: response.getMessage()
      }
    } catch (error) {
      console.error('gRPC joinRoom error:', error)
      throw new Error('加入房间失败: ' + error.message)
    }
  }

  async leaveRoom(userId, roomId) {
    try {
      const request = new LeaveRoomRequest()
      request.setUserId(userId)
      request.setRoomId(roomId)

      const response = await new Promise((resolve, reject) => {
        chatServiceClient.leaveRoom(request, {}, (err, response) => {
          if (err) reject(err)
          else resolve(response)
        })
      })

      return {
        success: response.getSuccess(),
        message: response.getMessage()
      }
    } catch (error) {
      console.error('gRPC leaveRoom error:', error)
      throw new Error('离开房间失败: ' + error.message)
    }
  }
}

// 创建单例实例
export const grpcClient = new GrpcClient()