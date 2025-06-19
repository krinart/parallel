# parallel

A lightweight Rust library providing declarative macros for running async tasks in parallel and managing timeouts with graceful fallbacks.

## Features

- **`parallel!`** — Run multiple async expressions concurrently and wait for all to complete.
- **`timeout!`** — Run an async expression with a timeout and a custom fallback.
- **`timeout_fallback!`** — Return a fallback value if a task exceeds the timeout.
- **`timeout_value!`** — Like `timeout_fallback!`, but accepts a fallback of a different type.
- **`timeout_with_result!`** — Returns a `TimeoutResult` enum (`Success`, `Error`, or `TimedOut`).
  - ✅ Supports the `?` operator inside Result-returning functions for clean error handling.
- **`first!`** — Run multiple async expressions and return the first one that completes successfully.

## Installation

Add this to your `Cargo.toml`:

```toml
parallel = { git = "https://github.com/krinart/parallel" }
```

## Import

```rust
use parallel_macro::{
    parallel,
    timeout,
    timeout_fallback,
    timeout_value,
    timeout_with_result,
    first,
};
```

## Example: `timeout_with_result!` + `parallel!` with `?` operator

```rust
use parallel_macro::{parallel, timeout_with_result};

async fn get_posts(user_id: u64) -> Vec<String> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    vec![format!("Post from {}", user_id)]
}

async fn get_followers(user_id: u64) -> Vec<String> {
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    vec![format!("Follower of {}", user_id)]
}

async fn get_user_data(user_id: u64) -> Result<usize, String> {
    let (posts, followers) = timeout_with_result!(1 {
        parallel! {
            get_posts(user_id),
            get_followers(user_id),
        }
    })?;

    Ok(posts.len() + followers.len())
}

#[tokio::main]
async fn main() {
    match get_user_data(42).await {
        Ok(count) => println!("User has {} total items", count),
        Err(err) => println!("Error: {}", err),
    }
}
```

## Example: `timeout_with_result!` + `parallel!` with `else` branch

```rust
use parallel_macro::{parallel, timeout_with_result};
use parallel_macro_core::TimeoutResult;

async fn get_posts(user_id: u64) -> Vec<String> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    vec![format!("Post from {}", user_id)]
}

async fn get_followers(user_id: u64) -> Vec<String> {
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    vec![format!("Follower of {}", user_id)]
}

async fn get_user_data(user_id: u64) -> Result<usize, String> {
    let result = timeout_with_result!(1 {
        parallel! {
            get_posts(user_id),
            get_followers(user_id),
        }
    } else {
        println!("Timeout occurred while fetching user data.");
        TimeoutResult::TimedOut
    });

    match result {
        TimeoutResult::Success((posts, followers)) => Ok(posts.len() + followers.len()),
        TimeoutResult::Error(err) => Err(format!("Task failed: {}", err)),
        TimeoutResult::TimedOut => Err("Timed out fetching user data".into()),
    }
}

#[tokio::main]
async fn main() {
    match get_user_data(42).await {
        Ok(count) => println!("User has {} total items", count),
        Err(err) => println!("Error: {}", err),
    }
}
```

## Use Cases

- Efficiently gather multiple async resources (e.g., APIs, DBs) in parallel
- Enforce timeouts with clean syntax and optional fallback behavior
- Write readable, fault-tolerant, and structured async logic with minimal boilerplate

## Dependencies

- **tokio** — Async runtime
- **Rust proc-macro** — For declarative macro definitions

## License

MIT
