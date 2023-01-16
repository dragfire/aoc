#![allow(unused)]

use aoc::prelude::*;

fn main() {
    println!("Rust-y went Reactive!!");

    let iter_observable = observable::from_iter(vec![0, 1, 2, 3])
        .map(|item| item * 2)
        .map(|item| item + 1_000);

    iter_observable.clone().subscribe(|item| {
        println!("Observer1 : {}", item);
    });
    iter_observable.clone().subscribe(|item| {
        println!("Observer2: {}", item);
    });
}