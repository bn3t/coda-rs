use coda::Coda;
use json;

use errors::*;

pub fn print_as_json(coda: &Coda) -> Result<()> {
    let j = json::to_json(coda).chain_err(|| "Could not make json")?;
    println!("{}", j);
    Ok(())
}
