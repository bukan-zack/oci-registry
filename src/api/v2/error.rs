use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct RegistryErrorResponse {
    code: &'static str,
    message: &'static str,
}

// See: https://github.com/opencontainers/distribution-spec/blob/main/spec.md#error-codes
#[allow(dead_code)]
pub enum RegistryError {
    Unknown,
    BlobUnknown,
    BlobUploadInvalid,
    BlobUploadUnknown,
    DigestInvalid,
    ManifestBlobUnknown,
    ManifestInvalid,
    ManifestUnknown,
    NameInvalid,
    NameUnknown,
    SizeInvalid,
    Unauthorized,
    Denied,
    Unsupported,
    TooManyRequests,
}

impl RegistryError {
    pub fn status_code(self) -> StatusCode {
        match self {
            RegistryError::BlobUnknown => StatusCode::NOT_FOUND,
            RegistryError::BlobUploadInvalid => StatusCode::NOT_FOUND,
            RegistryError::BlobUploadUnknown => StatusCode::NOT_FOUND,
            RegistryError::DigestInvalid => StatusCode::BAD_REQUEST,
            RegistryError::ManifestBlobUnknown => StatusCode::BAD_REQUEST,
            RegistryError::ManifestInvalid => StatusCode::BAD_REQUEST,
            RegistryError::ManifestUnknown => StatusCode::NOT_FOUND,
            RegistryError::NameInvalid => StatusCode::BAD_REQUEST,
            RegistryError::NameUnknown => StatusCode::NOT_FOUND,
            RegistryError::SizeInvalid => StatusCode::BAD_REQUEST,
            RegistryError::Unauthorized => StatusCode::UNAUTHORIZED,
            RegistryError::Denied => StatusCode::FORBIDDEN,
            RegistryError::Unsupported => StatusCode::METHOD_NOT_ALLOWED,
            RegistryError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            RegistryError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn into_error_response(self) -> RegistryErrorResponse {
        match self {
            RegistryError::BlobUnknown => RegistryErrorResponse {
                code: "BLOB_UNKNOWN",
                message: "blob unknown to registry",
            },
            RegistryError::BlobUploadInvalid => RegistryErrorResponse {
                code: "BLOB_UPLOAD_INVALID",
                message: "blob upload invalid",
            },
            RegistryError::BlobUploadUnknown => RegistryErrorResponse {
                code: "BLOB_UPLOAD_UNKNOWN",
                message: "blob upload unknown to registry",
            },
            RegistryError::DigestInvalid => RegistryErrorResponse {
                code: "DIGEST_INVALID",
                message: "provided digest did not match uploaded content",
            },
            RegistryError::ManifestBlobUnknown => RegistryErrorResponse {
                code: "MANIFEST_BLOB_UNKNOWN",
                message: "manifest references a manifest or blob unknown to registry",
            },
            RegistryError::ManifestInvalid => RegistryErrorResponse {
                code: "MANIFEST_INVALID",
                message: "manifest invalid",
            },
            RegistryError::ManifestUnknown => RegistryErrorResponse {
                code: "MANIFEST_UNKNOWN",
                message: "manifest unknown to registry",
            },
            RegistryError::NameInvalid => RegistryErrorResponse {
                code: "NAME_INVALID",
                message: "invalid repository name",
            },
            RegistryError::NameUnknown => RegistryErrorResponse {
                code: "NAME_UNKNOWN",
                message: "repository name not known to registry",
            },
            RegistryError::SizeInvalid => RegistryErrorResponse {
                code: "SIZE_INVALID",
                message: "provided length did not match content length",
            },
            RegistryError::Unauthorized => RegistryErrorResponse {
                code: "UNAUTHORIZED",
                message: "authentication required",
            },
            RegistryError::Denied => RegistryErrorResponse {
                code: "DENIED",
                message: "requested access to the resource is denied",
            },
            RegistryError::Unsupported => RegistryErrorResponse {
                code: "UNSUPPORTED",
                message: "the operation is unsupported",
            },
            RegistryError::TooManyRequests => RegistryErrorResponse {
                code: "TOOMANYREQUESTS",
                message: "too many requests",
            },
            RegistryError::Unknown => RegistryErrorResponse {
                code: "UNKNOWN",
                message: "unknown error",
            }
        }
    }
}