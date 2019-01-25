extern crate littify_repl;

use std::io::{stdin, stdout};

fn main() {
    let (i, o) = (stdin(), stdout());
    let i = i.lock();
    let mut o = o.lock();
    littify_repl::repl(i, &mut o);
}
