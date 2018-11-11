#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

extern crate clap;

use std::env;
use std::process::exit;

mod coda;
mod errors;
mod json;
mod options;
mod tools;
mod utils;

use coda::Coda;
use errors::*;
use options::*;

fn run() -> Result<()> {
    let options = Options::parse_options(env::args().collect())
        .map_err(|c| exit(c))
        .unwrap();
    let default_encoding = String::from("utf-8");
    let encoding_label = options.encoding_label.as_ref().unwrap_or(&default_encoding);
    let coda_list = options
        .coda_filenames
        .iter()
        .map(|f: &String| -> Result<Coda> {
            if options.debug {
                println!("Parsing file: {}", f);
            }
            Coda::parse(&f, encoding_label)
        }).collect::<Vec<_>>();

    let mut had_errors = false;
    coda_list.iter().by_ref().filter(|c| c.is_err()).for_each(|c| {
        println!("Error: {:?}", c);
        had_errors = true
    });

    if !had_errors {
        let mut coda_list = coda_list
            .into_iter()
            .filter(|coda| coda.is_ok())
            .map(|coda| coda.unwrap())
            .collect::<Vec<Coda>>();

        if options.sort_by_ref {
            coda_list.sort_by(|a, b| a.header.file_reference.cmp(&b.header.file_reference));
        }

        if options.command == Some(Command::Json) {
            tools::print_as_json_from_list(&coda_list).chain_err(|| "Error while printing json")?;
        }
        Ok(())
    } else {
        Err("Parsing ended with errors".into())
    }
}

quick_main!(run);
