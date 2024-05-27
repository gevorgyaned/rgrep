pub mod tests;

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum WildcardTok {
    MultipleAny,
    SingleAny,
	Digit,
    Symbol(char),
    LowerSymbol(Option<u32>), 
    HigherSymbol(Option<u32>),
	SymbolSet(HashSet<char>),
}

const SPECIAL_SYMBOLS: [char; 8] = ['*', '?', '\\', '[', ']', '#', '<', '>'];

const LOWER: &str = "lowcase";
const HIGHER: &str = "upcase";

fn match_wild(pattern: &str, num: u32) -> Option<WildcardTok> {
    match pattern.trim() {
        HIGHER => Some(WildcardTok::HigherSymbol(
            if num == 0 { None }
            else { Some(num) }
        )),
        LOWER => Some(WildcardTok::LowerSymbol(
            if num == 0 { None }
            else { Some(num) }
        )),
        _ => None, 
    }
}
 
fn handle_set(set: &str) -> Option<WildcardTok> {
	if set.is_empty() || !set.starts_with('<') || !set.ends_with('>') {
		return None;
	} 

	let symbols = &set[1..set.len() - 1];

	if symbols.is_empty() {
		return None;
	}

	let symbol_set: HashSet<char> = symbols.chars().collect();

	Some(WildcardTok::SymbolSet(symbol_set))
}

pub fn extract_from_brackets(pattern: &str) -> Option<WildcardTok> {
	println!("{}", pattern);
	if pattern.is_empty() || !pattern.starts_with('[') || !pattern.ends_with(']') {
		println!("dbg");
		return None;
	}

	println!("{}", pattern);

    let pattern = &pattern[1..pattern.len() - 1];

    if pattern.contains('|') {
		let pattern: Vec<&str> = pattern.split('|').collect();

		if pattern.len() != 2 || pattern[0].chars().any(|c| !c.is_ascii_digit()) {
			return None;
		}

		let number = pattern[0].parse::<u32>().unwrap_or(0);
		let keyword = pattern[1];

        match_wild(keyword, number)
    } else {
        match_wild(pattern, 0)
    }
}

pub fn compile_wildcard(pattern: &str) -> Result<Vec<WildcardTok>, String> {
    let mut res = Vec::new();
    let pat_len = pattern.len();

    let mut idx = 0;

    while idx < pat_len {
		let c = pattern.chars().nth(idx).unwrap();

        match c {
            '\\' => {
                if idx == pat_len - 1 {
                    return Err(format!("invlaid symbol found at {}", idx));
                } else if SPECIAL_SYMBOLS.iter().any(|c| pattern.chars().nth(idx + 1) == Some(*c)) {
                    res.push(WildcardTok::Symbol(pattern.chars().nth(idx + 1).unwrap()));
                    idx += 1;
                } else {
					return Err(String::from("backslash incorrect use"));
				}
            }
            '*' => res.push(WildcardTok::MultipleAny),
			'?' => res.push(WildcardTok::SingleAny),
			'[' => {
                let pass_val = match pattern[idx..].find(']') {
                    Some(i) => i,
                    None => return Err(String::from("missing closing bracket")),
                };

				println!("{} {}", pass_val, idx);
				
				res.push(match extract_from_brackets(&pattern[idx..idx + pass_val + 1]) {
                    Some(w) => w,
                    None => return Err(String::from("extract_from_brackets")),
                });

                idx += pass_val;
			}
			'<' =>  {
                let closing_sym = match pattern[idx..].find('>') {
                    Some(i) => i,
                    None => return Err(String::from("missing closing bracket")),
                };

				res.push(match handle_set(&pattern[idx..]) {
					Some(w) => w,
					None => return Err(String::from("unknown error")),
				});

                idx = closing_sym;
			}
			'#' => res.push(WildcardTok::Digit),
			_ => res.push(WildcardTok::Symbol(c)),
        }

        idx += 1;
    }

    Ok(res)
}

pub fn is_match(str: &str, pattern: &str) -> bool {
    let (str_len, pat_len) = (str.len(), pattern.len());

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut m: usize = 0;
    let mut start_idx: i32 = -1;

    while i < str_len {
        if j < pat_len
            && (pattern.chars().nth(j) == Some('?') || pattern.chars().nth(j) == str.chars().nth(i))
        {
            j += 1;
            i += 1;
        } else if j < pat_len && pattern.chars().nth(j) == Some('*') {
            start_idx = j as i32;
            m = i;
            j += 1;
        } else if start_idx != -1 {
            j = start_idx as usize + 1;
            m += 1;
            i = m;
        } else {
            return false;
        }
    }

    while j < pat_len && pattern.chars().nth(j) == Some('*') {
        j += 1;
    }

    j == pat_len
}

pub fn parse(input: &str) -> Option<Vec<String>> {
    if input.is_empty() {
        None
    } else {
        Some(
            input
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

