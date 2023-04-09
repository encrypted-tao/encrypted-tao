use crate::query;
use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

pub struct TaoServer {
    pub cache: Arc<Mutex<Vec<i32>>>, // placeholder for an actual cache
}

impl TaoServer {
    pub fn new() -> Self {
        let cache = Arc::new(Mutex::new(vec![]));
        TaoServer { cache }
    }

    pub fn pipeline(&self, query_input: String, encrypt: bool) -> HttpResponse {
        let tao_queries = query::parser::parse(query_input.as_str());
        let sql_queries = tao_queries
            .iter()
            .map(|q| query::translator::translate(q.clone()))
            .collect::<Vec<query::query::SqlQuery>>();
        return HttpResponse::Ok().json(sql_queries);
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("TAO Server")
}

#[post("/query")]
pub async fn query_handler(
    tao: Data<TaoServer>,
    query: Json<QueryRequest>,
) -> HttpResponse {
    return TaoServer::pipeline(&tao, query.into_inner().query, false);
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("").service(hello).service(query_handler));
}
