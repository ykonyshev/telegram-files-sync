mod client_factory;
mod models;
mod db;
mod settings;
mod utils;
mod synced_fs;

use client_factory::{ClientFactory, Result};
use sea_orm::EntityTrait;
use settings::Settings;
use simple_logger::SimpleLogger;
use std::path::Path;
use tokio::runtime;
use models::node;

const SESSIONS_FOLDER: &str = "sessions";

async fn async_main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let settings = Settings::new().unwrap();
    let db_connection = db::connect(&settings.db.connection_string).await;

    let nodes: Vec<node::Model> = node::Entity::find().all(&db_connection).await?;
    for node in nodes {
        println!("{:?}", node);
    }

    let mut client_factory = ClientFactory::new(
        Path::new(SESSIONS_FOLDER),
        settings.telegram.api_id,
        settings.telegram.api_hash,
    );

    let client = client_factory.make_client().await?;
    let me = client.get_me().await?;
    println!("{}", me.username().unwrap());

    Ok(())
}

fn main() -> Result<()> {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
