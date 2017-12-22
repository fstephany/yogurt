
//! This program starts a webserver on port 8080. It exposes a static file server
//! that can be used to develop Yogurt without hitting a real server.

extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;
use std::process::exit;
use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let port = 8080;
    let address = "127.0.0.1";
    let path: &Path = Path::new("./dev-server-files");

    if !path.exists() {
        println!("Path {:?} does not exist.", path);
        exit(1)
    }

    if !path.is_dir() {
        println!("Path {:?} is not a directory.", path);
        exit(1)
    }

    let mut mount: Mount = Mount::new();
    mount.mount("/", Static::new(path));

    match Iron::new(mount).http((address, port)) {
        Ok(_) => {
            println!("Starting up http-server, serving path {:?}", path);
            println!("Available on:");
            println!("  http://{}:{}", address, port);
            println!("Hit CTRL-C to stop the server")
        }
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    }
}