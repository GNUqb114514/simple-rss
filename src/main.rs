/// An easy-to-use RSS reader.
struct Cli {
    command: Option<Command>,
}

enum Command {
    /// Get registered feeds.
    GetFeeds {},
    /// Get unread articles from a feed / all feeds.
    GetUnread {feed: Option<String>},
    /// Get raw article content.
    GetRaw {feed: String, id: u64},
    /// Get HTML content of an article.
    GetHtml {feed: String, id: u64},
    /// Render an article into the terminal.
    Render {feed: String, id: u64},
    /// Read articles in the TUI.
    TuiRead {},
}

fn main() {
    println!("Hello, world!");
}
