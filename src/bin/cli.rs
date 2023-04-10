use awc::Client;
use encrypted_tao::service;
use std::env;
use std::io::{self, Write};
use tokio;

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

fn print_preamble(host: String, port: String) {
    println!("==========================================================================");
    println!("TAO COMMAND LINE INTERFACE");
    println!("==========================================================================");
    println!("Supported AssocTypes: FRIEND, LIKES, AUTHORED, CHECKIN");
    println!("Supported Association Queries:");
    println!(
        "    ASSOC ADD id1(int) assoc(AssocType) id2(int) time(int) data(str);"
    );
    println!("    ASSOC GET id(int) assoc(AssocType) idset([int]);");
    println!(
        "    ASSOC RGET id(int) assoc(AssocType) idset([int]) lo(int) hi(int);"
    );
    println!("    ASSOC COUNT id(int) assoc(AssocType);");
    println!(
        "    ASSOC TRANGE id(int) assoc(AssocType) lo(int) hi(int) lim(int);"
    );
    println!("");
    println!("Supported ObjTypes: USER, COMMENT, LOCATION, POST");
    println!("Supported Object Queries");
    println!("    OBJ ADD id(int) obj(ObjType) data(str)");
    println!("    OBJ GET id(int)");
    println!("==========================================================================");
    println!("");
    println!("Connecting to... host: {:#?} port: {:#?}", host, port);
    println!("");
}

#[actix_rt::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let host = &args[1];
    let port = &args[2];

    print_preamble(host.to_string(), port.to_string());
    loop {
        let mut query = String::new();
        print!("tao > ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut query);
        let query = query.trim_end();
        let res = execute_tao_query(
            host.to_string(),
            port.to_string(),
            query.to_string(),
        )
        .await;
        println!("");
        println!("Query: {:#?}", query);
        println!("Result:");
        println!("{:#?}", res);
        println!("");
    }
}
