extern crate base64;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;

fn main () {
    let filename: String = parse_filename_arg();
    println!("{:?}", filename);
    let buf: Vec<u8> = read_file_buf(&filename);
    println!("{:?}", buf);
    let based: String = base64::encode(&buf);
    println!("{:?}", based);
    concat_uri(&filename, &buf)
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

fn read_file_buf (filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("file not found");
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf).expect("file reading failed");
    buf
}

fn extract_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn concat_uri (filename: &str, buf: &Vec<u8>) -> () {
    let media_t: &str;
    let m_type: &str;
    let ext: &str = extract_extension(filename).expect("missing file extension");

    // let re_img_ext = Regex::new(IMG_EXTENSIONS).unwrap();
    // let re_svg_ext = Regex::new(SVG_EXTENSIONS).unwrap();
    // let re_font_ext = Regex::new(FONT_EXTENSIONS).unwrap();
    //
    // if re_img_ext.is_match(ext) {
    //     media_t = "image";
    // } else if re_font_ext.is_match(ext) {
    //     media_t = "font";
    // }
    //
    // if re_svg_ext.is_match(ext) {
    //     m_type = "svg+xml"
    // } else if media_t == "font" {
    //     m_type = ext;
    // } else {
    //     m_type = "*"
    // }
}
