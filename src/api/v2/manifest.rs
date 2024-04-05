use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/manifest-v2-1.md
#[derive(Default, Serialize, Deserialize)]
pub struct Manifest {
    name: String,
    tag: String,
    architecture: String,
    #[serde(rename = "fsLayers")]
    fs_layers: Vec<FsLayer>,
    history: Vec<History>,
    #[serde(rename = "schemaVersion")]
    schema_version: i32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct FsLayer {
    #[serde(rename = "blobSum")]
    blob_sum: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct History {
    #[serde(rename = "v1Compatibility")]
    v1_compatibility: String,
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#get-manifest
pub async fn get_reference(
    Path(name): Path<String>,
    Path(reference): Path<String>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // Digest of the targeted content for the request.
    headers.insert("Docker-Content-Digest", "".parse().unwrap());

    (headers, Json(Manifest::default()))
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#existing-manifests
pub async fn head_reference(
    Path(name): Path<String>,
    Path(reference): Path<String>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // The length of the targeted manifest.
    headers.insert("Content-Length", "".parse().unwrap());
    // Digest of the targeted content for the request.
    headers.insert("Docker-Content-Digest", "".parse().unwrap());

    headers
}


// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#put-manifest
pub async fn put_reference(
    Path(name): Path<String>,
    Path(reference): Path<String>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // The canonical location url of the uploaded manifest.
    headers.insert("Location", "".parse().unwrap());
    // Digest of the targeted content for the request.
    headers.insert("Docker-Content-Digest", "".parse().unwrap());

    (StatusCode::CREATED, headers)
}

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#delete-manifest
pub async fn delete_reference(
    Path(name): Path<String>,
    Path(reference): Path<String>,
) -> impl IntoResponse {
    StatusCode::ACCEPTED
}
