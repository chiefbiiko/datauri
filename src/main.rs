extern crate base64;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main () {
    let filename: String = parse_filename_arg();
    println!("{:?}", filename);
    let buf: Vec<u8> = read_file_buf(filename);
    println!("{:?}", buf);
    let based: String = base64::encode(&buf);
    println!("{:?}", based);
    // TODO: concat entire data uri by looking at the file extension ...
    // ... panic if the filename does not have a klnown extension
}

fn parse_filename_arg () -> String {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("filename missing");
    }
    args[1].to_string()
}

fn read_file_buf (filename: String) -> Vec<u8> {
    let mut f = File::open(filename).expect("file not found");
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf).expect("file reading failed");
    buf
}
