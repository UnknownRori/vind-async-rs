# vind-async

A fun lighweight runtime for writing weird asynchronous applications, it's still on low level,
it's doesn't have any built in stuff like [tokio](https://crates.io/crates/tokio) or [async-std](https://crates.io/crates/async-std) do

## Example

```rust
async fn hi() {
    println!("Hi!");
}

#[vind_async::main]
async main() {
    println!("Hello Async!");
    hi().await;
}
```

## 🚀 Installation & Running

> [!WARNING]
> This is fun project and there is no way that I will upload it into public registry (maybe)
> and see the example for detailed on the usage

```sh
# Clone repository and enter to cloned directory
git clone https://github.com/UnknownRori/vind-async-rs
cd vind-async-rs

# Run one of the example
cargo run--bin quote-api-fetcher
```

## License

This project is licensed under the [GLWTSPL](/LICENSE).

![Good Luck](https://github.com/me-shaon/GLWTPL/raw/master/good-luck.gif)

...and godspeed.
