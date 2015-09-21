use std::env;

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

    let _url = &args[1];
}
