use std::{env::*, process::*};
use http::*;

fn url_shortener(url: String) {
    println!("URL: {url}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <url>");
        std::process::exit(1);
    }
    url_shortener(args[1].clone());
}
