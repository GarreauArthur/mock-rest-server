# Simple mock REST API


./cargo run -- --help


if needed: rustup target add x86_64-unknown-linux-musl

build with

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

## Dependencies

* [warp](https://github.com/seanmonstar/warp) for the web server
    * [doc](https://docs.rs/warp/latest/warp/index.html)
    * [examples](https://github.com/seanmonstar/warp/tree/master/examples)
* [clap](https://docs.rs/clap/latest/clap/index.html) for manipulating arguments