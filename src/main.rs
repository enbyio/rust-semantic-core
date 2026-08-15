use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use pg_triple_store::store::TripleStore;

use crate::routes::root::root;

pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<TripleStore>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let store = TripleStore::new_from_env().unwrap();
    let state = AppState {
        store: Arc::new(store)
    };

    let router = Router::new()
        .route("/", get(root))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
        axum::serve(listener, router).await?;

    Ok(())
}
