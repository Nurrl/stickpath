/* Usage of this crate for error handling to facilitate error type creation */
extern crate derive_more;

use std::process;

mod error;
mod alg;

fn main() {
    /* Retrieve, parse and process the tree */
    let (head, tail) = alg::retrieve().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

    /* Print pairs */
    for (idx, e) in head.iter().enumerate() {
        println!("{}{}", e, tail[idx]);
    }
}
