use chrono::Utc;
use chrono::DateTime;
use std::fmt;
use crate::calc::*;


pub const DATA_HEADER: &str = "period start,symbol,price,change %,min,max,30d avg";

/// DataStamp creates the data model in memory to keep track, store
/// and move in memory the streamed data

#[derive(Clone)]
pub struct DataStamp {
    from: String,
    symbol: String,
    last: f64,
    change: f64,
    min: f64,
    max: f64,
    sma_30: f64,
}

impl DataStamp {
    pub async fn parse_stream(sym: &str, from: DateTime<Utc>, prices: Vec<f64>) -> DataStamp {
        DataStamp {
            from: from.to_rfc3339(),
            symbol: sym.to_owned(),
            last: *prices.last().unwrap(),
            change: change(&prices).await,
            min: min(&prices).await.unwrap(),
            max: max(&prices).await.unwrap(),
            sma_30: sma_last(&prices).await,
        }
    }
    pub fn get_symbol(&self) -> &str {
        self.symbol.as_ref()
    }
}

impl fmt::Display for DataStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
            self.from,
            self.symbol,
            self.last,
            self.change,
            self.min,
            self.max,
            self.sma_30)
    }
}

async fn sma_last(ts: &Vec<f64>) -> f64 {
    match n_window_sma(30, ts).await {
        Some(list) => *list.last().unwrap(),
        None => 0.0,
    }
}

async fn change(ts: &Vec<f64>) -> f64 {
    match price_diff(ts).await {
        Some((percent, _)) => percent,
        None => 0.0,
    }
}
