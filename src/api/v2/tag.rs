use axum::extract::Path;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

// See: https://github.com/opencontainers/distribution-spec/blob/ef28f81727c3b5e98ab941ae050098ea664c0960/detail.md#tags-1
pub async fn get_list(
    Path(name): Path<String>,
) -> impl IntoResponse {
    Json(json!({
        "name": name,
        "tags": vec![""]
    }))
}