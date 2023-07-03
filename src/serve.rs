use axum::{Router, routing::get, http::{Uri, header, StatusCode}, response::{IntoResponse, Response}, body::{boxed, Full}};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dist"]
struct Assets;

const INDEX_HTML: &str = "index.html";

pub fn router() -> Router {
    Router::new()
        .route("/events", get(todo!()))
        .fallback(static_handler)
}

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html();
    }

    if let Some(content) = Assets::get(path) {
        Response::builder()
            .header(header::CONTENT_TYPE, content.metadata.mimetype())
            .body(boxed(Full::from(content.data)))
            .unwrap()
    } else {
        if path.contains('.') {
            return StatusCode::NOT_FOUND.into_response();
        };
        index_html()
    }
}

fn index_html() -> Response {
    if let Some(content) = Assets::get(INDEX_HTML) {
        Response::builder()
            .header(header::CONTENT_TYPE, "text/html")
            .body(boxed(Full::from(content.data)))
            .unwrap()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
