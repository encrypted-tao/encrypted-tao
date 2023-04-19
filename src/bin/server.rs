use actix_web::{web::{Data}, App, HttpServer};
use encrypted_tao::service;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let env_path = &args[1];

    let tao_server = service::tao::TaoServer::new(env_path.to_string());
    let app_data = Data::new(tao_server);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(service::tao::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
