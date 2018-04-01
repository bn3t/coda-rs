extern crate argparse;

use std::io::{stderr, stdout};
use std::result::Result;

use self::argparse::{ArgumentParser, Print, Store, StoreTrue};

pub struct Options {
    pub coda_filename: String,
    pub json: bool,
}

impl Options {
    pub fn parse_options(args: Vec<String>) -> Result<Options, i32> {
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
}

#[cfg(test)]
mod test_options {
    use super::Options;

    #[test]
    fn parse_verbose() {
        let args = vec![String::from("coda-rs"), String::from("-v")];
        let options = Options::parse_options(args);
        assert_eq!(options.is_err(), true);
        assert_eq!(options.err(), Some(0));
    }

    #[test]
    fn parse_valid_params_all_params() {
        let args = vec![
            String::from("coda-rs"),
            String::from("-j"),
            String::from("coda_file.txt"),
        ];
        let options = Options::parse_options(args);
        assert_eq!(options.is_ok(), true, "Returned options should be Ok");
        let options = options.unwrap();
        assert_eq!(options.coda_filename, "coda_file.txt");
        assert_eq!(options.json, true);
    }
}
