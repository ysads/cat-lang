fn take_while(pred: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let last_index = s
        .char_indices()
        .find_map(|(idx, c)| if pred(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let digits = &s[..last_index];
    let rem = &s[last_index..];

    (rem, digits)
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_whitespaces(s: &str) -> (&str, &str) {
    take_while(|c| c.is_whitespace(), s)
}

pub(crate) fn extract_id(s: &str) -> (&str, &str) {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        take_while(|c| c.is_alphanumeric() || c == '_', s)
    } else {
        (s, "")
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(starting_text) {
        &s[starting_text.len()..]
    } else {
        panic!("expected `{}`", starting_text)
    }
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
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

    #[test]
    fn extract_alphanumeric_unicode_identifiers_but_not_emojis() {
        assert_eq!(extract_id("id   stop"), ("   stop", "id"));
        assert_eq!(extract_id("id123()"), ("()", "id123"));
        assert_eq!(extract_id("id_one(50)"), ("(50)", "id_one"));
        assert_eq!(extract_id("idOne = 23"), (" = 23", "idOne"));
        assert_eq!(extract_id("id_愛; 1+1"), ("; 1+1", "id_愛"));
        assert_eq!(extract_id("só💣   stop"), ("💣   stop", "só"));
    }

    #[test]
    fn do_not_extract_ids_starting_with_numbers() {
        assert_eq!(extract_id("1abc = 25"), ("1abc = 25", ""));
        assert_eq!(extract_id("123abc = 10"), ("123abc = 10", ""));
        assert_eq!(extract_id("123abc(var)"), ("123abc(var)", ""));
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), " a");
        assert_eq!(tag("=", "= 10"), " 10");
    }
}
