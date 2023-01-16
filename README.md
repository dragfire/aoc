# Rust-y went reactive!!

Implementing a very bare-bone reactivex for learning purposes by following https://github.com/rxRust/rxRust  
http://reactivex.io

### Example:

Only Map and Filter operators are supported right now. :D

```rust
fn main() {
    println!("Rust-y went Reactive!!");

    let mut total = 0;

    observable::from_iter(vec![1, 2, 3, 4, 5, 6])
        .map(|v| v + 100)
        .filter(|v| v % 2 == 0)
        .map(|v| v - 100)
        .map(|v| v * 100)
        .subscribe(|v| {
            total += v;
        });

    assert_eq!(1200, total);
}
```
