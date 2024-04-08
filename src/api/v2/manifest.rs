use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

// See: https://github.com/opencontainers/distribution-spec/blob/ef28f81727c3b5e98ab941ae050098ea664c0960/detail.md#get-manifest
// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/manifest-v2-2.md#image-manifest
#[derive(Default, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    schema_version: i64,
    #[serde(rename = "mediaType")]
    media_type: String,
    layers: Vec<ManifestLayer>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ManifestConfig {
    #[serde(rename = "mediaType")]
    media_type: String,
    size: i64,
    digest: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ManifestLayer {
    #[serde(rename = "mediaType")]
    media_type: String,
    size: i64,
    digest: String,
    urls: Vec<String>,
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
