extern crate url;

use std::env;
use url::Url;

const POSTGRES_HOST: &'static str = env!("POSTGRES_HOST");
const POSTGRES_PORT: &'static str = env!("POSTGRES_PORT");
const POSTGRES_USER: &'static str = env!("POSTGRES_USER");
const POSTGRES_PASSWORD: &'static str = env!("POSTGRES_PASSWORD");
const POSTGRES_DATABASE: &'static str = env!("POSTGRES_DATABASE");

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
