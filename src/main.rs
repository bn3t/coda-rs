#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

use std::process::exit;
use std::env;

mod coda;
mod options;
mod errors;
mod utils;
mod json;
mod tools;

use coda::Coda;
use options::Options;
use errors::*;

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
            Ok(Coda::parse(&f, encoding_label).chain_err(|| "Could not parse coda")?)
        })
        .collect::<Vec<_>>();
    //println!("{:?}", coda_list);

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
    if options.json {
        for coda in coda_list {
            if coda.is_ok() {
                tools::print_as_json(&coda.unwrap()).chain_err(|| "Error while printing json")?;
            }
        }
    }

    Ok(())
}

quick_main!(run);
