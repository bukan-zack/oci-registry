use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn version_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // If the Content-Type not present, default to application/json.
    if !headers.contains_key("Content-Type") {
        headers.insert("Content-Type", "application/json; charset=utf-8".parse().unwrap());
    }

    headers.insert("Docker-Distribution-API-Version", "registry/2.0".parse().unwrap());

    response
}