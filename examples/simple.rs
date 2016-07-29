extern crate pager;

use std::io;

fn main() {
    pager::start(io::stdin(), io::stdout(), "check", "akls\njdlksj\ndkjasjdks\nlakdjlskajsdklsjdjhk").unwrap();
}
