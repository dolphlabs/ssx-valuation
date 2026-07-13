/// Reads an environment variable, falling back to `default` if it's unset or
/// empty. Shared by every binary's `Config::from_env()` so the "env var with
/// a localhost-matching default" pattern isn't reimplemented per service.
pub fn env_or(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) if !val.trim().is_empty() => val,
        _ => default.to_string(),
    }
}

/// Same as [`env_or`], but parses the value as a `u16` (for ports). Falls
/// back to `default` if the var is unset, empty, or not a valid `u16`.
pub fn env_or_u16(key: &str, default: u16) -> u16 {
    std::env::var(key)
        .ok()
        .and_then(|val| val.trim().parse::<u16>().ok())
        .unwrap_or(default)
}
