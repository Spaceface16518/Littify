extern crate littify_core;
use littify_core::{LittifyStringExt, ProcessArgsExt};
use std::env::args;

fn main() {
    println!("{}", &args().skip(1).process_args().littify());
}
