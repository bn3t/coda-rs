use chrono;
use serde_json;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        ParseError(chrono::format::ParseError);
        SerdeJsonError(serde_json::error::Error);
    }
}
