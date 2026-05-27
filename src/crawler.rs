use crate::notice::Notice;
use reqwest::blocking::{Response, get};
use scraper::{Html, Selector};

const CRAWL_URL: &str = "https://cse.knu.ac.kr/bbs/board.php?bo_table=sub5_1";
const CRAWL_TAG: [&str; 12] = [
    "",
    "일반공지",
    "학사",
    "장학",
    "심컴",
    "첨컴",
    "인컴",
    "글솝",
    "플솝[구.심컴]",
    "ICT융합[학부]",
    "대학원",
    "대학원 계약학과",
];

pub fn crawl(tag: u8, amount: usize) -> Vec<Notice> {
    let mut notices: Vec<Notice> = Vec::new();

    let url = format!("{}&sca={}", CRAWL_URL, CRAWL_TAG[tag as usize]);
    let response = match get(&url) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Failed to fetch the URL: {}", err);
            return notices; // Return an empty vector if the request fails
        }
    };
    let html_content = response.text().unwrap();

    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("table tbody tr td.td_subject div.bo_tit").unwrap();
    for element in document.select(&selector) {
        println!(
            "{}",
            element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .attr("href")
                .unwrap()
        );
        println!("{}", element.text().collect::<Vec<_>>().join(" ").trim());
    }

    notices
}
