use warp::{Filter, http::HeaderMap, path::FullPath};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;
use std::net::{SocketAddrV4, Ipv4Addr};
use clap::Parser;

/// Simple program to mock a REST endpoint and store the request's body in a file.
/// You can make a POST request on localhost:<port>/<resource> and store the
/// request's body in the file <dir>/file-<timestamp>
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// The port to listen on
   #[clap(short, long, value_parser, default_value_t = 5123)]
   port: u16,

   /// Resource (no leading slash), default is "post"
   #[clap(short, long, value_parser, default_value = "post")]
   resource: String,

   /// File's directory where will be stored the request's body (no ending '/')
   #[clap(short, long, value_parser, default_value = "/tmp/mocked_endpoint")]
   dir: String,
}

use std::fs;

#[tokio::main]
async fn main() {

    let args = Args::parse();
    dbg!(&args);
    let post = warp::post()
    .and(warp::path(args.resource))
    .and(warp::header::headers_cloned())
    .and(warp::path::full())
    .and(warp::query::raw())
    .and(warp::body::bytes())
    .map(move |headers: HeaderMap, path: FullPath, params: String, bytes: warp::hyper::body::Bytes| {

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let filename_path = format!("{}/body-{}", &args.dir, time.as_secs());
        let filepath = Path::new(&filename_path);
        fs::create_dir_all(&args.dir).unwrap();
        let mut file = File::create(&filepath).unwrap();
        for chunk in bytes.chunks(1024) {
            file.write_all(chunk).unwrap();
        }

        let res = format!("Headers: {:?}\nPath: {:?}\nParams: {:?}\nFile stored in {}\n", headers, path, params, filename_path);
        println!("{}", &res);
        res

    });

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("*" / String)
        .map(|name| format!("Hello, {}!", name));

    let socket = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), args.port);
    warp::serve(hello.or(post))
        .run(socket)
        .await;
}