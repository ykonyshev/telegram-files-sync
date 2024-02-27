mod client_factory;
mod db;
mod models;
mod settings;
mod synced_fs;
mod utils;

use client_factory::{ClientFactory, Result};
use settings::Settings;
use simple_logger::SimpleLogger;
use std::ffi::OsStr;
use std::path::Path;
use synced_fs::SyncedFs;
use tokio::runtime;

const SESSIONS_FOLDER: &str = "./sessions/";
const FS_MOUNT_POINT: &str = "./test-fs/";

// TODO: Consider switching to diesel when this will be merged. I am not sure whether there is a
// real benefit to using seaorm in this project, as it's asyncronous.
// https://github.com/diesel-rs/diesel/pull/3940
// NOTE: https://github.com/adwhit/diesel-derive-enum

async fn async_main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let settings = Settings::new().unwrap();
    let db_connection = db::connect(&settings.db.connection_string).await;

    let mut client_factory = ClientFactory::new(
        Path::new(SESSIONS_FOLDER),
        settings.telegram.api_id,
        settings.telegram.api_hash,
    );

    let client = client_factory.make_client().await?;
    let me = client.get_me().await?;
    println!("{}", me.username().unwrap());


    log::info!("Trying to mount the fuse fs at {}", FS_MOUNT_POINT);
    tokio::task::spawn_blocking(move || {
        let fs = SyncedFs::new(&db_connection, &client);
        let options = ["-o", "ro", "-o", "fsname=hello"]
            .iter()
            .map(|o| o.as_ref())
            .collect::<Vec<&OsStr>>();

        fuse::mount(fs, &Path::new(FS_MOUNT_POINT), &options).unwrap();
    }).await.unwrap();

    Ok(())
}

fn main() -> Result<()> {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
