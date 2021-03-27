
use chrono::Utc;
use chrono::DateTime;
use std::fmt;
use serde::{Serialize, Serializer};
use crate::calc::*;


pub const DATA_HEADER: &str = "period start,symbol,price,change %,min,max,30d avg";

/// DataStamp creates the data model in memory to keep track, store
/// and move in memory the streamed data

#[derive(Clone, Serialize)]
pub struct DataStamp {
    from: String,
    symbol: String,
    #[serde(serialize_with = "round_serialize")]
    last: f64,
    #[serde(serialize_with = "round_serialize")]
    change: f64,
    #[serde(serialize_with = "round_serialize")]
    min: f64,
    #[serde(serialize_with = "round_serialize")]
    max: f64,
    #[serde(serialize_with = "round_serialize")]
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
        Some((percent, _)) => percent*100.0,
        None => 0.0,
    }
}

fn round_serialize<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let rounded = (x * 100.0).round() / 100.0;
    s.serialize_f64(rounded)
}