import { ChatServiceClient } from './grpc-generated/ChatServiceClientPb'
import './grpc-generated/chat_pb.js'

// 从全局对象获取protobuf类
const proto = global?.proto?.chat || window?.proto?.chat

if (!proto) {
  throw new Error('chat_pb.js not loaded properly')
}

const {
  GetMessagesRequest, 
  GetMessagesResponse,
  SendMessageRequest,
  SendMessageResponse
} = proto

/**
 * gRPC客户端服务 - 专注请求响应操作
 * 职责：处理数据查询、业务操作、消息存储
 */
class GrpcClientService {
  constructor() {
    // 创建gRPC客户端
    this.client = new ChatServiceClient('http://localhost:50051', null, null)
  }

  /**
   * 获取历史消息 - gRPC请求响应
   */
  async getMessages(roomId, limit = 100, beforeTimestamp = 0) {
    try {
      console.log('gRPC客户端获取历史消息:', { roomId, limit, beforeTimestamp })
      
      const request = new GetMessagesRequest()
      request.setRoomId(roomId)
      request.setLimit(limit)
      request.setBeforeTimestamp(beforeTimestamp)

      const response = await this.client.getMessages(request, {})
      console.log('gRPC客户端收到历史消息响应:', response)
      
      return response.getMessagesList().map(msg => ({
        id: msg.getId(),
        user_id: msg.getUserId(),
        username: msg.getUsername(),
        content: msg.getContent(),
        room_id: msg.getRoomId(),
        message_type: msg.getMessageType(),
        timestamp: msg.getTimestamp()
      }))
    } catch (error) {
      console.error('gRPC客户端获取历史消息失败:', error)
      throw error
    }
  }

  /**
   * 发送消息 - gRPC请求响应（用于消息存储）
   */
  async sendMessage(userId, roomId, content, messageType = 0) {
    try {
      console.log('gRPC客户端发送消息:', { userId, roomId, content, messageType })
      
      const request = new SendMessageRequest()
      request.setUserId(userId)
      request.setRoomId(roomId)
      request.setContent(content)
      request.setMessageType(messageType)
      request.setTimestamp(Date.now())

      const response = await this.client.sendMessage(request, {})
      console.log('gRPC客户端收到发送消息响应:', response)
      
      return {
        messageId: response.getMessage(),
        success: response.getSuccess()
      }
    } catch (error) {
      console.error('gRPC客户端发送消息失败:', error)
      throw error
    }
  }

  // 注意：获取在线用户现在通过WebSocket处理，不再使用gRPC

  // 注意：加入/离开房间现在通过WebSocket处理，不再使用gRPC
}

// 创建单例实例
const grpcClient = new GrpcClientService()

export default grpcClient
