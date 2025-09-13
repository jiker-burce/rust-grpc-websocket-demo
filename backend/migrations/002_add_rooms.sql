-- 添加技术讨论房间
INSERT INTO rooms (id, name, description, is_public, created_by) 
VALUES ('tech', '技术讨论', '技术讨论群，分享编程和技术相关内容', TRUE, 'system')
ON DUPLICATE KEY UPDATE name = name;

-- 添加闲聊房间
INSERT INTO rooms (id, name, description, is_public, created_by) 
VALUES ('random', '闲聊群', '闲聊群，随意聊天和讨论', TRUE, 'system')
ON DUPLICATE KEY UPDATE name = name;
