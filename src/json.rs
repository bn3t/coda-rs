extern crate chrono;
extern crate encoding;
extern crate serde;
extern crate serde_json;

use errors::*;
use coda::Coda;

pub mod date_serde {
    use chrono::NaiveDate;
    use json::serde::Serializer;

    pub fn serialize<S>(date: &NaiveDate, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return s.serialize_str(&format!("{}", date.format("%Y-%m-%d")));
    }

    // pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate>, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     let s: Option<String> = Option::deserialize(deserializer)?;
    //     if let Some(s) = s {
    //         return Ok(Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d")
    //             .map_err(serde::de::Error::custom)?));
    //     }

    //     Ok(None)
    // }
}

pub fn to_json(coda: &Coda) -> Result<String> {
    Ok(serde_json::to_string_pretty(&coda).chain_err(|| "Unable to generate json file")?)
}

#[cfg(test)]
mod test_json {
    use chrono::NaiveDate;

    use super::*;
    use coda::*;
    use utils::*;

    #[test]
    fn to_json_valid() {
        let coda = Coda {
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
                account_structure: AccountStructure::IBANBelgianAccountNumber,
                old_sequence: String::from("old_sequence"),
                account_currency: String::from("account_currency"),
                old_balance_sign: Sign::Credit,
                old_balance: 100000,
                old_balance_date: NaiveDate::from_ymd(2018, 4, 1),
                account_holder_name: String::from("account_holder_name"),
                account_description: String::from("account_description"),
                coda_sequence: String::from("coda_sequence"),
            },
            movements: Vec::new(),
            information: Vec::new(),
            free_communications: Vec::new(),
            new_balance: NewBalance {
                new_sequence: String::from("new_sequence"),
                account_currency: String::from("account_currency"),
                new_balance_sign: Sign::Credit,
                new_balance: 200000,
                new_balance_date: NaiveDate::from_ymd(2018, 4, 3),
            },
            trailer: Trailer {
                number_records: 123,
                total_debit: 4321000,
                total_credit: 123400,
            },
        };

        let j = to_json(&coda);

        assert_eq!(j.is_ok(), true, "to_json should be ok");
    }
}
