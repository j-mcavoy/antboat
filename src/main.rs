use std::path::PathBuf;

use directories::UserDirs;

mod newsboat;
pub use newsboat::*;

mod antennapod;
pub use antennapod::*;

fn main() {
    let mut newsboat_db: PathBuf = PathBuf::new();
    newsboat_db.push(UserDirs::new().unwrap().home_dir());
    newsboat_db.push(".local/share/newsboat/cache.db");

    let rss_feeds = get_rss_feeds(&newsboat_db);
}
