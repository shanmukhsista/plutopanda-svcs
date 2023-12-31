use lib_api_common::db::DatabasePool;

#[derive(Clone)]
pub struct AppState {
    pub database_pool: DatabasePool,
}

/*
    Initialize a state and take ownership of database pool.
 */
pub fn init(database_pool: DatabasePool) -> AppState {
    AppState { database_pool }
}
