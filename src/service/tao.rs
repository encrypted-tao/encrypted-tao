use crate::query;
use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::{Arc, Mutex};
use tokio_postgres::{NoTls, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub response: Vec<query::query::SqlQuery>,
}

pub struct TaoServer {
    pub cache: Arc<Mutex<Vec<i32>>>,
}

impl TaoServer {
    pub fn new() -> Self {
        let cache = Arc::new(Mutex::new(vec![]));

        TaoServer { cache }
    }

    pub async fn pipeline(&self, query_input: String, encrypt: bool) -> HttpResponse {
        println!("Received Query: {:#?}", query_input);
        let tao_queries = query::parser::parse(query_input.as_str());
        let sql_queries = tao_queries
            .iter()
            .map(|q| query::translator::translate(q.clone()))
            .collect::<Vec<query::query::SqlQuery>>();

        self.db_client().await;    
        return HttpResponse::Ok().json(&QueryResponse {
            response: sql_queries,
        });
    }

    pub async fn db_client(&self) -> Result<(), Error> {
        let (client, connection) = tokio_postgres::connect("host=encrypted-tao.clyigb9dssrd.us-east-1.rds.amazonaws.com user=dbuser password=dbuserdbuser dbname=postgres port=5432", NoTls).await.unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let query1 = client.query("
            INSERT INTO Objects (id, key, obj_type, val)
            VALUES (1234, 12, 13, 14)
            ", &[]).await?;        
        println!("{:#?}", query1);


        return Ok(());
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
    let res =  TaoServer::pipeline(&tao, query.into_inner().query, false).await;
    return res;
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("").service(hello).service(query_handler));
}
