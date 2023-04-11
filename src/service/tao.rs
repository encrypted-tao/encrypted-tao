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

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub response: Vec<Vec<query::results::DBRow>>,
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

    pub async fn db_connect(&self) -> Option<Client> {
        let db_url = format!(
            "host={} user={} password={} dbname={} port={}",
            self.db_config.host,
            self.db_config.user,
            self.db_config.password,
            self.db_config.db_name,
            self.db_config.port
        );
        let (client, conn) = connect(db_url.as_str(), NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });
        return Some(client);
    }

    pub async fn db_execute(
        &self,
        sql_query: query::query::SqlQuery,
    ) -> Option<Vec<query::results::DBRow>> {
        let client = self.db_connect().await.unwrap();
        let query_params: Vec<_> = sql_query
            .params
            .iter()
            .map(|x| x as &(dyn ToSql + Sync))
            .collect();
        let resp = client
            .query(&sql_query.query, query_params.as_slice())
            .await;
        match resp {
            Ok(r) => {
                let res = query::results::deserialize_rows(&sql_query.op, r);
                Some(res)
            }
            Error => panic!("oh no!"),
        }
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

        let results = join_all(
            sql_queries
                .iter()
                .map(|q| async { self.db_execute(q.clone()).await.unwrap() }),
        )
        .await;

        return HttpResponse::Ok().json(&QueryResponse { response: results });
        // return HttpResponse::Ok().json(sql_queries.collect::<Vec<query::query::SqlQuery>>());
    }

    pub async fn db_client(&self) -> Result<(), Error> {
        let client = self.db_connect().await.unwrap();
        let query1 = &client
            .query(
                "
                SELECT * From Objects
                ",
                &[],
            )
            .await?;
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
    let res = TaoServer::pipeline(&tao, query.into_inner().query, false).await;
    return res;
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("").service(hello).service(query_handler));
}
