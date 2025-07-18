Lo_ as Lodash
---
A modern Rust utility library delivering modularity, performance & extras ported from JavaScript Lodash

## Usage

Depend on lorust in Cargo.toml:

```toml
[dependencies]
lo_ = "0.2.0"
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
use lo_::words;

fn main() {
    let input = "fred, barney, & pebbles";
    let result = words(input);
    println!("{:?}", result); // ["fred", "barney", "pebbles"]
}

```

###### Implement the `Transform` trait to centralize string utilities, reduce boilerplate, and improve code clarity.

```rust
use lo_::Transform;

fn main() {
    let input = "fred, barney, & pebbles";
    println!("{:?}", input.words()); // ["fred", "barney", "pebbles"]
    
    println!("{:?}", "Rust is awesome 🚀".to_slug()); // "rust-is-awesome"

    let num: Option<i32> = "123".to_safe_parse();
    println!("{:?}", num);  // Output: Some(123)
}

```

#### General
```rust
use std::time::Duration;
use lo_::Retry;


fn might_fail(attempt: &mut i32) -> Result<&'static str, &'static str> {
    *attempt += 1;
    if *attempt < 3 {
        Err("fail")
    } else {
        Ok("success")
    }
}

fn main() {
    let mut count = 0;

    let result = Err::<&str, &str>("initial").retry(5, Duration::from_millis(50), || {
        might_fail(&mut count)
    });

    println!("{:?}", result); // prints Ok("success") after retrying
}

```

---
For more details and API documentation, please visit:
https://docs.rs/lo_/latest/lo_/

