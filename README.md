Lo_ as Lodash
---
A modern Rust utility library delivering modularity, performance & extras ported from JavaScript Lodash

## Usage

Depend on lorust in Cargo.toml:

```toml
[dependencies]
lo_ = { version = "0.2.0", features = ["transform", "async_retry"] }
```

### Example

#### Collection 
```rust
use lo_::chunk;

fn main() {
    let array = vec![1, 2, 3, 4, 5, 6, 7];
    let size = 3;
    let chunks = chunk(array, size);
    println!("array chunks: {:?}", chunks); // array chunks: [[1, 2, 3], [4, 5, 6], [7]]
}
```

#### String
```rust
use lo_::camel_case;

fn main() {
    let input = "Foo Bar";
    let result = camel_case(input);
    println!("{:?}", result); // "fooBar"
}

```

###### Implement the `Transform` trait to centralize string utilities, reduce boilerplate, and improve code clarity.

```rust
use lo_::Transform;

fn main() {
    let input = "fred, barney, & pebbles";
    println!("{:?}", input.to_words()); // ["fred", "barney", "pebbles"]

    println!("{:?}", "Rust is awesome 🚀".to_slug()); // "rust-is-awesome"

    let num: Option<i32> = "123".to_safe_parse();
    println!("{:?}", num); // Some(123)
}

```

#### General
```rust
use lo_::retry;
use std::time::Duration;

fn main() {
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
}

```

---
For more details and API documentation, please visit:
https://docs.rs/lo_/latest/lo_/

