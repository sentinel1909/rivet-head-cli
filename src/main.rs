// src/main.rs

// dependencies
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Error;
use std::io::{self, Write};

// function to construct the header with the API key
fn construct_apikey_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-api-key"),
        HeaderValue::from_static("metallica_1983"),
    );
    headers
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // create a new reqwest client
    let client = reqwest::Client::new();

    // create the url
    let url = "https://rivet-head-api.shuttleapp.rs/api/diary";
    
    // make the request
    let res = client.get(url).headers(construct_apikey_header()).send().await?;

    // print the status code and body
    let mut stdout = io::stdout();
    writeln!(stdout, "Status: {}", res.status()).expect("Unable to wite to stdout");
    let body = res.text().await?;
    writeln!(stdout, "Body:\n\n{:?}", body).expect("Unable to write to stdout");
    Ok(())
}
