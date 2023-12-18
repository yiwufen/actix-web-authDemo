use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;


// 定义一个包含 r2d2 Pool 的结构体
pub struct DbPool {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl DbPool {
    // 初始化连接池
    pub fn new(database_url: &str) -> DbPool {
        let manager = SqliteConnectionManager::file(database_url);
        let pool = Arc::new(Pool::new(manager).expect("Failed to create pool."));
        DbPool { pool }
    }

    // 从连接池获取一个连接
    pub fn get_connection(&self) -> PooledConnection<SqliteConnectionManager> {
        self.pool.get().expect("Failed to get a connection.")
    }
    
}

// 使用 lazy_static 创建一个全局的连接池
lazy_static! {
    pub static ref CONNECTION_POOL: DbPool = DbPool::new("swimrs.db");
}

