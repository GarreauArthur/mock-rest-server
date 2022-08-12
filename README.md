# Simple mock REST API

Only tested on Ubuntu x86_64

Simple program to mock a REST endpoint and store the request's body in a file.
You can make an HTTP POST request to `localhost:<port>/<resource>` and store the
request's body in the file `<dir>/file-<current-timestamp>`.

You can change the values of `port`, `resource` and `dir`.
The default values are:

* port: 5123
* resource: post
* dir: /tmp/mocked_server

to run the server use:

```sh
cargo run -- --help
```

or build it with:

```bash
cargo build --release --target x86_64-unknown-linux-musl
```
⚠️ if needed: `rustup target add x86_64-unknown-linux-musl`.

## Dependencies

* [warp](https://github.com/seanmonstar/warp) for the web server
    * [doc](https://docs.rs/warp/latest/warp/index.html)
    * [examples](https://github.com/seanmonstar/warp/tree/master/examples)
* [clap](https://docs.rs/clap/latest/clap/index.html) for manipulating arguments