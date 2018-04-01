use chrono;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        ParseError(chrono::format::ParseError);
    }
}
