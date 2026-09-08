use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Form,
};
use pg_triple_store::query::solution::QueryResult;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct DownloadForm {
    query: String,
}

pub async fn download_rdfxml(
    State(state): State<AppState>,
    Form(form): Form<DownloadForm>,
) -> Response {
    print!("{}", form.query);
    match state.store.query(form.query) {
        Ok(QueryResult::Graph(triples)) => {
            // adjust to whatever your store's actual serialization fn is called
            match state.store.export_as_rdfxml(&triples) {
                Ok(xml) => (
                    StatusCode::OK,
                    [
                        (header::CONTENT_TYPE, "application/rdf+xml"),
                        (
                            header::CONTENT_DISPOSITION,
                            "attachment; filename=\"result.rdf\"",
                        ),
                    ],
                    xml,
                )
                    .into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serialize RDF/XML: {e:?}"),
                )
                    .into_response(),
            }
        }
        Ok(_) => (
            StatusCode::BAD_REQUEST,
            "Query did not produce a graph result",
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("Error processing SPARQL query: {e:?}"),
        )
            .into_response(),
    }
}
