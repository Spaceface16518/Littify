extern crate littify_core;
use littify_core::{littify_string, process_args};
use std::env::args;

fn main() {
    println!(
        "{}",
        littify_string(process_args(args().skip(1).collect::<Vec<String>>()))
            .as_str()
    );
}
