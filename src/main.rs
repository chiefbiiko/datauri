// TODO: modularize with a library

extern crate base64;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;

fn main () {
    let filename: String = parse_filename_arg();
    println!(
        "data:image/{};base64,{}",
        infer_img_type(&filename),
        &base64::encode(&read_file_buf(&filename))
    );
}

// TODO: check argv types
fn parse_filename_arg () -> String {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("filename required");
    }
    args[1].to_string()
}

fn read_file_buf (filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("file not found");
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).expect("file reading failed");
    buf
}

fn extract_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn infer_img_type (filename: &str) -> &str {
    let ext: &str = extract_extension(filename).expect("missing file extension");
    match ext {
        "svg" => "svg+xml",
        _ => "*"
    }
}
