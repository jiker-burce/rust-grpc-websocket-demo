import * as jspb from 'google-protobuf'



export class User extends jspb.Message {
  getId(): string;
  setId(value: string): User;

  getUsername(): string;
  setUsername(value: string): User;

  getEmail(): string;
  setEmail(value: string): User;

  getAvatar(): string;
  setAvatar(value: string): User;

  getIsOnline(): boolean;
  setIsOnline(value: boolean): User;

  getCreatedAt(): number;
  setCreatedAt(value: number): User;

  getUpdatedAt(): number;
  setUpdatedAt(value: number): User;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): User.AsObject;
  static toObject(includeInstance: boolean, msg: User): User.AsObject;
  static serializeBinaryToWriter(message: User, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): User;
  static deserializeBinaryFromReader(message: User, reader: jspb.BinaryReader): User;
}

export namespace User {
  export type AsObject = {
    id: string,
    username: string,
    email: string,
    avatar: string,
    isOnline: boolean,
    createdAt: number,
    updatedAt: number,
  }
}

export class RegisterRequest extends jspb.Message {
  getUsername(): string;
  setUsername(value: string): RegisterRequest;

  getEmail(): string;
  setEmail(value: string): RegisterRequest;

  getPassword(): string;
  setPassword(value: string): RegisterRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RegisterRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RegisterRequest): RegisterRequest.AsObject;
  static serializeBinaryToWriter(message: RegisterRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RegisterRequest;
  static deserializeBinaryFromReader(message: RegisterRequest, reader: jspb.BinaryReader): RegisterRequest;
}

export namespace RegisterRequest {
  export type AsObject = {
    username: string,
    email: string,
    password: string,
  }
}

export class RegisterResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): RegisterResponse;

  getMessage(): string;
  setMessage(value: string): RegisterResponse;

  getUser(): User | undefined;
  setUser(value?: User): RegisterResponse;
  hasUser(): boolean;
  clearUser(): RegisterResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RegisterResponse.AsObject;
  static toObject(includeInstance: boolean, msg: RegisterResponse): RegisterResponse.AsObject;
  static serializeBinaryToWriter(message: RegisterResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RegisterResponse;
  static deserializeBinaryFromReader(message: RegisterResponse, reader: jspb.BinaryReader): RegisterResponse;
}

export namespace RegisterResponse {
  export type AsObject = {
    success: boolean,
    message: string,
    user?: User.AsObject,
  }
}

export class LoginRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): LoginRequest;

  getPassword(): string;
  setPassword(value: string): LoginRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoginRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LoginRequest): LoginRequest.AsObject;
  static serializeBinaryToWriter(message: LoginRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoginRequest;
  static deserializeBinaryFromReader(message: LoginRequest, reader: jspb.BinaryReader): LoginRequest;
}

export namespace LoginRequest {
  export type AsObject = {
    email: string,
    password: string,
  }
}

export class LoginResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): LoginResponse;

  getMessage(): string;
  setMessage(value: string): LoginResponse;

  getToken(): string;
  setToken(value: string): LoginResponse;

  getUser(): User | undefined;
  setUser(value?: User): LoginResponse;
  hasUser(): boolean;
  clearUser(): LoginResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoginResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LoginResponse): LoginResponse.AsObject;
  static serializeBinaryToWriter(message: LoginResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoginResponse;
  static deserializeBinaryFromReader(message: LoginResponse, reader: jspb.BinaryReader): LoginResponse;
}

export namespace LoginResponse {
  export type AsObject = {
    success: boolean,
    message: string,
    token: string,
    user?: User.AsObject,
  }
}

export class GetUserRequest extends jspb.Message {
  getUserId(): string;
  setUserId(value: string): GetUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetUserRequest): GetUserRequest.AsObject;
  static serializeBinaryToWriter(message: GetUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetUserRequest;
  static deserializeBinaryFromReader(message: GetUserRequest, reader: jspb.BinaryReader): GetUserRequest;
}

export namespace GetUserRequest {
  export type AsObject = {
    userId: string,
  }
}

export class GetUserResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): GetUserResponse;

  getUser(): User | undefined;
  setUser(value?: User): GetUserResponse;
  hasUser(): boolean;
  clearUser(): GetUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetUserResponse): GetUserResponse.AsObject;
  static serializeBinaryToWriter(message: GetUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetUserResponse;
  static deserializeBinaryFromReader(message: GetUserResponse, reader: jspb.BinaryReader): GetUserResponse;
}

export namespace GetUserResponse {
  export type AsObject = {
    success: boolean,
    user?: User.AsObject,
  }
}

export class UpdateUserRequest extends jspb.Message {
  getUserId(): string;
  setUserId(value: string): UpdateUserRequest;

  getUsername(): string;
  setUsername(value: string): UpdateUserRequest;

  getAvatar(): string;
  setAvatar(value: string): UpdateUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateUserRequest): UpdateUserRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateUserRequest;
  static deserializeBinaryFromReader(message: UpdateUserRequest, reader: jspb.BinaryReader): UpdateUserRequest;
}

export namespace UpdateUserRequest {
  export type AsObject = {
    userId: string,
    username: string,
    avatar: string,
  }
}

export class UpdateUserResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): UpdateUserResponse;

  getMessage(): string;
  setMessage(value: string): UpdateUserResponse;

  getUser(): User | undefined;
  setUser(value?: User): UpdateUserResponse;
  hasUser(): boolean;
  clearUser(): UpdateUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateUserResponse): UpdateUserResponse.AsObject;
  static serializeBinaryToWriter(message: UpdateUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateUserResponse;
  static deserializeBinaryFromReader(message: UpdateUserResponse, reader: jspb.BinaryReader): UpdateUserResponse;
}

export namespace UpdateUserResponse {
  export type AsObject = {
    success: boolean,
    message: string,
    user?: User.AsObject,
  }
}

export class ChatMessage extends jspb.Message {
  getId(): string;
  setId(value: string): ChatMessage;

  getUserId(): string;
  setUserId(value: string): ChatMessage;

  getUsername(): string;
  setUsername(value: string): ChatMessage;

  getContent(): string;
  setContent(value: string): ChatMessage;

  getRoomId(): string;
  setRoomId(value: string): ChatMessage;

  getMessageType(): MessageType;
  setMessageType(value: MessageType): ChatMessage;

  getTimestamp(): number;
  setTimestamp(value: number): ChatMessage;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ChatMessage.AsObject;
  static toObject(includeInstance: boolean, msg: ChatMessage): ChatMessage.AsObject;
  static serializeBinaryToWriter(message: ChatMessage, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ChatMessage;
  static deserializeBinaryFromReader(message: ChatMessage, reader: jspb.BinaryReader): ChatMessage;
}

export namespace ChatMessage {
  export type AsObject = {
    id: string,
    userId: string,
    username: string,
    content: string,
    roomId: string,
    messageType: MessageType,
    timestamp: number,
  }
}

export class SendMessageRequest extends jspb.Message {
  getUserId(): string;
  setUserId(value: string): SendMessageRequest;

  getContent(): string;
  setContent(value: string): SendMessageRequest;

  getRoomId(): string;
  setRoomId(value: string): SendMessageRequest;

  getMessageType(): MessageType;
  setMessageType(value: MessageType): SendMessageRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SendMessageRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SendMessageRequest): SendMessageRequest.AsObject;
  static serializeBinaryToWriter(message: SendMessageRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SendMessageRequest;
  static deserializeBinaryFromReader(message: SendMessageRequest, reader: jspb.BinaryReader): SendMessageRequest;
}

export namespace SendMessageRequest {
  export type AsObject = {
    userId: string,
    content: string,
    roomId: string,
    messageType: MessageType,
  }
}

export class SendMessageResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): SendMessageResponse;

  getMessage(): string;
  setMessage(value: string): SendMessageResponse;

  getChatMessage(): ChatMessage | undefined;
  setChatMessage(value?: ChatMessage): SendMessageResponse;
  hasChatMessage(): boolean;
  clearChatMessage(): SendMessageResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SendMessageResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SendMessageResponse): SendMessageResponse.AsObject;
  static serializeBinaryToWriter(message: SendMessageResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SendMessageResponse;
  static deserializeBinaryFromReader(message: SendMessageResponse, reader: jspb.BinaryReader): SendMessageResponse;
}

export namespace SendMessageResponse {
  export type AsObject = {
    success: boolean,
    message: string,
    chatMessage?: ChatMessage.AsObject,
  }
}

export class GetMessagesRequest extends jspb.Message {
  getRoomId(): string;
  setRoomId(value: string): GetMessagesRequest;

  getLimit(): number;
  setLimit(value: number): GetMessagesRequest;

  getBeforeTimestamp(): number;
  setBeforeTimestamp(value: number): GetMessagesRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMessagesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetMessagesRequest): GetMessagesRequest.AsObject;
  static serializeBinaryToWriter(message: GetMessagesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMessagesRequest;
  static deserializeBinaryFromReader(message: GetMessagesRequest, reader: jspb.BinaryReader): GetMessagesRequest;
}

export namespace GetMessagesRequest {
  export type AsObject = {
    roomId: string,
    limit: number,
    beforeTimestamp: number,
  }
}

export class GetMessagesResponse extends jspb.Message {
  getMessagesList(): Array<ChatMessage>;
  setMessagesList(value: Array<ChatMessage>): GetMessagesResponse;
  clearMessagesList(): GetMessagesResponse;
  addMessages(value?: ChatMessage, index?: number): ChatMessage;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMessagesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetMessagesResponse): GetMessagesResponse.AsObject;
  static serializeBinaryToWriter(message: GetMessagesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMessagesResponse;
  static deserializeBinaryFromReader(message: GetMessagesResponse, reader: jspb.BinaryReader): GetMessagesResponse;
}

export namespace GetMessagesResponse {
  export type AsObject = {
    messagesList: Array<ChatMessage.AsObject>,
  }
}

export class GetOnlineUsersRequest extends jspb.Message {
  getRoomId(): string;
  setRoomId(value: string): GetOnlineUsersRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetOnlineUsersRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetOnlineUsersRequest): GetOnlineUsersRequest.AsObject;
  static serializeBinaryToWriter(message: GetOnlineUsersRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetOnlineUsersRequest;
  static deserializeBinaryFromReader(message: GetOnlineUsersRequest, reader: jspb.BinaryReader): GetOnlineUsersRequest;
}

export namespace GetOnlineUsersRequest {
  export type AsObject = {
    roomId: string,
  }
}

export class GetOnlineUsersResponse extends jspb.Message {
  getUsersList(): Array<User>;
  setUsersList(value: Array<User>): GetOnlineUsersResponse;
  clearUsersList(): GetOnlineUsersResponse;
  addUsers(value?: User, index?: number): User;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetOnlineUsersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetOnlineUsersResponse): GetOnlineUsersResponse.AsObject;
  static serializeBinaryToWriter(message: GetOnlineUsersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetOnlineUsersResponse;
  static deserializeBinaryFromReader(message: GetOnlineUsersResponse, reader: jspb.BinaryReader): GetOnlineUsersResponse;
}

export namespace GetOnlineUsersResponse {
  export type AsObject = {
    usersList: Array<User.AsObject>,
  }
}

export class JoinRoomRequest extends jspb.Message {
  getUserId(): string;
  setUserId(value: string): JoinRoomRequest;

  getRoomId(): string;
  setRoomId(value: string): JoinRoomRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JoinRoomRequest.AsObject;
  static toObject(includeInstance: boolean, msg: JoinRoomRequest): JoinRoomRequest.AsObject;
  static serializeBinaryToWriter(message: JoinRoomRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JoinRoomRequest;
  static deserializeBinaryFromReader(message: JoinRoomRequest, reader: jspb.BinaryReader): JoinRoomRequest;
}

export namespace JoinRoomRequest {
  export type AsObject = {
    userId: string,
    roomId: string,
  }
}

export class JoinRoomResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): JoinRoomResponse;

  getMessage(): string;
  setMessage(value: string): JoinRoomResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JoinRoomResponse.AsObject;
  static toObject(includeInstance: boolean, msg: JoinRoomResponse): JoinRoomResponse.AsObject;
  static serializeBinaryToWriter(message: JoinRoomResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JoinRoomResponse;
  static deserializeBinaryFromReader(message: JoinRoomResponse, reader: jspb.BinaryReader): JoinRoomResponse;
}

export namespace JoinRoomResponse {
  export type AsObject = {
    success: boolean,
    message: string,
  }
}

export class LeaveRoomRequest extends jspb.Message {
  getUserId(): string;
  setUserId(value: string): LeaveRoomRequest;

  getRoomId(): string;
  setRoomId(value: string): LeaveRoomRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LeaveRoomRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LeaveRoomRequest): LeaveRoomRequest.AsObject;
  static serializeBinaryToWriter(message: LeaveRoomRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LeaveRoomRequest;
  static deserializeBinaryFromReader(message: LeaveRoomRequest, reader: jspb.BinaryReader): LeaveRoomRequest;
}

export namespace LeaveRoomRequest {
  export type AsObject = {
    userId: string,
    roomId: string,
  }
}

export class LeaveRoomResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): LeaveRoomResponse;

  getMessage(): string;
  setMessage(value: string): LeaveRoomResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LeaveRoomResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LeaveRoomResponse): LeaveRoomResponse.AsObject;
  static serializeBinaryToWriter(message: LeaveRoomResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LeaveRoomResponse;
  static deserializeBinaryFromReader(message: LeaveRoomResponse, reader: jspb.BinaryReader): LeaveRoomResponse;
}

export namespace LeaveRoomResponse {
  export type AsObject = {
    success: boolean,
    message: string,
  }
}

export enum MessageType { 
  TEXT = 0,
  IMAGE = 1,
  FILE = 2,
  SYSTEM = 3,
}
