#![feature(slice_patterns)]

extern crate getopts;
extern crate base64;

use std::path::Path;
use std::fs::File;
use std::io::stdin;
use std::io::Read;
use getopts::Options;
use base64::{encode, decode};

#[allow(dead_code)]
fn main() {
    let mut opts = Options::new();
    opts.optflag("h", "help", "show usage");
    opts.optflag("d", "decode", "decode");
    opts.optflag("e", "encode", "encode");

    let args: Vec<String> = std::env::args().collect();
    let m = opts.parse(&args[1..]).unwrap();

    if m.opt_present("h") {
        println!("{}", opts.usage("Base64 encoding and decoding"));
        return;
    }

    let input: Vec<u8> = match m.free.as_slice() {
        [ref s, ..] => {
            let file = File::open(&Path::new(s));
            let mut buf = Vec::new();
            let _ = file.unwrap().read_to_end(&mut buf).unwrap();
            buf
        }
        _ => {
            let mut buf = Vec::new();
            let _ = stdin().read_to_end(&mut buf);
            buf
        }
    };

    if m.opt_present("d") {
        let result = decode(input.as_slice());
        print!("{}", String::from_utf8(result).unwrap());
    } else {
        let result = encode(input.as_slice());
        print!("{}", String::from_utf8(result).unwrap());
    }
}
