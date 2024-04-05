use axum::Router;
use axum::routing::{delete, get, head, patch, post, put};

mod error;
mod root;
mod tag;
mod manifest;
mod blob;
mod catalog;

// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#detail
pub fn routing() -> Router {
    Router::new()
        .route("/", get(root::get_root))
        .route("/:name/tags/list", get(tag::get_list))
        .route("/:name/manifests/:reference", get(manifest::get_reference))
        .route("/:name/manifests/:reference", head(manifest::head_reference))
        .route("/:name/manifests/:reference", put(manifest::put_reference))
        .route("/:name/manifests/:reference", delete(manifest::delete_reference))
        .route("/:name/blobs/:digest", get(blob::get_digest))
        .route("/:name/blobs/:digest", head(blob::head_digest))
        .route("/:name/blobs/:digest", delete(blob::delete_digest))
        .route("/:name/blobs/uploads", post(blob::post_upload))
        .route("/:name/blobs/uploads/:uuid", get(blob::get_upload))
        .route("/:name/blobs/uploads/:uuid", patch(blob::patch_upload))
        .route("/:name/blobs/uploads/:uuid", put(blob::put_upload))
        .route("/:name/blobs/uploads/:uuid", delete(blob::delete_upload))
        .route("/_catalog", get(catalog::get_list))
}
