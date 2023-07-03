use axum::{Router, routing::get, http::{Uri, header, StatusCode}, response::{IntoResponse, Response}, body::{boxed, Full}};
use hyper::{Client, client::HttpConnector, Request, Body, http::uri::{Parts, PathAndQuery}};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./frontend/dist"]
struct Assets;

const INDEX_HTML: &str = "index.html";

static CLIENT: Lazy<Client<HttpConnector>> = Lazy::new(|| Client::new());

pub fn router(beam_proxy_url: Uri, monitoring_secret: String) -> Router {
    let monitoring_endpoint: Uri = {
        let mut parts = beam_proxy_url.into_parts();
        parts.path_and_query = Some(PathAndQuery::try_from("/v1/monitor/events").unwrap());
        Uri::from_parts(parts).unwrap()
    };
    Router::new()
        .route("/events", get(move || async move {
            CLIENT.request(Request::get(monitoring_endpoint).header(header::COOKIE, format!("{monitoring_secret}")).body(Body::empty()).expect("Building request failed")).await.expect("Failed to make request to beam proxy")
        }))
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
