# cacheForge

<div style="text-align: center;">
  <img src="cache_forge.jpg" alt="cacheForge Logo" width="100"/>
</div>

## Overview
`cacheForge` is a Rust project that provides caching utilities and macros to enhance performance and efficiency in your applications. It includes several crates:

- `cache_demo`: A demonstration crate showcasing the usage of caching macros.
- `cache_macro`: A procedural macro crate that provides caching capabilities.
- `memory_box`: A crate that includes various memory management utilities, including an LRU cache.


### Features:

#### lru_cache :
- **Efficient Memory Management**: Implements an LRU (Least Recently Used) cache to optimize memory usage.
- **Thread-Safe**: Designed to be used in multi-threaded environments without compromising performance.
- **Customizable**: Allows customization of cache size and eviction policies to suit different use cases.
- **Easy Integration**: Simple API for integrating the LRU cache into your existing projects.

#### expire_cache :
- **Time-Based Expiration**: Automatically removes entries after a specified duration to ensure data freshness.
- **Configurable**: Allows setting custom expiration times for different cache entries.
- **Thread-Safe**: Safe to use in concurrent environments, ensuring data integrity.
- **Lightweight**: Minimal overhead, designed to be efficient and fast.

### cache_demo
This crate demonstrates how to use the caching macros provided by `cache_macro`.

#### Usage
To run the demo, navigate to the `cache_demo` directory and use Cargo:
```sh
cd cache_demo
cargo run
