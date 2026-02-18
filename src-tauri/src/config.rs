/// Compile-time configuration sourced from environment variables.
///
/// Values are baked in at build time via `cargo:rustc-env` directives in build.rs.
/// Override by setting the corresponding env var or editing `.env` before building.

pub const GOOGLE_CLIENT_ID: &str = env!("GOOGLE_CLIENT_ID");
pub const GOOGLE_AUTH_ENDPOINT: &str = env!("GOOGLE_AUTH_ENDPOINT");
pub const GOOGLE_TOKEN_ENDPOINT: &str = env!("GOOGLE_TOKEN_ENDPOINT");
pub const OAUTH_REDIRECT_URI: &str = env!("OAUTH_REDIRECT_URI");
pub const GOOGLE_DRIVE_API_BASE: &str = env!("GOOGLE_DRIVE_API_BASE");
pub const GOOGLE_DRIVE_UPLOAD_URL: &str = env!("GOOGLE_DRIVE_UPLOAD_URL");
