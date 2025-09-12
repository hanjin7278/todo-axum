use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use uuid::Uuid;
use crate::models::{CreateTodoRequest, Todo, UpdateTodoRequest};

pub struct DataBase {
    pub pool: MySqlPool,
}

impl DataBase {
    /// 创建数据库链接
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 查询所有 todo
    pub async fn list_todos(&self) -> Result<Vec<Todo>, sqlx::Error> {
        // 注意这里：给时间字段加上 `: DateTime<Utc>`，sqlx 自动转换
        let rows = sqlx::query!(
            r#"
            SELECT id, title, description, completed,
                   created_at as "created_at: DateTime<Utc>",
                   updated_at as "updated_at: DateTime<Utc>"
            FROM todos
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        let todos = rows
            .into_iter()
            .map(|row| Todo {
                id: row.id,
                title: row.title,
                description: row.description.unwrap_or_default(),
                completed: row.completed != 0,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();
        Ok(todos)
    }

    /// 根据 id 查询 todo
    pub async fn get_todo(&self, id: &str) -> Result<Option<Todo>, sqlx::Error> {
        let r = sqlx::query!(
            r#"
            SELECT id, title, description, completed,
                   created_at as "created_at: DateTime<Utc>",
                   updated_at as "updated_at: DateTime<Utc>"
            FROM todos
            WHERE id = ?
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = r {
            Ok(Some(Todo {
                id: row.id,
                title: row.title,
                description: row.description.unwrap_or_default(),
                completed: row.completed != 0,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// 创建 todo
    pub async fn create_todo(&self, input: CreateTodoRequest) -> Result<Todo, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query!(
            "INSERT INTO todos (id, title, description, completed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            id,
            input.title,
            input.description,
            0i32, // completed = false
            now.naive_utc(), // 存数据库
            now.naive_utc()
        )
            .execute(&self.pool)
            .await?;

        Ok(Todo {
            id,
            title: input.title,
            description: input.description.unwrap_or_default(),
            completed: false,
            created_at: now,  // API 用 UTC
            updated_at: now,
        })
    }

    /// 更新 todo
    pub async fn update_todo(&self, id: &str, input: UpdateTodoRequest) -> Result<Option<Todo>, sqlx::Error> {
        // 先查是否存在
        let current = match self.get_todo(id).await? {
            Some(todo) => todo,
            None => return Ok(None),
        };

        let new_title = input.title.unwrap_or(current.title.clone());
        let new_description = input.description.or(Some(current.description.clone()));
        let new_completed = input.completed.unwrap_or(current.completed);

        let now = Utc::now();
        sqlx::query!(
            "UPDATE todos SET title = ?, description = ?, completed = ?, updated_at = ? WHERE id = ?",
            new_title,
            new_description,
            (if new_completed { 1 } else { 0 }) as i32,
            now.naive_utc(),
            id
        )
            .execute(&self.pool)
            .await?;

        Ok(Some(Todo {
            id: id.to_string(),
            title: new_title,
            description: new_description.unwrap_or_default(),
            completed: new_completed,
            created_at: current.created_at,
            updated_at: now,
        }))
    }

    /// 删除 todo
    pub async fn delete_todo(&self, id: &str) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
