use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{bearer::{self, BearerAuth}, AuthenticationError, basic::{BasicAuth, self}};

use bcrypt::verify;
use jsonwebtoken::{Validation, Algorithm, decode, DecodingKey};
use rusqlite::params;

use crate::{db::db::CONNECTION_POOL, api::Claims, myconfig::IConfig};

pub async fn bearer_validator(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // 添加自己的验证_credentials是否有效，
    // 获取密钥
    let _config = crate::myconfig::Config {};
    let secret_key = _config.get_config_with_key("SECRET_KEY");

    // 设置 JWT 验证参数
    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.validate_exp = true;
    validation.algorithms = vec![Algorithm::HS256];

    // 解码和验证 JWT
    match decode::<Claims>(&credentials.token(), &DecodingKey::from_secret(secret_key.as_ref()), &validation) {
        Ok(_) => Ok(req),
        Err(_) => {
            let config = req.app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .realm("Restricted area");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
pub async fn basicauth_validator(
    req: ServiceRequest,
    credentials: BasicAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // 添加自己的验证_credentials是否有效，
    // 提取用户名和密码
    let username = credentials.user_id();
    let password = credentials.password().unwrap_or("");

    // 这里是验证逻辑的伪代码，您需要根据实际情况来实现
    // 比如查询数据库，比较用户名和密码等
    if validate_credentials(username, password) {
        // 如果凭证有效
        Ok(req)
    } else {
        // 如果凭证无效
        let config = req.app_data::<basic::Config>()
            .cloned()
            .unwrap_or_default()
            .realm("Restricted area");
        Err((AuthenticationError::from(config).into(), req))
    }
}

fn validate_credentials(username: &str, password: &str) -> bool {
    // 假设这是从全局连接池获取连接
    let conn = CONNECTION_POOL.get_connection();

    // 查询数据库中的用户信息
    let mut stmt = match conn.prepare("SELECT Password FROM Users WHERE Username = ?1") {
        Ok(stmt) => stmt,
        Err(_) => return false,
    };

    let mut rows = match stmt.query(params![username]) {
        Ok(rows) => rows,
        Err(_) => return false,
    };

    if let Some(row) = rows.next().unwrap_or(None) {
        let stored_password: String = row.get(0).unwrap_or_default();
        // 对比提供的密码与存储的哈希密码
        match verify(password, &stored_password) {
            Ok(valid) => valid,
            Err(_) => false,
        }
    } else {
        // 用户名不存在
        false
    }
}