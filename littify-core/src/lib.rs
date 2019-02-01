#[cfg(test)]
extern crate rand;
// #[cfg(test)]
// extern crate test;

use std::{env::args, iter::IntoIterator};

/// This is the old `main` method from when this program was just a binary
/// application without a library. Now it is based on connection to a library,
/// but until the API is finished (version `2.x`) the binary application will
/// simply run this method inside its main method. A lot of the library is
/// based on code originally created in this method.
///
/// # Examples
///
/// ```rust
/// extern crate littify_core;
///
/// fn main() { littify_core::legacy_main(); }
/// ```
///
/// # Testing
///
/// The nature of method makes it untestable; another reason this method is
/// deprecated as of version `2.X`. You can test it manually by calling it
/// inside a main method, as seen in the [Examples][1] section.
///
/// [1]: #examples
#[deprecated]
pub fn legacy_main() {
    let argv: Vec<String> = args().skip(1).collect();
    let string: String = if argv.len() == 0 {
        String::default()
    } else {
        let mut o_string: String = String::new();
        let mut index = 0;
        for i in argv.join(" ").chars() {
            if i.is_alphabetic() {
                if index & 1 == 0 {
                    o_string.push(i.to_ascii_lowercase());
                } else {
                    o_string.push(i.to_ascii_uppercase());
                }
                index += 1;
            } else {
                o_string.push(i);
            }
        }
        o_string
    };
    println!("{}", string);
}

/// A simple method that formats a vector of strings into a single string that
/// can be processed by the `littify_string` method.
///
/// # Examples
///
/// Refer to to unit tests for this function for more extensive examples.
/// ```rust
/// extern crate littify_core;
/// use littify_core::process_args;
///
/// fn main() {
///     let argv = vec![
///         "These".to_string(),
///         "are".to_string(),
///         "some".to_string(),
///         "command-line".to_string(),
///         "arguments".to_string(),
///     ];
///     assert_eq!(
///         process_args(argv),
///         "These are some command-line arguments".to_string()
///     );
/// }
/// ```
///
/// # Testing
///
/// The test key for this method is "process_args" (the name of the method)
///
/// ```shell
/// cargo test process_args --all
/// ```
///
/// or
///
/// ```shell
/// cargo test process_args --all -p littify_core
/// ```
pub fn process_args<S: ToString>(argv: Vec<S>) -> String {
    argv.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ")
}

pub trait ProcessArgsExt<S> {
    fn process_args(self) -> String;
}

impl<S: IntoIterator> ProcessArgsExt<S> for S
where
    S::Item: ToString,
{
    fn process_args(self) -> String {
        self.into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// "littify" a string. This method takes `S where S: ToString` and returns a
/// `String` that is the original, but littified. Littifying a string includes
/// all of the following. There are some small nuances, but these three things
/// are the main process.
///
/// - Alternate alphabetic characters (lowercase, uppercase)
/// - Normalize the pattern, starting with a lowercase
/// - Skip non-alphabetic characters
///
/// # Examples
///
/// ```rust
/// extern crate littify_core;
/// use littify_core::littify_string;
///
/// fn main() {
///     let orig: String = "Littify this string".to_string();
///     assert_eq!(littify_string(orig), "lItTiFy ThIs StRiNg".to_string());
/// }
/// ```
///
/// # Testing
///
/// The test key for this method is "littify_string" (the name of this method)
///
/// ```shell
/// cargo test littify_string --all
/// ```
///
/// or
///
/// ```shell
/// cargo test littify_string -p littify-core
/// ```
#[deprecated]
pub fn littify_string<S: ToString>(orig: S) -> String { orig.littify() }

pub trait LittifyStringExt {
    fn littify(&self) -> String;
}

impl<S: ToString> LittifyStringExt for S {
    fn littify(&self) -> String {
        let orig = self.to_string();
        if orig.is_empty() {
            // COMBAK: is this edge case necessary?
            return orig;
        } else {
            // Start with a lowercase letter (flip this boolean to switch that)
            let mut b = false;

            // Iterate of individual characters
            orig.chars()
                .map(|c| {
                    // Technically the `to_ascii_whatever` functions will ignore
                    // non-ASCII characters, but this is necessary so that we
                    // don't flip the flag for non-alphebetic characters
                    if c.is_alphabetic() {
                        b = !b;
                        if b {
                            c.to_ascii_lowercase()
                        } else {
                            c.to_ascii_uppercase()
                        }
                    } else {
                        c
                    }
                })
                .collect()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn test_littify_string() {
        let orig = "This is some test text".to_string();
        assert_eq!(littify_string(orig), "tHiS iS sOmE tEsT tExT".to_string());
    }

    #[test]
    fn test_with_symbols() {
        let orig = "This is s0me test text with s0me $yMb0L&".to_string();
        assert_eq!(
            littify_string(orig),
            "tHiS iS s0Me TeSt TeXt WiTh S0mE $yMb0L&".to_string()
        );
    }

    #[test]
    fn test_littify_string_ext_no_input() {
        let orig = String::new();
        assert_eq!(orig.littify(), String::new());
    }

    #[test]
    fn test_littify_string_ext() {
        let orig = "This is some test text".to_string();
        assert_eq!(orig.littify(), "tHiS iS sOmE tEsT tExT".to_string());
    }

    #[test]
    fn test_ext_with_symbols() {
        let orig = "This is s0me test text with s0me $yMb0L&".to_string();
        assert_eq!(
            orig.littify(),
            "tHiS iS s0Me TeSt TeXt WiTh S0mE $yMb0L&".to_string()
        );
    }

    #[test]
    fn test_littify_string_no_input() {
        let orig = String::new();
        assert_eq!(littify_string(orig), String::new());
    }

    #[test]
    fn test_process_args() {
        let argv: Vec<String> = vec!["Littify".to_string(), "this".to_string()];
        assert_eq!(process_args(argv), "Littify this".to_string());
    }

    #[test]
    fn test_process_args_no_input() {
        let argv: Vec<String> = vec![String::new()];
        assert_eq!(process_args(argv), String::new());
    }

    #[test]
    fn test_process_args_single_input() {
        let argv: Vec<String> = vec!["This input has spaces".to_string()];
        assert_eq!(process_args(argv), "This input has spaces".to_string());
    }

    #[test]
    fn test_process_args_newlines() {
        let argv: Vec<String> =
            vec!["This is some text\nwith newlines".to_string()];
        assert_eq!(
            process_args(argv),
            "This is some text\nwith newlines".to_string()
        );
    }

    #[test]
    fn test_process_args_mixed_input() {
        let argv: Vec<String> = vec![
            "This input has multiple".to_string(),
            "and".to_string(),
            "single".to_string(),
            "input".to_string(),
        ];
        assert_eq!(
            process_args(argv),
            "This input has multiple and single input".to_string()
        );
    }

    // #[bench]
    // fn bench_littify_string_small_string_lowercase(b: &mut Bencher) {
    //     b.iter(|| littify_string("asdfasdfas"));
    // }

    // #[bench]
    // fn bench_littify_string_small_string_uppercase(b: &mut Bencher) {
    //     b.iter(|| littify_string("ASDFASDFAS"));
    // }

    // #[bench]
    // fn bench_littify_string_small_mixed(b: &mut Bencher) {
    //     b.iter(|| littify_string("aSdfAsdFaS"))
    // }

    // #[bench]
    // fn bench_littify_string_large_string_lowercase(b: &mut Bencher) {
    //     b.iter(||
    // littify_string("asdfasdfasdfasdfasdfasdfadfasdfasfasdfasdfasdfadfadfasdfsadfasdfadfadsfasdfasdfasdfadsfasdfasdfasdfasdfasdfadsfasdfasdfadfasdfasdfafsdfadsfasdfasdfasdfasdfasdfasdfasdfasdfasdf"));
    // }
}
