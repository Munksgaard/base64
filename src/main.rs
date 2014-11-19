extern crate getopts;
extern crate base64;

use std::os;
use std::io::{stdin, File};
use getopts::{optflag, getopts, usage};
use base64::{encode, decode};

#[allow(dead_code)]
fn main() {
    let opts = [optflag("h", "help", "show usage"),
                optflag("d", "decode", "decode"),
                optflag("e", "encode", "encode")];

    let m = getopts(os::args().tail(), &opts).ok().expect("Fail");

    if m.opt_present("h") {
        println!("{}", usage("Base64 encoding and decoding", &opts));
        return;
    }

    let input: Vec<u8> = match m.free.as_slice() {
        [ref s, ..] => {
            let mut file = File::open(&Path::new(s));
            let input = file.read_to_end();
            input.ok().expect("Fail")},
        _ => stdin().read_to_end().ok().expect("Fail"),
    };

    if m.opt_present("d") {
        let result = decode(input.as_slice());
        print!("{}", result.into_ascii().into_string());
    } else {
        let result = encode(input.as_slice());
        print!("{}", result.into_ascii().into_string());
    }
}
