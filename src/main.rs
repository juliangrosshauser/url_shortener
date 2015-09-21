extern crate url;

use std::env;
use url::Url;

fn usage() {
    println!("\
URL Shortener

Usage:
  url_shortener <url>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
        return;
    }

    let url = Url::parse(&args[1]);

    if url.is_err() {
        usage();
        return;
    }

    let _url = url.unwrap();
}
