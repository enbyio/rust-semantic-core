use axum::extract::State;
use axum::response::Html;

use crate::AppState;

pub async fn root(State(_): State<AppState>) -> Html<String> {
    Html("<h1>Hello World</h1>".to_string())
}
