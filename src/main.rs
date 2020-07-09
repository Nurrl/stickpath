/* Usage of this crate for error handling to facilitate error type creation */
extern crate derive_more;

use std::process;

mod alg;
mod error;
mod input;

fn main() {
    /* Retrieve, parse and process the tree */
    let (head, pipes, tail) = input::retrieve().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });

//    println!("{:?} {:?} {:?}", head, pipes, tail);
    /* Combinate with pipes */
    let (head, tail) = alg::resolve(head, pipes, tail);

    /* Print the combination according to the requested format */
    for (idx, el) in head.iter().enumerate() {
        println!("{}{}", el, tail[idx]);
    }
}
