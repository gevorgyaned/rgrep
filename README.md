Wildcard Grep is a simplified version of the classic grep command-line tool, written in Rust. It allows you to search for a specified pattern in a file and displays the lines that contain the pattern.

pattern formats supported:
'*' - matches any character multiple or zero times
'?' - matches any single character
'#' - matches a digit
"[n|upcase or lowcase]" - matches upcase or lowcase letter n times
"[upcase or lowcase]" = matches upcase or lowcase letter single time
"'<'set of symbols'>'" - matches a set of symbols 

Usage: cargo run <PATTERN> "'<'FILENAME or DIRECTORY NAME>"...
 
