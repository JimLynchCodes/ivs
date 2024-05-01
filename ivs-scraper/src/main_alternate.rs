use chrono::{self, DateTime, Local};
use cli_table::{print_stdout, Cell, Style, Table};

//  "Derive Macro Table Style" (See main_alternate.rs for the trait style syntax)

struct Scraping {
    date_scraped: DateTime<Local>,
    date_released: DateTime<Local>,
    is_new: bool,
    title: String,
    share_price_str: String,
    share_price_num: f64,
    url: String,
}

fn main() {
    println!("Running!");

    let investor_relations_page_urls = vec![
        // TODO - replace with real data
        "abc.com/investor-relations",
        "def.com/investor-relations",
    ];

    let scrapings = investor_relations_page_urls.into_iter().map(|url| {
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
    });

    let table = scrapings
        .map(|scraping| {
            vec![
                scraping.date_scraped.cell(),
                scraping.date_released.cell(),
                scraping.is_new.cell(),
                scraping.title.cell(),
                scraping.share_price_str.cell(),
                scraping.share_price_num.cell(),
                scraping.url.cell(),
            ]
        })
        .table()
        .title(vec![
            "Date Scraped".cell().bold(true),
            "Date Released".cell().bold(true),
            "New?".cell().bold(true),
            "Title".cell().bold(true),
            "Price (text)".cell().bold(true),
            "Price (number)".cell().bold(true),
            "url".cell().bold(true),
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());

    println!("Finished!");
}
