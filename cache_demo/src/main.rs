use cacheForge::{cachable, lru_cache, expire_cache};
use std::time::{Duration as StdDuration};
use std::thread::sleep;

#[expire_cache(size = 2)]
fn expensive_computation1(x: i32, y: i32) -> i32 {
    println!("Computing {} + {}", x, y); // This will print only when the function is not cached
    x + y
}

#[lru_cache(size = 2)]
fn expensive_computation(x: i32, y: i32) -> i32 {
    println!("Computing {} + {}", x, y); // This will print only when the function is not cached
    x + y
}

#[cachable]
fn concatenate_strings(a: &str, b: &str) -> String {
    println!("Concatenating {} and {}", a, b); // This will print only when the function is not cached
    format!("{}{}", a, b)
}

fn main() {
    // Test integer addition
    println!("{}", expensive_computation(1, 2)); // Computes and caches
    println!("{}", expensive_computation(1, 2)); // Uses cache
    println!("{}", expensive_computation(3, 4)); // Computes and caches
    println!("{}", expensive_computation(3, 4)); // Uses cache
    println!("{}", expensive_computation(4, 4)); // Computes and caches

    println!("{}", expensive_computation(1, 2)); // not in cache will compute
    println!("{}", expensive_computation(3, 4)); // Uses cache

    // Test string concatenation
    println!("{}", concatenate_strings("Hello, ", "World!")); // Computes and caches
    println!("{}", concatenate_strings("Hello, ", "World!")); // Uses cache
    println!("{}", concatenate_strings("Rust", "Lang")); // Computes and caches
    println!("{}", concatenate_strings("Rust", "Lang")); // Uses cache



    println!("{}", expensive_computation1(3, 4)); // Uses cache
    println!("{}", expensive_computation1(3, 4)); // Uses cache

    sleep(StdDuration::from_secs(3));

    println!("sleep ended"); // Uses cache
    println!("{}", expensive_computation1(3, 4)); // Uses cache

}
