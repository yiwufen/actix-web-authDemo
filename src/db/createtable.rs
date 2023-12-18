use crate::db::db::CONNECTION_POOL;
use crate::models::user::User;
// create User table
pub fn create_user_table() {
    let conn = CONNECTION_POOL.get_connection();
    let sql = User::create_table_sql();
    conn.execute(&sql, []).expect("Failed to create table.");
}

#[cfg(test)]
mod usertable {
    use super::*;
    #[test]
    fn test_create_user_table() {
        create_user_table();
    }
}