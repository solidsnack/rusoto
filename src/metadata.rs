use std::default::Default;
use std::error::Error;
use std::io::Read;
use std::time::Duration;

use hyper::Client;
use hyper::header::Connection;


const DEFAULT_BASE: &'static str = "http://169.254.169.254/latest/meta-data";


#[derive(Debug)]
pub struct MetadataService(pub String);

impl Default for MetadataService {
    fn default() -> MetadataService { MetadataService(DEFAULT_BASE.into()) }
}

impl MetadataService {
    pub fn read(&self, path: &str) -> Result<String, Box<Error>> {
        let mut client = Client::new();
        client.set_read_timeout(Some(Duration::from_secs(15)));

        let MetadataService(ref base) = *self;
        let url = [base.trim_right_matches('/'),
                   path.trim_left_matches('/')].join("/");
        let request = client.get(&url).header(Connection::close());

        let mut response = try!(request.send());
        let mut body = String::new();
        try!(response.read_to_string(&mut body));
        Ok(body.clone())
    }

    pub fn list(&self, path: &str) -> Result<Vec<String>, Box<Error>> {
        let s = try!(self.read(path));
        Ok(s.lines().map(|s| s.into()).collect())
    }
}
