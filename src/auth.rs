use crate::config::AkeylessMcpConfig;
use crate::error::{AkeylessMcpError, Result};
use cofre_secret::Secret;
use std::path::PathBuf;
use std::sync::Arc;

/// The environment variable that supplies the API key.
///
/// This is the repo's established name (it predates the deprecation of
/// `--api-key`) and the one the Nix HM module and the error text already
/// use, so it is kept rather than renamed to something like
/// `AKEYLESS_ACCESS_KEY` — which in this fleet names a *file path*, not a
/// key value, and would therefore be actively misleading here.
pub const API_KEY_ENV: &str = "AKEYLESS_MCP_API_KEY";

/// Resolve the API key from (in priority order):
/// 1. The `AKEYLESS_MCP_API_KEY` environment variable
/// 2. Contents of the configured `api_key_file`
/// 3. An explicit `--api-key` flag value, if a caller passes one
///
/// The result is a [`Secret`], not a `String`: it implements no
/// `Display`/`AsRef<str>`/`AsRef<OsStr>`, so it cannot be interpolated
/// into a format string, printed by a derived `Debug`, or placed on a
/// child process's command line.
///
/// `Arc` only so that the holding client stays `Clone`; `Secret` is
/// deliberately not `Clone`.
///
/// # Why the flag is LAST
///
/// Source (3) is the deprecated `--api-key` flag. A value passed on the
/// command line is readable from the process table by every local user
/// and is written to shell history, so it is the least-safe of the three
/// and is consulted only when neither of the others supplies a key. It
/// is still honoured rather than removed so that no existing caller
/// silently loses its credential.
///
/// Note this ordering is defence in depth, not the only guard: the sole
/// production caller (`mcp::AkeylessMcpMcp::new`) passes `None` here, so
/// in this binary the flag cannot supply a key at all. The ordering is
/// what keeps that true if the flag is ever threaded through.
pub fn resolve_api_key(explicit: Option<&str>, config: &AkeylessMcpConfig) -> Result<Arc<Secret>> {
    // 1. Environment variable. `Secret::from_env` reads it straight into
    //    the credential type; an unset OR empty var is an `Err` here, and
    //    both mean "not supplied by this source", so we fall through.
    if let Ok(key) = Secret::from_env(API_KEY_ENV) {
        return Ok(Arc::new(key));
    }

    let path = expand_tilde(&config.api_key_file);

    // 2. File (expected to be mode 0600).
    if let Ok(content) = std::fs::read_to_string(&path) {
        let key = content.trim();
        if !key.is_empty() {
            return wrap(key, path.clone());
        }
    }

    // 3. Deprecated --api-key, last resort.
    if let Some(key) = explicit.filter(|k| !k.is_empty()) {
        return wrap(key, path.clone());
    }

    Err(AkeylessMcpError::NoApiKey { path })
}

/// Build the `Secret`. `Secret::new` rejects an empty value; every
/// caller above has already filtered empties, so the error branch is
/// mapped back to the same "no usable key" shape rather than inventing
/// a second failure mode.
fn wrap(key: &str, path: PathBuf) -> Result<Arc<Secret>> {
    Secret::new(key)
        .map(Arc::new)
        .map_err(|_| AkeylessMcpError::NoApiKey { path })
}

fn expand_tilde(path: &PathBuf) -> PathBuf {
    let s = path.to_string_lossy();
    if let Some(rest) = s.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    path.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_at(key_file: PathBuf) -> AkeylessMcpConfig {
        AkeylessMcpConfig { api_url: "https://example.invalid".into(), api_key_file: key_file }
    }

    /// Every assertion that touches `AKEYLESS_MCP_API_KEY` lives in this
    /// ONE test function on purpose. `std::env::set_var` mutates
    /// process-global state and cargo runs test functions on parallel
    /// threads, so splitting these into separate `#[test]`s would let two
    /// of them race on the same variable and flake.
    #[test]
    fn env_var_takes_precedence_over_the_deprecated_flag() {
        let dir =
            std::env::temp_dir().join(format!("akeyless_mcp_auth_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let key_file = dir.join("api-key");
        std::fs::write(&key_file, "from-file\n").unwrap();
        let config = config_at(key_file.clone());

        // The property this test exists for: both sources present, env wins.
        // SAFETY: single-threaded within this test; see the note above.
        unsafe { std::env::set_var(API_KEY_ENV, "from-env") };
        let resolved = resolve_api_key(Some("from-argv"), &config).unwrap();
        assert_eq!(
            resolved.expose(),
            "from-env",
            "AKEYLESS_MCP_API_KEY must win over the --api-key flag"
        );

        // The env var also beats the key file.
        let resolved = resolve_api_key(None, &config).unwrap();
        assert_eq!(resolved.expose(), "from-env");

        // With the env var gone the FILE wins, not the flag: argv is the
        // least-safe source and is the last one consulted.
        // SAFETY: as above.
        unsafe { std::env::remove_var(API_KEY_ENV) };
        let resolved = resolve_api_key(Some("from-argv"), &config).unwrap();
        assert_eq!(
            resolved.expose(),
            "from-file",
            "a 0600 key file must win over a credential on the command line"
        );

        // Positive control. Without this the three assertions above would
        // also pass if the flag were simply ignored outright, which would
        // make them prove nothing about *precedence*. With neither env nor
        // file present the flag IS still honoured, so the flag is a live
        // source that the other two genuinely outrank.
        std::fs::remove_file(&key_file).unwrap();
        let resolved = resolve_api_key(Some("from-argv"), &config).unwrap();
        assert_eq!(resolved.expose(), "from-argv", "the flag must still work when nothing else supplies a key");

        // And with no source at all, a typed error rather than an empty key.
        assert!(resolve_api_key(None, &config).is_err());

        std::fs::remove_dir_all(&dir).ok();
    }
}
