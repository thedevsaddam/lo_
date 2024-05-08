Lo_ as Lodash
---
A modern Rust utility library delivering modularity, performance & extras ported from JavaScript Lodash

## Usage

Depend on lorust in Cargo.toml:

```toml
[dependencies]
lo_ = "0.1.10"
```

#### Example

##### Array
```rust
use lo_::chunk;

fn main() {
    let array = vec![1, 2, 3, 4, 5, 6, 7];
    let size = 3;
    let chunks = chunk(array, size);
    println!("array chunks: {:?}", chunks); // array chunks: [[1, 2, 3], [4, 5, 6], [7]]
}
```

##### String
```rust
use lo_::words;

fn main() {
    let input = "fred, barney, & pebbles";
    let result = words(input);
    println!("{:?}", result); // ["fred", "barney", "pebbles"]
}

```

##### Condition
```rust
use lo_::ternary;

let iam_tom_or_zerry = |args: i32| -> String {
    if args == 1 {
        String::from("Tom")
    } else {
        String::from("Zerry")
    }
};

fn main() {
    println!("{:?}", ternary("Tom" == iam_tom_or_zerry(1), "Yes", "No")); // Yes
    println!("{:?}", ternary("Tom" == iam_tom_or_zerry(2), "Yes", "No")); // No
}

```