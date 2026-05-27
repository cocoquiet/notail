mod crawler;

mod notice;

use crawler::crawl;

fn main() {
    let _notices = crawl(0, 10);
}
