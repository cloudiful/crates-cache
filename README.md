# Introduction

***

This is my way of persistent data storage in Rust.

# Usage

## Save a struct to local file

```rust
fn save() {
    let apple = Apple::new();
    let apple_cache = Cache::new("apple_cache");
    apple_cache.save(&apple);
}
```

The cache file will be at ./temp/apple_cache.json

***

## Read a struct from local file

```rust
fn read() {
    let apple_cache = Cache::new("apple_cache");
    let cached_apple: Option<Apple> = apple_cache.read();
}
```

It will read from ./temp/apple_cache.json and parse it into Option<Apple>

***

## Clear cache

```rust
fn clear() {
    let apple_cache = Cache::new("apple1");
    apple_cache.clear();
}
```

It will simply delete ./temp/apple1.json file