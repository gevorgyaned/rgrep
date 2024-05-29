Wildcard Grep is a Rust-based tool that allows you to search for specified patterns in files or directories, displaying the lines that contain the pattern. It supports various wildcard characters.

Supported Pattern Formats: <br>
`* - Matches any character zero or more times.` <br>
`? - Matches any single character.` <br>
`# - Matches any single digit.` <br>
`[n|upcase or lowcase] - Matches an uppercase or lowercase letter exactly n times.` <br>
`[upcase or lowcase] - Matches an uppercase or lowercase letter exactly once.` <br>
`<set of symbols> - Matches any character within the set of symbols.` <br>

`cargo run <PATTERN> <FILENAME or DIRECTORY NAME> ...`
