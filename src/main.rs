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

    println!("header creation_date=[{}]", coda.header.creation_date);
    println!("header name_addressee=[{}]", coda.header.name_addressee);
    println!(
        "oldbalance account_currency=[{}]",
        coda.old_balance.account_currency
    );
    println!(
        "movement coda.movements[0].amount=[{}]",
        coda.movements[0].amount
    );
    println!("movements  coda.movements.len()=[{}]", coda.movements.len());
    println!(
        "movement coda.movements[0].counterparty_account=[{:?}]",
        coda.movements[0].counterparty_account
    );

    println!(
        "information  coda.informations.len()=[{}]",
        coda.information.len()
    );
    println!(
        "information coda.information[0].commmunication=[{:?}]",
        coda.information[0].communication
    );
    Ok(())
}

quick_main!(run);
