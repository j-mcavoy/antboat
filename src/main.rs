use rusqlite::{params, Result};

mod newsboat;
pub use newsboat::*;

mod antennapod;
pub use antennapod::*;

fn main() -> Result<()> {
    let path = "/home/john/.local/share/newsboat/cache.db";
    let conn = rusqlite::Connection::open(&path)?;
    println!("{}", conn.is_autocommit());
    let mut stmt = conn.prepare(
        "SELECT rssurl, url, title, lastmodified, is_rtl, etag
        FROM rss_feed
        ",
    )?;
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

    for rss_feed in rss_feed_iter {
        println!("{:?}", rss_feed.unwrap());
    }
    Ok(())
}
