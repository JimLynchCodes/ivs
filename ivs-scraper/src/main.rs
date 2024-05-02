use chrono::{self, DateTime, Datelike, Local, Month};
use cli_table::{print_stdout, Table, WithTitle};

#[derive(Table)]
struct Scraping {
    #[table(title = "Date Scraped")]
    date_scraped: DateTime<Local>,

    #[table(title = "Date Released")]
    date_released: String,

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
    println!("\nRunning!\n");

    let investor_relations_page_urls = vec![
        "https://www.acurxpharma.com/news-media/press-releases",
        "https://www.adaptimmune.com/investors-and-media/news-center/press-releases",
        // "http://www.investor.adagene.com/news-events/news-releases",  // this one doesn't work for some reason?
    ];

    let dt = Local::now();

    println!("Checking for: {}", dt);
    println!("Checking for: {}", dt.naive_local());
    println!("Checking for: {}", dt.to_utc());
    println!("Checking for: {}", dt.year());
    println!("Checking for: {}", dt.month());
    println!("Checking for: {}", dt.day());

    let month = Month::try_from(u8::try_from(dt.month()).unwrap()).ok();

    match month {
        None => {
            println!("couldn't conert to month prefix :(");
        }
        Some(month) => {
            println!("month is {:?}", month);

            let day = dt.day();
            let month_num = dt.month();
            let year = dt.year();

            let day_with_zero_padding = if day < 10 {
                format!("0{day}")
            } else {
                day.to_string()
            };

            let month_with_zero_padding = if month_num < 10 {
                format!("0{month_num}")
            } else {
                month_num.to_string()
            };

            // v1, "Slash Format"         ->   YYYY/MM/DD
            let date_to_search_for_v1 =
                format!("{year}/{month_with_zero_padding}/{day_with_zero_padding}");

            // v2, "Dots Format"          ->   YYYY.MM.DD
            let date_to_search_for_v2 =
                format!("{year}.{month_with_zero_padding}.{day_with_zero_padding}");

            // v3, "Month Prefix Format"  ->   MMM DD, YYY
            let date_to_search_for_v3 = format!("{:?} {day_with_zero_padding}, {year}", month);

            println!("\nSearching for these dates representing today:");
            println!("{date_to_search_for_v1}");
            println!("{date_to_search_for_v2}");
            println!("{date_to_search_for_v3}");

            let mut scrapings_filtered = vec![];

            investor_relations_page_urls.into_iter().for_each(|url| {
                let response = reqwest::blocking::get(url).unwrap().text().unwrap();

                if response.contains(&date_to_search_for_v1)
                    || response.contains(&date_to_search_for_v2)
                    || response.contains(&date_to_search_for_v3)
                {
                    scrapings_filtered.push(Scraping {
                        date_scraped: chrono::offset::Local::now(),
                        date_released: "?".to_string(),
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
    }
}
