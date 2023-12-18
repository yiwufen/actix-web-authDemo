use actix_cors::Cors;
use actix_web::{ App, HttpServer, middleware::Logger};
use api::{authconfig, config, tokenconfig};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod auth;


mod api;


mod db;
mod models;
mod myconfig;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置openssl
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("cert/key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert/cert.pem").unwrap();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600)
            )
            .configure(authconfig)
            .configure(config)
            .configure(tokenconfig)
    })
    // .bind_openssl("127.0.0.1:8080",builder)?
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}
