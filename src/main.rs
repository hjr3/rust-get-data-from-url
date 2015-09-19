extern crate curl;
extern crate serde_json;

use curl::http;
use serde_json::Value;

pub fn main() {

    let url = "https://www.hautelook.com/api";
    let resp = http::handle()
        .get(url)
        .exec()
        .unwrap_or_else(|e| {
            panic!("Failed to get {}; error is {}", url, e);
        });

    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code());
        return;
    }

    let body = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
        panic!("Failed to parse response from {}; error is {}", url, e);
    });

    let json: Value = serde_json::from_str(body).unwrap_or_else(|e| {
        panic!("Failed to parse json; error is {}", e);
    });

    let links = json.as_object()
        .and_then(|object| object.get("_links"))
        .and_then(|links| links.as_object())
        .unwrap_or_else(|| {
            panic!("Failed to get '_links' value from json");
        });

    for (rel, link) in links.iter() {
        let href = link.as_object()
            .and_then(|object| object.get("href"))
            .and_then(|value| value.as_string())
            .unwrap_or_else(|| {
                panic!("Failed to get 'href' value from within '_links'");
            });

        println!("{} -> {}", rel, href);
    }
}
