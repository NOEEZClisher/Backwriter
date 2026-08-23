#[cfg(test)]
pub(crate) fn is_safe_relative_path(value: &str) -> bool {
    validate_logical_path(value).is_ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LogicalPathError {
    Unsafe,
    Reserved,
    Private,
}

pub(crate) fn validate_logical_path(value: &str) -> Result<(), LogicalPathError> {
    if value.is_empty()
        || value.contains(['\0', '\\'])
        || value.starts_with('/')
        || value.ends_with('/')
        || value.chars().any(char::is_control)
    {
        return Err(LogicalPathError::Unsafe);
    }
    for component in value.split('/') {
        if component.is_empty()
            || component == "."
            || component == ".."
            || component.contains(':')
            || component.ends_with(['.', ' '])
            || is_windows_device_name(component)
        {
            return Err(LogicalPathError::Unsafe);
        }
        if is_reserved_component(component) {
            return Err(LogicalPathError::Reserved);
        }
        if is_private_component(component) {
            return Err(LogicalPathError::Private);
        }
    }
    Ok(())
}

pub(crate) fn is_reserved_component(component: &str) -> bool {
    component.eq_ignore_ascii_case(".git")
}

pub(crate) fn is_private_component(component: &str) -> bool {
    component.eq_ignore_ascii_case("id_rsa")
        || component.eq_ignore_ascii_case("id_ed25519")
        || component.eq_ignore_ascii_case(".env")
        || component.eq_ignore_ascii_case("credential")
        || component.eq_ignore_ascii_case("credentials")
        || starts_with_ascii_case_insensitive(component, ".env.")
        || ends_with_ascii_case_insensitive(component, ".pem")
        || ends_with_ascii_case_insensitive(component, ".key")
}

fn is_windows_device_name(component: &str) -> bool {
    let base = component.split('.').next().unwrap_or_default();
    let bytes = base.as_bytes();
    base.eq_ignore_ascii_case("CON")
        || base.eq_ignore_ascii_case("PRN")
        || base.eq_ignore_ascii_case("AUX")
        || base.eq_ignore_ascii_case("NUL")
        || (bytes.len() == 4
            && (bytes[..3].eq_ignore_ascii_case(b"COM") || bytes[..3].eq_ignore_ascii_case(b"LPT"))
            && matches!(bytes[3], b'1'..=b'9'))
}

fn starts_with_ascii_case_insensitive(value: &str, prefix: &str) -> bool {
    value
        .as_bytes()
        .get(..prefix.len())
        .is_some_and(|bytes| bytes.eq_ignore_ascii_case(prefix.as_bytes()))
}

fn ends_with_ascii_case_insensitive(value: &str, suffix: &str) -> bool {
    let Some(start) = value.len().checked_sub(suffix.len()) else {
        return false;
    };
    value
        .as_bytes()
        .get(start..)
        .is_some_and(|bytes| bytes.eq_ignore_ascii_case(suffix.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::is_safe_relative_path;

    #[test]
    fn paths_use_an_unbounded_platform_neutral_grammar() {
        assert!(is_safe_relative_path("docs/한글.md"));
        assert!(is_safe_relative_path("docs/invalid"));
        assert!(is_safe_relative_path(&format!(
            "docs/{}",
            "a".repeat(8_192)
        )));
        for path in [
            "",
            "/absolute.md",
            "../escape.md",
            "a/../b",
            "a\\b",
            "C:/x",
            "doc:stream",
            ".GIT/config",
            "docs/NUL.txt",
            "docs/file. ",
        ] {
            assert!(!is_safe_relative_path(path));
        }
    }
}
