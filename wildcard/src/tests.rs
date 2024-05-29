#[cfg(test)]
mod additional_tests {
    use crate::*;

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
        assert_eq!(is_match("abcdef", "a?d*"), false);
    }

    #[test]
    fn test_is_match_complex() {
        assert!(is_match("abchhhu", "abc[4|lowcase]"));
        assert!(is_match("ABC", "[3|upcase]"));
        assert!(is_match("ab78", "ab*##*"));
        assert!(!is_match("abchhhu", "abc[6|lowcase]"));
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

    #[test]
    fn test_valid_patterns() {
        assert_eq!(extract_from_brackets("[123|upcase]"), Some(WildcardTok::HigherSymbol(Some(123))));
        assert_eq!(extract_from_brackets("[456|lowcase]"), Some(WildcardTok::LowerSymbol(Some(456))));
        assert_eq!(extract_from_brackets("[upcase]"), Some(WildcardTok::HigherSymbol(None)));
        assert_eq!(extract_from_brackets("[lowcase]"), Some(WildcardTok::LowerSymbol(None)));
    }

    #[test]
    fn test_invalid_patterns() {
        assert_eq!(extract_from_brackets("[]"), None);
        assert_eq!(extract_from_brackets("[123|UNKNOWN]"), None);
        assert_eq!(extract_from_brackets("123|lowcase]"), None);
        assert_eq!(extract_from_brackets("[lowcase"), None);
        assert_eq!(extract_from_brackets("[lowcase123]"), None);
        assert_eq!(extract_from_brackets("[123hhjke]"), None);
    }

    #[test]
    fn test_set_pattern() {
		let mut hash_set = HashSet::new();
		hash_set.insert('a');
		hash_set.insert('b');
		hash_set.insert('c');
		hash_set.insert('d');
		hash_set.insert('e');
        assert_eq!(handle_set("<abcde>"), Some(WildcardTok::SymbolSet(hash_set)));
    }

	#[test]
	fn test_wildcard_compile() {
		assert_eq!(compile_wildcard("a[12|upcase]").unwrap(), vec![WildcardTok::Symbol('a'), WildcardTok::HigherSymbol(Some(12))]);
		assert_eq!(compile_wildcard("a[upcase]").unwrap(), vec![WildcardTok::Symbol('a'), WildcardTok::HigherSymbol(None)]);
		assert_eq!(compile_wildcard("a[upcase]#").unwrap(), vec![WildcardTok::Symbol('a'), WildcardTok::HigherSymbol(None), WildcardTok::Digit]);
		assert_eq!(compile_wildcard("*??a[lowcase]#").unwrap(), vec![WildcardTok::MultipleAny, WildcardTok::SingleAny,WildcardTok::SingleAny, WildcardTok::Symbol('a'), WildcardTok::LowerSymbol(None), WildcardTok::Digit]);
	}
}
