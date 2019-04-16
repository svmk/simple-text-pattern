[![Build Status](https://travis-ci.com/svmk/simple-text-pattern.svg?branch=master)](https://travis-ci.com/svmk/simple-text-pattern)
[![](https://meritbadge.herokuapp.com/regex)](https://crates.io/crates/simple-text-pattern)

# Rust simple text pattern library

This crate provides a library for compiling and matching simple text patterns.

# Example
Passed pattern `some*text` will be compiled into equivalent regexp `^some.*text$`.

# Syntax
* `*` - one or more any symbol.
* `any other text` - interpreted as simple text.

# Usage
```rust
use simple_text_pattern::Pattern;
let pattern = Pattern::new("some*text").expect("Unable to compile pattern");
assert_eq!(true, pattern.is_match("sometext"));
assert_eq!(true, pattern.is_match("some text"));
assert_eq!(false, pattern.is_match("not some text"));
```
