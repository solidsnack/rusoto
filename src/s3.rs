//! S3 bindings for Rust
#![allow(unused_variables, unused_mut)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use xmlutil::*;
use std::str::FromStr;

// include the code generated from the s3 botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/s3.rs"));

pub struct S3Helper<'a> {
	client: S3Client<'a>
}

impl<'a> S3Helper<'a> {
	pub fn new(credentials:&'a AWSCredentials, region:&'a str) -> S3Helper<'a> {
		S3Helper { client: S3Client::new(credentials, region) }
	}

    pub fn list_buckets(&self) -> Result<ListBucketsOutput, AWSError> {
        self.client.list_buckets()
    }
    //
	// pub fn list_queues(&self) -> Result<ListQueuesResult, AWSError> {
	// 	self.client.list_queues(&ListQueuesRequest::default())
	// }

}
