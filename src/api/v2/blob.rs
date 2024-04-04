use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#fetch-blob
pub async fn get_digest() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/octet-stream".parse().unwrap());
    // The length of the requested blob content.
    headers.insert("Content-Length", "".parse().unwrap());
    // Digest of the targeted content for the request.
    headers.insert("Docker-Content-Digest", "".parse().unwrap());

    headers
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#existing-layers
pub async fn head_digest() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // The length of the requested blob content.
    headers.insert("Content-Length", "".parse().unwrap());
    // Digest of the targeted content for the request.
    headers.insert("Docker-Content-Digest", "".parse().unwrap());

    headers
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#delete-blob
pub async fn delete_digest() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // 0.
    headers.insert("Content-Length", "0".parse().unwrap());
    // Digest of the targeted content for the request.
    headers.insert("Docker-Content-Digest", "".parse().unwrap());

    (StatusCode::ACCEPTED, headers)
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#initiate-resumable-blob-upload
pub async fn post_upload() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // The Content-Length header must be zero and the body must be empty.
    headers.insert("Content-Length", "0".parse().unwrap());
    // The location of the created upload. Clients should use the contents verbatim to complete the upload, adding parameters where required.
    headers.insert("Location", "".parse().unwrap());
    // Range header indicating the progress of the upload. When starting an upload, it will return an empty range, since no content has been received.
    headers.insert("Range", "".parse().unwrap());
    // Identifies the docker upload uuid for the current request.
    headers.insert("Docker-Upload-UUID", "".parse().unwrap());

    (StatusCode::ACCEPTED, headers)
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#get-blob-upload
pub async fn get_upload() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // The Content-Length header must be zero and the body must be empty.
    headers.insert("Content-Length", "0".parse().unwrap());
    // Range indicating the current progress of the upload.
    headers.insert("Range", "".parse().unwrap());
    // Identifies the docker upload uuid for the current request.
    headers.insert("Docker-Upload-UUID", "".parse().unwrap());

    (StatusCode::NO_CONTENT, headers)
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#patch-blob-upload
pub async fn patch_upload() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // The Content-Length header must be zero and the body must be empty.
    headers.insert("Content-Length", "0".parse().unwrap());
    // The location of the upload. Clients should assume this changes after each request. Clients should use the contents verbatim to complete the upload, adding parameters where required.
    headers.insert("Location", "".parse().unwrap());
    // Range indicating the current progress of the upload.
    headers.insert("Range", "".parse().unwrap());
    // Identifies the docker upload uuid for the current request.
    headers.insert("Docker-Upload-UUID", "".parse().unwrap());

    (StatusCode::NO_CONTENT, headers)
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#chunked-upload-1
pub async fn put_upload() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // The Content-Length header must be zero and the body must be empty.
    headers.insert("Content-Length", "0".parse().unwrap());
    // The location of the upload. Clients should assume this changes after each request. Clients should use the contents verbatim to complete the upload, adding parameters where required.
    headers.insert("Location", "".parse().unwrap());
    // Range indicating the current progress of the upload.
    headers.insert("Range", "".parse().unwrap());
    // Identifies the docker upload uuid for the current request.
    headers.insert("Docker-Upload-UUID", "".parse().unwrap());

    (StatusCode::NO_CONTENT, headers)
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#delete-blob-upload
pub async fn delete_upload() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // The Content-Length header must be zero and the body must be empty.
    headers.insert("Content-Length", "0".parse().unwrap());

    (StatusCode::NO_CONTENT, headers)
}

