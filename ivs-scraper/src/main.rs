use chrono::{self, DateTime, Local};
use cli_table::{print_stdout, Table, WithTitle};

//  "Display Trait Table Style" (To be honest though I kind of like the Derive Macro Syntax Better)

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
        // TODO - replace with real data
        "abc.com/investor-relations",
        "def.com/investor-relations",
    ];

    let scrapings: Vec<Scraping> = investor_relations_page_urls
        .into_iter()
        .map(|url| {
            // TODO - scrape for latest article

            Scraping {
                date_scraped: chrono::offset::Local::now(),
                date_released: chrono::offset::Local::now(),
                is_new: true,
                title: "title".to_string(),
                share_price_str: "$2.54".to_string(),
                share_price_num: 2.54,
                url: url.to_string(),
            }
        })
        .collect();

    assert!(print_stdout(scrapings.with_title()).is_ok());

    println!("Finished!");
}
