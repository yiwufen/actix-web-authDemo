use actix_web::{get, post, web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{Header, encode, EncodingKey};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::{bearer_validator, basicauth_validator}, models::user::User, myconfig::IConfig};
use actix_web_httpauth::{middleware::HttpAuthentication, extractors::basic::BasicAuth};

use crate::db::db::CONNECTION_POOL;

pub fn authconfig(ctg: &mut web::ServiceConfig) {
    ctg.service(register);
    ctg.service(hello_name);
}
pub fn config(ctg: &mut web::ServiceConfig) {
    ctg.service(
        web::scope("/api")
            .wrap(HttpAuthentication::bearer(bearer_validator))
            .service(hello),
    );
}
pub fn tokenconfig(ctg: &mut web::ServiceConfig) {
    ctg.service(
        web::scope("/tokens")
            .wrap(HttpAuthentication::basic(basicauth_validator))
            .service(token),
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,  // 用户唯一标识，例如用户名或用户 ID
    pub exp: usize,   // 过期时间（时间戳）
    // 可以添加更多的自定义字段
}
#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

#[get("/token")]
async fn token(credentials: BasicAuth) -> HttpResponse {
    // 准备 JWT 的 Claims
    let claims = Claims {
        username: credentials.user_id().to_owned(),
        exp: (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    let _config = crate::myconfig::Config {};
    let secret_key = _config.get_config_with_key("SECRET_KEY");
    // 使用秘钥编码 JWT
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
        .unwrap();  // 在实际应用中应该处理所有可能的错误

    HttpResponse::Ok().json(TokenResponse { token })
}

#[get("/hello")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("hello")
}

#[get("/hello/{name}")]
async fn hello_name(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().json(format!("Hello {}!", path.0))
}

#[derive(Deserialize)]
struct Register {
    username: String,
    password: String,
    email: String,
}

#[post("/register")]
async fn register(user: web::Json<Register>) -> HttpResponse {
    let conn = CONNECTION_POOL.get_connection();

    // 数据验证（示例代码，需要根据实际情况进行完善）
    if user.username.is_empty() || user.email.is_empty() || user.password.len() < 8 {
        return HttpResponse::BadRequest().json("Invalid data provided");
    }

    // 密码加密
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().json("Password hashing failed"),
    };

    let user: User = User {
        user_id: Uuid::new_v4(),
        username: user.username.clone(),
        email: user.email.clone(),
        password: hashed_password.clone(),
        registration_date: Utc::now().to_string(),
        user_type: "user".to_string(),
    };
    // 插入数据到数据库
    let insert_result = conn.execute(
        "INSERT INTO Users (UserID, Username, Email, Password, RegistrationDate, UserType) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            user.user_id.to_string(),
            user.username,
            user.email,
            user.password,
            user.registration_date,
            user.user_type
        ],
    );

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("User registered successfully"),
        Err(e) => {
            // 处理可能的错误，例如用户名或邮箱重复
            HttpResponse::InternalServerError().json(format!("Error: {}", e))
        }
    }
}
// async fn register(user: web::Json<Register>) -> HttpResponse {
//     // let _connection: Connection = Connection {};
//     // let _repository: UserRepository = UserRepository {
//     //     connection: _connection.init(),
//     // };
//     // HttpResponse::Ok().json(_repository.register(user.into_inner()))

// }
