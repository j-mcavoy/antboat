use rusqlite::{params, Result};
use schema::*;

pub mod schema;

use std::path::PathBuf;

pub fn get_rss_feeds(path: &PathBuf) -> Result<Vec<RssFeed>> {
    let conn = rusqlite::Connection::open(&path)?;
    let mut stmt =
        conn.prepare("SELECT rssurl, url, title, lastmodified, is_rtl, etag FROM rss_feed ")?;

    let rss_feed_iter = stmt.query_map(params![], |row| {
        Ok(RssFeed {
            rssurl: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            lastmodified: row.get(3)?,
            is_rtl: row.get(4)?,
            etag: row.get(5)?,
        })
    })?;

    let mut rss_feeds = Vec::new();
    for rss_feed in rss_feed_iter {
        rss_feeds.push(rss_feed?);
    }

    Ok(rss_feeds)
}

pub fn get_rss_items(path: &PathBuf) -> Result<Vec<RssItem>> {
    let conn = rusqlite::Connection::open(&path)?;
    let mut stmt =
        conn.prepare("SELECT
                     guid, title, author, url, feedurl, pubDate, content, unread, enclosure_url, enclosure_type, enqueued, flags, deleted, base
                     FROM rss_item")?;

    let rss_item_iter = stmt.query_map(params![], |row| {
        Ok(RssItem {
            guid: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            url: row.get(3)?,
            feedurl: row.get(4)?,
            pubDate: row.get(5)?,
            content: row.get(6)?,
            unread: row.get(7)?,
            enclosure_url: row.get(8)?,
            enclosure_type: row.get(9)?,
            enqueued: row.get(10)?,
            flags: row.get(11)?,
            deleted: row.get(12)?,
            base: row.get(13)?,
        })
    })?;

    let mut rss_items = Vec::new();
    for rss_item in rss_item_iter {
        rss_items.push(rss_item?);
    }

    Ok(rss_items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_rss_feeds() {
        let newsboat_test_db: PathBuf = PathBuf::from("./test/cache.db");
        let rss_feeds = get_rss_feeds(&newsboat_test_db);
        let j: String = serde_json::to_string(&rss_feeds.unwrap()).unwrap();

        let foo: String = fs::read_to_string("./test/cache.test.json").unwrap();
        assert_eq!(j + "\n", foo);
    }

    #[test]
    fn test_get_rss_items() {
        let newsboat_test_db: PathBuf = PathBuf::from("./test/cache.db");
        let rss_items = get_rss_items(&newsboat_test_db);
        let j: String = serde_json::to_string(&rss_items.unwrap()).unwrap();

        let foo: String = fs::read_to_string("./test/cache.test.json").unwrap();
        assert_eq!(j + "\n", foo);
    }
}
