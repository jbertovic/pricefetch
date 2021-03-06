// Need to add currency and percent format
//

use argh::FromArgs;
use yahoo_finance_api as yahoo;
use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc, ParseError};
use std::time::{UNIX_EPOCH, Duration};

/// Struct to contain CLI arguments and configuration
#[derive(FromArgs)]
/// Fetch close history from Yahoo Finance API
struct PriceFetch {
    /// from date in format yyyy-mm-dd
    #[argh(positional)]
    from: String,
    /// define symbols to fetch
    #[argh(positional)]
    symbols: Vec<String>,
}

fn main() {
    let sym: PriceFetch = argh::from_env();
    let from_date: DateTime<Utc> = date_parse(&sym.from).unwrap();
    let to_date: DateTime<Utc> = Utc::now();

    println!("period start,symbol,price,change %,min,max,30d avg");

    // run fetch_price for all symbols and output to CSV format
    for sym in &sym.symbols {
       // fetch prices
       match fetch_price(sym, &from_date, &to_date, "1d") {
        Ok((_, prices)) => {

            let last_price = *prices.last().unwrap();
            let change_percent = price_diff(&prices).unwrap().0;
            let price_min = min(&prices).unwrap();
            let price_max = max(&prices).unwrap();
            let price_thirty_day = n_window_sma(30,&prices).unwrap();

            println!("{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}", 
                from_date.to_rfc3339(), 
                sym,
                last_price,
                change_percent,
                price_min,
                price_max,
                price_thirty_day.last().unwrap_or(&0.0),
            );
        },
        Err(e) => eprintln!("Error: {}", e),
       }
    }

}

fn fetch_price(symbol: &str, start: &DateTime<Utc>, end: &DateTime<Utc>, interval: &str) -> Result<(Vec<String>, Vec<f64>), yahoo::YahooError> {
    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_history_interval(symbol, start.clone(), end.clone(), interval)?;
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

fn date_parse(date_str: &str) -> Result<DateTime<Utc>, ParseError>  {
    // From string to a NaiveDate
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    // Add some default time to convert it into a NaiveDateTime
    let naive_datetime: NaiveDateTime = naive_date.and_hms(0,0,0);
    // Add a timezone to the object to convert it into a DateTime<UTC>
    Ok(DateTime::<Utc>::from_utc(naive_datetime, Utc))
}

fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        let mut v = series.to_vec();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Some(v[0])
    }
}

fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        let mut v = series.to_vec();
        v.sort_by(|a, b| b.partial_cmp(a).unwrap());
        Some(v[0])
    }
}

fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.len() < n { 
        None 
    } else {
        let sma: Vec<f64> = series.windows(n).map(|slice|slice.iter().sum::<f64>()/(30 as f64)).collect();
        Some(sma)
    }
}

fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.is_empty() {
        None
    } else {
        let start = *series.first().unwrap();
        let end = *series.last().unwrap();
        if end==start {
            Some((0.0, 0.0))
        } else {
            Some(((start-end)/start, start-end))
        }
    }
}
