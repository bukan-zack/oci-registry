use axum::Router;
use axum::routing::get;

mod root;

pub fn routing() -> Router {
    let router = Router::new()
        .route("/", get(root::root));

    router
}