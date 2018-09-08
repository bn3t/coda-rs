#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate ansi_term;
extern crate chrono;

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
use options::Options;

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
        })
        .collect::<Vec<_>>();

    let mut had_errors = false;
    coda_list.iter().by_ref().filter(|c| c.is_err()).for_each(|c| {
        println!("Error: {:?}", c);
        had_errors = true
    });

    if !had_errors {
        // println!("{:?}", coda_list);

        // if options.debug {
        //     coda_list.
        //     println!("header=[{:?}]", coda.header);
        //     println!("old_balance=[{:?}]", coda.old_balance);
        //     println!("movements=[{:?}]", coda.movements);

        //     println!("information=[{:?}]", coda.information);

        //     println!("free_communications=[{:?}]", coda.free_communications);
        //     println!("New balance: {:?}", coda.new_balance);
        //     println!("Trailer: {:?}", coda.trailer);
        // }

        let mut coda_list = coda_list
            .into_iter()
            .filter(|coda| coda.is_ok())
            .map(|coda| coda.unwrap())
            .collect::<Vec<Coda>>();

        if options.sort_by_ref {
            coda_list.sort_by(|a, b| a.header.file_reference.cmp(&b.header.file_reference));
        }

        if options.json {
            for coda in coda_list {
                tools::print_as_json(&coda).chain_err(|| "Error while printing json")?;
            }
        } else if options.list_summary || options.list_all {
            coda_list.iter().for_each(|coda| {
                if options.list_summary || options.list_all {
                    tools::print_header(&mut std::io::stdout(), &coda, options.colored);
                    tools::print_footer(&mut std::io::stdout(), &coda, options.colored);
                };
            });
        }
        Ok(())
    } else {
        Err("Parsing ended with errors".into())
    }
}

quick_main!(run);
