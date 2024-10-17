use axum::{
    routing::{get, post, delete},
    Router,
};

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
        .with_state(AppState {
            configuration: configuration::get_configuration()
                .expect("Failed to load configuration"),
        });
    app
}
