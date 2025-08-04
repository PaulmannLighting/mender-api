//! Prototype for testing the API.

use std::fs::read;

use clap::Parser;
use log::error;
use mender_free_ext::{ApiVersion, Management, MenderServer, UserAdm};
use reqwest::Certificate;

const CERT_FILE: &str = "/home/mender/automatization-sources/certificates/DigiCertCA.pem";

#[derive(Debug, Parser)]
struct Args {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cert = read(CERT_FILE)
        .inspect_err(|error| error!("Failed to read certificate file: {error}"))
        .ok()
        .map(|cert| Certificate::from_pem(&cert).expect("Failed to parse certificate: {error}"));

    let server = MenderServer::new(
        "https://mender-acc.paulmann.com"
            .parse()
            .expect("Invalid URL"),
        cert,
    )
    .expect("Failed to create MenderServer");

    let response = server
        .management(ApiVersion::V1)
        .useradm()
        .login(args.username, args.password)
        .await
        .expect("Failed to login MenderServer");
    println!("Raw response: {response:?}");
    let response_str = String::from_utf8(response).expect("Failed to convert response to string");
    println!("Response string: {response_str}");
}
