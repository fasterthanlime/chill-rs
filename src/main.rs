extern crate reqwest;

fn main() -> Result<(), reqwest::Error> {
    // let url = "https://itch.io/country";
    // let url = "http://neverssl.com/";
    let url = "http://slowwly.robertomurray.co.uk/delay/1000/url/https://itch.io/country";

    let client = reqwest::Client::new();
    let req = client.get(url);
    println!("Called get()...");

    let mut res = req.send()?;

    println!("Called send...");
    if let Some(loc) = res.headers().get("content-type") {
        if let Ok(loc) = loc.to_str() {
            println!("Location: {}", loc);
        }
    }

    println!("Parsing as text...");
    println!("{}", res.text()?);
    Ok(())
}
