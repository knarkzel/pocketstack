use axum::{
    body::{boxed, Full},
    http::{header, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use rust_embed::RustEmbed;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Server routes
    let app = Router::new()
        .route("/api/hello", get(|| async { "Hello, world!" }))
        .route("/", get(index_handler))
        .route("/*path", get(static_handler))
        .layer(CorsLayer::new().allow_origin([
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://localhost:5173"),
        ]));

    // Start server
    let port = 3000;
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://localhost:{port}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("/index.html")).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/");

    // Add .html to routes
    match PathBuf::from(path).extension() {
        Some(_) => StaticFile(path.to_string()),
        None => StaticFile(format!("{path}.html")),
    }
}

#[derive(RustEmbed)]
#[folder = "web/build"]
struct Assets;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Assets::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}
