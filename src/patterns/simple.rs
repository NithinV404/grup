use std::cmp::max;

/* Function to calculate the bad character heuristic by mapping the characters usize values
or ASCII equivalent with position of the character in the pattern*/

//Helper function to calculate the bad character heuristic
fn badcharheuristic(s: &str) -> Vec<usize> {
    let ascii_char = 256;
    let mut bad_char = vec![usize::MAX; ascii_char];
    for (i, c) in s.chars().enumerate() {
        bad_char[c as usize] = i;
    }
    bad_char
}

/* Uses Boyer-Moore algorithm take text and pattern as input*/
pub fn simple_pattern(text: &str, pattern: &str, case_sensitive: bool) -> Vec<(usize, usize)> {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let pattern_len = pattern_bytes.len();
    let text_len = text_bytes.len();
    let mut skip: usize = 0;
    let bad_char = badcharheuristic(pattern);

    let mut index = Vec::new();
    let mut matches = Vec::new();

    while skip <= text_len - pattern_len {
        let mut j = pattern_len as i32 - 1;

        // Modified character comparison based on case sensitivity
        while j >= 0 {
            let text_char = text_bytes[skip + j as usize];
            let pattern_char = pattern_bytes[j as usize];

            let match_chars = if case_sensitive {
                // Case-sensitive: direct byte comparison
                text_char == pattern_char
            } else {
                // Case-insensitive: compare lowercase versions
                text_char.to_ascii_lowercase() == pattern_char.to_ascii_lowercase()
            };

            if match_chars {
                j -= 1;
            } else {
                break;
            }
        }

        if j < 0 {
            index.push(skip);
            skip += if skip + pattern_len < text_len {
                let bad_char_value = bad_char[text_bytes[skip + pattern_len] as usize];
                if bad_char_value == usize::MAX {
                    pattern_len
                } else {
                    pattern_len - bad_char_value as usize
                }
            } else {
                1
            };
        } else {
            let text_char = text_bytes[skip + j as usize] as usize;
            let shift = max(1, j - bad_char[text_char] as i32);
            skip += shift as usize;
        }
    }
    for pos in index {
        matches.push((pos, pattern_len));
    }
    matches
}
