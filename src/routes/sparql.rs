use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde::Deserialize;

use crate::AppState;

#[derive(Template)]
#[template(path = "sparql.html")]
struct SparqlTemplate {
    original: String,
    processed: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct FormInput {
    text: String
}

pub(crate) async fn sparql() -> impl IntoResponse {
    let template = SparqlTemplate {
        original: "SELECT ?person ?org WHERE {
           ?person ex:knows ex:bob .
           ?org ex:partOf ex:consortium .
         }".to_string(),
        processed: None,
    };
    Html(template.render().unwrap())
}

pub(crate) async fn process(
    State(state): State<AppState>,
    Form(input): Form<FormInput>) -> impl IntoResponse {
    let result = match state.store.query(input.text.clone()) {
        Ok(o) => o.to_string(),
        Err(e) => format!("Error Processing sparql Query: {e:?}"),
    };

    let template = SparqlTemplate {
        original: input.text,
        processed: Some(result),
    };
    Html(template.render().unwrap())
}
