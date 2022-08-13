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
#[derive(Parser, Debug, Clone)]
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

fn do_the_work(args: &Args, headers: HeaderMap, path: FullPath, params: String, bytes: warp::hyper::body::Bytes) -> String {
    let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(dur) => dur.as_secs(),
        Err(e) => return format!("Error getting the current time: {}", e),
    };

    let filename_path = format!("{}/body-{}", &args.dir, time);
    let filepath = Path::new(&filename_path);

    match fs::create_dir_all(&args.dir) {
        Ok(_) => (),
        Err(e) => return format!("Error creating the parent(s) directory/ies: {}", e),
    };
    let mut file = match File::create(&filepath) {
        Ok(file) => file,
        Err(e) => return format!("Error creating file: {}", e),
    };

    for chunk in bytes.chunks(1024) {
        match file.write_all(chunk) {
            Ok(_) => (),
            Err(e) => return format!("Error writing chunk in file: {}", e),
        };
    }

    format!("Headers: {:?}\nPath: {:?}\nParams: {:?}\nFile stored in {}\n", headers, path, params, filename_path)
}

#[tokio::main]
async fn main() {

    let args = Args::parse();
    let port = args.port;
    let resource = args.resource.clone();
    dbg!(&args);

    let post = warp::post()
    .and(warp::path(resource))
    .and(warp::header::headers_cloned())
    .and(warp::path::full())
    .and(warp::query::raw())
    .and(warp::body::bytes())
    .map(move |headers: HeaderMap, path: FullPath, params: String, bytes: warp::hyper::body::Bytes| {

        let res = do_the_work(&args, headers, path, params, bytes);
        println!("{}", &res);
        res
    });

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("*" / String)
        .map(|name| format!("Hello, {}!", name));

    let socket = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), port);
    warp::serve(hello.or(post))
        .run(socket)
        .await;
}