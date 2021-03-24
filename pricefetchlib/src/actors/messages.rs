use chrono::{DateTime, Utc};
use xactor::*;
use yahoo_finance_api as yahoo;

#[message]
#[derive(Debug, Clone)]
pub struct Quotes {
    pub symbol: String,
    pub from: DateTime<Utc>,
    pub quotes: Vec<yahoo::Quote>,
}

#[message]
#[derive(Debug, Clone)]
pub struct QuoteRequest {
    pub symbol: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[message]
#[derive(Debug, Clone)]
pub struct Output(pub String);

#[message(result = "String")]
#[derive(Debug, Clone)]
pub struct GetSymbol(pub String);