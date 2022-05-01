#[derive(Debug)]
pub struct Feed {
    pub title: String,
    pub subtitle: String,
    pub updated: String,
    pub id: String,
    pub link: String,
    pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub link: String,
    pub id: String,
    pub updated: String,
    pub free_term_start_date: String,
    pub content: String,
    pub thumbnail_link: String,
    pub author: Author,
}

#[derive(Debug)]
pub struct Author {
    pub name: String,
}
