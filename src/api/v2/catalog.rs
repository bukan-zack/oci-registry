use axum::response::{IntoResponse, Json};
use serde_json::json;

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#catalog-fetch
pub async fn get_list() -> impl IntoResponse {
    Json(json!({
        "repositories": vec![""]
    }))
}