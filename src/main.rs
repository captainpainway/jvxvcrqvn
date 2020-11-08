mod cipher;

use std::env;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use router::Router;
use serde_json::Value;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use scraper::Html;
use tera::{Context, Tera};
use regex::Regex;
use cipher::rot13;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/enaqbz", random_handler, "random");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    Iron::new(router).http(addr).unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    let content_type = ContentType::html().0;
    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = Context::new();
    context.insert("title", &rot13("ROT13 Wikipedia".to_owned()));
    context.insert("description", &rot13("Wikipedia articles returned encoded in ROT13. Click the button below for a random article. This site is meant to be nonsense.".to_owned()));
    context.insert("article_button", &rot13("Get a random Wikipedia article".to_owned()));
    context.insert("my_button", &rot13("Check out my site".to_owned()));
    let rendered = tera.render("index.html", &context).expect("Failed to render template.");

    Ok(Response::with((content_type, status::Ok, rendered)))
//    Ok(Response::with((status::Ok, Path::new("pages/index.html"))))
}

fn random_handler(_: &mut Request) -> IronResult<Response> {
    let text = get_json().ok().unwrap();
    let title = text.0;
    let body = text.1;

    let content_type = ContentType::html().0;
    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = Context::new();
    context.insert("title", &title);
    context.insert("content", &body);
    context.insert("back_button", &rot13("Go back to the main page".to_owned()));
    context.insert("refresh_button", &rot13("Get new article".to_owned()));
    let rendered = tera.render("random.html", &context).expect("Failed to render template.");

    Ok(Response::with((content_type, status::Ok, rendered)))
}

fn get_json() -> Result<(String, Vec<String>), reqwest::Error> {
    // Get a random Wikipedia article.
    let url = "https://en.wikipedia.org/w/api.php?action=query&list=random&rnnamespace=0&rnlimit=1&format=json";
    let json: Value = reqwest::blocking::get(url)?.json()?;
    let title = json["query"]["random"][0]["title"].as_str().unwrap();
    let page_id = json["query"]["random"][0]["id"].as_u64().unwrap();

    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'(').add(b')');
    let encoded_title = utf8_percent_encode(title, FRAGMENT).to_string();

    // Take the random Wikipedia article info and get the actual article text.
    let page_url = "https://en.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&titles=".to_owned() + &encoded_title;
    let page_json: Value = reqwest::blocking::get(&page_url)?.json()?;
    let html = page_json["query"]["pages"][page_id.to_string()]["extract"].as_str().unwrap();

    Ok((rot13(title.to_string()), parse_text(html.to_string())))
}

fn parse_text(html: String) -> Vec<String> {
    let scraped_text = Html::parse_fragment(&html);
    let mut vec: Vec<String> = Vec::new();
    let mut str = "".to_owned();
    for node in scraped_text.tree {
        if let scraper::node::Node::Text(text) = node {
            let text_node = &text.text.to_string();
            let re = Regex::new(r"^\n").unwrap();
            if re.is_match(text_node) {
                vec.push(rot13(str));
                str = "".to_owned();
            } else {
                str += text_node;
            }
        }
    }
    vec.push(rot13(str));
    vec
}
