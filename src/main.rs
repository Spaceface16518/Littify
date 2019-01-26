extern crate littify_core;
use littify_core::{littify_string, ProcessArgsExt};
use std::env::args;

fn main() {
    println!(
        "{}",
        littify_string(args().skip(1).collect::<Vec<String>>().process_args())
            .as_str()
    );
}
