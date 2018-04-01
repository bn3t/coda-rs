extern crate chrono;

use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::NaiveDate;

use errors::*;
use utils::{parse_date, parse_duplicate, parse_field, parse_sign, parse_str, Sign, parse_u64,
            parse_u8};

#[derive(PartialEq, Debug)]
pub enum AccountStructure {
    BelgianAccountNumber,
    ForeignAccountNumber,
    IBANBelgianAccountNumber,
    IBANForeignAccountNumber,
}

#[allow(dead_code)]
pub struct OldBalance {
    pub account_structure: AccountStructure, // ': (slice(1, 2), str),
    pub old_sequence: String,                // ': (slice(2, 5), str),
    pub account_currency: String,            // ': (slice(5, 42), str),
    pub old_balance_sign: Sign,
    pub old_balance: u64,            // ': (slice(43, 58), _amount),
    pub old_balance_date: NaiveDate, // ': (slice(58, 64), _date),
    pub account_holder_name: String, // ': (slice(64, 90), _string),
    pub account_description: String, // ': (slice(90, 125), _string),
    pub coda_sequence: String,       // ': (slice(125, 128), str),
}

impl OldBalance {
    fn parse_accountstructure(s: &str) -> Result<AccountStructure> {
        match s {
            "0" => Ok(AccountStructure::BelgianAccountNumber),
            "1" => Ok(AccountStructure::ForeignAccountNumber),
            "2" => Ok(AccountStructure::IBANBelgianAccountNumber),
            "3" => Ok(AccountStructure::IBANForeignAccountNumber),
            _ => Err(format!("Invalid AccountStructure value [{}]", s).into()),
        }
    }

    pub fn parse(line: &str) -> Result<OldBalance> {
        Ok(OldBalance {
            account_structure: parse_field(line, 1..2, OldBalance::parse_accountstructure)
                .chain_err(|| "Could not parse account_structure")?,
            old_sequence: parse_field(line, 2..5, parse_str)
                .chain_err(|| "Could not parse old_sequence")?,
            account_currency: parse_field(line, 5..42, parse_str)
                .chain_err(|| "Could not parse account_currency")?,
            old_balance_sign: parse_field(line, 42..43, parse_sign)
                .chain_err(|| "Could not parse old_balance_sign")?,
            old_balance: parse_field(line, 43..58, parse_u64)
                .chain_err(|| "Could not parse old_balance")?,
            old_balance_date: parse_field(line, 58..64, parse_date)
                .chain_err(|| "Could not parse old_balance_date")?,
            account_holder_name: parse_field(line, 64..90, parse_str)
                .chain_err(|| "Could not parse account_holder_name")?,
            account_description: parse_field(line, 90..125, parse_str)
                .chain_err(|| "Could not parse account_description")?,
            coda_sequence: parse_field(line, 125..128, parse_str)
                .chain_err(|| "Could not parse coda_sequence")?,
        })
    }
}

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

/*
MOVE_COMMON = {
    'sequence': (slice(2, 6), str),
    'detail_sequence': (slice(6, 10), str),
    }
MOVE = {
    '1': {
        'bank_reference': (slice(10, 31), str),
        'amount': (slice(31, 47), _amount),
        'value_date': (slice(47, 53), _date),
        'transaction_code': (slice(53, 61), str),
        '_communication': (slice(61, 115), str),
        'entry_date': (slice(115, 121), _date),
        'statement_number': (slice(121, 124), str),
        },
    '2': {
        '_communication': (slice(10, 63), str),
        'customer_reference': (slice(63, 98), _string),
        'counterparty_bic': (slice(98, 109), _string),
        'r_transaction': (slice(112, 113), _string),
        'r_reason': (slice(113, 117), _string),
        'category_purpose': (slice(117, 121), _string),
        'purpose': (slice(121, 125), _string),
        },
    '3': {
        'counterparty_account': (slice(10, 47), _string),
        'counterparty_name': (slice(47, 82), _string),
        '_communication': (slice(82, 125), str),
        },
    }
*/

#[allow(dead_code)]
pub struct Movement1 {
    pub sequence: String,         //': (slice(2, 6), str),
    pub detail_sequence: String,  //': (slice(6, 10), str),
    pub bank_reference: String,   //': (slice(10, 31), str),
    pub amount: u64,              //': (slice(31, 47), _amount),
    pub value_date: NaiveDate,    //': (slice(47, 53), _date),
    pub transaction_code: String, //': (slice(53, 61), str),
    pub communication: String,    //': (slice(61, 115), str),
    pub entry_date: NaiveDate,    //': (slice(115, 121), _date),
    pub statement_number: String, //': (slice(121, 124), str),
}

impl Movement1 {
    fn parse(line: &str) -> Result<Movement1> {
        Ok(Movement1 {
            sequence: parse_field(line, 2..6, parse_str).chain_err(|| "Could not parse sequence")?,
            detail_sequence: parse_field(line, 6..10, parse_str)
                .chain_err(|| "Could not parse detail_sequence")?,
            bank_reference: parse_field(line, 10..31, parse_str)
                .chain_err(|| "Could not parse bank_reference")?,
            amount: parse_field(line, 31..47, parse_u64).chain_err(|| "Could not parse amount")?,
            value_date: parse_field(line, 47..53, parse_date)
                .chain_err(|| "Could not parse value_date")?,
            transaction_code: parse_field(line, 53..61, parse_str)
                .chain_err(|| "Could not parse transaction_code")?,
            communication: parse_field(line, 61..115, parse_str)
                .chain_err(|| "Could not parse transaction_code")?,
            entry_date: parse_field(line, 115..121, parse_date)
                .chain_err(|| "Could not parse entry_date")?,
            statement_number: parse_field(line, 121..124, parse_str)
                .chain_err(|| "Could not parse statement_number")?,
        })
    }
}

#[allow(dead_code)]
pub struct Coda {
    pub header: Header,
    pub old_balance: OldBalance,
    pub movement1: Movement1,
}

impl Coda {
    pub fn parse(coda_filename: &str) -> Result<Coda> {
        println!("Parsing file: {}", coda_filename);
        // This operation will fail
        let f =
            File::open(coda_filename).chain_err(|| format!("Unable to open {}", coda_filename))?;

        let reader = BufReader::new(f);
        let mut header: Option<Header> = None;
        let mut old_balance: Option<OldBalance> = None;
        let mut movement1: Option<Movement1> = None;
        for line in reader.lines() {
            let l = line.unwrap();
            let t: u8 = match l.get(0..1) {
                Some("0") => 0,
                Some("1") => 1,
                Some("2") => 2,
                _ => 255,
            };
            match t {
            0 => {
                header = Some(Header::parse(&l).chain_err(||->Error  {"Could not parse header".into()})?);
                //let header  = Header {};
                //coda.statements.push(statement);
            },
            1 => {
                old_balance = Some(OldBalance::parse(&l).chain_err(||->Error  {"Could not parse oldbalance".into()})?)
            },
            2 => {
                movement1 = Some(Movement1::parse(&l).chain_err(||->Error  {"Could not parse Movement1".into()})?)
            },
            _ => {}
            // _ => return Err("Unknown type".into()),
        }
        }
        if let Some(header) = header {
            if let Some(old_balance) = old_balance {
                if let Some(movement1) = movement1 {
                    return Ok(Coda {
                        header: header,
                        old_balance: old_balance,
                        movement1: movement1,
                    });
                }
            }
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

#[cfg(test)]
#[allow(non_snake_case)]
mod test_parse_oldbalance {
    use chrono::NaiveDate;

    use utils::Sign;
    use super::OldBalance;
    use super::AccountStructure;

    #[test]
    fn parse_oldbalance_valid() {
        let line = "10001435000000080 EUR0BE                  0000000000000000061206Testgebruiker21           KBC-Bedrijfsrekening               001";

        let actual = OldBalance::parse(line);

        assert_eq!(actual.is_ok(), true, "OldBalance shoud be ok");
        let actual = actual.unwrap();
        assert_eq!(actual.old_sequence, "001", "old_sequence should be '001'");
        assert_eq!(
            actual.account_structure,
            AccountStructure::BelgianAccountNumber,
            "account_structure should be BelgianAccountNumber"
        );
        assert_eq!(
            actual.account_currency,
            "435000000080 EUR0BE                  ",
            "account_currency should be '435000000080 EUR0BE                  '"
        );
        assert_eq!(
            actual.old_balance_sign,
            Sign::Credit,
            "old_balance_sign should be 'Credit'"
        );
        assert_eq!(actual.old_balance, 0, "old_balance should be ''");
        assert_eq!(
            actual.old_balance_date,
            NaiveDate::from_ymd(2006, 12, 06),
            "creation_date should be 06/12/2006"
        );
        assert_eq!(
            actual.account_holder_name,
            "Testgebruiker21           ",
            "account_currency should be 'Testgebruiker21           '"
        );
        assert_eq!(
            actual.account_description,
            "KBC-Bedrijfsrekening               ",
            "account_currency should be 'KBC-Bedrijfsrekening               '"
        );
        assert_eq!(
            actual.coda_sequence,
            "001",
            "account_currency should be '001'"
        );
    }

    #[test]
    fn parse_accountstructure_valid_BelgianAccountNumber() {
        let actual = OldBalance::parse_accountstructure("0");
        assert_eq!(actual.is_ok(), true, "'0' should be ok");
        assert_eq!(
            actual.unwrap(),
            AccountStructure::BelgianAccountNumber,
            "'0' should be BelgianAccountNumber"
        );
    }

    #[test]
    fn parse_accountstructure_valid_ForeignAccountNumber() {
        let actual = OldBalance::parse_accountstructure("1");
        assert_eq!(actual.is_ok(), true, "'1' should be ok");
        assert_eq!(
            actual.unwrap(),
            AccountStructure::ForeignAccountNumber,
            "'0' should be ForeignAccountNumber"
        );
    }

    #[test]
    fn parse_accountstructure_valid_IBANBelgianAccountNumber() {
        let actual = OldBalance::parse_accountstructure("2");
        assert_eq!(actual.is_ok(), true, "'2' should be ok");
        assert_eq!(
            actual.unwrap(),
            AccountStructure::IBANBelgianAccountNumber,
            "'0' should be IBANBelgianAccountNumber"
        );
    }

    #[test]
    fn parse_accountstructure_valid_IBANForeignAccountNumber() {
        let actual = OldBalance::parse_accountstructure("3");
        assert_eq!(actual.is_ok(), true, "'3' should be ok");
        assert_eq!(
            actual.unwrap(),
            AccountStructure::IBANForeignAccountNumber,
            "'0' should be IBANForeignAccountNumber"
        );
    }

    #[test]
    fn parse_accountstructure_valid_invalid() {
        let actual = OldBalance::parse_accountstructure("4");
        assert_eq!(actual.is_ok(), false, "'4' should not be ok");
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_parse_movement1 {
    use chrono::NaiveDate;

    use super::Movement1;

    #[test]
    fn parse_movement1_valid() {
        let line = "2100010000EPIB00048 AWIUBTKAPUO1000000002578250061206007990000BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D'OPERATI06120600111 0";

        let actual = Movement1::parse(line);

        assert_eq!(actual.is_ok(), true, "Movement1 shoud be ok");
        let actual = actual.unwrap();
        assert_eq!(actual.sequence, "0001", "sequence should be '0001'");
        assert_eq!(
            actual.detail_sequence,
            "0000",
            "detail_sequence should be '0000'"
        );
        assert_eq!(
            actual.bank_reference,
            "EPIB00048 AWIUBTKAPUO",
            "bank_reference should be 'EPIB00048 AWIUBTKAPUO'"
        );
        assert_eq!(
            actual.amount,
            1000000002578250,
            "amount should be '1000000002578250'"
        );
        assert_eq!(
            actual.value_date,
            NaiveDate::from_ymd(2006, 12, 6),
            "value_date should be '06/12/2006'"
        );
        assert_eq!(
            actual.transaction_code,
            "00799000",
            "bank_reference should be '00799000'"
        );
        assert_eq!(
            actual.communication,
            "0BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D\'OPERATI"
        );
        assert_eq!(actual.entry_date, NaiveDate::from_ymd(2006, 12, 6));
        assert_eq!(actual.statement_number, "001");
    }
}
