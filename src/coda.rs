extern crate chrono;

use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::NaiveDate;

use errors::*;
use utils::{parse_date, parse_duplicate, parse_field, parse_str, parse_u8};

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

#[allow(dead_code)]
pub struct Header {
    pub creation_date: NaiveDate,
    pub bank_id: String,
    pub duplicate: bool,
    pub file_reference: String,
    pub name_addressee: String,
    pub bic: String,
    pub company_id: String,
    pub reference: String,
    pub related_reference: String,
    pub version: u8,
}

impl Header {
    pub fn parse(line: &str) -> Result<Header> {
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
            reference: parse_field(line, 88..104, parse_str)
                .chain_err(|| "Could not parse reference")?,
            related_reference: parse_field(line, 105..120, parse_str)
                .chain_err(|| "Could not parse related_reference")?,
            version: parse_field(line, 127..128, parse_u8).chain_err(|| "Could not parse version")?,
        })
    }
}

#[allow(dead_code)]
pub struct Coda {
    pub header: Header,
}

impl Coda {
    pub fn parse(coda_filename: &str) -> Result<Coda> {
        println!("Parsing file: {}", coda_filename);
        // This operation will fail
        let f =
            File::open(coda_filename).chain_err(|| format!("Unable to open {}", coda_filename))?;

        let reader = BufReader::new(f);
        let mut header: Option<Header> = None;
        for line in reader.lines() {
            let l = line.unwrap();
            let t: u8 = match l.get(0..1) {
                Some("0") => 0,
                _ => 255,
            };
            match t {
            0 => {
                header = Some(Header::parse(&l).chain_err(||->Error  {"Could not parse header".into()})?);
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
}

#[cfg(test)]
mod test_parse_header {
    use chrono::NaiveDate;

    use super::Header;

    #[test]
    fn parse_header_valid() {
        let line_header = "0000029031872505        00099449  Testgebruiker21           KREDBEBB   00630366277 00000                                       2";
        let actual = Header::parse(line_header);

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
}
