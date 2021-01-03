//! # mønster
//!
//! Simple glob-style pattern matching for strings.
//! Always matches the whole string from beginning to end.
//!
//! | Wildcard | Description | Note |
//! | -------- | ----------- | ---- |
//! | *        | matches any number of any characters including none | |
//! | ?        | matches any single character | does not handle multi-byte UTF-8 codepoints |
//! | \[abc]   | matches one character given in the bracket | taken as byte values |
//! | \[a-z]   | matches one character from the range given in the bracket | range taken from their byte values |
//! | \[^abc]  | matches one character that is not given in the bracket | taken as byte values |
//! | \[^a-z]  | matches one character that is not from the range given in the bracket | range taken from their byte values |
//!
//! _Note: An empty bracket can never match anything._
//!
//! ## Example
//! ```
//! # use moenster::stringmatch;
//! assert!(stringmatch("m*nster", "mønster"));
//! ```

pub fn stringmatch(pattern: &str, string: &str) -> bool {
    stringmatch_bytes(pattern.as_bytes(), string.as_bytes(), Case::Sensitive)
}

// FIXME: Remove dead_code allowance.
#[allow(dead_code)]
#[derive(Copy, Clone)]
enum Case {
    Sensitive,
    Insensitive,
}

fn stringmatch_bytes(mut pattern: &[u8], mut string: &[u8], case: Case) -> bool {
    while !pattern.is_empty() && !string.is_empty() {
        match pattern[0] {
            // any number of any characters
            b'*' => {
                while pattern.len() > 2 && pattern[1] == b'*' {
                    pattern = &pattern[1..];
                }
                if pattern.len() == 1 {
                    return true;
                }

                while !string.is_empty() {
                    if stringmatch_bytes(&pattern[1..], string, case) {
                        return true;
                    }
                    string = &string[1..];
                }

                return false;
            }
            // any single character
            b'?' => {
                string = &string[1..];
            }
            // bracketed patterns such as `[abc]` or `[a-z]`
            b'[' => {
                pattern = &pattern[1..];
                let not = pattern[0] == b'^';
                if not {
                    pattern = &pattern[1..];
                }
                let mut matched = false;
                loop {
                    if pattern.len() == 0 {
                        break;
                    } else if pattern[0] == b'\\' && pattern.len() >= 2 {
                        pattern = &pattern[1..];

                        if pattern[0] == string[0] {
                            matched = true;
                        }
                    } else if pattern[0] == b']' {
                        break;
                    } else if pattern.len() >= 3 && pattern[1] == b'-' {
                        let mut start = pattern[0];
                        let mut end = pattern[2];
                        let mut c = string[0];
                        if start > end {
                            let tmp = start;
                            start = end;
                            end = tmp;
                        }

                        if matches!(case, Case::Insensitive) {
                            start = start.to_ascii_lowercase();
                            end = end.to_ascii_lowercase();
                            c = c.to_ascii_lowercase();
                        }

                        pattern = &pattern[2..];
                        if c >= start && c <= end {
                            matched = true;
                        }
                    } else {
                        if matches!(case, Case::Sensitive) {
                            if pattern[0] == string[0] {
                                matched = true;
                            }
                        } else {
                            if pattern[0].to_ascii_lowercase() != string[0].to_ascii_lowercase() {
                                matched = true;
                            }
                        }
                    }
                    pattern = &pattern[1..];
                }

                if not {
                    matched = !matched;
                }

                if !matched {
                    return false;
                }

                string = &string[1..];
            }
            // everything else
            _ => {
                // Ignore escaped characters
                if pattern[0] == b'\\' && pattern.len() >= 2 {
                    pattern = &pattern[1..];
                }

                let p = pattern[0];
                if matches!(case, Case::Sensitive) {
                    if p != string[0] {
                        return false;
                    }
                    string = &string[1..];
                } else {
                    if p.to_ascii_lowercase() != string[0].to_ascii_lowercase() {
                        return false;
                    }
                    string = &string[1..];
                }
            }
        }

        // Need to handle the case that a bracketed pattern wasn't properly closed and we ran out
        // of patterns to match.
        if !pattern.is_empty() {
            pattern = &pattern[1..];
        }
        if string.is_empty() {
            while !pattern.is_empty() && pattern[0] == b'*' {
                pattern = &pattern[1..];
            }
            break;
        }
    }

    pattern.is_empty() && string.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_string() {
        assert!(stringmatch("moenster", "moenster"));
    }

    #[test]
    fn escaped() {
        assert!(stringmatch("moenste\\r", "moenster"));
    }

    #[test]
    fn questionmark() {
        assert!(stringmatch("mo?nster", "moenster"));
        assert!(stringmatch("m??nster", "moenster"));
        assert!(stringmatch("mo?nst?r", "moenster"));
        assert!(!stringmatch("moenster?", "moenster"));
    }

    #[test]
    fn wildcard() {
        assert!(stringmatch("*", "moenster"));
        assert!(stringmatch("*****", "moenster"));
    }

    #[test]
    fn wildcard_and_more() {
        assert!(stringmatch("m*oenster", "moenster"));
        assert!(stringmatch("m*", "moenster"));
        assert!(stringmatch("*r", "moenster"));
    }

    #[test]
    fn bracketed_chars() {
        assert!(stringmatch("m[oei]enster", "moenster"));
        assert!(!stringmatch("m[bcd]enster", "moenster"));
    }

    #[test]
    fn not_bracketed_chars() {
        assert!(stringmatch("m[^bcd]enster", "moenster"));
        assert!(!stringmatch("m[^oei]enster", "moenster"));
    }

    #[test]
    fn bracketed_range() {
        assert!(stringmatch("m[n-p]enster", "moenster"));
        assert!(!stringmatch("m[a-c]enster", "moenster"));
    }

    #[test]
    fn not_bracketed_range() {
        assert!(stringmatch("m[^a-c]enster", "moenster"));
        assert!(!stringmatch("m[^n-p]enster", "moenster"));
    }

    #[test]
    fn wrong_bracket() {
        assert!(stringmatch("m[n-p", "mo"));
        assert!(!stringmatch("m[n-pt", "mot"));
    }

    #[test]
    fn escaped_in_bracket() {
        assert!(stringmatch("m[\\].;]o", "m]o"));
        assert!(stringmatch("m[\\].;]o", "m;o"));
        assert!(stringmatch("m[\\].;]o", "m.o"));
    }

    #[test]
    fn empty_bracket() {
        assert!(!stringmatch("m[]", "m"));
    }
}
