// ES6模块包装器，用于chat_pb.js
// 导入chat_pb.js
import './chat_pb.js'

// 从全局对象中获取导出的类
const proto = global?.proto?.chat || window?.proto?.chat

if (!proto) {
  throw new Error('chat_pb.js not loaded properly')
}

// 导出所有需要的类
export const ChatMessage = proto.ChatMessage
export const GetMessagesRequest = proto.GetMessagesRequest
export const GetMessagesResponse = proto.GetMessagesResponse
export const GetOnlineUsersRequest = proto.GetOnlineUsersRequest
export const GetOnlineUsersResponse = proto.GetOnlineUsersResponse
export const GetUserRequest = proto.GetUserRequest
export const GetUserResponse = proto.GetUserResponse
export const JoinRoomRequest = proto.JoinRoomRequest
export const JoinRoomResponse = proto.JoinRoomResponse
export const LeaveRoomRequest = proto.LeaveRoomRequest
export const LeaveRoomResponse = proto.LeaveRoomResponse
export const LoginRequest = proto.LoginRequest
export const LoginResponse = proto.LoginResponse
export const RegisterRequest = proto.RegisterRequest
export const RegisterResponse = proto.RegisterResponse
export const SendMessageRequest = proto.SendMessageRequest
export const SendMessageResponse = proto.SendMessageResponse
export const UpdateUserRequest = proto.UpdateUserRequest
export const UpdateUserResponse = proto.UpdateUserResponse
export const User = proto.User
export const MessageType = proto.MessageType
