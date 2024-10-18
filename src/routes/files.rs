use std::io;

use axum::{
    body::Bytes,
    extract::{Multipart, Path, State},
    http::StatusCode,
    BoxError,
};
use futures::{Stream, TryStreamExt};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tracing::debug;

use crate::state::AppState;

#[axum_macros::debug_handler]
pub async fn post_create_dir(
    State(state): State<AppState>,
    Path(dir_id): Path<String>,
) -> Result<(), (StatusCode, String)> {
    Ok(create_dir(state.configuration.ftp_home.as_str(), dir_id.as_str()).await?)
}

async fn create_dir(root: &str, dir_id: &str) -> Result<(), (StatusCode, String)> {
    let path = std::path::Path::new(root).join(dir_id);
    tokio::fs::create_dir_all(path)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(())
}

#[axum_macros::debug_handler]
pub async fn post_create_file(
    State(state): State<AppState>,
    Path(dir_id): Path<String>,
    mut multipart: Multipart,
) -> Result<(), (StatusCode, String)> {
    debug!("Creating file in dir {}", dir_id);
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        stream_to_file(
            state.configuration.ftp_home.as_str(),
            dir_id.as_str(),
            &file_name,
            field,
        )
        .await?;
    }

    Ok(())
}

#[axum_macros::debug_handler]
pub async fn delete_file(
    State(state): State<AppState>,
    Path((dir_id, file_name)): Path<(String, String)>,
) -> Result<(), (StatusCode, String)> {
    if !path_is_valid(file_name.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }
    let path = std::path::Path::new(state.configuration.ftp_home.as_str())
        .join(dir_id)
        .join(file_name);

    tokio::fs::remove_file(path)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(())
}

#[axum_macros::debug_handler]
pub async fn get_file(
    State(state): State<AppState>,
    Path((dir_id, file_name)): Path<(String, String)>,
) -> Result<Bytes, (StatusCode, String)> {
    if !path_is_valid(file_name.as_str()) || !path_is_valid(dir_id.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }
    let path = std::path::Path::new(state.configuration.ftp_home.as_str())
        .join(dir_id)
        .join(file_name);

    let bytes = tokio::fs::read(path)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Bytes::from(bytes))
}
//
// Save a `Stream` to a file
async fn stream_to_file<S, E>(
    root: &str,
    dir_id: &str,
    path: &str,
    stream: S,
) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) || !path_is_valid(dir_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(root).join(dir_id).join(path);
        let cloned_path = path.clone();
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;
        debug!("File saved to {:?}", cloned_path);

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly
// one normal component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
