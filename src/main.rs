use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use std::time::Duration;
//use yahoo_finance_api as yahoo;
use xactor::*;
mod calc;
//use calc::*;

mod actors;
use actors::{QuoteDownloader, StockDataProcessor, QuoteRequest};

#[macro_use]
extern crate clap;
use clap::App;

#[xactor::main]
async fn main() -> Result<()> {

    let (from, symbols) = cli_args();
    let from_date: DateTime<Utc> = date_parse(&from).unwrap();
    let to_date: DateTime<Utc> = Utc::now();

    println!("period start,symbol,price,change %,min,max,30d avg");

    let _downloader = Supervisor::start(||QuoteDownloader).await?;
    let _processor = Supervisor::start(||StockDataProcessor).await?;

    let mut symbroker: Addr<Broker<QuoteRequest>> = Broker::from_registry().await?;

    for sym in symbols {
        let msg = QuoteRequest {
            symbol: sym,
            start: from_date,
            end: to_date,
            };
        symbroker.publish(msg)?;
    }

    //wait for an actor to exit
    sleep(Duration::from_secs(5)).await;

    Ok(())
}

fn cli_args() -> (String, Vec<String>) {
    let yaml = load_yaml!("app.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let symbols = matches
        .values_of("symbols")
        .unwrap()
        .map(String::from)
        .collect();
    let from = matches.value_of("from").unwrap().to_owned();
    (from, symbols)
}


fn date_parse(date_str: &str) -> Result<DateTime<Utc>> {
    // From string to a NaiveDate
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    // Add some default time to convert it into a NaiveDateTime
    let naive_datetime: NaiveDateTime = naive_date.and_hms(0, 0, 0);
    // Add a timezone to the object to convert it into a DateTime<UTC>
    Ok(DateTime::<Utc>::from_utc(naive_datetime, Utc))
}
