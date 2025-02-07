use axum::{
    extract::{DefaultBodyLimit, MatchedPath},
    http::Request,
    routing::{delete, get, post},
    Router,
};
use primitypes::consts::{MAX_SUBMISSION_FILE_SIZE_IN_BYTES};
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

use crate::{configuration, routes::files};

#[derive(Clone)]
pub struct AppState {
    pub configuration: configuration::Settings,
}

pub async fn startup() -> Router<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/file/:dir_id", post(files::post_create_file))
        .route("/dir/:dir_id", post(files::post_create_dir))
        .route("/file/:dir_id/:file_name", delete(files::delete_file))
        .route("/file/:dir_id/:file_name", get(files::get_file))
        .route("/checker/:dir_id", post(files::new_checker))
        .layer(DefaultBodyLimit::disable())
        .layer(DefaultBodyLimit::max(MAX_SUBMISSION_FILE_SIZE_IN_BYTES))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(AppState {
            configuration: configuration::get_configuration()
                .expect("Failed to load configuration"),
        });
    app
}
