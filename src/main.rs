#[macro_use]
extern crate error_chain;

extern crate chrono;

use std::process::exit;
use std::env;

mod coda;
mod options;
mod errors;
mod utils;

use coda::Coda;
use options::Options;
use errors::*;

fn run() -> Result<()> {
    let options = Options::parse_options(env::args().collect())
        .map_err(|c| exit(c))
        .unwrap();

    let coda = Coda::parse(&options.coda_filename, options.encoding_label)
        .chain_err(|| "Could not parse coda")?;

    println!("header=[{:?}]", coda.header);
    println!("old_balance=[{:?}]", coda.old_balance);
    println!("movements=[{:?}]", coda.movements);

    println!("information=[{:?}]", coda.information);

    println!("free_communications=[{:?}]", coda.free_communications);
    println!("New balance: {:?}", coda.new_balance);
    println!("Trailer: {:?}", coda.trailer);

    Ok(())
}

quick_main!(run);
