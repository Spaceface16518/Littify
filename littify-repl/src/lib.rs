extern crate littify_core;

use littify_core::littify_string;
use std::io::{BufRead, Write};

pub const EXIT: u8 = b'e';
pub const ESCAPE: u8 = b'!';
pub const NEWLINE: u8 = b'n';
pub const START: &str = "littify>> ";

pub fn repl<I: BufRead, O: Write>(input: I, output: &mut O) {
    for line in input.lines() {
        match repl_cycle(line.unwrap()) {
            CycleCommand::Continue(s) => {
                writeln!(output, "{}", s).expect("Could not write to output");
            }
            CycleCommand::Exit(e) => {
                writeln!(output, "{}", e).expect("Could not write to output");
                break;
            }
        }
    }
}

fn repl_cycle(line: String) -> CycleCommand<String> {
    match escape_bytes(line.clone()) {
        CharCommand::Reg(s) => CycleCommand::Continue(littify_string(s)),
        CharCommand::Exit(i) => {
            CycleCommand::Exit(line.get(0..i).unwrap().to_string())
        }
    }
}

enum CycleCommand<T> {
    Continue(T),
    Exit(T),
}

fn escape_bytes(orig: String) -> CharCommand<String> {
    // BUG: escaping "!" doesn't invalidate next "!" as an escaper
    let orig = orig.as_bytes();
    let mut new = Vec::new();
    for i in 0..orig.len() {
        match orig[i] {
            ESCAPE => {
                if i + 1 < orig.len() {
                    match orig[i + 1] {
                        ESCAPE => new.push(ESCAPE),
                        EXIT => return CharCommand::Exit(i),
                        NEWLINE => new.push(b'\n'),
                        _ => (),
                    }
                } else {
                    new.push(ESCAPE)
                }
            }
            _ => new.push(orig[i]),
        }
    }
    return CharCommand::Reg(
        String::from_utf8_lossy(new.as_slice()).to_string(),
    );
}

enum CharCommand<T> {
    Reg(T),
    Exit(usize),
}
