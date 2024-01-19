// src/main.rs

// dependencies
use clap::Parser;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Error;
use std::collections::HashMap;
use std::io::{self, Write};

// command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    key: String,
    #[arg(short, long)]
    verb: String,
    #[arg(short, long)]
    band: String,
    #[arg(short, long)]
    album: String,
    #[arg(short, long)]
    thoughts: String,
}

// function to construct the header with the API key
fn construct_apikey_header(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-api-key"),
        HeaderValue::from_str(&api_key).expect("Unable to construct header"),
    );
    headers
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // parse the command line arguments
    let args = Args::parse();

    // create a new reqwest client
    let client = reqwest::Client::new();

    // make the request, depending on the action
    match args.verb {
        verb if verb == "get" => {
            let res = client
                .get("https://api.rivet-head.app/diary")
                .headers(construct_apikey_header(args.key))
                .send()
                .await?;

            // print the status code and body
            let mut stdout = io::stdout();
            writeln!(stdout, "Status: {}", res.status()).expect("Unable to write to stdout");
            let body = res.text().await?;
            writeln!(stdout, "Body:\n\n{:?}", body).expect("Unable to write to stdout");
        }

        verb if verb == "post" => {
            let mut params = HashMap::new();
            params.insert("band_content", args.band);
            params.insert("album_content", args.album);
            params.insert("thoughts_content", args.thoughts);
            let res = client
                .post("https://api.rivet-head.app/diary/new")
                .headers(construct_apikey_header(args.key))
                .form(&params)
                .send()
                .await?;

            // print the status code and body// print the status code and body
            let mut stdout = io::stdout();
            writeln!(stdout, "Status: {}", res.status()).expect("Unable to write to stdout");
            let body = res.text().await?;
            writeln!(stdout, "Body: {:?}", body).expect("Unable to write to stdout");
        }

        // handle incorrectly spelled verbs
        _ => {
            let mut stdout = io::stdout();
            writeln!(stdout, "Invalid verb, enter get, post, put or delete")
                .expect("Unable to write to stdout");
        }
    }

    Ok(())
}
