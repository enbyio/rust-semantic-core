use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use pg_triple_store::store::TripleStore;

use crate::routes::import::{import_data, process_import};
use crate::routes::sparql::{process, sparql};

// use crate::routes::root::root;

pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<TripleStore>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let store = TripleStore::new_from_env().unwrap();

    store.reset_db().unwrap();

    store.import_turtle_file("test_data/data.ttl").unwrap();

    let state = AppState {
        store: Arc::new(store)
    };

    let router = Router::new()
        .route("/", get(sparql))
        .route("/import_data", get(import_data))
        .route("/import_data", post(process_import))
        .route("/process", post(process))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
        axum::serve(listener, router).await?;

    Ok(())
}
