[![Crates.io](https://img.shields.io/crates/v/moenster.svg)](https://crates.io/crates/moenster)
[![Workflow Status](https://github.com/badboy/moenster/workflows/CI/badge.svg)](https://github.com/badboy/moenster/actions?query=workflow%3A%22CI%22)

# moenster

## mønster (n) - pattern.

Simple glob-style pattern matching for strings.
Always matches the whole string from beginning to end.

| Wildcard | Description | Note |
| -------- | ----------- | ---- |
| *        | matches any number of any characters including none | |
| ?        | matches any single character | does not handle multi-byte UTF-8 codepoints |
| \[abc]   | matches one character given in the bracket | taken as byte values |
| \[a-z]   | matches one character from the range given in the bracket | range taken from their byte values |
| \[^abc]  | matches one character that is not given in the bracket | taken as byte values |
| \[^a-z]  | matches one character that is not from the range given in the bracket | range taken from their byte values |

_Note: An empty bracket can never match anything._

## Example

```rust
assert!(stringmatch("m*nster", "mønster"));
```

## License

The code is under a MIT license. See [LICENSE](LICENSE).
