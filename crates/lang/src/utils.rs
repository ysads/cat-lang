fn take_while(pred: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let last_index = s
        .char_indices()
        .find_map(|(idx, c)| if pred(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let digits = &s[..last_index];
    let rem = &s[last_index..];

    (rem, digits)
}

fn take_while_1(
    pred: impl Fn(char) -> bool,
    s: &str,
    err_msg: String,
) -> Result<(&str, &str), String> {
    let (rem, extracted) = take_while(pred, s);

    if extracted.is_empty() {
        Err(err_msg)
    } else {
        Ok((rem, extracted))
    }
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while_1(
        |c| c.is_ascii_digit(),
        s,
        format!("Expected digits: `{}`", s),
    )
}

pub(crate) fn extract_whitespaces(s: &str) -> (&str, &str) {
    take_while(|c| c.is_whitespace(), s)
}

pub(crate) fn extract_whitespaces_1(s: &str) -> Result<(&str, &str), String> {
    take_while_1(|c| c.is_whitespace(), s, "Expected space".to_string())
}

pub(crate) fn extract_id(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        Ok(take_while(|c| c.is_alphanumeric() || c == '_', s))
    } else {
        Err(format!("Identifier expected: `{}`", s))
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("Expected `{}` not found in `{}`", starting_text, s))
    }
}

fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[..1])
}

pub(crate) fn sequence<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    mut s: &str,
) -> Result<(&str, Vec<T>), String> {
    let mut items = Vec::new();

    while let Ok((new_s, item)) = parser(s) {
        s = new_s;
        items.push(item);

        let (new_s, _) = extract_whitespaces(s);
        s = new_s;
    }

    Ok((s, items))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }

    #[test]
    fn do_not_extract_anything_when_input_is_invalid() {
        assert_eq!(extract_digits(""), Err(String::from("Expected digits: ``")));
        assert_eq!(
            extract_digits("abc"),
            Err(String::from("Expected digits: `abc`"))
        );
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn extract_spaces_at_start_of_string() {
        assert_eq!(extract_whitespaces_1("   234a"), Ok(("234a", "   ")));
        assert_eq!(
            extract_whitespaces_1("   \nabc123"),
            Ok(("abc123", "   \n"))
        );
    }

    #[test]
    fn fails_to_extract_spaces_when_they_are_not_required() {
        assert_eq!(
            extract_whitespaces_1("blah"),
            Err("Expected space".to_string())
        )
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
        assert_eq!(extract_id("id   stop"), Ok(("   stop", "id")));
        assert_eq!(extract_id("id123()"), Ok(("()", "id123")));
        assert_eq!(extract_id("id_one(50)"), Ok(("(50)", "id_one")));
        assert_eq!(extract_id("idOne = 23"), Ok((" = 23", "idOne")));
        assert_eq!(extract_id("id_愛; 1+1"), Ok(("; 1+1", "id_愛")));
        assert_eq!(extract_id("só💣   stop"), Ok(("💣   stop", "só")));
    }

    #[test]
    fn fail_to_extract_ids_starting_with_numbers() {
        assert_eq!(
            extract_id("1abc = 25"),
            Err("Identifier expected: `1abc = 25`".to_string())
        );
        assert_eq!(
            extract_id("123abc = 10"),
            Err("Identifier expected: `123abc = 10`".to_string())
        );
        assert_eq!(
            extract_id("123abc(var)"),
            Err("Identifier expected: `123abc(var)`".to_string())
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
        assert_eq!(tag("=", "= 10"), Ok(" 10"));
    }

    #[test]
    fn fails_to_tag_word_if_string_does_not_start_with_given_substr() {
        assert_eq!(
            tag("let", "cat a"),
            Err("Expected `let` not found in `cat a`".to_string())
        )
    }
}
