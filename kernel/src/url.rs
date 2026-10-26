pub fn encode_path_segment(value: &str) -> String {
    let mut out = String::with_capacity(value.len());

    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::encode_path_segment;

    #[test]
    fn leaves_unreserved_characters_untouched() {
        assert_eq!(encode_path_segment("01HQZX-abc_123.~"), "01HQZX-abc_123.~");
    }

    #[test]
    fn escapes_path_traversal() {
        assert_eq!(
            encode_path_segment("../../../etc/passwd"),
            "..%2F..%2F..%2Fetc%2Fpasswd"
        );
    }

    #[test]
    fn escapes_a_space_as_percent_twenty_not_plus() {
        assert_eq!(encode_path_segment("ref 1"), "ref%201");
    }

    #[test]
    fn escapes_a_query_separator() {
        assert_eq!(encode_path_segment("a&b=c"), "a%26b%3Dc");
    }
}
