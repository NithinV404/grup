use regex::RegexBuilder;

/// Matches a regex pattern in the input and returns a vector of tuples containing match positions and lengths
pub fn regex_match(input: &str, pattern: &str) -> Result<Vec<(usize, usize)>, regex::Error> {
    // Use RegexBuilder with dotall mode set to false (dot doesn't match newlines)
    let re = RegexBuilder::new(pattern)
        .dot_matches_new_line(false) // Changed to false so the dot doesn't match newlines
        .build()?;

    // Collect matches with their positions and lengths
    let matches: Vec<(usize, usize)> = re
        .find_iter(input)
        .map(|m| (m.start(), m.end() - m.start()))
        .collect();

    Ok(matches) // Return the matches vector wrapped in Ok
}
