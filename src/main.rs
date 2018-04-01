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

    let coda = Coda::parse(&options.coda_filename).chain_err(|| "Could not parse coda")?;

    println!("creation_date=[{}]", coda.header.creation_date);
    println!("name_addressee=[{}]", coda.header.name_addressee);

    Ok(())
}

quick_main!(run);
