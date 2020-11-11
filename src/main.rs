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

    // Heroku requires a dynamic port.
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    Iron::new(router).http(addr).unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    // Set the mime type for the page.
    let content_type = ContentType::html().0;
    let tera = Tera::new("templates/**/*").unwrap();

    // Template strings.
    let mut context = Context::new();
    context.insert("title", &rot13("ROT13 Wikipedia".to_owned()));
    context.insert("description", &rot13("Wikipedia articles returned encoded in ROT13. Click the button below for a random article. This site is meant to be nonsense.".to_owned()));
    context.insert("article_button", &rot13("Get a random Wikipedia article".to_owned()));
    context.insert("my_button", &rot13("Check out my site".to_owned()));
    let rendered = tera.render("index.html", &context).expect("Failed to render template.");

    Ok(Response::with((content_type, status::Ok, rendered)))
}

fn random_handler(_: &mut Request) -> IronResult<Response> {
    let text = get_json().ok().unwrap();
    let title = text.0;
    let body = text.1;

    // Set the mime type for the page.
    let content_type = ContentType::html().0;
    let tera = Tera::new("templates/**/*").unwrap();

    // Template strings.
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

    // Percent-encode the article title.
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'(').add(b')');
    let encoded_title = utf8_percent_encode(title, FRAGMENT).to_string();

    // Take the random Wikipedia article info and get the actual article text.
    let page_url = "https://en.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&titles=".to_owned() + &encoded_title;
    let page_json: Value = reqwest::blocking::get(&page_url)?.json()?;
    let html = page_json["query"]["pages"][page_id.to_string()]["extract"].as_str().unwrap();

    // Return the page title and vector of paragraph strings.
    Ok((rot13(title.to_string()), parse_text(html.to_string())))
}

fn parse_text(html: String) -> Vec<String> {
    // Use scraper::Html to parse the incoming string.
    let scraped_text = Html::parse_fragment(&html);

    // Add a new empty vector to hold the parsed strings.
    let mut vec: Vec<String> = Vec::new();

    // This is an empty string for concatenating the paragraphs.
    // Because we're going to be dealing with a lot of inline HTML tags,
    // we only want to push the string to the vector when there's a newline.
    let mut str = "".to_owned();

    //Iterate over the DOM node tree.
    for node in scraped_text.tree {

        // Each node is either a Text or an Element node.
        // Element nodes are HTML tags.
        // We just want text nodes.
        if let scraper::node::Node::Text(text) = node {

            // Cast the text node to a String.
            let text_node = &text.text.to_string();

            // Check for text nodes that begin with "\n",
            // which indicates a new paragraph.
            let re = Regex::new(r"^\n").unwrap();

            // If the regex matches, it's a new paragraph,
            // so push the temp string to the vector,
            // and reinitialize an empty string.
            if re.is_match(text_node) {

                // Here's where I'm calling my rot13 function,
                // which will be added next.
                vec.push(rot13(str));
                str = "".to_owned();
            } else {

                // If there's no newline found, concatenate
                // onto the current temp string.
                str += text_node;
            }
        }
    }

    // One last push of the remaining temp string contents to the vec.
    vec.push(rot13(str));

    // Return the vector.
    vec
}

