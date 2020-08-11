#[derive(Debug)]
pub struct Metadata {
    pub db_schema_version_major: u8,
    pub db_schema_version_minor: u8,
}

#[derive(Debug)]
pub struct GoogleReplay {
    pub id: u64,
    pub guid: String,
    pub state: i32,
    pub ts: i32,
}

#[derive(Debug)]
pub struct RssFeed {
    pub rssurl: String,
    pub url: String,
    pub title: String,
    pub lastmodified: i64,
    pub is_rtl: bool,
    pub etag: String,
}

#[derive(Debug)]
pub struct RssItem {
    pub guid: String,
    pub title: String,
    pub author: String,
    pub url: String,
    pub feedurl: String,
    pub pubDate: i32,
    pub content: String,
    pub unread: bool,
    pub enclosure_url: String,
    pub enclosure_type: String,
    pub enqueued: bool,
    pub flags: String,
    pub deleted: bool,
    pub base: String,
}
