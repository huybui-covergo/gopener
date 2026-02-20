use std::env;

const ENV_VARS: &[(&str, Option<&str>)] = &[
    // OAuth credentials — no default; must be set in .env or environment
    ("GOOGLE_CLIENT_ID", None),
    // OAuth endpoints — stable Google defaults
    (
        "GOOGLE_AUTH_ENDPOINT",
        Some("https://accounts.google.com/o/oauth2/v2/auth"),
    ),
    (
        "GOOGLE_TOKEN_ENDPOINT",
        Some("https://oauth2.googleapis.com/token"),
    ),
    // OAuth redirect
    ("OAUTH_REDIRECT_URI", Some("http://localhost:8085")),
    // Google Drive API
    (
        "GOOGLE_DRIVE_API_BASE",
        Some("https://www.googleapis.com/drive/v3"),
    ),
    (
        "GOOGLE_DRIVE_UPLOAD_URL",
        Some("https://www.googleapis.com/upload/drive/v3/files"),
    ),
];

fn main() {
    // Load .env file if present (won't override existing env vars)
    let _ = dotenvy::dotenv();

    for &(key, default) in ENV_VARS {
        // Re-run build if the env var changes
        println!("cargo:rerun-if-env-changed={}", key);

        match env::var(key) {
            Ok(val) => println!("cargo:rustc-env={}={}", key, val),
            Err(_) => {
                if let Some(fallback) = default {
                    println!("cargo:rustc-env={}={}", key, fallback);
                } else {
                    panic!(
                        "\n\n  ERROR: Required environment variable `{}` is not set.\n  \
                         Set it in your .env file or export it before building.\n  \
                         See .env.example for reference.\n\n",
                        key
                    );
                }
            }
        }
    }

    // Re-run if .env file changes
    println!("cargo:rerun-if-changed=.env");

    tauri_build::build()
}
