use async_std::{fs::File, io::{BufWriter, prelude::WriteExt}};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use xactor::*;
//use yahoo::YahooError;
use crate::calc::*;
use yahoo_finance_api as yahoo;

/// Actor design
///
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
/// StockDataProcessor
/// Start - subscribed to <Quotes>
/// <Quotes>
/// - calculate stats
/// - publish<Output>
/// 
/// DataWriterStdout
/// Start - subscribed to <Output>
/// <Output>
/// - write to stdout
/// 
/// DataWriterCSV
/// Start - subscribed to <Output>
/// <Output>
/// - write to csv file
/// 


#[message]
#[derive(Debug, Clone)]
struct Quotes {
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
pub struct Output(String);

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

//#[derive(Default)]
pub struct StockDataProcessor;

#[async_trait]
impl Handler<Quotes> for StockDataProcessor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Quotes) {
        // Process a Quotes message here
        // convert to [f64]
        let prices: Vec<f64>;
        if msg.quotes.is_empty() {
            prices = vec![0.0 as f64];
        } else {
            prices = msg.quotes.iter().map(|q| q.close).collect();
        }
        let outputmsg = Output(output_to_stdout(&msg.symbol, msg.from, prices).await);
        Broker::from_registry().await.unwrap().publish(outputmsg).unwrap();
    }
}

#[async_trait]
impl Actor for StockDataProcessor {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like subscribing to a Broker
        ctx.subscribe::<Quotes>().await?;
        Ok(())
    }
}

//#[derive(Default)]
pub struct DataWriterStdout;

#[async_trait]
impl Actor for DataWriterStdout {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like subscribing to a Broker
        ctx.subscribe::<Output>().await?;
        println!("period start,symbol,price,change %,min,max,30d avg");
        Ok(())
    }
}

#[async_trait]
impl Handler<Output> for DataWriterStdout {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Output) {
        println!("{}", msg.0);
    }
}


//#[derive(Default)]
pub struct DataWriterCsv{
    writer: BufWriter<File>,
    // Write Buffer
    // Write File
}

impl DataWriterCsv {
    pub fn new(writer: BufWriter<File>) -> DataWriterCsv {
        DataWriterCsv {
            writer
        }
    }
}



#[async_trait]
impl Actor for DataWriterCsv {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like subscribing to a Broker
        ctx.subscribe::<Output>().await?;
        self.writer.write(b"period start,symbol,price,change %,min,max,30d avg");
        Ok(())
    }
}

#[async_trait]
impl Handler<Output> for DataWriterCsv {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Output) {
        println!("{}", msg.0);
    }
}



async fn output_to_stdout(sym: &str, from: DateTime<Utc>, prices: Vec<f64>) -> String {
    let last_price = *prices.last().unwrap();
    let change_percent = price_diff(&prices).await.unwrap_or((0.0, 0.0));
    let price_min = min(&prices).await.unwrap();
    let price_max = max(&prices).await.unwrap();
    let price_thirty_day = n_window_sma(30, &prices).await.unwrap_or(vec![0.0]);

    format!(
        "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
        from.to_rfc3339(),
        sym,
        last_price,
        change_percent.0 * 100.0,
        price_min,
        price_max,
        price_thirty_day.last().unwrap_or(&0.0),
    )
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
