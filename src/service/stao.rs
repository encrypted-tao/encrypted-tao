use crate::query;
use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};
use core::marker::Sync;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::{Arc, Mutex};
use tokio_postgres::{connect, types::ToSql, Client, Error, NoTls, Row};
use sqlx::{postgres::PgPool};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub response: String,

    // pub response: Vec<Vec<query::results::DBRow>>,
    //    pub response: Vec<query::query::SqlQuery>,
}

pub struct DBConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub db_name: String,
    pub port: String,
}

impl DBConfig {
    pub fn new(env_path: String) -> Self {
        dotenv::from_path(env_path).ok();
        let host = dotenv::var("DATABASE_HOST").unwrap();
        let user = dotenv::var("DATABASE_USERNAME").unwrap();
        let password = dotenv::var("DATABASE_PASSWORD").unwrap();
        let db_name = dotenv::var("DATABASE_NAME").unwrap();
        let port = dotenv::var("DATABASE_PORT_NUM").unwrap();
        DBConfig {
            host,
            user,
            password,
            db_name,
            port,
        }
    }
}

pub struct TaoServer {
    pub cache: Arc<Mutex<Vec<i32>>>,
    pub db_config: DBConfig,
}

impl TaoServer {
    pub fn new(env_path: String) -> Self {
        let cache = Arc::new(Mutex::new(vec![]));
        let db_config = DBConfig::new(env_path);
        TaoServer { cache, db_config }
    }

    pub async fn db_connect(&self) -> Option<PgPool> {
        let db_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.db_config.user,
            self.db_config.password,
            self.db_config.host,
            self.db_config.port,
            self.db_config.db_name,
        );

        let pool = PgPool::connect(&db_url).await.ok()?;
    
        return Some(pool);
    }

    pub async fn db_execute(
        &self,
    ) -> Result<(), Error> {
        let pool = self.db_connect().await.unwrap();
        let row = sqlx::query("SELECT * FROM Objects").execute(&pool).await;
        println!("{:#?}", row);
        return Ok(());
    }

    pub async fn pipeline(
        &self,
        query_input: String,
        encrypt: bool,
    ) -> HttpResponse {
        println!("Received Query: {:#?}", query_input);
        let tao_queries = query::parser::parse(query_input.as_str());
        let sql_queries = tao_queries
            .iter()
            .map(|q| query::translator::translate(q.clone()))
            .collect::<Vec<query::query::SqlQuery>>();
        
        self.db_execute();

        return HttpResponse::Ok().json(&QueryResponse { response: "ok".to_string()});
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
    let res = TaoServer::pipeline(&tao, query.into_inner().query, false).await;
    return res;
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("").service(hello).service(query_handler));
}
