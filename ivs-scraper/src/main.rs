use chrono::{self, DateTime, Local};
use cli_table::{print_stdout, Table, WithTitle};

#[derive(Table)]
struct Scraping {
    #[table(title = "Date Scraped")]
    date_scraped: DateTime<Local>,
    #[table(title = "Date Released")]
    date_released: DateTime<Local>,
    #[table(title = "New?")]
    is_new: bool,
    #[table(title = "Title")]
    title: String,
    #[table(title = "Price (text)")]
    share_price_str: String,
    #[table(title = "Price (number)")]
    share_price_num: f64,
    #[table(title = "url")]
    url: String,
}

fn main() {
    println!("Running!");

    let investor_relations_page_urls = vec![
        "https://www.acurxpharma.com/news-media/press-releases",
        "https://www.adaptimmune.com/investors-and-media/news-center/press-releases",
        // "http://www.investor.adagene.com/news-events/news-releases",  // this one doesn't work for some reason?
    ];

    let date_to_search_for_v1 = "May 01, 2024";
    let date_to_search_for_v2 = "2024.05.01";

    let mut scrapings_filtered = vec![];

    investor_relations_page_urls.into_iter().for_each(|url| {
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();

        if response.contains(date_to_search_for_v1) || response.contains(date_to_search_for_v2) {
            scrapings_filtered.push(Scraping {
                date_scraped: chrono::offset::Local::now(),
                date_released: chrono::offset::Local::now(),
                is_new: true,
                title: "title".to_string(),
                share_price_str: "$2.54".to_string(),
                share_price_num: 2.54,
                url: url.to_string(),
            });
        }
    });

    assert!(print_stdout(scrapings_filtered.with_title()).is_ok());

    println!("Finished!");
}
