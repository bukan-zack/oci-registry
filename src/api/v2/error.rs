// See: https://github.com/distribution/distribution/blob/5cb406d511b7b9163bff9b6439072e4892e5ae3b/docs/spec/api.md#errors-2
pub enum RegistryError {
    BlobUnknown,
    BlobUploadInvalid,
    BlobUploadUnknown,
    DigestInvalid,
    ManifestBlobUnknown,
    ManifestInvalid,
    ManifestUnknown,
    ManifestUnverified,
    NameInvalid,
    NameUnknown,
    SizeInvalid,
    TagInvalid,
    Unauthorized,
    Denied,
    Unsupported,
}

// RegistryError::Unauthorized => (StatusCode::UNAUTHORIZED, "authentication required", "The access controller was unable to authenticate the client. Often this will be accompanied by a Www-Authenticate HTTP response header indicating how to authenticate."),
// RegistryError::Denied => (StatusCode::FORBIDDEN, "requested access to the resource is denied", "The access controller denied access for the operation on a resource."),
// RegistryError::Unsupported => (StatusCode::METHOD_NOT_ALLOWED, "The operation is unsupported.", "The operation was unsupported due to a missing implementation or invalid set of parameters.")
