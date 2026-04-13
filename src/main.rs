#![deny(clippy::implicit_return)]
#![feature(macro_metavar_expr_concat)]
#![feature(try_trait_v2)]
#![feature(associated_type_defaults)]

mod app;
mod config;
mod db;
mod emailer;
mod errors;
mod models;
mod routers;
use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()>
{
    let config = toml::from_str::<config::Config>(&std::fs::read_to_string(
        "./config.toml",
    )?)?;
    let log_level = match config
        .log_level
        .as_ref()
        .unwrap_or(&"INFO".to_string())
        .as_str()
    {
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "trace" => tracing::Level::TRACE,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => panic!("Unkown log level."),
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();
    let database =
        crate::db::Database::init(&config.postgres, &config.migrations, &config.redis).await?;
    let listener = tokio::net::TcpListener::bind((
        config.host.to_owned(),
        config.port.to_owned(),
    ))
    .await?;
    let app = crate::app::App::init(Arc::new(database), config).await?;
    let routes = crate::routers::generate_routers(app).await;
    info!(
        "Starting http listener on {:?}",
        listener.local_addr()?.to_string()
    );
    axum::serve(
        listener,
        routes.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    return Ok(());
}
