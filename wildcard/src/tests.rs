

#[cfg(test)]
mod additional_tests {
    use crate::*;

    use super::*;

    #[test]
    fn test_valid_set_patterns() {
        let mut hash_set1 = HashSet::new();
        hash_set1.insert('x');
        hash_set1.insert('y');
        hash_set1.insert('z');
        assert_eq!(handle_set("<xyz>"), Some(WildcardTok::SymbolSet(hash_set1)));

        let mut hash_set2 = HashSet::new();
        hash_set2.insert('1');
        hash_set2.insert('2');
        hash_set2.insert('3');
        assert_eq!(handle_set("<123>"), Some(WildcardTok::SymbolSet(hash_set2)));
    }

    #[test]
    fn test_invalid_set_patterns() {
        assert_eq!(handle_set(""), None);
        assert_eq!(handle_set("<>"), None);
        assert_eq!(handle_set("<abc"), None);
        assert_eq!(handle_set("abc>"), None);
    }

    #[test]
    fn test_is_match_simple() {
        assert!(is_match("abc", "abc"));
        assert!(is_match("abcdef", "abc*"));
        assert!(is_match("abcdef", "a*d*f"));
        assert!(is_match("abcdef", "*def"));
        assert!(!is_match("abcdef", "a?d*"));
    }

    #[test]
    fn test_is_match_complex() {
        assert!(is_match("abc123", "abc[lowcase]123"));
        assert!(is_match("ABC123", "ABC[upcase]123"));
        assert!(!is_match("abc123", "ABC[upcase]123"));
        assert!(is_match("abcXYZ", "abc<XYZ>"));
        assert!(!is_match("abcXYZ", "abc<xyz>"));
        assert!(is_match("abcxyz", "abc<xyz>"));
    }

    #[test]
    fn test_is_match_with_digits() {
        assert!(is_match("a1b2c3", "a#b#c#"));
        assert!(is_match("123456", "######"));
        assert!(!is_match("12345", "######"));
    }

    #[test]
    fn test_wildcard_compile_invalid() {
        assert_eq!(compile_wildcard("a[12|unknown]"), Err(String::from("extract_from_brackets")));
        assert_eq!(compile_wildcard("a[lowcase"), Err(String::from("missing closing bracket")));
        assert_eq!(compile_wildcard("a[lowcase123]"), Err(String::from("extract_from_brackets")));
        assert_eq!(compile_wildcard("<abc"), Err(String::from("missing closing bracket")));
    }

    #[test]
    fn test_parse_function() {
        assert_eq!(parse("a b c"), Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        assert_eq!(parse(""), None);
        assert_eq!(parse("   "), None);
    }

    #[test]
    fn test_special_symbols() {
        assert_eq!(compile_wildcard(r"a\*b\?c\/d"), Ok(vec![
            WildcardTok::Symbol('a'),
            WildcardTok::Symbol('*'),
            WildcardTok::Symbol('b'),
            WildcardTok::Symbol('?'),
            WildcardTok::Symbol('c'),
            WildcardTok::Symbol('/'),
            WildcardTok::Symbol('d')
        ]));

        assert_eq!(compile_wildcard(r"a\*b\?c\d"), Err(String::from("invlaid symbol found at 7")));
        assert_eq!(compile_wildcard(r"a\*b\?c"), Ok(vec![
            WildcardTok::Symbol('a'),
            WildcardTok::Symbol('*'),
            WildcardTok::Symbol('b'),
            WildcardTok::Symbol('?'),
            WildcardTok::Symbol('c')
        ]));
    }

    #[test]
    fn test_escape_sequences() {
        assert_eq!(compile_wildcard(r"a\\b"), Ok(vec![
            WildcardTok::Symbol('a'),
            WildcardTok::Symbol('\\'),
            WildcardTok::Symbol('b')
        ]));

        assert_eq!(compile_wildcard(r"a\\*"), Ok(vec![
            WildcardTok::Symbol('a'),
            WildcardTok::Symbol('\\'),
            WildcardTok::MultipleAny
        ]));
    }
}

