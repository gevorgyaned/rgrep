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

pub fn kmp(text: &str, pattern: &str) -> Option<Vec<usize>> {
    // preprocessing
    let pre: Vec<i32> = Vec::with_capacity(pattern.len());



    Some(vec![])   
}
