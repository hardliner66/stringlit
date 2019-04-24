# stringlit

A macro to convert from str to String.

```rust
// import the macros
//
use stringlit::{s, string};

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    // now you can use
    print_string(s!("test"));
    // or
    print_string(string!("test"));
    // instead of
    print_string("test".to_owned());
}
```
