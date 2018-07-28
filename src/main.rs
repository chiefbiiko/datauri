extern crate base64;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;

struct ExtensionRegex {
    font: Regex,
    img: Regex,
    svg: Regex
}

// static ExtRe: ExtensionRegex = ExtensionRegex {
//     font: Regex::new(r"").unwrap(),
//     img: Regex::new(r"(?:jpg|jpeg|png|svg|gif)$").unwrap(),
//     svg: Regex::new(r"\\.svg$").unwrap()
// };

fn main () {
    let ExtRe: ExtensionRegex = ExtensionRegex { // TODO: make this static
        font: Regex::new(r"").unwrap(), // TODO: allow GoogleFont font exetensions
        img: Regex::new(r"(?:jpg|jpeg|png|svg|gif)$").unwrap(), // TODO: ignore case
        svg: Regex::new(r"\\.svg$").unwrap() // TODO: ignore case
    };

    let filename: String = parse_filename_arg();
    let buf: Vec<u8> = read_file_buf(&filename);
    println!("{}", concat_uri(&filename, &buf, &ExtRe));
}

// TODO: move below to lib.rs
fn parse_filename_arg () -> String {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("filename missing");
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

fn concat_uri (filename: &str, buf: &Vec<u8>, ExtRe: &ExtensionRegex) -> String {
    let media_t: &str;
    let m_type: &str;
    let ext: &str = extract_extension(filename).expect("missing file extension");

    // TODO: match
    if ExtRe.img.is_match(ext) {
        media_t = "image";
    }
    // else if ExtRe.font.is_match(ext) {
    //     media_t = "font";
    // }
    else {
        panic!("unknown file extension");
    }

    if ExtRe.svg.is_match(ext) {
        m_type = "svg+xml"
    } else if media_t == "font" {
        m_type = ext;
    } else {
        m_type = "*"
    }

    format!("data:{}/{};base64,{}", media_t, m_type, base64::encode(buf))
}
