use std::fs;
// use std::io;
use std::fs::File;
use std::io::Read;
use std::fmt::Debug;
use std::env;
use std::process;

use owo_colors::{OwoColorize};
use owo_colors::colors::*;
// use utf16string::{BigEndian, BE, WString, LE, LittleEndian, WStr};
use anyhow::{Context, Result};
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};
use prettytable::{Table, Row, Cell};
use unbytify::*;
// use hex::decode;
use ascii_converter::*;


#[macro_use] extern crate prettytable;
#[derive(Debug, PartialEq)]

struct Point {
    x: i32,
    y: i32,
}
static EPOCH: u64 = 116444736000000000;
static HUNDREDS_OF_NANOSECONDS: u64 = 10000000;

const BANNER: &str = r#"
 ____  _       _____                         
|  _ \(_)     |  __ \                        
| |_) |_ _ __ | |__) |_ _ _ __ ___  ___ _ __ 
|  _ <| | '_ \|  ___/ _` | '__/ __|/ _ \ '__|
| |_) | | | | | |  | (_| | |  \__ \  __/ |   
|____/|_|_| |_|_|   \__,_|_|  |___/\___|_|   
                                            ♥ Rust
    ----- Built with ♥ by greycat -----
"#;

fn main() {
    banner();

    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args[1]);
    let mut dir: String = String::new();
    if args.len() > 1 {
        if args[1].contains("--dir") {
            let arg1: Vec<&str> = args[1].split("=").collect();
            dir = arg1[1].trim().to_string();
            // println!("{:?}", &dir);
            // ./examples/$RECYCLE.BIN/S-1-5-21-4144826732-2003267707-115468498-1001
        }
    } else {
        println!("Please supply argument !");
        process::exit(1);
    }

    if (dir.trim().is_empty()) {
        println!("Directory is empty !");
        process::exit(1);
    }
    
    if !path_exists(&dir) {
        println!("Directory not found !");
        process::exit(1);
    }

    // let dir = "./examples/$RECYCLE.BIN/S-1-5-21-4144826732-2003267707-115468498-1001";
    let paths = fs::read_dir(dir).unwrap();

    // Create the table
    let mut table = Table::new();
    // Add a row per time
    table.add_row(row!["Index", "Deletion Time", "File Size", "Version", "Original Path"]);

    for path in paths {
        let s = path.unwrap().path().display().to_string();

        if s.contains("$I") {
            let vec_path: Vec<&str> = s.split('/').collect();
            let id_file_name = vec_path.len() - 1;
            let file_name = vec_path[id_file_name];
            // println!("{}", file_name);

            let result = hex_repl(get_file_as_byte_vec(&s));
            let bytes  = chunks(&result, 2).collect::<Vec<&str>>();

            //  get delete time
            let deletion_time = &deletion_time(bytes.clone());

            // filesze
            let filesize         = &filesize(bytes.clone());
            let approximate_size = &approximate_size(filesize.to_string());

            // version
            let version = &version(bytes.clone());

            // ori path
            let ori_path = &original_path(bytes.clone(), version.to_string());

            table.add_row(
                Row::new(
                    vec![
                        Cell::new(file_name),
                        Cell::new(deletion_time),
                        Cell::new(approximate_size),
                        Cell::new(version),
                        Cell::new(ori_path)
                    ]
                )
            );
        }
    }
    table.printstd();
}

fn banner() {
    println!("\n\n{}", BANNER.fg::<Green>().bold());
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn deletion_time<'a>(bytes: Vec<&'a str>) -> String {
    let string_byte = &bytes[16..24];
    let val = hex_bytes_to_u64(string_byte).unwrap();
    let deletion_time = time_to_date(val);

    deletion_time
}

fn filesize<'a>(bytes: Vec<&'a str>) -> String {
    let string_byte = &bytes[8..16];
    let val = hex_bytes_to_u64(string_byte).unwrap();
    let filesize = val.to_string();

    filesize
}

fn version<'a>(bytes: Vec<&'a str>) -> String {
    let version = &bytes[..1][0].to_string();

    let mut ret = "Win Vista";
    if version == "02" {
        ret = "Win 10";
    }

    ret.to_string()
}

fn original_path<'a>(bytes: Vec<&'a str>, version: String) -> String {
    let mut pathname: Vec<&'a str> = vec![];
    if version == "Win 10" {
        pathname = bytes[28..].to_vec();
    }
    else {
        pathname = bytes[24..].to_vec();
    }
    let some_x = "00";
    pathname.retain(|&x| x != some_x);

    // let mut string_vec: Vec<String> = Vec::new();
    // for s in &pathname {
    //     string_vec.push(s.to_string());
    // }
    let path_string = pathname.into_iter().map(|s| s.to_string()).collect();

    hexadecimal_to_string(path_string).unwrap()
}

fn approximate_size(filesize: String) -> String {
    let fixsize: u64 = filesize.parse().unwrap_or(0);
    let bytify = bytify(fixsize);
    let pure_size: String = bytify.0.to_string();
    let unit: String = bytify.1.to_string();
    
    let ret = pure_size + " "+ &unit;

    ret
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn time_to_date(val: u64) -> String {
    let calc_time = (val - EPOCH) / HUNDREDS_OF_NANOSECONDS;
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(calc_time);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    timestamp_str
}

fn chunks(s: &str, length: usize) -> impl Iterator<Item=&str> {
    assert!(length > 0);
    let mut indices = s.char_indices().map(|(idx, _)| idx).peekable();
    
    std::iter::from_fn(move || {
        let start_idx = match indices.next() {
            Some(idx) => idx,
            None => return None,
        };
        for _ in 0..length - 1 {
            indices.next();
        }
        let end_idx = match indices.peek() {
            Some(idx) => *idx,
            None => s.bytes().len(),
        };
        Some(&s[start_idx..end_idx])
    })
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn hex_repl(byte_array: Vec<u8>) -> String {
    let build_string_vec: Vec<String> = byte_array.chunks(2)
        .map(|c| {
            if c.len() == 2 { 
                format!("{:02x}{:02x}", c[0], c[1]) 
            }
            else { 
                format!("{:02x}", c[0]) 
            }
        }).collect();

    build_string_vec.join("")
}

fn hex_bytes_to_u64(input: &[&str]) -> anyhow::Result<u64> {
    anyhow::ensure!(input.len() == 8, "expected 8 hex bytes");
    let bytes = input
        .iter()
        .map(|s| u8::from_str_radix(s, 16))
        .collect::<Result<Vec<_>, _>>()
        .context("Hex byte parse failure")?;
    Ok(u64::from_le_bytes(bytes.try_into().expect("u64 doesn't have 8 bytes?")))
}