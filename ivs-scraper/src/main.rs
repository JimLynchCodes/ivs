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
        "https://www.agilent.com/about/newsroom/presrel.html",
        "https://www.aadibio.com/latest-news/",
        "https://www.prnewswire.com/news-releases/abbvie-reports-first-quarter-2024-financial-results-302128164.html",
        "https://www.amerisourcebergen.com/newsroom",
        "https://investors.abcellera.com/news/default.aspx",
        "https://corporate.abcam.com/media-centre/our-latest-news/",
        "https://investors.abeonatherapeutics.com/press-releases",
        "https://arcabio.com/press-releases/",
        // "https://investors.acumenpharm.com/news-events/news-releases",
        // "https://investors.absci.com/news-and-events/news-releases",
        "https://abbott.mediaroom.com/",
        // "https://investor.arbutusbio.com/press-releases",
        "https://abvcpharma.com/?page_id=17318",
        "https://acadia.com/media/news-releases/",
        "https://www.alsetinc.com/press-releases",
        "https://www.alsetinc.com/press-releases",
        "https://www.alsetinc.com/press-releases",
        "https://www.auroramj.com/press-releases",
        "https://www.acertx.com/investor-relations/press-releases/",
        // "https://investor.adicetbio.com/news-and-events/news-releases",
        "https://www.aceragen.com/latest-news/press-releases/",
        "https://www.acadiahealthcare.com/investors/press-releases/",
        // "https://ir.achillestx.com/press-releases",
        "https://ir.achievelifesciences.com/news-events/press-releases",
        // "https://ir.acimmune.com/news",
        "https://finance.yahoo.com/quote/ACLX/press-releases/",
        "https://ir.acorda.com/investors/investor-news/default.aspx",
        // "https://investor.aclaristx.com/press-releases",
        // "https://ir.acrivon.com/news-events/news-releases",
        "https://www.prnewswire.com/news-releases/acelrx-announces-rebranding-with-name-change-to-talphera-inc-302029288.html",
        "https://www.acasti.com/en/investors/news-events/press-releases",
        "https://www.acurxpharma.com/news-media/press-releases",
        // "https://investor.adagene.com/news-events/news-releases",
        "https://www.adaptimmune.com/investors-and-media/news-center/press-releases",
        "https://ir.adctherapeutics.com/press-releases/default.aspx",
        "https://www.investor.agilent.com/overview/default.aspx",
        "https://www.aadibio.com/latest-news/",
        // "https://investors.abbvie.com/",
        "https://investor.amerisourcebergen.com/overview/default.aspx",
        "https://investors.abcellera.com/",
        "https://corporate.abcam.com/investors/",
        "https://investors.abeonatherapeutics.com/",
        "https://arcabio.com/investors/",
        // "https://investors.acumenpharm.com/",
        // "https://investors.absci.com/",
        "https://www.abbott.com/investors.html",
        // "http://investor.arbutusbio.com/",
        "https://abvcpharma.com/?page_id=16826",
        "http://ir.acadia.com/",
        "https://www.sec.gov/edgar/browse/?CIK=0001897245",
        "https://www.sec.gov/edgar/browse/?CIK=0001897245",
        "https://www.sec.gov/edgar/browse/?CIK=0001897245",
        "https://www.auroramj.com/investors/",
        "https://www.acertx.com/investor-relations/press-releases/",
        // "https://investor.adicetbio.com/",
        "http://investors.aceragen.com/",
        "https://www.acadiahealthcare.com/investors/",
        // "https://ir.achillestx.com/",
        "https://ir.achievelifesciences.com/",
        // "http://ir.acimmune.com/",
        "https://ir.arcellx.com/",
        "https://ir.acorda.com/investors/default.aspx",
        // "http://investor.aclaristx.com/",
        // "https://ir.acrivon.com/",
        "https://www.acelrx.com/static-files/ce7dc6eb-429f-4926-b29c-bbc6df6df6a4",
        "https://www.acasti.com/en/investors",
        "https://ir.acurxpharma.com/",
        // "https://investor.adagene.com/",
        "https://www.adaptimmune.com/investors-and-media",
        "https://ir.adctherapeutics.com/overview/default.aspx",
        // "http://www.investor.adagene.com/news-events/news-releases",  // this one doesn't work for some reason?
    ];

    let dt = Local::now();

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
            println!("{date_to_search_for_v3}\n");

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

            println!("\nFinished!\n");
        }
    }
}
