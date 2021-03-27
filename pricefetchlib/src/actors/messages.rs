use crate::DataStamp;
use chrono::{DateTime, Utc};
use xactor::*;
use yahoo_finance_api as yahoo;

#[message]
#[derive(Clone)]
pub struct Quotes {
    pub symbol: String,
    pub from: DateTime<Utc>,
    pub quotes: Vec<yahoo::Quote>,
}

#[message]
#[derive(Clone)]
pub struct QuoteRequest {
    pub symbol: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[message(result = "Vec<DataStamp>")]
#[derive(Clone)]
pub struct Getn(pub usize);

#[message]
#[derive(Clone)]
pub struct TimeStamp(pub DataStamp);
