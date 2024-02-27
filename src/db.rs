use diesel::prelude::*;

pub async fn connect(connection_string: &String) -> SqliteConnection {
    let connection_result = SqliteConnection::establish(connection_string);

    match connection_result {
        Ok(connection) => connection,
        Err(err) => panic!(
            "Could not connect to the db at \"{}\"\nError: {}",
            connection_string, err
        ),
    }
}
