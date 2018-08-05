#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StatusCode {
    code: String,
    phrase: String,
    description: String,
    spec_title: String,
    spec_href: String,
}

impl StatusCode {
    fn include(&self, keyword: &str) -> bool {
        self.code.contains(keyword)
            || self.phrase.contains(keyword)
            || self.description.contains(keyword)
    }
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

fn get_codes_by_keyword(
    codes: &Vec<StatusCode>,
    keyword: &str,
) -> Result<Vec<StatusCode>, &'static str> {
    let mut ret = Vec::new();
    for c in codes.iter().cloned() {
        if c.include(keyword) {
            ret.push(c)
        }
    }
    match ret.len() {
        0 => Err("Can't find the code"),
        _ => Ok(ret),
    }
}

// Print-out HTTP status code information
fn print(codes: &Vec<StatusCode>) {
    for c in codes.iter() {
        println!(
            "{} {}\n{}\n{} ({})\n",
            c.code, c.phrase, c.description, c.spec_title, c.spec_href
        )
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("CODE")
                .required(false)
                .takes_value(true)
                .index(1)
                .help("HTTP status code"),
        )
        .arg(
            Arg::with_name("KEYWORD")
                .short("s")
                .long("search")
                .value_name("KEYWORD")
                .required(false)
                .takes_value(true)
                .help("Sets a keyword to search"),
        )
        .get_matches();
    let path = Path::new("./src/status-codes.json");
    let file = File::open(&path).expect("file not found");
    let codes: Vec<StatusCode> = serde_json::from_reader(file).expect("error while reading json");

    if let Some(keyword) = matches.value_of("KEYWORD") {
        match get_codes_by_keyword(&codes, &keyword) {
            Ok(v) => print(&v),
            Err(e) => println!("{}", e),
        }
    } else {
        let code = matches.value_of("CODE");
        match code {
            Some(c) => match get_codes_by_code(&codes, &c) {
                Ok(v) => print(&v),
                Err(e) => println!("{}", e),
            },
            None => print(&codes), // Print all HTTP status codes
        }
    }
}
