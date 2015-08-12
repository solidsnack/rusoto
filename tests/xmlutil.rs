extern crate rusoto;
extern crate xml;
extern crate hyper;

use xml::reader::*;
use rusoto::s3::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rusoto::xmlutil::*;
use hyper::client::response::*;

#[test]
fn parse_s3_getbuckets() {
    let mut tag_name = "tag";

    // populate from tests/sample-inputs/s3-getbuckets.xml
    let path = Path::new("tests/sample-inputs/s3-getbuckets.xml");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut reader = EventReader::new(file);
    let mut stack = reader.events().peekable();

    // match ListBucketsOutputParser::parse_xml(tag_name, &stack) {
    //     Ok(result) => panic!("Got okay, disregard"),
    //     Err(why) => panic!("Errored reading S3 list buckets result")
    // }

}
