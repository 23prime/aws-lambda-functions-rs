use std::env;

use dotenv::dotenv;
use log::{error, info};
use roxmltree::{Document, Node};

use types::{Author, Entry, Feed};

type BoxError = Box<dyn std::error::Error>;

pub mod error;
pub mod event;
pub mod logger;
pub mod types;

pub async fn run() -> Result<(), BoxError> {
    dotenv().ok();

    let series = fetch_series().await?;
    let document = Document::parse(&series)?;
    let node = document.root_element();

    let feed = parse_feed(node);
    info!("{:?}", feed);

    return Ok(());
}

async fn fetch_series() -> Result<String, BoxError> {
    let entry_id = env::var("TYJ_SERIES_ID").expect("TYJ_SERIES_ID must be set");
    let url = format!("https://tonarinoyj.jp/atom/series/{}", entry_id);
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        error!("Failed to get series from tonarinoyj.jp: {:?}", response);
        return Err(Box::new(error::LambdaGeneralError::none()));
    }

    return Ok(response.text().await?);
}

fn parse_feed(node: Node) -> Feed {
    return Feed {
        title: get_child_text(node, "title"),
        subtitle: get_child_text(node, "subtitle"),
        updated: get_child_text(node, "updated"),
        id: get_child_text(node, "id"),
        link: get_child_attribute(node, "link", "href"),
        entries: parse_entries(node),
    };
}

fn parse_entries(node: Node) -> Vec<Entry> {
    return node
        .children()
        .filter(|&n| n.tag_name().name() == "entry")
        .map(parse_entry)
        .collect::<Vec<_>>();
}

fn parse_entry(node: Node) -> Entry {
    return Entry {
        title: get_child_text(node, "title"),
        link: get_child_attribute(node, "link", "href"),
        id: get_child_text(node, "id"),
        updated: get_child_text(node, "updated"),
        free_term_start_date: get_child_text(node, "freeTermStartDate"),
        content: get_child_text(node, "content"),
        thumbnail_link: "".to_string(), // TODO: implement
        author: parse_author(node),
    };
}

fn parse_author(node: Node) -> Author {
    let author = find_child(node, "author");

    if author.is_none() {
        return Author {
            name: "".to_string(),
        };
    }

    return Author {
        name: get_child_text(author.unwrap(), "name"),
    };
}

fn find_child<'a>(node: Node<'a, 'a>, tag_name: &'a str) -> Option<Node<'a, 'a>> {
    let result = node.children().find(|&n| n.tag_name().name() == tag_name);

    if result.is_none() {
        error!("The tag name [{:?}] is not found in {:?}", tag_name, node);
    }

    return result;
}

fn get_child_text(node: Node, tag_name: &str) -> String {
    let child = find_child(node, tag_name);

    if child.is_none() {
        return "".to_string();
    }

    return child
        .unwrap()
        .children()
        .next()
        .unwrap()
        .text()
        .unwrap()
        .to_string();
}

fn get_child_attribute(node: Node, tag_name: &str, attribute: &str) -> String {
    let child = find_child(node, tag_name);

    if child.is_none() {
        return "".to_string();
    }

    return child.unwrap().attribute(attribute).unwrap().to_string();
}
