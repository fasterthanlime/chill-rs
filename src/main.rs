extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    // let url = "https://itch.io/country";
    let url = "http://neverssl.com/";
    let uri: hyper::Uri = url.parse().unwrap();

    rt::run(fetch_url(uri));
}

fn fetch_url(uri: hyper::Uri) -> impl Future<Item = (), Error = ()> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);

    client
        .get(uri)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            let body = res.into_body();
            body.for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error{}", e))
            })
        }).map(|_| {
            println!("\n\nDone.");
        }).map_err(|err| {
            eprintln!("\n\nError {}", err);
        })
}
