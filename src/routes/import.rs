use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum FileType {
    Ttl,
    Rdf,
}

// Mirrors what your template accesses as `form.*`
struct ImportFormState {
    file_type: FileType,
    data: String,
}

#[derive(Template)]
#[template(path = "import.html")]
struct ImportTemplate {
    form: ImportFormState,
    result: Option<String>,
}

// What actually gets deserialized from the POSTed form body
#[derive(Deserialize)]
pub(crate) struct ImportInput {
    text: String,
    format: FileType, // matches name="format" on the radios
}

pub(crate) async fn import_data(State(_): State<AppState>) -> impl IntoResponse {
    let template = ImportTemplate {
        form: ImportFormState {
            file_type: FileType::Ttl, // default radio selection
            data: String::new(),
        },
        result: None,
    };
    Html(template.render().unwrap())
}

pub(crate) async fn process_import(
    State(state): State<AppState>,
    Form(input): Form<ImportInput>,
) -> impl IntoResponse {
    let input_data = input.text.clone();
    // Replace with your actual import function on AppState
    let outcome = match input.format {
        FileType::Ttl => state.store.import_turtle_data(input_data),
        FileType::Rdf => state.store.import_rdfxml_data(input_data),
    };

    let (data, result) = match outcome {
        Ok(_) => (String::new(), Some("Import successful".to_string())),
        Err(_) => (
            input.text,
            Some("Error importing data, see internal log for details".to_string()),
        ),
    };

    let template = ImportTemplate {
        form: ImportFormState {
            file_type: input.format,
            data,
        },
        result,
    };
    Html(template.render().unwrap())
}
