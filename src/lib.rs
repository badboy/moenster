//! # mønster
//!
//! Simple glob-style pattern matching for strings.
//! Always matches the whole string from beginning to end.
//!
//! | Wildcard | Description |
//! | -------- | ----------- |
//! | *        | matches any number of any characters including none |
//! | ?        | matches any single character  |
//!
//! mønster (n) - pattern.
//!
//! Example
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

        pattern = &pattern[1..];
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
}
