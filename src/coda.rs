extern crate chrono;
extern crate encoding;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read};

use chrono::NaiveDate;

use coda::encoding::label::encoding_from_whatwg_label;
use coda::encoding::DecoderTrap;
use json::date_serde;

use errors::*;
use utils::{parse_date, parse_duplicate, parse_field, parse_sign, parse_str, parse_str_append, parse_str_trim, Sign,
            StringUtils, parse_u32, parse_u64, parse_u8};

#[derive(PartialEq, Debug, Serialize)]
pub enum Account {
    BelgianAccountNumber {
        number: String,
        currency: String,
        country: String,
    },
    ForeignAccountNumber {
        number: String,
        currency: String,
    },
    IBANBelgianAccountNumber {
        number: String,
        currency: String,
    },
    IBANForeignAccountNumber {
        number: String,
        currency: String,
    },
}

#[derive(PartialEq, Debug, Serialize)]
pub enum CommunicationStructure {
    Structured,
    Unstructured,
}

fn parse_account(s: &str) -> Result<Account> {
    match s.get(0..1).unwrap() {
        "0" => Ok(Account::BelgianAccountNumber {
            number: String::from(s.get(4..16).unwrap().trim_right()),
            currency: String::from(s.get(17..20).unwrap()),
            country: String::from(s.get(21..23).unwrap()),
        }),
        "1" => Ok(Account::ForeignAccountNumber {
            number: String::from(s.get(4..38).unwrap().trim_right()),
            currency: String::from(s.get(38..41).unwrap()),
        }),
        "2" => Ok(Account::IBANBelgianAccountNumber {
            number: String::from(s.get(4..35).unwrap().trim_right()),
            currency: String::from(s.get(38..41).unwrap()),
        }),
        "3" => Ok(Account::IBANForeignAccountNumber {
            number: String::from(s.get(4..38).unwrap().trim_right()),
            currency: String::from(s.get(38..41).unwrap()),
        }),
        _ => Err(format!("Invalid AccountStructure value [{}]", s).into()),
    }
}

fn parse_communicationstructure(s: &str) -> Result<CommunicationStructure> {
    match s {
        "0" => Ok(CommunicationStructure::Unstructured),
        "1" => Ok(CommunicationStructure::Structured),
        _ => Err(format!("Invalid CommunicationStructure value [{}]", s).into()),
    }
}

#[derive(Debug, Serialize)]
pub struct Coda {
    pub header: Header,
    pub old_balance: OldBalance,
    pub movements: Vec<Movement>,
    pub information: Vec<Information>,
    pub free_communications: Vec<FreeCommunication>,
    pub new_balance: NewBalance,
    pub trailer: Trailer,
}

#[derive(Debug, Serialize)]
pub struct Header {
    #[serde(with = "date_serde")] pub creation_date: NaiveDate,
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

#[derive(Debug, Serialize)]
pub struct OldBalance {
    pub account: Account,     // ': (slice(1, 2), str),
    pub old_sequence: String, // ': (slice(2, 5), str),
    // pub account_currency: String, // ': (slice(5, 42), str),
    pub old_balance_sign: Sign,
    pub old_balance: u64,                                          // ': (slice(43, 58), _amount),
    #[serde(with = "date_serde")] pub old_balance_date: NaiveDate, // ': (slice(58, 64), _date),
    pub account_holder_name: String,                               // ': (slice(64, 90), _string),
    pub account_description: String,                               // ': (slice(90, 125), _string),
    pub coda_sequence: String,                                     // ': (slice(125, 128), str),
}

#[derive(Debug, Serialize)]
pub struct Movement {
    pub sequence: String,                                    //': (slice(2, 6), str),
    pub detail_sequence: String,                             //': (slice(6, 10), str),
    pub bank_reference: String,                              //': (slice(10, 31), str),
    pub amount: u64,                                         //': (slice(31, 47), _amount),
    #[serde(with = "date_serde")] pub value_date: NaiveDate, //': (slice(47, 53), _date),
    pub transaction_code: String,                            //': (slice(53, 61), str),
    pub communication: String,                               //': (slice(61, 115), str),
    #[serde(with = "date_serde")] pub entry_date: NaiveDate, //': (slice(115, 121), _date),
    pub statement_number: String,                            //': (slice(121, 124), str),
    // type 2
    //pub _communication: String,     //': (slice(10, 63), str),
    pub customer_reference: Option<String>, //': (slice(63, 98), _string),
    pub counterparty_bic: Option<String>,   //': (slice(98, 109), _string),
    pub r_transaction: Option<String>,      //': (slice(112, 113), _string),
    pub r_reason: Option<String>,           //': (slice(113, 117), _string),
    pub category_purpose: Option<String>,   //': (slice(117, 121), _string),
    pub purpose: Option<String>,            //': (slice(121, 125), _string),
    // type 3
    pub counterparty_account: Option<String>, //': (slice(10, 47), _string),
    pub counterparty_name: Option<String>,    //': (slice(47, 82), _string),
}

#[derive(Debug, Serialize)]
pub struct Information {
    pub sequence: String,         //': (slice(2, 6), str),
    pub detail_sequence: String,  //': (slice(6, 10), str),
    pub bank_reference: String,   //': (slice(10, 31), str),
    pub transaction_code: String, //': (slice(31, 39), str),
    pub communication_structure: CommunicationStructure,
    pub communication: String, //': (slice(39, 113), str),
}

#[derive(Debug, Serialize)]
pub struct FreeCommunication {
    pub sequence: String,        //': (slice(2, 6), str),
    pub detail_sequence: String, //': (slice(6, 10), str),
    pub text: String,            //': (slice(32, 112), str),
}

#[derive(Debug, Serialize)]
pub struct NewBalance {
    pub new_sequence: String, //': (slice(1, 4), str),
    // We don't store the account coming from the new balance
    pub new_balance_sign: Sign,
    pub new_balance: u64,                                          //': (slice(41, 57), _amount),
    #[serde(with = "date_serde")] pub new_balance_date: NaiveDate, //': (slice(57, 63), _date),
}

#[derive(Debug, Serialize)]
pub struct Trailer {
    pub number_records: u32, //': (slice(16, 22), int),
    pub total_debit: u64,    //': (slice(22, 37), _amount),
    pub total_credit: u64,   //': (slice(37, 52), _amount),
}

impl Trailer {
    pub fn parse(line: &str) -> Result<Trailer> {
        Ok(Trailer {
            number_records: parse_field(line, 16..22, parse_u32).chain_err(|| "Could not parse old_balance")?,
            total_debit: parse_field(line, 22..37, parse_u64).chain_err(|| "Could not parse old_balance")?,
            total_credit: parse_field(line, 37..52, parse_u64).chain_err(|| "Could not parse old_balance")?,
        })
    }
}

impl OldBalance {
    pub fn parse(line: &str) -> Result<OldBalance> {
        Ok(OldBalance {
            account: parse_field(line, 1..42, parse_account).chain_err(|| "Could not parse account_structure")?,
            old_sequence: parse_field(line, 2..5, parse_str).chain_err(|| "Could not parse old_sequence")?,
            old_balance_sign: parse_field(line, 42..43, parse_sign).chain_err(|| "Could not parse old_balance_sign")?,
            old_balance: parse_field(line, 43..58, parse_u64).chain_err(|| "Could not parse old_balance")?,
            old_balance_date: parse_field(line, 58..64, parse_date).chain_err(|| "Could not parse old_balance_date")?,
            account_holder_name: parse_field(line, 64..90, parse_str_trim)
                .chain_err(|| "Could not parse account_holder_name")?,
            account_description: parse_field(line, 90..125, parse_str_trim)
                .chain_err(|| "Could not parse account_description")?,
            coda_sequence: parse_field(line, 125..128, parse_str).chain_err(|| "Could not parse coda_sequence")?,
        })
    }
}

impl Header {
    pub fn parse(line: &str) -> Result<Header> {
        Ok(Header {
            creation_date: parse_field(line, 5..11, parse_date).chain_err(|| "Could not parse creation_date")?,
            bank_id: parse_field(line, 11..14, parse_str).chain_err(|| "Could not parse bank_id")?,
            duplicate: parse_field(line, 16..17, parse_duplicate).chain_err(|| "Could not parse duplicate")?,
            file_reference: parse_field(line, 24..34, parse_str).chain_err(|| "Could not parse file_reference")?,
            name_addressee: parse_field(line, 34..60, parse_str_trim).chain_err(|| "Could not parse name_addressee")?,
            bic: parse_field(line, 60..71, parse_str_trim).chain_err(|| "Could not parse bic")?,
            company_id: parse_field(line, 71..82, parse_str).chain_err(|| "Could not parse company_id")?,
            reference: parse_field(line, 88..104, parse_str_trim).chain_err(|| "Could not parse reference")?,
            related_reference: parse_field(line, 105..120, parse_str_trim)
                .chain_err(|| "Could not parse related_reference")?,
            version: parse_field(line, 127..128, parse_u8).chain_err(|| "Could not parse version")?,
        })
    }
}

impl Movement {
    fn parse_type1(line: &str) -> Result<Movement> {
        Ok(Movement {
            sequence: parse_field(line, 2..6, parse_str).chain_err(|| "Could not parse sequence")?,
            detail_sequence: parse_field(line, 6..10, parse_str).chain_err(|| "Could not parse detail_sequence")?,
            bank_reference: parse_field(line, 10..31, parse_str).chain_err(|| "Could not parse bank_reference")?,
            amount: parse_field(line, 31..47, parse_u64).chain_err(|| "Could not parse amount")?,
            value_date: parse_field(line, 47..53, parse_date).chain_err(|| "Could not parse value_date")?,
            transaction_code: parse_field(line, 53..61, parse_str).chain_err(|| "Could not parse transaction_code")?,
            communication: parse_field(line, 62..115, parse_str_trim).chain_err(|| "Could not parse transaction_code")?,
            entry_date: parse_field(line, 115..121, parse_date).chain_err(|| "Could not parse entry_date")?,
            statement_number: parse_field(line, 121..124, parse_str).chain_err(|| "Could not parse statement_number")?,
            customer_reference: None,
            counterparty_bic: None,
            r_transaction: None,
            r_reason: None,
            category_purpose: None,
            purpose: None,
            counterparty_account: None,
            counterparty_name: None,
        })
    }

    pub fn parse_type2(&mut self, line: &str) -> Result<()> {
        self.customer_reference =
            Some(parse_field(line, 121..124, parse_str_trim).chain_err(|| "Could not parse customer_reference")?);
        self.counterparty_bic =
            Some(parse_field(line, 98..109, parse_str_trim).chain_err(|| "Could not parse counterparty_bic")?);
        self.r_transaction =
            Some(parse_field(line, 112..113, parse_str_trim).chain_err(|| "Could not parse r_transaction")?);
        self.r_reason = Some(parse_field(line, 113..117, parse_str_trim).chain_err(|| "Could not parse r_reason")?);
        self.category_purpose =
            Some(parse_field(line, 117..121, parse_str_trim).chain_err(|| "Could not parse category_purpose")?);
        self.purpose = Some(parse_field(line, 121..125, parse_str_trim).chain_err(|| "Could not parse purpose")?);

        let communication = parse_field(line, 10..63, parse_str_append).chain_err(|| "Could not parse communication")?;
        self.communication.push_str(&communication);

        Ok(())
    }

    pub fn parse_type3(&mut self, line: &str) -> Result<()> {
        self.counterparty_name =
            Some(parse_field(line, 10..47, parse_str_trim).chain_err(|| "Could not parse counterparty_name")?);
        self.counterparty_account =
            Some(parse_field(line, 47..82, parse_str_trim).chain_err(|| "Could not parse counterparty_account")?);

        let communication = parse_field(line, 82..125, parse_str_append).chain_err(|| "Could not parse communication")?;
        self.communication.push_str(&communication);

        Ok(())
    }
}

impl Information {
    fn parse_type1(line: &str) -> Result<Information> {
        Ok(Information {
            sequence: parse_field(line, 2..6, parse_str).chain_err(|| "Could not parse sequence")?,
            detail_sequence: parse_field(line, 6..10, parse_str).chain_err(|| "Could not parse detail_sequence")?,
            bank_reference: parse_field(line, 10..31, parse_str).chain_err(|| "Could not parse detail_sequence")?,
            transaction_code: parse_field(line, 31..39, parse_str).chain_err(|| "Could not parse detail_sequence")?,
            communication_structure: parse_field(line, 39..40, parse_communicationstructure)
                .chain_err(|| "Could not parse communication_structure")?,
            communication: parse_field(line, 40..113, parse_str_trim).chain_err(|| "Could not parse detail_sequence")?,
        })
    }

    pub fn parse_type2(&mut self, line: &str) -> Result<()> {
        let communication = parse_field(line, 10..115, parse_str_append).chain_err(|| "Could not parse communication")?;
        self.communication.push_str(&communication);

        Ok(())
    }

    pub fn parse_type3(&mut self, line: &str) -> Result<()> {
        let communication = parse_field(line, 10..100, parse_str_append).chain_err(|| "Could not parse communication")?;
        self.communication.push_str(&communication);

        Ok(())
    }
}

impl FreeCommunication {
    pub fn parse_line1(line: &str) -> Result<FreeCommunication> {
        Ok(FreeCommunication {
            sequence: parse_field(line, 2..6, parse_str).chain_err(|| "Could not parse sequence")?,
            detail_sequence: parse_field(line, 6..10, parse_str).chain_err(|| "Could not parse detail_sequence")?,
            text: parse_field(line, 32..112, parse_str_trim).chain_err(|| "Could not parse text")?,
        })
    }

    pub fn parse_following(&mut self, line: &str) -> Result<()> {
        let text = parse_field(line, 32..112, parse_str_append).chain_err(|| "Could not parse text")?;
        self.text.push_str(&text);

        Ok(())
    }
}

impl NewBalance {
    fn parse(line: &str) -> Result<NewBalance> {
        Ok(NewBalance {
            new_sequence: parse_field(line, 1..4, parse_str).chain_err(|| "Could not parse new_sequence")?,
            new_balance_sign: parse_field(line, 42..43, parse_sign).chain_err(|| "Could not parse old_balance_sign")?,
            new_balance: parse_field(line, 41..57, parse_u64).chain_err(|| "Could not parse new_balance")?,
            new_balance_date: parse_field(line, 57..63, parse_date).chain_err(|| "Could not parse new_balance_date")?,
        })
    }
}

impl Coda {
    pub fn parse(coda_filename: &str, encoding_label: &str) -> Result<Coda> {
        let f = File::open(coda_filename).chain_err(|| format!("Unable to open {}", coda_filename))?;

        let encoding = encoding_from_whatwg_label(encoding_label).unwrap();

        let mut reader = BufReader::new(f);
        let mut buf = Vec::new();

        reader
            .read_to_end(&mut buf)
            .chain_err(|| "Error reading into buffer")?;

        let decoded = encoding.decode(&buf, DecoderTrap::Strict).unwrap();
        let cursor = Cursor::new(decoded);

        let mut header: Option<Header> = None;
        let mut old_balance: Option<OldBalance> = None;
        let mut new_balance: Option<NewBalance> = None;
        let mut trailer: Option<Trailer> = None;
        let mut movements: Vec<Movement> = Vec::new();
        let mut informations: Vec<Information> = Vec::new();
        let mut free_communications: Vec<FreeCommunication> = Vec::new();
        for (num, line) in cursor.lines().enumerate() {
            let line = line.unwrap();
            match line.get_range(0..1).as_str() {
                "0" => {
                    header = Some(Header::parse(&line).chain_err(|| -> Error { "Could not parse header".into() })?);
                }
                "1" => {
                    old_balance =
                        Some(OldBalance::parse(&line).chain_err(|| -> Error { "Could not parse oldbalance".into() })?)
                }
                "2" => match line.get_range(1..2).as_str() {
                    "1" => {
                        let movement = Some(Movement::parse_type1(&line)
                            .chain_err(|| -> Error { format!("Could not parse Movement (line {})", num + 1).into() })?);
                        movements.push(movement.unwrap());
                    }
                    "2" => {
                        let movement = movements.last_mut();
                        let mut movement = movement.unwrap();
                        movement
                            .parse_type2(&line)
                            .chain_err(|| "Error parsing movement type 2")?;
                    }
                    "3" => {
                        let movement = movements.last_mut();
                        let mut movement = movement.unwrap();
                        movement
                            .parse_type3(&line)
                            .chain_err(|| "Error parsing movement type 3")?;
                    }
                    _ => {}
                },
                "3" => match line.get_range(1..2).as_str() {
                    "1" => {
                        let information = Some(Information::parse_type1(&line)
                            .chain_err(|| -> Error { "Could not parse Information".into() })?);
                        informations.push(information.unwrap());
                    }
                    "2" => {
                        let information = informations.last_mut();
                        let mut information = information.unwrap();
                        information
                            .parse_type2(&line)
                            .chain_err(|| "Error parsing information type 2")?;
                    }
                    "3" => {
                        let information = informations.last_mut();
                        let mut information = information.unwrap();
                        information
                            .parse_type3(&line)
                            .chain_err(|| "Error parsing information type 3")?;
                    }
                    _ => {}
                },
                "4" => match line.get_range(6..10).as_str() {
                    "0000" => {
                        let free_communication = Some(FreeCommunication::parse_line1(&line)
                            .chain_err(|| -> Error { "Could not parse FreeCommunication".into() })?);
                        free_communications.push(free_communication.unwrap());
                    }
                    _ => {
                        let free_communication = free_communications.last_mut().unwrap();
                        free_communication
                            .parse_following(&line)
                            .chain_err(|| "Error parsing FreeCommunication following lines")?;
                    }
                },
                "8" => {
                    new_balance = Some(NewBalance::parse(&line).chain_err(|| "Could not parse NewBalance")?);
                }
                "9" => {
                    trailer = Some(Trailer::parse(&line).chain_err(|| "Could not parse Trailer")?);
                }
                _ => {}
            };
        }
        if header.is_some() && old_balance.is_some() && old_balance.is_some() && new_balance.is_some()
            && trailer.is_some()
        {
            Ok(Coda {
                header: header.unwrap(),
                old_balance: old_balance.unwrap(),
                movements: movements,
                information: informations,
                free_communications: free_communications,
                new_balance: new_balance.unwrap(),
                trailer: trailer.unwrap(),
            })
        } else {
            Err("Could not parse coda - Missing parts".into())
        }
    }
}

#[cfg(test)]
mod test_parse_coda {
    use super::*;

    #[test]
    fn parse_coda_valid() {
        let coda = Coda::parse("test-data/CODA.txt", "latin1");

        assert_eq!(coda.is_ok(), true, "CODA.txt should be ok");
    }

    #[test]
    fn parse_coda_invalid() {
        let coda = Coda::parse("test-data/CODA-bad.txt", "latin1");

        assert_eq!(coda.is_ok(), false, "CODA-bad.txt should not be ok");
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
            "Testgebruiker21",
            // // "address should be 'Testgebruiker21'"
        );
        assert_eq!(actual.bic, "KREDBEBB", "bic should be 'KREDBEBB'");
        assert_eq!(
            actual.company_id,
            "00630366277",
            "company_id should be '00630366277'"
        );
        assert_eq!(actual.reference, "", "reference should be ''");
        assert_eq!(
            actual.related_reference,
            "",
            "related_reference should be ''"
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
    use super::Account;

    #[test]
    fn parse_oldbalance_valid() {
        let line = "10001435000000080 EUR0BE                  0000000000000000061206Testgebruiker21           KBC-Bedrijfsrekening               001";

        let actual = OldBalance::parse(line);

        assert_eq!(actual.is_ok(), true, "OldBalance shoud be ok");
        let actual = actual.unwrap();
        assert_eq!(actual.old_sequence, "001", "old_sequence should be '001'");
        assert_eq!(
            actual.account,
            Account::BelgianAccountNumber {
                number: String::from("435000000080"),
                currency: String::from("EUR"),
                country: String::from("BE"),
            },
            "account_structure should be BelgianAccountNumber"
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
            "Testgebruiker21",
            "account_currency should be 'Testgebruiker21'"
        );
        assert_eq!(
            actual.account_description,
            "KBC-Bedrijfsrekening",
            "account_currency should be 'KBC-Bedrijfsrekening'"
        );
        assert_eq!(
            actual.coda_sequence,
            "001",
            "account_currency should be '001'"
        );
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_parse_account {
    use super::Account;
    use super::parse_account;

    #[test]
    fn parse_account_valid_BelgianAccountNumber() {
        let actual = parse_account("0xxx435000000080 EUR0BE                  ");
        assert_eq!(actual.is_ok(), true, "'0' should be ok");
        assert_eq!(
            actual.unwrap(),
            Account::BelgianAccountNumber {
                number: String::from("435000000080"),
                currency: String::from("EUR"),
                country: String::from("BE"),
            },
            "'0' should be BelgianAccountNumber"
        );
    }

    #[test]
    fn parse_account_valid_ForeignAccountNumber() {
        let actual = parse_account("1xxx1234567890123456789012345678901234EUR");
        assert_eq!(actual.is_ok(), true, "'1' should be ok");
        assert_eq!(
            actual.unwrap(),
            Account::ForeignAccountNumber {
                number: String::from("1234567890123456789012345678901234"),
                currency: String::from("EUR"),
            },
            "'0' should be ForeignAccountNumber"
        );
    }

    #[test]
    fn parse_account_valid_IBANBelgianAccountNumber() {
        let actual = parse_account("2xxx1234567890123456789012345678901xxxEUR");
        assert_eq!(actual.is_ok(), true, "'2' should be ok");
        assert_eq!(
            actual.unwrap(),
            Account::IBANBelgianAccountNumber {
                number: String::from("1234567890123456789012345678901"),
                currency: String::from("EUR"),
            },
            "'0' should be IBANBelgianAccountNumber"
        );
    }

    #[test]
    fn parse_account_valid_IBANForeignAccountNumber() {
        let actual = parse_account("3xxx1234567890123456789012345678901234EUR");
        assert_eq!(actual.is_ok(), true, "'3' should be ok");
        assert_eq!(
            actual.unwrap(),
            Account::IBANForeignAccountNumber {
                number: String::from("1234567890123456789012345678901234"),
                currency: String::from("EUR"),
            },
            "'3' should be IBANForeignAccountNumber"
        );
    }

    #[test]
    fn parse_accountstructure_valid_invalid() {
        let actual = parse_account("4BLAH");
        assert_eq!(actual.is_ok(), false, "'4' should not be ok");
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_parse_newbalance {
    use chrono::NaiveDate;

    use utils::Sign;
    use super::NewBalance;

    #[test]
    fn parse_newbalance_valid() {
        let line = "8001435000000080 EUR0BE                  0000009405296990071206                                                                0";

        let actual = NewBalance::parse(line);

        assert_eq!(actual.is_ok(), true, "NewBalance shoud be ok");
        let actual = actual.unwrap();
        assert_eq!(actual.new_sequence, "001", "old_sequence should be '001'");
        assert_eq!(
            actual.new_balance_sign,
            Sign::Credit,
            "new_balance_sign should be 'Credit'"
        );
        assert_eq!(
            actual.new_balance,
            9405296990,
            "new_balance should be '9405296990'"
        );
        assert_eq!(
            actual.new_balance_date,
            NaiveDate::from_ymd(2006, 12, 07),
            "new_balance_date should be 07/12/2006"
        );
    }
}

#[cfg(test)]
mod test_parse_trailer {
    use super::Trailer;

    #[test]
    fn parse_newbalance_valid() {
        let line = "9               000260000003085871600000012491168590                                                                           2";

        let actual = Trailer::parse(line);

        assert_eq!(actual.is_ok(), true, "Trailer shoud be ok");
        let actual = actual.unwrap();
        assert_eq!(actual.number_records, 260, "number_records should be '260'");
        assert_eq!(
            actual.total_debit,
            3085871600,
            "total_debit should be '3085871600'"
        );
        assert_eq!(
            actual.total_credit,
            12491168590,
            "total_credit should be '12491168590'"
        );
    }
}

#[cfg(test)]
mod test_parse_movement {

    use chrono::NaiveDate;

    use super::Movement;

    #[test]
    fn parse_movement_type1_valid() {
        let line = "2100010000EPIB00048 AWIUBTKAPUO1000000002578250061206007990000BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D'OPERATI06120600111 0";

        let actual = Movement::parse_type1(line);

        assert_eq!(actual.is_ok(), true, "Movement shoud be ok");
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
            "BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D\'OPERATI"
        );
        assert_eq!(actual.entry_date, NaiveDate::from_ymd(2006, 12, 6));
        assert_eq!(actual.statement_number, "001");
    }

    #[test]
    fn parse_movement_type1_other_valid() {
        let line = "2100010000080072N026408        1000000002400000260218001030000Rénumération                                         26021801001 0";

        let actual = Movement::parse_type1(line);

        assert_eq!(actual.is_ok(), true, "Movement shoud be ok");
        // let actual = actual.unwrap();
        // assert_eq!(actual.sequence, "0001", "sequence should be '0001'");
        // assert_eq!(
        //     actual.detail_sequence,
        //     "0000",
        //     "detail_sequence should be '0000'"
        // );
        // assert_eq!(
        //     actual.bank_reference,
        //     "EPIB00048 AWIUBTKAPUO",
        //     "bank_reference should be 'EPIB00048 AWIUBTKAPUO'"
        // );
        // assert_eq!(
        //     actual.amount,
        //     1000000002578250,
        //     "amount should be '1000000002578250'"
        // );
        // assert_eq!(
        //     actual.value_date,
        //     NaiveDate::from_ymd(2006, 12, 6),
        //     "value_date should be '06/12/2006'"
        // );
        // assert_eq!(
        //     actual.transaction_code,
        //     "00799000",
        //     "bank_reference should be '00799000'"
        // );
        // assert_eq!(
        //     actual.communication,
        //     "BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D\'OPERATI"
        // );
        // assert_eq!(actual.entry_date, NaiveDate::from_ymd(2006, 12, 6));
        // assert_eq!(actual.statement_number, "001");
    }

    #[test]
    fn parse_movement_type2_valid() {
        let line1 = "2100010000EPIB00048 AWIUBTKAPUO1000000002578250061206007990000BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D'OPERATI06120600111 0";
        let line2 = "2200010000ON 495953                                                                                                          0 0";

        let actual = Movement::parse_type1(line1);
        let mut actual = actual.unwrap();
        let result = actual.parse_type2(line2);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            actual.customer_reference.is_some(),
            true,
            "customer_reference should be there"
        );
        assert_eq!(
            actual.customer_reference.unwrap(),
            "",
            "customer_reference should be ''"
        );
        assert_eq!(
            actual.counterparty_bic.unwrap(),
            "",
            "counterparty_bic should be ''"
        );
        assert_eq!(
            actual.r_transaction.unwrap(),
            "",
            "r_transaction should be ''"
        );
        assert_eq!(actual.r_reason.unwrap(), "", "r_reason should be ''");
        assert_eq!(
            actual.category_purpose.unwrap(),
            "",
            "category_purpose should be ''"
        );
        assert_eq!(actual.purpose.unwrap(), "", "purpose should be ''");

        assert_eq!(
            actual.communication,
            "BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D\'OPERATI\nON 495953"
        );
    }

    #[test]
    fn parse_movement_type3_valid() {
        let line1 = "2100010000EPIB00048 AWIUBTKAPUO1000000002578250061206007990000BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D'OPERATI06120600111 0";
        let line3 = "2300070003068226750863                         T.P.F.  S.A.                                                                  0 1";

        let actual = Movement::parse_type1(line1);
        let mut actual = actual.unwrap();
        let result = actual.parse_type3(line3);

        assert_eq!(result.is_ok(), true);

        assert_eq!(
            actual.counterparty_name.unwrap(),
            "068226750863",
            "counterparty_name should be '068226750863'"
        );
        assert_eq!(
            actual.counterparty_account.unwrap(),
            "T.P.F.  S.A.",
            "counterparty_account should be 'T.P.F.  S.A.'"
        );

        assert_eq!(
            actual.communication,
            "BORDEREAU DE DECOMPTE AVANCES    015 NUMERO D\'OPERATI\n"
        );
    }
}

#[cfg(test)]
mod test_parse_freecommunication {

    use std::io::{self, BufRead};

    use coda::encoding::label::encoding_from_whatwg_label;
    use coda::encoding::DecoderTrap;

    use super::FreeCommunication;

    #[test]
    fn parse_freecommunication_windows1252() {
        let encoding = encoding_from_whatwg_label("windows-1252").unwrap();
        let line = b"D'INVESTISSEMENT N\xB0 123\n";
        let line = encoding.decode(line, DecoderTrap::Strict).unwrap();
        let cursor = io::Cursor::new(line);
        let mut lines_iter = cursor.lines().map(|l| l.unwrap());
        assert_eq!(
            lines_iter.next(),
            Some(String::from("D\'INVESTISSEMENT N° 123"))
        );
    }

    #[test]
    fn parse_freecommunication_line1_valid() {
        let line = "4 00010000                      LINE 1 FREE COMMUNICATION                                                      X               1";

        let actual = FreeCommunication::parse_line1(line);

        assert_eq!(actual.is_ok(), true, "FreeCommunication shoud be ok");
        let actual = actual.unwrap();
        assert_eq!(actual.sequence, "0001", "sequence should be '0001'");
        assert_eq!(
            actual.detail_sequence,
            "0000",
            "detail_sequence should be '0000'"
        );
        assert_eq!(
            actual.text,
            "LINE 1 FREE COMMUNICATION                                                      X",
            "communication should be 'LINE 1 FREE COMMUNICATION                                                       '"
        );
    }

    #[test]
    fn parse_freecommunication_line1_trimvalid() {
        let line = "4 00010000                      LINE 1 FREE COMMUNICATION                                                                      1";

        let actual = FreeCommunication::parse_line1(line);

        assert_eq!(actual.is_ok(), true, "FreeCommunication shoud be ok");
        let actual = actual.unwrap();
        assert_eq!(actual.sequence, "0001", "sequence should be '0001'");
        assert_eq!(
            actual.detail_sequence,
            "0000",
            "detail_sequence should be '0000'"
        );
        assert_eq!(
            actual.text,
            "LINE 1 FREE COMMUNICATION",
            "communication should be 'LINE 1 FREE COMMUNICATION'"
        );
    }

    #[test]
    fn parse_freecommunication_following_valid() {
        let line1 = "4 00010000                      LINE 1 FREE COMMUNICATION                                                      X               1";
        let line2 = "4 00010001                      LINE 2 FREE COMMUNICATION                                                      Y               1";

        let actual = FreeCommunication::parse_line1(line1);
        assert_eq!(actual.is_ok(), true, "FreeCommunication shoud be ok");
        let mut actual = actual.unwrap();
        let result = actual.parse_following(line2);

        assert_eq!(result.is_ok(), true, "Result should be ok");
        assert_eq!(actual.sequence, "0001", "sequence should be '0001'");
        assert_eq!(
            actual.detail_sequence,
            "0000",
            "detail_sequence should be '0000'"
        );
        assert_eq!(
            actual.text,
            "LINE 1 FREE COMMUNICATION                                                      X\nLINE 2 FREE COMMUNICATION                                                      Y",
            "communication should be 'LINE 1 FREE COMMUNICATION                                                       '"
        );
    }

    #[test]
    fn parse_freecommunication_following_trim_valid() {
        let line1 = "4 00010000                      LINE 1 FREE COMMUNICATION                                                                      1";
        let line2 = "4 00010001                      LINE 2 FREE COMMUNICATION                                                                      1";

        let actual = FreeCommunication::parse_line1(line1);
        assert_eq!(actual.is_ok(), true, "FreeCommunication shoud be ok");
        let mut actual = actual.unwrap();
        let result = actual.parse_following(line2);

        assert_eq!(result.is_ok(), true, "Result should be ok");
        assert_eq!(actual.sequence, "0001", "sequence should be '0001'");
        assert_eq!(
            actual.detail_sequence,
            "0000",
            "detail_sequence should be '0000'"
        );
        assert_eq!(
            actual.text,
            "LINE 1 FREE COMMUNICATION\nLINE 2 FREE COMMUNICATION",
            "communication should be 'LINE 1 FREE COMMUNICATION\nLINE 2 FREE COMMUNICATION'"
        );
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_parse_information {
    use super::Information;
    use super::CommunicationStructure;
    use super::parse_communicationstructure;

    #[test]
    fn parse_information_type1_valid() {
        let line = "3100070006IHMI00001 TBOGOVOVERS501130001001TPF CONSULTING                                                                    1 0";

        let actual = Information::parse_type1(line);

        assert_eq!(actual.is_ok(), true, "Information shoud be ok");
        let actual = actual.unwrap();
        assert_eq!(actual.sequence, "0007", "sequence should be '0007'");
        assert_eq!(
            actual.detail_sequence,
            "0006",
            "detail_sequence should be '0006'"
        );
        assert_eq!(
            actual.bank_reference,
            "IHMI00001 TBOGOVOVERS",
            "bank_reference should be 'IHMI00001 TBOGOVOVERS'"
        );
        assert_eq!(
            actual.transaction_code,
            "50113000",
            "transaction_code should be '50113000'"
        );
        assert_eq!(
            actual.communication,
            "001TPF CONSULTING",
            "communication should be ''"
        );
    }

    #[test]
    fn parse_information_type2_valid() {
        let line1 = "3100070006IHMI00001 TBOGOVOVERS501130001001TPF CONSULTING                                                                    1 0";
        let line2 = "3200070006AV. DE HAVESKERCKE  46             1190   BRUXELLES                                                                0 0";

        let actual = Information::parse_type1(line1);
        let mut actual = actual.unwrap();
        let result = actual.parse_type2(line2);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            actual.communication,
            "001TPF CONSULTING\nAV. DE HAVESKERCKE  46             1190   BRUXELLES",
            "communication should be '1001TPF CONSULTING                                                        AV. DE HAVESKERCKE  46             1190   BRUXELLES                                                      '"
        );
    }

    #[test]
    fn parse_information_type3_valid() {
        let line1 = "3100070006IHMI00001 TBOGOVOVERS501130001001TPF CONSULTING                                                                    1 0";
        let line3 = "3300370001THIRD LINE                                                                                                         0 0";

        let actual = Information::parse_type1(line1);
        let mut actual = actual.unwrap();
        let result = actual.parse_type3(line3);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            actual.communication,
            "001TPF CONSULTING\nTHIRD LINE",
            "communication should be '001TPF CONSULTING\nTHIRD LINE'"
        );
    }

    #[test]
    fn parse_communicationstructure_valid_Unstructured() {
        let actual = parse_communicationstructure("0");
        assert_eq!(actual.is_ok(), true, "'0' should be ok");
        assert_eq!(
            actual.unwrap(),
            CommunicationStructure::Unstructured,
            "'0' should be Unstructured"
        );
    }

    #[test]
    fn parse_communicationstructure_valid_Structured() {
        let actual = parse_communicationstructure("1");
        assert_eq!(actual.is_ok(), true, "'1' should be ok");
        assert_eq!(
            actual.unwrap(),
            CommunicationStructure::Structured,
            "'1' should be Structured"
        );
    }
}
