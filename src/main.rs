extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StatusCode {
    code: String,
    phrase: String,
    description: String,
    spec_title: String,
    spec_href: String,
}

fn get_codes_by_code(codes: &Vec<StatusCode>, code: &str) -> Result<Vec<StatusCode>, &'static str> {
    let mut ret = Vec::new();
    for c in codes.iter().cloned() {
        if c.code == code {
            ret.push(c)
        }
    }
    match ret.len() {
        0 => Err("Can't find the code"),
        _ => Ok(ret),
    }
}

// Print-out HTTP status code information
fn print(c: &StatusCode) {
    println!(
        "{} {}\n{}\n{} ({})\n",
        c.code, c.phrase, c.description, c.spec_title, c.spec_href
    )
}

fn main() {
    let matches = App::new("hsc")
        .version("0.1.0")
        .author("Nobuhide Yayoshi <n.yayoshi@gmail.com>")
        .about("Explains the meaning of HTTP status code written in Rust")
        .arg(
            Arg::with_name("CODE")
                .required(false)
                .takes_value(true)
                .index(1)
                .help("HTTP status code"),
        )
        .get_matches();
    let file = File::open("status-codes.json").expect("file not found");
    let codes: Vec<StatusCode> = serde_json::from_reader(file).expect("error while reading json");
    let code = matches.value_of("CODE");
    match code {
        Some(c) => match get_codes_by_code(&codes, &c) {
            Ok(v) => {
                for v in v.iter() {
                    print(&v);
                }
            }
            Err(e) => println!("{}", e),
        },
        None => {
            for c in codes.iter() {
                print(&c);
            }
        }
    }
}
