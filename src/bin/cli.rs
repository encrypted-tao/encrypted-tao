use awc::Client;
use encrypted_tao::service;
use std::env;
use std::io::{self, Write};

pub struct Config {
    pub server_addr: String,
    pub server_port: String,
}

impl Config {
    pub fn new(env_path: String) -> Self {
        dotenv::from_path(env_path).ok();
        let server_addr = dotenv::var("SERVER_ADDR").unwrap();
        let server_port = dotenv::var("SERVER_PORT").unwrap();
        Config { server_addr, server_port }
    }
}

fn print_help() {
    println!("--------------------------------------------------------------------------");
    println!("Supported AssocTypes: FRIEND, LIKES, AUTHORED, CHECKIN");
    println!("Supported Association Queries:");
    println!(
        "    ASSOC ADD id1(int) assoc(AssocType) id2(int) time(int) data(str);"
    );
    println!("    ASSOC GET id(int) assoc(AssocType) idset([int]);");
    println!(
        "    ASSOC RGET id(int) assoc(AssocType) idset([int]) time-lo(int) time-hi(int);"
    );
    println!("    ASSOC COUNT id(int) assoc(AssocType);");
    println!(
        "    ASSOC RANGE id(int) assoc(AssocType) time-lo(int) time-hi(int) lim(int);"
    );
    println!("");
    println!("Supported ObjTypes: USER, COMMENT, LOCATION, POST");
    println!("Supported Object Queries");
    println!("    OBJ ADD id(int) obj(ObjType) data(str);");
    println!("    OBJ GET id(int);");
    println!("--------------------------------------------------------------------------");
}

async fn execute_tao_query(
    host: String,
    port: String,
    query: String,
) -> service::tao::QueryResponse {
    let client = Client::new();
    let endpoint = format!("http://{}:{}/query", host, port);
    let resp = client
        .post(endpoint)
        .send_json(&service::tao::QueryRequest { query: query })
        .await
        .unwrap()
        .json::<service::tao::QueryResponse>()
        .await
        .unwrap();

    let res = match resp {
        service::tao::QueryResponse { response: _ } => resp,
        _ => panic!("oops"),
    };
    return res;
}

#[actix_rt::main]
async fn main() {
    let mut env_path = env!("CARGO_MANIFEST_DIR").to_string();
    env_path.push_str("/.env");
    let config = Config::new(env_path);
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    match query.as_str() {
        "--help" => print_help(),
        _ => {
            let res = execute_tao_query(
                config.server_addr,
                config.server_port,
                query.to_string(),
            )
            .await;
            println!("\n{:#?}\n", res)
        }
    }

}
