use async_std::task;
use std::time::Instant;
//use async_std::prelude::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseError, Utc};
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;

mod calc;
use calc::*;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let start_time = Instant::now();

    let (from, symbols) = cli_args();
    let from_date: DateTime<Utc> = date_parse(&from).unwrap();
    let to_date: DateTime<Utc> = Utc::now();

    println!("period start,symbol,price,change %,min,max,30d avg");

    let mut tasks = Vec::with_capacity(symbols.len());

    // create task for each symbol to run fetch_price and output to CSV format
    for sym in symbols {
        tasks.push(task::spawn(async move {
            run_output(&sym, &from_date, &to_date).await;
        }));
    }

    // await all tasks to complete
    task::block_on(async {
        for t in tasks {
            t.await;
        }
    });

    println!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );
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

async fn run_output(sym: &str, from_date: &DateTime<Utc>, to_date: &DateTime<Utc>) {
    match fetch_price(&sym, &from_date, &to_date, "1d").await {
        Ok((_, prices)) => {
            let last_price = *prices.last().unwrap();
            let change_percent = price_diff(&prices).await.unwrap().0;
            let price_min = min(&prices).await.unwrap();
            let price_max = max(&prices).await.unwrap();
            let price_thirty_day = n_window_sma(30, &prices).await.unwrap();

            println!(
                "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
                from_date.to_rfc3339(),
                sym,
                last_price,
                change_percent * 100.0,
                price_min,
                price_max,
                price_thirty_day.last().unwrap_or(&0.0),
            );
        }
        Err(e) => eprintln!("Error on symbol {}: {}", &sym, e),
    }
}

async fn fetch_price(
    symbol: &str,
    start: &DateTime<Utc>,
    end: &DateTime<Utc>,
    interval: &str,
) -> Result<(Vec<String>, Vec<f64>), yahoo::YahooError> {
    let provider = yahoo::YahooConnector::new();
    let response = provider
        .get_quote_history_interval(symbol, *start, *end, interval)
        .await?;
    let quotes = response.quotes()?;
    let mut timestamps = Vec::new();
    let mut prices = Vec::new();
    for q in quotes {
        let t: DateTime<Utc> = DateTime::from(UNIX_EPOCH + Duration::from_secs(q.timestamp));
        timestamps.push(t.to_rfc3339());
        prices.push(q.close);
    }
    Ok((timestamps, prices))
}

fn date_parse(date_str: &str) -> Result<DateTime<Utc>, ParseError> {
    // From string to a NaiveDate
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    // Add some default time to convert it into a NaiveDateTime
    let naive_datetime: NaiveDateTime = naive_date.and_hms(0, 0, 0);
    // Add a timezone to the object to convert it into a DateTime<UTC>
    Ok(DateTime::<Utc>::from_utc(naive_datetime, Utc))
}
