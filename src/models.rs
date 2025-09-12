use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 对外 API 返回的 Todo 结构
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Todo{
    pub id:String,
    // 标题
    pub title:String,
    // 描述
    pub description:String,
    // 是否完成
    pub completed:bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


/// 创建Todo的请求体(POST /todos)
#[derive(Debug,Serialize,Deserialize)]
pub struct CreateTodoRequest{
    // 标题
    pub title:String,
    // 描述
    pub description:Option<String>,
}

/// 更新Todo的请求体(PUT /todos/:id)
#[derive(Debug,Serialize,Deserialize)]
pub struct UpdateTodoRequest{
    // 标题
    pub title:Option<String>,
    // 描述
    pub description:Option<String>,
    // 是否完成
    pub completed:Option<bool>,
}