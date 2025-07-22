Lo_ as Lodash &emsp;
---

[<img alt="github" src="https://img.shields.io/badge/github-thedevsaddam/lo__-34495e?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/thedevsaddam/lo_)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-lo__-2471a3?style=for-the-badge&labelColor=#D1A980&logo=docs.rs" height="20">](https://docs.rs/lo_/latest/lo_/)
[<img alt="crates.io" src="https://img.shields.io/crates/v/lo_.svg?style=for-the-badge&color=f39c12&logo=rust" height="20">](https://crates.io/crates/lo_)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/thedevsaddam/lo_/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/thedevsaddam/lo_/actions?query=branch%3Amain)
<!-- [<img alt="license" src="https://img.shields.io/github/license/thedevsaddam/lo_.svg?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/thedevsaddam/lo_/blob/main/LICENSE.md) -->

A modern Rust utility library delivering modularity, performance & extras ported from JavaScript Lodash

## Installation

Add `lo_` to your `Cargo.toml`:

```toml
[dependencies]
lo_ = { version = "0.3.0", features = ["async_retry"] }
```

Or run the following command:

```sh
cargo add lo_ --features "async_retry"
```

## Usage Examples

### 🧵 String Manipulation Utilities

#### Basic Case Conversions
For quick use, you can import and call built-in helpers like camel_case, snake_case, etc., without needing the trait:
```rust
use lo_::camel_case;

let input = "Foo Bar";
let result = camel_case(input);
println!("{:?}", result); // "fooBar"
```

Use the Transform trait for chainable and expressive string utilities:

```rust
use lo_::CaseTransform;

let s = "HelloWorld";
println!("to_snake_case: {:?}", s.to_snake_case()); // "hello_world"
println!("to_camel_case: {:?}", s.to_camel_case()); // "helloWorld"
println!("to_title_case: {:?}", s.to_title_case()); // "Hello World"
println!("to_lower_first: {:?}", s.to_lower_first()); // "helloWorld"
println!("to_upper_first: {:?}", s.to_upper_first()); // "HelloWorld"

use lo_::WordTransform;

let input = "fred, barney, & pebbles";
println!("to_words: {:?}", input.to_words()); // ["fred", "barney", "pebbles"]
println!("to_slug: {:?}", "Rust is awesome 🚀".to_slug()); // "rust-is-awesome"

use lo_::UtilityTransform;

let num: Option<i32> = "123".to_safe_parse();
println!("to_safe_parse: {:?}", num); // Some(123)
```

#### Word Wrapping
```rust
use lo_::WordTransform;

let text = "Rust is blazing fast and memory-efficient.";
let wrapped = text.wordwrap(10, "\n", false);
println!("{}", wrapped);

/*
Rust is
blazing
fast and
memory-efficient.
*/
```


#### Padding
```rust
use lo_::{UtilityTransform, Alignment};

let s = "42";
assert_eq!(s.pad(5, "0", Alignment::Left), "00042");
assert_eq!(s.pad(5, " ", Alignment::Right), "42   ");
assert_eq!(s.pad(6, "-", Alignment::Center), "--42--");
```

#### Word Extraction
```rust
use lo_::WordTransform;

let s = "Hello world, this is Rust!";
let words = s.to_words();
assert_eq!(words, vec!["Hello", "world", "this", "is", "Rust"]);
```

#### String Templating
```rust
use std::collections::HashMap;
use lo_::UtilityTransform;

let mut data = HashMap::new();
data.insert("name", "Ragnar");
data.insert("lang", "Rust");

let template = "Hi {name}, welcome to {lang}!";
let rendered = template.to_template(&data);
assert_eq!(rendered, "Hi Ragnar, welcome to Rust!");
```

### 📦 Collection Utilities
```rust
use lo_::{chunk, find, uniq};

let array = vec![1, 2, 3, 4, 5, 6, 7];
let size = 3;
let chunks = chunk(array, size);
println!("array chunks: {:?}", chunks); // array chunks: [[1, 2, 3], [4, 5, 6], [7]]


let numbers = vec![1, 2, 3, 4, 5];
let is_even = |x: &i32| *x % 2 == 0;
println!("{:?}", find(&numbers, is_even)); // Some(2)


let input = vec![2, 1, 2];
let output = uniq(input);
println!("{:?}", output) // [2, 1]
```

### ✂️ General Utilities
```rust
use lo_::retry;
use std::time::Duration;

let mut count = 0;
let result = retry(4, Duration::from_millis(10), || {
    count += 1;
    if count < 3 {
        Err("fail")
    } else {
        Ok("success")
    }
});
println!("{:?} after {:?} retry", result, count); // Ok("success") after 3 retry
```
