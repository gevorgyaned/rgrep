pub mod tests;

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum WildcardTok {
    MultipleAny,
    SingleAny,
	Digit,
    Symbol(char),
    LowerSymbol(Option<usize>), 
    HigherSymbol(Option<usize>),
	SymbolSet(HashSet<char>),
}

const SPECIAL_SYMBOLS: [char; 9] = ['*', '?', '\\', '/', '[', ']', '#', '<', '>'];

const LOWER: &str = "lowcase";
const HIGHER: &str = "upcase";

fn match_wild(pattern: &str, num: usize) -> Option<WildcardTok> {
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
	if pattern.is_empty() || !pattern.starts_with('[') || !pattern.ends_with(']') {
		return None;
	}

    let pattern = &pattern[1..pattern.len() - 1];

    if pattern.contains('|') {
		let pattern: Vec<&str> = pattern.split('|').collect();

		if pattern.len() != 2 || pattern[0].chars().any(|c| !c.is_ascii_digit()) {
			return None;
		}

		let number = pattern[0].parse::<usize>().unwrap_or(0);
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

pub fn is_match(s: &str, pattern: &str) -> bool {
    let tokens = match compile_wildcard(pattern) {
        Ok(tokens) => tokens,
        Err(_) => return false,
    };

    let s_chars: Vec<char> = s.chars().collect();
    let n = s_chars.len();
    let m = tokens.len();

    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[0][0] = true;

    for j in 1..=m {
        if let WildcardTok::MultipleAny = tokens[j - 1] {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=n {
        for j in 1..=m {
            match &tokens[j - 1] {
                WildcardTok::SingleAny => {
                    dp[i][j] = dp[i - 1][j - 1];
                }
                WildcardTok::MultipleAny => {
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                }
                WildcardTok::Symbol(c) => {
                    dp[i][j] = dp[i - 1][j - 1] && s_chars[i - 1] == *c;
                }
                WildcardTok::Digit => {
                    dp[i][j] = dp[i - 1][j - 1] && s_chars[i - 1].is_digit(10);
                }
                WildcardTok::SymbolSet(set) => {
                    dp[i][j] = dp[i - 1][j - 1] && set.contains(&s_chars[i - 1]);
                }
                WildcardTok::LowerSymbol(None) => {
                    dp[i][j] = dp[i - 1][j - 1] && s_chars[i - 1].is_lowercase();
                }
                WildcardTok::HigherSymbol(None) => {
                    dp[i][j] = dp[i - 1][j - 1] && s_chars[i - 1].is_uppercase();
                }
                WildcardTok::LowerSymbol(Some(n)) => {
                    dp[i][j] = false;
                    if s_chars[i - 1].is_lowercase() {
                        let mut count: usize = 0;
                        while count < *n && i > count && s_chars[i - 1 - count].is_lowercase() {
                            count += 1;
                        }
                        if count == *n {
                            dp[i][j] = dp[i - count][j - 1];
                        }
                    }
                }
                WildcardTok::HigherSymbol(Some(n)) => {
                    dp[i][j] = false;
                    if s_chars[i - 1].is_uppercase() {
                        let mut count: usize = 0;
                        while count < *n && i > count && s_chars[i - 1 - count].is_uppercase() {
                            count += 1;
                        }
                        if count == *n {
                            dp[i][j] = dp[i - count][j - 1];
                        }
                    }
                }
            }
        }
    }

    dp[n][m]
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
