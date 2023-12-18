use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,  // 注意：实际应用中应使用哈希和盐来存储密码
    pub registration_date: String,
    pub user_type: String,
}

impl User {
    // 返回用于创建 `Users` 表的 SQL 语句
    #[warn(dead_code)]
    pub fn create_table_sql() -> String {
        let sql = "
            CREATE TABLE IF NOT EXISTS Users (
                UserID TEXT PRIMARY KEY,
                Username TEXT NOT NULL UNIQUE,
                Email TEXT NOT NULL UNIQUE,
                Password TEXT NOT NULL,
                RegistrationDate TEXT NOT NULL,
                UserType TEXT NOT NULL
            )";
        sql.to_string()
    }
}