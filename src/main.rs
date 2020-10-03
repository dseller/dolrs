use log::{info};
use dolrs::parser::{parse, ParseError};
use simple_logger::SimpleLogger;
use dolrs::document::DocumentEntry;

fn main() {
    SimpleLogger::new().init().unwrap();

    let entries = parse("Hello World! $CL$ $FG,RED$ What's up?");

    println!("{:?}", entries);
}
