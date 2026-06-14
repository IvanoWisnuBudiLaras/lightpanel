mod config;
mod core;
mod db;
mod models;
mod routes;
mod security;
mod services;
mod utils;

use crate::core::state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let database = db::connection::open_database("lightpanel.db")
        .expect("failed to open LightPanel database");

    db::migrations::run_migrations(&database)
        .expect("failed to run LightPanel migrations");

    let state = AppState::new(database);
    let app = routes::router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind LightPanel backend");

    println!("LightPanel backend listening on http://{addr}");
    axum::serve(listener, app)
        .await
        .expect("LightPanel backend server failed");
}
