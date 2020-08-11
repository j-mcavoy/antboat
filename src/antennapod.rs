#[derive(Debug)]
pub struct Metadata {
    pub db_schema_version_major: u8,
    pub db_schema_version_minor: u8,
}


#[derive(Debug)]
pub struct AndroidMetaData {}

#[derive(Debug)]
pub struct Favorites {
    pub id: u64,
    pub feedItem: i32,
    pub feed: i32,
}

#[derive(Debug)]
pub struct Queue{
    pub id: u64,
    pub feedItem: u64,
    pub feed: i32,
}

#[derive(Debug)]
pub struct SimpleChapters{
    pub id: u64,
    pub title: String,
    pub start: u64,
    pub feeditem: u64,
    pub link: String,
    pub r#type: u64,
}

#[derive(Debug)]
pub struct Feeds
{
    pub id: u64,
    pub title: String,
    pub custom_title: String,
    pub file_url: String,
    pub download_url: String,
    pub downloaded: bool,
    pub link: String,
    pub description: String,
    pub payment_link: String,
    pub last_update: String,
    pub language: String,
    pub author: String,
    pub image_url: String,
    pub r#type: String,
    pub feed_identifier: String,
    pub auto_download: bool,
    pub username: String,
    pub password: String,
    pub include_filter: String,
    pub exclude_filter: String,
    pub keep_updated: bool,
    pub is_paged: bool,
    pub next_page_link: String,
    pub hide: String,
    pub sort_order: String,
    pub last_update_failed: bool,
    pub auto_delete_action: bool,
    pub feed_playback_speed: u64,
}

#[derive(Debug)]
pub struct FeedItems {
    pub id: u64,
    pub title: String,
    pub content_encoded: String,
    pub pubDate: u64,
    pub read: bool,
    pub link: String,
    pub description: String,
    pub payment_link: String,
    pub media: u64,
    pub feed: u64,
    pub has_simple_chapters: bool,
    pub item_identifier: bool,
    pub image_url: String,
    pub auto_download: bool,
}

#[derive(Debug)]
pub struct FeedMedia {
    pub id: u64,
    pub duration: u64,
    pub file_url: String,
    pub download_url: String,
    pub downloaded: bool,
    pub position: u64,
    pub filesize: u64,
    pub mime_type: String,
    pub playback_completion_date: u64,
    pub feeditem: u64,
    pub played_duration: u64,
    pub has_embedded_picture: bool,
    pub last_played_time: u64,
}

#[derive(Debug)]
pub struct DownloadLog {
    pub id: u64,
    pub feedfile: u64,
    pub feedfile_type: u8,
    pub reason: u8,
    pub successful: bool,
    pub completion_date: u64,
    pub reason_detailed: String,
    pub title: String
}
