struct Cli {
    command: Option<Command>,
}

enum Command {
    GetFeeds {},
    GetUnread {feed: Option<String>},
    GetRaw {feed: String, id: u64},
    GetHtml {feed: String, id: u64},
    Render {feed: String, id: u64},
    TuiRead {},
}

fn main() {
    println!("Hello, world!");
}
