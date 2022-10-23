# invaders-rs
Final project of [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course/)

## 🎬 How to start
1. Get all dependencies by running `cargo build && cargo build --release`
2. For testing just run `cargo run`

## 🚨 Notice
1. For macOS user (and maybe other OS too), you need to set the sleeping duration in `src/main`

```
        // Sleep to wait for the render loop ready
        // Configure the sleep time. Each computer is different (IDK why ???)
        thread::sleep(Duration::from_millis(20));
        |-----------------------------------^^^
        |___Change this___
```