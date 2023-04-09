use actix_web::{
    get, post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use encrypted_tao::service;
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tao_server = service::tao::TaoServer::new();
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
