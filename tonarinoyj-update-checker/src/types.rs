use chrono::{DateTime, FixedOffset, TimeZone};
use log::error;
use roxmltree::Node;

#[derive(Debug)]
pub struct Feed {
    pub title: String,
    pub subtitle: String,
    pub updated: DateTime<FixedOffset>,
    pub id: String,
    pub link: String,
    pub entries: Vec<Entry>,
}

impl Feed {
    pub fn parse(node: Node) -> Self {
        return Self {
            title: get_child_text(node, "title"),
            subtitle: get_child_text(node, "subtitle"),
            updated: parse_datetime(&get_child_text(node, "updated")),
            id: get_child_text(node, "id"),
            link: get_child_attribute(node, "link", "href"),
            entries: Entry::parse_entries(node),
        };
    }
}

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub link: String,
    pub id: String,
    pub updated: DateTime<FixedOffset>,
    pub free_term_start_date: String,
    pub content: String,
    pub thumbnail_link: String,
    pub author: Author,
}

impl Entry {
    pub fn parse_entries(node: Node) -> Vec<Self> {
        return node
            .children()
            .filter(|&n| n.tag_name().name() == "entry")
            .map(Self::parse)
            .collect::<Vec<_>>();
    }

    fn parse(node: Node) -> Self {
        return Self {
            title: get_child_text(node, "title"),
            link: get_child_attribute(node, "link", "href"),
            id: get_child_text(node, "id"),
            updated: parse_datetime(&get_child_text(node, "updated")),
            free_term_start_date: get_child_text(node, "freeTermStartDate"),
            content: get_child_text(node, "content"),
            thumbnail_link: "".to_string(), // TODO: implement
            author: Author::parse(node),
        };
    }
}

#[derive(Debug)]
pub struct Author {
    pub name: String,
}

impl Author {
    pub fn parse(node: Node) -> Self {
        let author = find_child(node, "author");

        if author.is_none() {
            return Self {
                name: "".to_string(),
            };
        }

        return Self {
            name: get_child_text(author.unwrap(), "name"),
        };
    }
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

fn parse_datetime(s: &str) -> DateTime<FixedOffset> {
    let result = DateTime::parse_from_rfc3339(s);

    if let Ok(datetime) = result {
        return datetime;
    }

    return FixedOffset::east(0).ymd(1970, 1, 1).and_hms(0, 0, 0);
}
