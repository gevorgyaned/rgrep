Wildcard Grep is a Rust-based tool that allows you to search for specified patterns in files or directories, displaying the lines that contain the pattern. It supports various wildcard characters.

Supported Pattern Formats:
* - Matches any character zero or more times.
? - Matches any single character.
# - Matches any single digit.
[n|upcase or lowcase] - Matches an uppercase or lowercase letter exactly n times.
[upcase or lowcase] - Matches an uppercase or lowercase letter exactly once.
<'set of symbols'> - Matches any character within the set of symbols.

cargo run <PATTERN> <'FILENAME or DIRECTORY NAME'> [OPTIONS]
