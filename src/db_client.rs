use sqlx::{postgres::PgPool};

mod data;

pub async fn db_connect() -> Result<(), sqlx::Error> {

    dotenv::from_path("../.env").ok();

    let host = dotenv::var("DATABASE_HOST").unwrap();
    let port = dotenv::var("DATABASE_PORT_NUM").unwrap().parse::<u16>().unwrap();
    let database_name = dotenv::var("DATABASE_NAME").unwrap();
    let username = dotenv::var("DATABASE_USERNAME").unwrap();
    let password = dotenv::var("DATABASE_PASSWORD").unwrap();

    // Create a connection pool to the database
    let pool = PgPool::connect(&format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, database_name
        ))
        .await?;

    // Create Objects Table
    sqlx::query(
        "DROP TABLE IF EXISTS Objects",
    ).execute(&pool)
    .await?;

    let create_object_table = sqlx::query(
        "CREATE TABLE Objects (
            id              SERIAL PRIMARY KEY,
            key             TEXT NOT NULL,
            obj_type     TEXT NOT NULL,
            val           TEXT NOT NULL
            )",
    ).execute(&pool)
    .await?;

    // Insert into Objects Table
    for object in data::OBJECTS {
        sqlx::query(
            "INSERT INTO Objects (id, key, obj_type, val)
                VALUES ($1, $2, $3, $4)",
        )
        .bind(object.id)
        .bind(&object.key)
        .bind(&object.obj_type)
        .bind(&object.val)
        .execute(&pool)
        .await?;
    }

    sqlx::query(
        "DROP TABLE IF EXISTS Associations",
    ).execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE Associations (
            id                SERIAL PRIMARY KEY,
            obj_1             INTEGER NOT NULL,
            obj_2             INTEGER NOT NULL,
            assoc_type        TEXT NOT NULL,
            time_stamp        TEXT NOT NULL,
            key               TEXT NOT NULL,
            val               TEXT NOT NULL
            )",
    ).execute(&pool)
    .await?;

    for association in data::ASSOCIATIONS {
        sqlx::query(
            "INSERT INTO Associations (id, obj_1, obj_2, assoc_type, time_stamp, key, val)
                  VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(association.id)
        .bind(association.obj_1)
        .bind(association.obj_2)
        .bind(&association.assoc_type)
        .bind(&association.time_stamp)
        .bind(&association.key)
        .bind(&association.val)
        .execute(&pool)
        .await?;
    }


    println!("COMPLETE!");
    Ok(())
}
