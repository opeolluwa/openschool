use uuid::Uuid;

/// Longest display name kept from a client-supplied filename.
const MAX_FILE_NAME_LEN: usize = 120;

/// Builds a collision-free object key from an optional prefix and the upload's
/// original filename.
///
/// The override suffix guarantees that two uploads with the same name cannot
/// replace each other, and sanitizing the name keeps a client from choosing a
/// path of their own.
pub(crate) fn build_object_key(prefix: Option<&str>, file_name: &str) -> String {
    let name = sanitize_file_name(file_name);
    let unique = Uuid::new_v4().simple().to_string();
    let object_name = format!("{unique}-{name}");

    let prefix = prefix
        .map(|prefix| {
            prefix
                .split('/')
                .map(str::trim)
                .filter(|segment| !segment.is_empty() && *segment != "." && *segment != "..")
                .collect::<Vec<_>>()
                .join("/")
        })
        .unwrap_or_default();

    if prefix.is_empty() {
        object_name
    } else {
        format!("{prefix}/{object_name}")
    }
}

/// Strips directory components and characters that are unsafe in an object key
/// or an upload filename.
///
/// This is deliberately conservative: only ASCII letters, digits, `.`, `-` and
/// `_` survive. Everything else becomes a hyphen so an extension remains
/// readable without allowing path traversal.
pub(crate) fn sanitize_file_name(file_name: &str) -> String {
    let base = file_name
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("file")
        .trim();

    let mut safe = String::with_capacity(base.len());
    let mut last_was_separator = false;

    for character in base.chars() {
        if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
            safe.push(character);
            last_was_separator = false;
        } else if !last_was_separator {
            safe.push('-');
            last_was_separator = true;
        }
    }

    let safe = safe
        .trim_matches(|character| matches!(character, '.' | '-'))
        .chars()
        .take(MAX_FILE_NAME_LEN)
        .collect::<String>();

    if safe.is_empty() {
        "file".to_string()
    } else {
        safe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_directory_components() {
        assert_eq!(sanitize_file_name("../../etc/passwd"), "passwd");
        assert_eq!(sanitize_file_name("C:\\tmp\\evil.png"), "evil.png");
    }

    #[test]
    fn replaces_unsafe_characters() {
        assert_eq!(sanitize_file_name("holiday photo (1).png"), "holiday-photo-1-.png");
        assert_eq!(sanitize_file_name("résumé.pdf"), "r-sum-.pdf");
    }

    #[test]
    fn rejects_dot_only_names() {
        assert_eq!(sanitize_file_name(".."), "file");
        assert_eq!(sanitize_file_name(".env"), "env");
        assert_eq!(sanitize_file_name("   "), "file");
    }

    #[test]
    fn caps_length() {
        let long = format!("{}.png", "a".repeat(400));
        assert_eq!(sanitize_file_name(&long).len(), MAX_FILE_NAME_LEN);
    }

    #[test]
    fn prefix_is_joined_and_traversal_segments_are_dropped() {
        let key = build_object_key(Some("/uploads/../students/"), "photo.png");

        assert!(key.starts_with("uploads/students/"));
        assert!(key.ends_with("-photo.png"));
        assert!(!key.contains(".."));
    }

    #[test]
    fn each_upload_gets_a_distinct_key() {
        let first = build_object_key(None, "photo.png");
        let second = build_object_key(None, "photo.png");

        assert_ne!(first, second);
        assert!(first.ends_with("-photo.png"));
    }
}
