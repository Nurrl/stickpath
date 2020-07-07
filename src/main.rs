/* Usage of this crate for error handling to facilitate error type creation */
extern crate derive_more;

use std::process;

mod error;
mod input;

fn main() {
    let s = input::retrieve().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    println!("{:?}", s);
}
