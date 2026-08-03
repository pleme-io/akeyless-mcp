use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum AkeylessMcpError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API returned {status}: {body}")]
    Api { status: u16, body: String },

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    // Deliberately does NOT mention --api-key: pointing an operator at a
    // flag is what puts the credential in the process table and their
    // shell history in the first place.
    #[error("API key not found -- set AKEYLESS_MCP_API_KEY, or create {path} (mode 0600)")]
    NoApiKey { path: PathBuf },
}

pub type Result<T> = std::result::Result<T, AkeylessMcpError>;
