mod client_factory;
mod settings;
mod utils;

use client_factory::{ClientFactory, Result};
use settings::Settings;
use simple_logger::SimpleLogger;
use std::path::Path;
use tokio::runtime;

const SESSIONS_FOLDER: &str = "sessions";

async fn async_main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let settings = Settings::new().unwrap();

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
