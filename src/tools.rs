use coda::Coda;
use json;

use errors::*;
use std::io::Write;

use ansi_term::Colour::Blue;

pub fn print_as_json(coda: &Coda) -> Result<()> {
    let j = json::to_json(coda).chain_err(|| "Could not make json")?;
    println!("{}", j);
    Ok(())
}

pub fn print_header<W: Write>(w: &mut W, coda: &Coda, colored: bool) {
    let mut str = format!(
        "-----------------  {:<20} -- {:<19} ------------------\n---- {} - {} - {} - {} {}{} -----",
        coda.old_balance.account_holder_name,
        coda.old_balance.account,
        coda.header.creation_date,
        coda.header.file_reference,
        coda.old_balance.old_balance_date,
        coda.old_balance.account.get_currency(),
        coda.old_balance.old_balance_sign.to_sign(),
        coda.old_balance.old_balance as f64 / 1000.0
    );

    if colored {
        str = Blue.paint(str).to_string();
    }
    let _ = w.write_all(str.as_bytes());
}

pub fn print_footer<W: Write>(w: &mut W, coda: &Coda, _colored: bool) {
    let _ = writeln!(
        w,
        "<<<<<<<<<<<<<<<< {} - {}{}",
        coda.new_balance.new_balance_date,
        coda.new_balance.new_balance_sign.to_sign(),
        coda.new_balance.new_balance as f64 / 1000.0
    );
}

#[cfg(test)]
mod test_account {
    use super::*;
    use chrono::*;
    use coda::*;
    use utils::*;

    #[test]
    fn print_header_valid() {
        let coda = make_test_coda();

        let mut buf: Vec<u8> = Vec::new();
        print_header(&mut buf, &coda, false);
        let actual = String::from_utf8(buf);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(
            actual.unwrap(),
            "-----------------  account_holder       -- BE123457EUR ------------------\n---- 2018-04-02 - file_reference - 2018-04-01 - EUR +100.5 -----\n"
        );
    }

    #[test]
    fn print_footer_valid() {
        let coda = make_test_coda();

        let mut buf: Vec<u8> = Vec::new();
        print_footer(&mut buf, &coda, false);
        let actual = String::from_utf8(buf);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "<<<<<<<<<<<<<<<< 2018-04-03 - +200.5\n");
    }

    fn make_test_coda() -> Coda {
        Coda {
            header: Header {
                creation_date: NaiveDate::from_ymd(2018, 4, 2),
                bank_id: String::from("bank_id"),
                duplicate: false,
                file_reference: String::from("file_reference"),
                name_addressee: String::from("name_addressee"),
                bic: String::from("bic"),
                company_id: String::from("company_id"),
                reference: String::from("reference"),
                related_reference: String::from("related_reference"),
                version: 1,
            },
            old_balance: OldBalance {
                account: Account::IBANBelgianAccountNumber {
                    number: String::from("BE123457"),
                    currency: String::from("EUR"),
                },
                old_sequence: String::from("old_sequence"),
                old_balance_sign: Sign::Credit,
                old_balance: 100500,
                old_balance_date: NaiveDate::from_ymd(2018, 4, 1),
                account_holder_name: String::from("account_holder"),
                account_description: String::from("account_description"),
                coda_sequence: String::from("coda_sequence"),
            },
            movements: Vec::new(),
            information: Vec::new(),
            free_communications: Vec::new(),
            new_balance: NewBalance {
                new_sequence: String::from("new_sequence"),
                new_balance_sign: Sign::Credit,
                new_balance: 200500,
                new_balance_date: NaiveDate::from_ymd(2018, 4, 3),
            },
            trailer: Trailer {
                number_records: 123,
                total_debit: 4321000,
                total_credit: 123400,
            },
        }
    }
}
