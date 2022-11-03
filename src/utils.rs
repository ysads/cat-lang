pub(crate) fn extract_digits (s: &str) -> (&str, &str) {
    let last_index = s
    .char_indices()
    .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
    .unwrap_or_else(|| s.len());

    let digits = &s[..last_index];
    let rem = &s[last_index..];

    (rem, digits)
}

pub(crate) fn extract_op (s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {},
        _ => panic!("bad operator")
    }

    (&s[1..], &s[..1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"));
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_operations() {
        assert_eq!(extract_op("+12"), ("12", "+"));
        assert_eq!(extract_op("-45"), ("45", "-"));
        assert_eq!(extract_op("*23"), ("23", "*"));
        assert_eq!(extract_op("/10"), ("10", "/"));
    }
}