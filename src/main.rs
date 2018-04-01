extern crate argparse;
#[macro_use]
extern crate error_chain;

extern crate chrono;

use std::ops::Range;
use std::fs::File;
use argparse::{ArgumentParser, Print, Store, StoreTrue};
use std::io::{stderr, stdout, BufRead, BufReader};
use std::env;
use std::process::exit;

use chrono::NaiveDate;

struct Options {
    coda_filename: String,
    json: bool,
}

/*
HEADER = {
    'creation_date': (slice(5, 11), _date),
    'bank_id': (slice(11, 14), int),
    'duplicate': (slice(16, 17), lambda c: c == 'D'),
    'file_reference': (slice(24, 34), _string),
    'address': (slice(34, 60), _string),
    'bic': (slice(60, 71), _string),
    'company_id': (slice(71, 82), str),
    'reference': (slice(88, 104), _string),
    'related_reference': (slice(105, 120), _string),
    'version': (slice(127, 128), int),
    }
*/

struct Header {
    creation_date: NaiveDate,
    bank_id: String,
    duplicate: bool,
    file_reference: String,
    name_addressee: String,
    bic: String,
    company_id: String,
    reference: String,
    related_reference: String,
    version: u8,
}

#[allow(dead_code)]
struct Coda {
    header: Header,
}

mod errors {
    use chrono;
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        foreign_links {
            ParseError(chrono::format::ParseError);
        }
    }
}

use errors::*;

fn parse_options(args: Vec<String>) -> std::result::Result<Options, i32> {
    let mut options = Options {
        coda_filename: String::new(),
        json: false,
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Parse coda files");
        ap.refer(&mut options.json).add_option(
            &["-j", "--json"],
            StoreTrue,
            "Convert coda files to json",
        );
        ap.refer(&mut options.coda_filename)
            .add_argument("coda", Store, "Coda file to parse")
            .required();
        ap.add_option(
            &["-v", "--version"],
            Print(
                format!(
                    "{} {} ({} {})",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                    env!("GIT_COMMIT"),
                    env!("BUILD_DATE")
                ).to_string(),
            ),
            "Show version",
        );
        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(options)
}

fn parse_date(s: &str) -> Result<NaiveDate> {
    let date: NaiveDate =
        NaiveDate::parse_from_str(s, "%d%m%y").chain_err(|| "Could not parse date")?;

    Ok(date)
}

fn parse_str(s: &str) -> Result<String> {
    Ok(String::from(s))
}

fn parse_u8(s: &str) -> Result<u8> {
    Ok(s.parse::<u8>().chain_err(|| "Could not parse u8")?)
}

fn parse_duplicate(s: &str) -> Result<bool> {
    match s {
        "D" => Ok(true),
        " " => Ok(false),
        _ => Err(format!("Invalid duplicate value [{}]", s).into()),
    }
}

fn parse_field<T>(line: &str, range: Range<usize>, convert: fn(s: &str) -> Result<T>) -> Result<T> {
    if let Some(part) = line.get(range) {
        convert(part)
    } else {
        Err("Could not parse field".into())
    }
}

fn parse_header(line: &str) -> Result<Header> {
    Ok(Header {
        creation_date: parse_field(line, 5..11, parse_date)
            .chain_err(|| "Could not parse creation_date")?,
        bank_id: parse_field(line, 11..14, parse_str).chain_err(|| "Could not parse bank_id")?,
        duplicate: parse_field(line, 16..17, parse_duplicate)
            .chain_err(|| "Could not parse duplicate")?,
        file_reference: parse_field(line, 24..34, parse_str)
            .chain_err(|| "Could not parse file_reference")?,
        name_addressee: parse_field(line, 34..60, parse_str)
            .chain_err(|| "Could not parse name_addressee")?,
        bic: parse_field(line, 60..71, parse_str).chain_err(|| "Could not parse bic")?,
        company_id: parse_field(line, 71..82, parse_str)
            .chain_err(|| "Could not parse company_id")?,
        reference: parse_field(line, 88..104, parse_str).chain_err(|| "Could not parse reference")?,
        related_reference: parse_field(line, 105..120, parse_str)
            .chain_err(|| "Could not parse related_reference")?,
        version: parse_field(line, 127..128, parse_u8).chain_err(|| "Could not parse version")?,
    })
}

fn parse_coda(coda_filename: &str) -> Result<Coda> {
    println!("Parsing file: {}", coda_filename);
    // This operation will fail
    let f = File::open(coda_filename).chain_err(|| format!("Unable to open {}", coda_filename))?;

    let reader = BufReader::new(f);
    let mut header: Option<Header> = None;
    for line in reader.lines() {
        let l = line.unwrap();
        let t: u8 = match l.get(0..1) {
            Some("0") => 0,
            _ => 255,
        };
        println!("t=[{}]", l.get(0..1).unwrap());
        println!("t={}", t);
        match t {
            0 => {
                header = Some(parse_header(&l).chain_err(||->Error  {"Could not parse header".into()})?);
                //let header  = Header {};
                //coda.statements.push(statement);
            },
            _ => {}
            // _ => return Err("Unknown type".into()),
        }
    }
    if let Some(header) = header {
        return Ok(Coda { header: header });
    }
    Err("Could not parse code".into())
}

fn run() -> Result<()> {
    let options = parse_options(env::args().collect())
        .map_err(|c| exit(c))
        .unwrap();

    parse_coda(&options.coda_filename).chain_err(|| "Could not parse coda")?;

    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test_parse_header {
    use chrono::NaiveDate;

    use super::parse_header;
    use super::parse_field;
    use super::parse_date;
    use super::parse_str;
    use super::parse_duplicate;
    use super::parse_u8;

    #[test]
    fn parse_header_valid() {
        let line_header = "0000029031872505        00099449  Testgebruiker21           KREDBEBB   00630366277 00000                                       2";
        let actual = parse_header(line_header);

        assert_eq!(actual.is_ok(), true, "Returned header should be ok");
        let actual = actual.unwrap();

        assert_eq!(
            actual.creation_date,
            NaiveDate::from_ymd(2018, 3, 29),
            "creation_date should be 29/03/2018"
        );
        assert_eq!(
            actual.bank_id,
            String::from("725"),
            "bank_id should be 72505"
        );
        assert_eq!(actual.duplicate, false, "duplicate should be false");
        assert_eq!(
            actual.file_reference,
            "00099449  ",
            "File reference should be '00099449  '"
        );
        assert_eq!(
            actual.name_addressee,
            "Testgebruiker21           ",
            "address should be 'Testgebruiker21           '"
        );
        assert_eq!(actual.bic, "KREDBEBB   ", "bic should be 'KREDBEBB   '");
        assert_eq!(
            actual.company_id,
            "00630366277",
            "company_id should be '00630366277'"
        );
        assert_eq!(
            actual.reference,
            "                ",
            "reference should be '                '"
        );
        assert_eq!(
            actual.related_reference,
            "               ",
            "related_reference should be '               '"
        );
        assert_eq!(actual.version, 2, "version should be '2'");
    }

    #[test]
    fn parse_date_valid() {
        let actual = parse_date("290318");

        assert_eq!(actual.is_ok(), true, "Date should be ok");
        assert_eq!(
            actual.unwrap(),
            NaiveDate::from_ymd(2018, 3, 29),
            "creation_date should be 29/03/2018"
        )
    }

    #[test]
    fn parse_str_valid() {
        let actual = parse_str("05505");

        assert_eq!(actual.is_ok(), true, "String should be ok");
        assert_eq!(
            actual.unwrap(),
            String::from("05505"),
            "String should be 05505"
        );
    }

    #[test]
    fn parse_duplicate_valid_true() {
        let actual = parse_duplicate("D");

        assert_eq!(actual.is_ok(), true, "Duplicate 'D' should be ok");
        assert_eq!(actual.unwrap(), true, "Duplicate 'D' should be true");
    }

    #[test]
    fn parse_duplicate_valid_false() {
        let actual = parse_duplicate(" ");

        assert_eq!(actual.is_ok(), true, "Duplicate ' ' should be ok");
        assert_eq!(actual.unwrap(), false, "Duplicate ' ' should be false");
    }

    #[test]
    fn parse_duplicate_invalid() {
        let actual = parse_duplicate("B");

        assert_eq!(actual.is_ok(), false, "Duplicate 'B' should not be ok");
    }

    #[test]
    fn parse_u8_valid() {
        let actual = parse_u8("2");

        assert_eq!(actual.is_ok(), true, "u8 '2' should be ok");
        assert_eq!(actual.unwrap(), 2, "u8 '2' should be 2");
    }

    #[test]
    fn parse_u8_invalid() {
        let actual = parse_u8("200000");

        assert_eq!(actual.is_ok(), false, "u8 '200000' should be ok");
    }

    #[test]
    fn parse_field_valid() {
        let line_header = "0000029031872505        00099449  Testgebruiker21           KREDBEBB   00630366277 00000                                       2";
        // let range: Range<usize> = 5..11;
        let actual = parse_field(line_header, 5..11, parse_date);
        assert_eq!(actual.is_ok(), true, "Date should be ok");
        assert_eq!(
            actual.unwrap(),
            NaiveDate::from_ymd(2018, 3, 29),
            "creation_date should be 29/03/2018"
        )
    }
}
