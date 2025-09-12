# 基于Axum + SQLX 的待办事项工具
## 创建数据库
```
create database simple_todo;
CREATE TABLE todos (
                       id VARCHAR(36) PRIMARY KEY,         -- 使用 uuid 字符串作为主键
                       title VARCHAR(255) NOT NULL,        -- 任务标题
                       description TEXT NULL,              -- 任务描述，可空
                       completed TINYINT(1) NOT NULL DEFAULT 0,  -- 是否完成，0/1
                       created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

## 构建执行
```
    cargo run
```