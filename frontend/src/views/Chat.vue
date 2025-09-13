<template>
  <div class="chat-container">
    <!-- 侧边栏 -->
    <div class="chat-sidebar">
      <div class="sidebar-header">
        <div class="user-info">
          <el-avatar :src="userStore.user?.avatar" :size="40">
            {{ userStore.user?.username?.charAt(0).toUpperCase() }}
          </el-avatar>
          <div class="user-details">
            <h3>{{ userStore.user?.username }}</h3>
            <p class="user-status">
              <el-icon><CircleCheck /></el-icon>
              在线
            </p>
          </div>
        </div>
        <el-dropdown @command="handleUserCommand">
          <el-button type="text" :icon="Setting" />
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="profile">个人资料</el-dropdown-item>
              <el-dropdown-item command="logout" divided>退出登录</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
      
      <div class="sidebar-content">
        <div class="room-section">
          <h4>聊天房间</h4>
          <div class="room-list">
            <div
              v-for="room in rooms"
              :key="room.id"
              :class="['room-item', { active: chatStore.currentRoom === room.id }]"
              @click="switchRoom(room.id)"
            >
              <el-icon><ChatDotRound /></el-icon>
              <span>{{ room.name }}</span>
            </div>
          </div>
        </div>
        
        <div class="online-section">
          <h4>在线用户 ({{ allOnlineUsers.length }})</h4>
          <div class="online-users">
            <div
              v-for="user in allOnlineUsers"
              :key="user.id"
              :class="['online-user', { 'current-user': user.id === userStore.user?.id }]"
            >
              <el-avatar :src="user.avatar" :size="24">
                {{ user.username?.charAt(0).toUpperCase() }}
              </el-avatar>
              <span>{{ user.username }}</span>
              <div class="online-indicator"></div>
            </div>
            <!-- 调试信息 -->
            <div v-if="allOnlineUsers.length === 0" style="color: #999; font-size: 12px;">
              暂无在线用户
            </div>
            <!-- <div style="color: #999; font-size: 10px; margin-top: 5px;">
              调试: {{ JSON.stringify(allOnlineUsers) }}
            </div> -->
          </div>
        </div>
      </div>
    </div>
    
    <!-- 主聊天区域 -->
    <div class="chat-main">
      <div class="chat-header">
        <h2>{{ getCurrentRoomName() }}</h2>
        <div class="connection-status">
          <el-icon v-if="wsStore.connected" color="#67c23a"><CircleCheck /></el-icon>
          <el-icon v-else color="#f56c6c"><CircleClose /></el-icon>
          <span>{{ wsStore.connected ? '已连接' : '连接中...' }}</span>
        </div>
      </div>
      
      <div class="chat-messages" ref="messagesContainer">
        <div
          v-for="message in chatStore.messages"
          :key="message.id"
          :class="['message-item', { 'own-message': message.user_id === userStore.user?.id }]"
        >
          <div class="message-avatar">
            <el-avatar :size="32">
              {{ message.username?.charAt(0).toUpperCase() }}
            </el-avatar>
          </div>
          <div class="message-content">
            <div class="message-header">
              <span class="message-username">{{ message.username }}</span>
              <span class="message-time">{{ message.timestamp }}</span>
            </div>
            <div class="message-text">{{ message.content }}</div>
          </div>
        </div>
      </div>
      
      <div class="chat-input">
        <el-input
          v-model="messageInput"
          placeholder="输入消息..."
          size="large"
          @keyup.enter="sendMessage"
        >
          <template #append>
            <el-button
              type="primary"
              :icon="Position"
              :loading="sending"
              @click="sendMessage"
            >
              发送
            </el-button>
          </template>
        </el-input>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, nextTick, watch } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Setting, CircleCheck, CircleClose, ChatDotRound, Position } from '@element-plus/icons-vue'
import { useUserStore } from '@/stores/user'
import { useChatStore } from '@/stores/chat'
import { useWebSocketStore } from '@/stores/websocket'

const router = useRouter()
const userStore = useUserStore()
const chatStore = useChatStore()
const wsStore = useWebSocketStore()

const messageInput = ref('')
const sending = ref(false)
const messagesContainer = ref()

const rooms = ref([
  { id: 'general', name: '公共聊天室' },
  { id: 'tech', name: '技术讨论' },
  { id: 'random', name: '闲聊' }
])

// 计算属性：合并当前用户和在线用户
const allOnlineUsers = computed(() => {
  const users = [...chatStore.onlineUsers]
  
  // 如果当前用户不在在线用户列表中，添加当前用户
  if (userStore.user && !users.find(user => user.id === userStore.user.id)) {
    users.unshift({
      id: userStore.user.id,
      username: userStore.user.username,
      avatar: userStore.user.avatar
    })
  }
  
  return users
})

onMounted(async () => {
  // 加入默认房间
  await chatStore.joinRoom('general')
  
  // 连接WebSocket
  if (!wsStore.connected) {
    await wsStore.connect()
  }
})

// 监听消息变化，自动滚动到底部
watch(() => chatStore.messages.length, () => {
  nextTick(() => {
    scrollToBottom()
  })
})

const scrollToBottom = () => {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
}

const getCurrentRoomName = () => {
  const room = rooms.value.find(r => r.id === chatStore.currentRoom)
  return room ? room.name : '未知房间'
}

const switchRoom = async (roomId) => {
  if (roomId === chatStore.currentRoom) return
  
  try {
    // 先通过WebSocket离开当前房间
    wsStore.leaveRoom(chatStore.currentRoom)
    
    // 更新前端状态（这会清空消息并加载新房间的消息）
    await chatStore.joinRoom(roomId)
    
    // 再通过WebSocket加入新房间
    wsStore.joinRoom(roomId)
  } catch (error) {
    console.error('切换房间失败:', error)
  }
}

const sendMessage = async () => {
  if (!messageInput.value.trim() || sending.value) return
  
  sending.value = true
  
  try {
    const messageContent = messageInput.value.trim()
    const userStore = useUserStore()
    
    // 立即在页面上显示消息
    const tempMessage = {
      id: Date.now().toString(),
      user_id: userStore.user.id,
      username: userStore.user.username,
      content: messageContent,
      message_type: 'text',
      room_id: chatStore.currentRoom,
      timestamp: Math.floor(Date.now() / 1000),
      is_temp: true // 标记为临时消息
    }
    
    chatStore.addMessage(tempMessage)
    messageInput.value = ''
    
    // 通过WebSocket发送消息
    wsStore.sendChatMessage(messageContent, 'text')
  } catch (error) {
    ElMessage.error('发送失败，请重试')
  } finally {
    sending.value = false
  }
}

const handleUserCommand = async (command) => {
  switch (command) {
    case 'profile':
      // 显示个人资料对话框
      ElMessage.info('个人资料功能开发中...')
      break
    case 'logout':
      try {
        await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning'
        })
        
        wsStore.disconnect()
        userStore.logout()
        router.push('/login')
        ElMessage.success('已退出登录')
      } catch {
        // 用户取消
      }
      break
  }
}
</script>

<style scoped>
.chat-container {
  display: flex;
  height: 100vh;
  background: #f5f5f5;
}

.chat-sidebar {
  width: 280px;
  background: white;
  border-right: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-details h3 {
  margin: 0;
  font-size: 16px;
  color: #303133;
}

.user-status {
  margin: 0;
  font-size: 12px;
  color: #67c23a;
  display: flex;
  align-items: center;
  gap: 4px;
}

.sidebar-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.room-section,
.online-section {
  margin-bottom: 30px;
}

.room-section h4,
.online-section h4 {
  margin: 0 0 15px 0;
  font-size: 14px;
  color: #909399;
  font-weight: 500;
}

.room-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.room-item {
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: all 0.2s;
}

.room-item:hover {
  background: #f0f2f5;
}

.room-item.active {
  background: #409eff;
  color: white;
}

.online-users {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.online-user {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px;
  border-radius: 6px;
  position: relative;
}

.online-indicator {
  width: 8px;
  height: 8px;
  background: #67c23a;
  border-radius: 50%;
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
}

.chat-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
}

.chat-header {
  padding: 20px;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.chat-header h2 {
  margin: 0;
  font-size: 20px;
  color: #303133;
}

.connection-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  color: #909399;
}

.chat-messages {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.message-item {
  display: flex;
  gap: 12px;
  max-width: 70%;
}

.message-item.own-message {
  align-self: flex-end;
  flex-direction: row-reverse;
}

.message-content {
  flex: 1;
}

.message-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.message-username {
  font-weight: 500;
  color: #303133;
  font-size: 14px;
}

.message-time {
  font-size: 12px;
  color: #909399;
}

.message-text {
  background: #f0f2f5;
  padding: 10px 14px;
  border-radius: 12px;
  color: #303133;
  word-wrap: break-word;
}

.own-message .message-text {
  background: #409eff;
  color: white;
}

/* 当前用户样式 */
.online-user.current-user {
  background: #e6f7ff;
  border: 1px solid #91d5ff;
  border-radius: 6px;
  padding: 4px 8px;
}

.online-user.current-user .online-indicator {
  background: #52c41a;
}

.chat-input {
  padding: 20px;
  border-top: 1px solid #e4e7ed;
}
</style>
