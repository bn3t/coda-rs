use std::result::Result;

use clap::{App, AppSettings, Arg, ErrorKind, SubCommand};

#[derive(PartialEq, Debug)]
pub enum Command {
    Json,
}
pub struct Options {
    pub command: Option<Command>,
    pub coda_filenames: Vec<String>,
    pub debug: bool,
    pub encoding_label: Option<String>,
    pub sort_by_ref: bool,
}

impl Options {
    pub fn parse_options(args: Vec<String>) -> Result<Options, i32> {
        let mut options = Options {
            command: None,
            debug: false,
            encoding_label: None,
            sort_by_ref: false,
            coda_filenames: vec![],
        };

        let matches = App::new("Parse coda files")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Bernard Niset")
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .after_help(format!("Build: {} - {}", env!("GIT_COMMIT"), env!("BUILD_DATE")).as_str())
            .arg(
                Arg::with_name("debug")
                    .long("debug")
                    .short("d")
                    .required(false)
                    .takes_value(false)
                    .help("Debug parsed coda data on the console"),
            ).arg(
                Arg::with_name("encoding")
                    .long("encoding")
                    .short("e")
                    .required(false)
                    .takes_value(true)
                    .help("Encoding for reading, use a whatwg label - See https://encoding.spec.whatwg.org/#concept-encoding-get (default to utf-8)"),
            )
            .arg(
                Arg::with_name("sort-ref")
                        .long("sort-ref")
                        .help("Sort by file reference")
                        .required(false)
                        .takes_value(false)
            )
            .subcommand(
                SubCommand::with_name("json").about("Convert coda files to json")
                    .arg(Arg::with_name("files").takes_value(true).multiple(true).help("List of Coda files to parse"))
            ).get_matches_from_safe(args);
        if let Ok(matches) = matches {
            options.encoding_label = matches.value_of("encoding").map(|v| String::from(v));
            options.sort_by_ref = matches.is_present("sort-ref");
            match matches.subcommand() {
                ("json", Some(sub_m)) => {
                    options.command = Some(Command::Json);

                    for input in sub_m.values_of("files").unwrap() {
                        options.coda_filenames.push(String::from(input));
                    }
                }
                _ => (),
            }
        } else {
            let err = matches.err().unwrap();
            match err.kind {
                ErrorKind::HelpDisplayed => {
                    eprintln!("{}", err.message);
                }
                _ => (),
            }
            return Err(1);
        }

        Ok(options)
    }
}

#[cfg(test)]
mod test_options {
    use super::*;

    #[test]
    fn parse_help() {
        let args = vec![String::from("coda-rs"), String::from("-h")];
        let options = Options::parse_options(args);
        assert_eq!(options.is_err(), true);
        assert_eq!(options.err(), Some(1));
    }

    #[test]
    fn parse_valid_params_all_params() {
        let args = vec![
            String::from("coda-rs"),
            String::from("-e"),
            String::from("windows-1252"),
            String::from("--sort-ref"),
            String::from("json"),
            String::from("coda_file1.txt"),
            String::from("coda_file2.txt"),
            String::from("coda_file3.txt"),
        ];
        let options = Options::parse_options(args);
        assert_eq!(options.is_ok(), true, "Returned options should be Ok");
        let options = options.unwrap();
        assert_eq!(
            options.coda_filenames,
            vec!["coda_file1.txt", "coda_file2.txt", "coda_file3.txt"]
        );
        assert_eq!(options.command.unwrap(), Command::Json);
        assert_eq!(options.sort_by_ref, true);
        assert_eq!(options.encoding_label.is_some(), true);
        assert_eq!(options.encoding_label.unwrap(), "windows-1252");
    }
}
