extern crate iron;
extern crate router;
extern crate reqwest;
extern crate serde_json;
extern crate percent_encoding;

use std::path::Path;
use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json::Value;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/enaqbz", random_handler, "random");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("pages/index.html"))))
}

fn random_handler(_: &mut Request) -> IronResult<Response> {
    let json = get_json().ok().unwrap();
    Ok(Response::with((status::Ok, json)))
}

fn get_json() -> Result<String, reqwest::Error> {
    let url = "https://en.wikipedia.org/w/api.php?action=query&list=random&rnnamespace=0&rnlimit=1&format=json";
    let json: Value = reqwest::blocking::get(url)?.json()?;
    let title = json["query"]["random"][0]["title"].as_str().unwrap();

    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'(').add(b')');
    let encoded_title = utf8_percent_encode(title, FRAGMENT).to_string();

    let page_url = "https://en.wikipedia.org/w/api.php?action=parse&page=".to_owned() + &encoded_title + "&prop=text&format=json";
    let page_json: Value = reqwest::blocking::get(&page_url)?.json()?;
    let text = &page_json["parse"]["text"]["*"].as_str().unwrap();

    Ok(text.to_string())
}
