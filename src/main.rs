extern crate iron;
extern crate router;
extern crate reqwest;

use std::path::Path;
use iron::prelude::*;
use iron::status;
use router::Router;

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
    let url = "https://en.wikipedia.org/w/api.php?format=json&action=query&generator=random&grnnamespace=0&grnlimit=1";
    let json = reqwest::blocking::get(url)?.text()?;
    println!("{:?}", json);
    Ok(json)
}
