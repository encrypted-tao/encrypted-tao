mod db_client;
use db_client::db_connect as db_main;


async fn async_main() {
    db_main().await.unwrap();
}

fn main() {
    async_std::task::block_on(async_main());
}

