use coda::Coda;
use json;

use errors::*;

pub fn _print_as_json(coda: &Coda) -> Result<()> {
    let j = json::_to_json(coda).chain_err(|| "Could not make json")?;
    println!("{}", j);
    Ok(())
}

pub fn print_as_json_from_list(coda_list: &Vec<Coda>) -> Result<()> {
    let j = json::to_json_from_list(coda_list).chain_err(|| "Could not make json")?;
    println!("{}", j);
    Ok(())
}
