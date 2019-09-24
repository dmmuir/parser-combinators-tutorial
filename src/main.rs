mod lib;

use std::io::Read;
use std::fs::File;
// use std::io::{Write, BufReader, BufRead, Error};

use lib::{Element};
use lib::Parser;

const XML_FILE: &str = "/mnt/c/Users/DM065392/dev/sandbox/regs-document-analysis-utilty/mockdocs/IRF FY2020 Final Rule.xml";

fn main() -> std::io::Result<()> {
    let mut xml_raw_file = File::open(XML_FILE)?;
    // let xml_buffer = BufReader::new(xml_raw_file);
    let mut xml = String::new();

    xml_raw_file.read_to_string(&mut xml)?;

    let result = match lib::element().parse(&xml) {
        Ok((next_input, newElement)) => newElement,
        Err(err) => {
            // println!("Error {}", err);
            println!("Error!");
            Element {
                name: err.to_string(),
                attributes: vec![],
                children: vec![],
            }
        },
    };

    // println!("{}", result.name);
    // println!("{:?}", result.children[0].children[0].name);
    // println!("{:?}", result.children[0].children[0].attributes);

    Ok(())
}
