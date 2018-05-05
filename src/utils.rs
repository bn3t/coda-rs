extern crate chrono;

use std::ops::Range;
use chrono::NaiveDate;

use errors::*;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Sign {
    Credit,
    Debit,
}

impl Sign {
    pub fn to_sign(&self) -> String {
        match *self {
            Sign::Credit { .. } => String::from("+"),
            Sign::Debit { .. } => String::from("-"),
        }
    }
}

pub fn parse_sign(s: &str) -> Result<Sign> {
    match s {
        "0" => Ok(Sign::Credit),
        "1" => Ok(Sign::Debit),
        _ => Err(format!("Invalid Sign value [{}]", s).into()),
    }
}

pub fn parse_date(s: &str) -> Result<NaiveDate> {
    let date: NaiveDate = NaiveDate::parse_from_str(s, "%d%m%y").chain_err(|| "Could not parse date")?;

    Ok(date)
}

pub fn parse_str(s: &str) -> Result<String> {
    Ok(String::from(s))
}

pub fn parse_str_trim(s: &str) -> Result<String> {
    Ok(String::from(s.trim_right()))
}

pub fn parse_str_append(s: &str) -> Result<String> {
    Ok(format!("\n{}", s.trim_right()))
}

pub fn parse_u8(s: &str) -> Result<u8> {
    Ok(s.parse::<u8>().chain_err(|| "Could not parse u8")?)
}

pub fn parse_u64(s: &str) -> Result<u64> {
    Ok(s.parse::<u64>().chain_err(|| "Could not parse u64")?)
}

pub fn parse_u32(s: &str) -> Result<u32> {
    Ok(s.parse::<u32>().chain_err(|| "Could not parse u32")?)
}

pub fn parse_duplicate(s: &str) -> Result<bool> {
    match s {
        "D" => Ok(true),
        " " => Ok(false),
        _ => Err(format!("Invalid duplicate value [{}]", s).into()),
    }
}

pub fn parse_field<T>(line: &str, range: Range<usize>, convert: fn(s: &str) -> Result<T>) -> Result<T> {
    Ok(convert(line.to_string().get_range(range).as_str()).chain_err(|| "Could not parse field")?)
}

pub trait StringUtils {
    fn get_range(&self, range: Range<usize>) -> Self;
}

impl StringUtils for String {
    fn get_range(&self, range: Range<usize>) -> Self {
        let result: String = self.chars()
            .skip(range.start)
            .take(range.end - range.start)
            .collect();
        result
    }
}

#[cfg(test)]
mod test_substring {
    use super::*;

    #[test]
    fn sign_credit_to_sign() {
        assert_eq!(Sign::Credit.to_sign(), "+");
    }

    #[test]
    fn sign_debit_to_sign() {
        assert_eq!(Sign::Debit.to_sign(), "-");
    }

    #[test]
    fn substring_0_to_3() {
        assert_eq!("012345678901234567890".to_string().get_range(0..3), "012");
    }

    #[test]
    fn substring_5_to_10() {
        assert_eq!(
            "012345678901234567890".to_string().get_range(5..10),
            "56789"
        );
    }
    #[test]
    fn substring_5_to_10_multibytes() {
        assert_eq!(
            "01é345678901234567890".to_string().get_range(5..10),
            "56789"
        );
    }
}

#[cfg(test)]
mod test_parse_utils {
    use chrono::NaiveDate;

    use super::*;

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

        assert_eq!(actual.is_ok(), false, "u8 '200000' should not be ok");
    }

    #[test]
    fn parse_u64_valid() {
        let actual = parse_u64("20000");

        assert_eq!(actual.is_ok(), true, "u64 '20000' should be ok");
        assert_eq!(actual.unwrap(), 20000, "u64 '20000' should be 20000");
    }

    #[test]
    fn parse_u32_valid() {
        let actual = parse_u32("200000");

        assert_eq!(actual.is_ok(), true, "u32 '200000' should be ok");
        assert_eq!(actual.unwrap(), 200000, "u32 '200000' should be 200000");
    }

    #[test]
    fn parse_str_trim_valid() {
        let actual = parse_str_trim("BLAH   ");

        assert_eq!(actual.is_ok(), true, "str 'BLAH   ' should be ok");
        assert_eq!(actual.unwrap(), "BLAH", "str 'BLAH   ' should be 'BLAH'");
    }

    #[test]
    fn parse_str_append_valid() {
        let actual = parse_str_append("BLAH   ");

        assert_eq!(actual.is_ok(), true, "str 'BLAH   ' should be ok");
        assert_eq!(
            actual.unwrap(),
            "\nBLAH",
            "str 'BLAH   ' should be '\\nBLAH'"
        );
    }

    #[test]
    fn parse_u64_invalid() {
        let actual = parse_u8("200000200000200000200000200000200000");

        assert_eq!(
            actual.is_ok(),
            false,
            "u8 '200000200000200000200000200000200000' should not be ok"
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn parse_sign_valid_Credit() {
        let actual = parse_sign("0");
        assert_eq!(actual.is_ok(), true, "'0' should be ok");
        assert_eq!(actual.unwrap(), Sign::Credit, "'0' should be Credit");
    }

    #[test]
    #[allow(non_snake_case)]
    fn parse_sign_valid_Debit() {
        let actual = parse_sign("1");
        assert_eq!(actual.is_ok(), true, "'1' should be ok");
        assert_eq!(actual.unwrap(), Sign::Debit, "'1' should be Debit");
    }

    #[test]
    fn parse_sign_valid_invalid() {
        let actual = parse_sign("3");
        assert_eq!(actual.is_ok(), false, "'3' should not be ok");
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
