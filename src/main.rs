/*!
This tool outputs a CSV file with random data.
*/
use clap::{Arg, App, ArgMatches};
use std::process::Output;
use std::error::Error;

// Alias our own Result type to avoid verbosity
type SchemaResult<T> = Result<T, Box<dyn Error>>;

struct CSV;

trait Schema {

    type Output;

    fn build_schema(&self, schema_file: &str) -> SchemaResult<Output>;

}

#[allow(dead_code)]
impl Schema for CSV {

    type Output = Self;

    fn build_schema(&self, schema_file: &str) -> SchemaResult<Output> {
        unimplemented!()
    }
}

// struct Schema;
//
// #[allow(dead_code)]
// impl Schema {
//     pub fn new() -> Self {
//         Schema {}
//     }
// }
//
// fn build_schema(schema_file: &str) -> Schema {
//     Schema
// }


fn get_cmd_argument_matches<'a>() -> ArgMatches<'a> {

    App::new("CSV Generator")
    .version("0.1.0")
    .about("Generates a CSV with random data")
    .arg(Arg::with_name("schema")
             .short("s")
             .long("schema")
             .takes_value(true)
             .required(true)
             .help("A schema file"))
    .arg(Arg::with_name("outfile")
             .short("o")
             .long("outfile")
             .takes_value(true)
             .help("Output file"))                 
    .arg(Arg::with_name("rows")
             .short("r")
             .long("rows")
             .takes_value(true)
             .help("Number of data rows to generate"))
    .get_matches()
}

fn main() -> Result<(), Box<dyn Error>> {

    let matches = get_cmd_argument_matches();
    let outfile = matches.value_of("outfile").unwrap_or("output.csv");
    let schema_file = matches.value_of("schema").unwrap();
    let rows = matches.value_of("rows").unwrap_or("10").parse::<i32>().expect("Failed to parse 'rows' argument");

    println!("The output will be written to: {}, using schema file: {}, generating {} rows", outfile, schema_file, rows);

    //parse schema toml
    //let schema = build_schema(schema_file);
    let csv = CSV.build_schema(schema_file);

    //generate csv from schema
    //BONUS concurrent line generation, feed to writer
    
    //println!("Hello, world!");

    Ok(())
}