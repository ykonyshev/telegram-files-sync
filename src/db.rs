use sea_orm::{Database, DatabaseConnection};

pub async fn connect(connection_string: &String) -> DatabaseConnection {
    let connection_result = Database::connect(connection_string).await;

    match connection_result {
        Ok(connection) => connection,
        Err(err) => panic!(
            "Could not connect to the db at \"{}\"\nError: {}",
            connection_string, err
        ),
    }
}
