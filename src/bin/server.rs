use std::env;
use std::sync::Mutex;

use actix_web::{web::Data, App, HttpServer};

use encrypted_tao::service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let env_path = &args[1];
    let cache_size: usize = match args[2].parse() {
        Ok(n) => n,
        Err(e) => panic!("parse error {}", e),
    };
    let encrypt: bool = match args[3].as_str() {
        "yes" => true,
        _ => false,
    };
    
    let mut tao_server = service::tao::TaoServer::new(env_path.to_string(), cache_size, encrypt);
    let app_data = Data::new(Mutex::new(tao_server));

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(service::tao::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
