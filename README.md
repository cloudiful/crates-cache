# Introduction

***

This is my way of persistent data storage in Rust.

# Usage

## Save data

```rust
#[derive(Serialize, Deserialize)]
struct Apple {
    pub(crate) name: String,
    price: i32,
}

fn save() {
    let apple = Apple::new();
    let apple_cache = Cache::new("apple_cache");
    apple_cache.save(&apple);
}
```

***

## Read data

```rust
fn read() {
    let apple_cache = Cache::new("apple_cache");
    let cached_apple: Option<Apple> = apple_cache.read();
}
```

***

## Clear

```rust
fn clear() {
    let apple_cache = Cache::new("apple1");
    apple_cache.clear();
}
```

## Advanced configuration

### set_storing_method

Change storing method

1. JSON: Save and read from a JSON file.

2. SQLite: Insert, update and read from a local sqlite db file.

```rust
fn save_to_file() {
    let apple = Apple::new(4);
    let mut apple_cache = Cache::new("apple2");
    apple_cache.set_storing_method(StoringMethod::JSON); // change storing method
    apple_cache.save(&apple);
}
```

### set_valid_period

Change valid period

Set how much time cache is valid

```rust
fn save_to_file() {
    let apple = Apple::new(4);
    let mut apple_cache = Cache::new("apple2");
    apple_cache.set_valid_period(TimeDelta::minute(60)); // set valid period to 10 minute 
    apple_cache.save(&apple);
}
```

### set_table

Change sql table

```rust
fn save_to_file() {
    let apple = Apple::new(4);
    let mut apple_cache = Cache::new("apple2");
    apple_cache.set_table("data");
    apple_cache.save(&apple);
}

```