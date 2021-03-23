use async_trait::async_trait;
use xactor::*;
use super::messages::{QuoteRequest, Quotes};
use yahoo_finance_api as yahoo;
use chrono::{DateTime, Utc};

/// QuoteRouter
/// Start 
/// - create pool of 5 QuoteDownloader actors and 1 StockDataProcessor
/// - subscribe to <QuoteRequest>
/// <QuoteRequest>
/// - Use one of the pooled QuoteDownloader actors
///
/// QuoteDownloader
/// <QuoteRequest>
/// - will use yahoo api to fetch quotes
/// - Broker to publish <Quotes> message
///

pub struct QuoteRouter {
    pub quotedownloader: Vec<Addr<QuoteDownloader>>,
    pub poolsize: usize,
    index: usize,
}

impl QuoteRouter {
    pub fn new(poolsize: usize) -> QuoteRouter {
        QuoteRouter {
            quotedownloader: Vec::new(),
            poolsize,
            index: 0,
        }
    }
}

#[async_trait]
impl Actor for QuoteRouter {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like downloading at a specific interval, or subscribing to a Broker
        //let qdaddr = QuoteDownloader.start().await.unwrap();
        for _i in 0..self.poolsize {
            self.quotedownloader
                .push(QuoteDownloader.start().await.unwrap());
        }
        ctx.subscribe::<QuoteRequest>().await?;
        Ok(())
    }
}

#[async_trait]
impl Handler<QuoteRequest> for QuoteRouter {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: QuoteRequest) {
        if self.index + 1 >= self.quotedownloader.len() {
            self.index = 0;
        } else {
            self.index += 1;
        }
        self.quotedownloader[self.index].send(msg).unwrap();
    }
}

//#[derive(Default)]
pub struct QuoteDownloader;

#[async_trait]
impl Handler<QuoteRequest> for QuoteDownloader {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: QuoteRequest) {
        // Process a QuoteRequest message
        let q: Quotes;
        match fetch_price(&msg.symbol, msg.start, msg.end, "1d").await {
            Ok(resquotes) => {
                q = Quotes {
                    symbol: msg.symbol.clone(),
                    from: msg.start,
                    quotes: resquotes,
                }
            }
            Err(_) => {
                q = Quotes {
                    symbol: msg.symbol.clone(),
                    from: msg.start,
                    quotes: vec![],
                }
            }
        }
        Broker::from_registry().await.unwrap().publish(q).unwrap();
    }
}

#[async_trait]
impl Actor for QuoteDownloader {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like downloading at a specific interval, or subscribing to a Broker
        Ok(())
    }
}

async fn fetch_price(
    symbol: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    interval: &str,
) -> Result<Vec<yahoo::Quote>> {
    let provider = yahoo::YahooConnector::new();
    let response = provider
        .get_quote_history_interval(symbol, start, end, interval)
        .await?;
    let quotes = response.quotes()?;
    Ok(quotes)
}