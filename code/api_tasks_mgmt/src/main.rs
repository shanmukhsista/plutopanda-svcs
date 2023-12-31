
mod handlers;
pub mod models;
mod db;
mod state;
use axum::{
    routing::{post},
    Router,
};
use std::net::SocketAddr;
use axum::routing::{get, put};
use sqlx::migrate::{Migrator};
use lib_api_common;
use crate::handlers::projects::{create_new_project, get_all_projects};
use crate::handlers::work_items::{complete_work_item, log_work_item, get_work_item_by_id, start_work_item};

static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"

#[tokio::main]
async fn main() {
    // initialinze tracing subscriber.
    lib_api_common::tracing::init_tracing_subscriber();
    let database_pool = lib_api_common::db::get_database_pool().await.unwrap();
    MIGRATOR.run(&database_pool).await.unwrap();
    tracing::debug!("Database initialized and migrated. ");
    // run the migrations on server startup

    let state = state::init(database_pool);

    // build our application with some routes
    // https://www.reddit.com/r/rust/comments/15159w6/with_axum_do_you_put_all_dependencies_in_the_state/
    let app = Router::new()
        .route(
            "/projects",
            post(create_new_project).get(get_all_projects)

        )        .route("/projects/work-items/:id",get(get_work_item_by_id))
        .route("/projects/work-items",post(log_work_item))
        .route("/projects/work-items/:id/start", put(start_work_item))
        .route("/projects/work-items/:id/complete", put(complete_work_item))
        .with_state(state);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    tracing::debug!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
