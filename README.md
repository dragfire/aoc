# Rust-y went reactive!!
Implementing a very bare-bone reactivex for learning purposes by following https://github.com/rxRust/rxRust  
http://reactivex.io

### Example:
```rust
fn main() {
    println!("Rust-y went Reactive!!");

    observable::from_iter(vec![0, 1, 2, 3])
        .map(|item| item * 2)
        .map(|item| item + 1_000)
        .subscribe(|item| {
            println!("item: {}", item);
        });
}
```
