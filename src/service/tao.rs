use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use core::marker::Sync;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use tokio_postgres::{connect, types::ToSql, Client, NoTls};

use crate::query::{
    parser,
    query::{format_in_clause, Query, TaoArgs, TaoOp},
    results::{deserialize_rows, DBRow},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub response: Vec<Vec<DBRow>>,
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
    pub db_config: DBConfig,
    pub encrypted: bool,
}

impl TaoServer {
    pub fn new(env_path: String, encrypted: bool) -> Self {
        let db_config = DBConfig::new(env_path);
        TaoServer { db_config, encrypted }
    }

    async fn db_connect(&self) -> Option<Client> {
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
                println!("connection error: {}", e);
            }
        });
        return Some(client);
    }

    async fn db_execute(&self, query: Query) -> Option<Vec<DBRow>> {
        let res = match query.op {
            TaoOp::AssocAdd => self.assoc_add(query).await,
            TaoOp::AssocGet => self.assoc_get(query).await,
            TaoOp::AssocRangeGet => self.assoc_range_get(query).await,
            TaoOp::AssocCount => self.assoc_count(query).await,
            TaoOp::AssocRange => self.assoc_range(query).await,
            TaoOp::ObjAdd => self.obj_add(query).await,
            TaoOp::ObjGet => self.obj_get(query).await,
            _ => panic!("todo!"),
        };

        return res;
    }

    pub async fn pipeline(&self, query_input: String) -> HttpResponse {
        println!("Received Query: {:#?}", query_input);
        let tao_queries = parser::parse(query_input.as_str());
        let results = join_all(
            tao_queries
                .iter()
                .map(|q| async { self.db_execute(q.clone()).await.unwrap() }),
        )
        .await;

        return HttpResponse::Ok().json(&QueryResponse { response: results });
    }

    async fn assoc_add(&self, query: Query) -> Option<Vec<DBRow>> {
        let client = self.db_connect().await.unwrap();

        let sql_query = "INSERT INTO Associations(id1, atype, id2, t, data) \
                         VALUES ($1, $2, $3, $4, $5)";

        let (id1, ty, id2, time, data) = match query.args {
            TaoArgs::AssocAddArgs {
                id1,
                atype,
                id2,
                time,
                data,
            } => (id1, atype, id2, time, data),
            _ => panic!("Incorrect args to assoc add"),
        };


        let resp = &client
            .query(
                sql_query,
                &[&id1, &ty.as_str(), &id2, &time, &data.as_str()],
            )
            .await
            .unwrap();

        let res = deserialize_rows(&query.op, resp);
        return Some(res);
    }

    async fn assoc_get(&self, query: Query) -> Option<Vec<DBRow>> {
        let client = self.db_connect().await.unwrap();

        let (id, ty, idset) = match query.args {
            TaoArgs::AssocGetArgs { id, atype, idset } => (id, atype, idset),
            _ => panic!("Incorrect args to assoc get"),
        };

        let in_set = format_in_clause(&idset, 2);
        let sql_query = format!(
            "SELECT * \
             FROM Associations \
             WHERE id1 = $1 \
             AND atype = $2 \
             AND id2 in {in_set}"
        );

        let idset: Vec<_> =
            idset.iter().map(|x| x as &(dyn ToSql + Sync)).collect();

        let mut params =
            vec![&id as &(dyn ToSql + Sync), &ty as &(dyn ToSql + Sync)];
        params.extend(idset);
        let resp = &client.query(&sql_query, &params).await.unwrap();

        let res = deserialize_rows(&query.op, resp);
        return Some(res);
    }

    async fn assoc_range_get(&self, query: Query) -> Option<Vec<DBRow>> {
        let client = self.db_connect().await.unwrap();

        let (id, ty, idset, tstart, tend) = match query.args {
            TaoArgs::AssocRangeGetArgs {
                id,
                atype,
                idset,
                tstart,
                tend,
            } => (id, atype, idset, tstart, tend),
            _ => panic!("Incorrect args to assoc get"),
        };

        let in_set = format_in_clause(&idset, 4);
        let sql_query = format!(
            "SELECT * \
             FROM Associations \
             WHERE id1 = $1 \
             AND atype = $2 \
             AND t >= $3 \
             AND t <= $4 \
             AND id2 in {in_set}"
        );

        let idset: Vec<_> =
            idset.iter().map(|x| x as &(dyn ToSql + Sync)).collect();

        let mut params = vec![
            &id as &(dyn ToSql + Sync),
            &ty as &(dyn ToSql + Sync),
            &tstart as &(dyn ToSql + Sync),
            &tend as &(dyn ToSql + Sync),
        ];
        params.extend(idset);

        let resp = &client.query(&sql_query, &params).await.unwrap();

        let res = deserialize_rows(&query.op, resp);
        return Some(res);
    }

    async fn assoc_count(&self, query: Query) -> Option<Vec<DBRow>> {
        let client = self.db_connect().await.unwrap();
        let sql_query = "SELECT COUNT(*) \
                     FROM Associations \
                     WHERE id1 = $1 \
                       AND atype = $2";

        let (id, atype) = match query.args {
            TaoArgs::AssocCountArgs { id, atype } => (id, atype),
            _ => panic!("Incorrect args to obj get"),
        };
        // here !
        let resp = &client
            .query(sql_query, &[&id, &atype.as_str()])
            .await
            .unwrap();

        let res = deserialize_rows(&query.op, resp);
        return Some(res);
    }
    // TODO encrypt here
    async fn assoc_range(&self, query: Query) -> Option<Vec<DBRow>> {
        let client = self.db_connect().await.unwrap();

        let sql_query = "SELECT * \
                     FROM Associations \
                     WHERE id1 = $1 \
                       AND atype = $2 \
                       AND t >= $3 \
                       AND t <= $4 \
                     ORDER BY t DESC \
                     LIMIT $5";

        let (id, atype, tstart, tend, lim) = match query.args {
            TaoArgs::AssocRangeArgs {
                id,
                atype,
                tstart,
                tend,
                lim,
            } => (id, atype, tstart, tend, lim),
            _ => panic!("Incorrect args to obj get"),
        };
        // here !!
        let resp = &client
            .query(sql_query, &[&id, &atype.as_str(), &tstart, &tend, &lim])
            .await
            .unwrap();

        let res = deserialize_rows(&query.op, resp);
        return Some(res);
    }

    async fn obj_get(&self, query: Query) -> Option<Vec<DBRow>> {
        let client = self.db_connect().await.unwrap();

        let sql_query = "SELECT * \
                         FROM Objects \
                         WHERE id = $1";

        let id = match query.args {
            TaoArgs::ObjGetArgs { id } => id,
            _ => panic!("Incorrect args to obj get"),
        };

        let resp = &client.query(sql_query, &[&id]).await.unwrap();

        let res = deserialize_rows(&query.op, resp);
        return Some(res);
    }

    async fn obj_add(&self, query: Query) -> Option<Vec<DBRow>> {
        let client = self.db_connect().await.unwrap();

        let sql_query = "INSERT INTO Objects(id, otype, data) \
                         VALUES ($1, $2, $3)";

        let (id, ty, data) = match query.args {
            TaoArgs::ObjAddArgs { id, otype, data } => (id, otype, data),
            _ => panic!("Incorrect args to obj add"),
        };

        

        let resp = &client
            .query(sql_query, &[&id, &ty.as_str(), &data.as_str()])
            .await
            .unwrap();

        let res = deserialize_rows(&query.op, resp);
        return Some(res);
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
    let res = TaoServer::pipeline(&tao, query.into_inner().query).await;
    return res;
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("").service(hello).service(query_handler));
}

/*
 * TAO test
 * run `via cargo test`
 */
 #[cfg(test)]
 mod tests {
 
    use crate::query::{
        parser,
        query::{format_in_clause, Query, TaoArgs, TaoOp},
        results::{deserialize_rows, DBRow},
    };
     use crate::ope::ope::ope::OPE;
     use crate::ope::ope::ope::Range;
   
     pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
     pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;
 
     #[test]
     fn test_assoc_get() {

        let query_input = "ASSOC RANGE 55 AUTHORED 0 100 10;".to_string();
        let tao_queries = parser::parse(query_input.as_str());
        
       
     }
 }
