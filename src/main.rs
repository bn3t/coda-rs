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

use coda::Coda;
use options::Options;
use errors::*;

fn run() -> Result<()> {
    let options = Options::parse_options(env::args().collect())
        .map_err(|c| exit(c))
        .unwrap();

    let coda = Coda::parse(&options.coda_filename, options.encoding_label)
        .chain_err(|| "Could not parse coda")?;

    if options.debug {
        println!("header=[{:?}]", coda.header);
        println!("old_balance=[{:?}]", coda.old_balance);
        println!("movements=[{:?}]", coda.movements);

        println!("information=[{:?}]", coda.information);

        println!("free_communications=[{:?}]", coda.free_communications);
        println!("New balance: {:?}", coda.new_balance);
        println!("Trailer: {:?}", coda.trailer);
    }
    if options.json {
        let j = json::to_json(&coda).chain_err(|| "Could not make json")?;
        println!("{}", j);
    }

    Ok(())
}

quick_main!(run);
