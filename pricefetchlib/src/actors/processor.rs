use super::{Quotes, TimeStamp};
use crate::DataStamp;
use async_trait::async_trait;
use xactor::*;

/// StockDataProcessor
/// Start - subscribed to <Quotes>
/// <Quotes>
/// - calculate stats
/// - publish<TimeStamp>

pub struct StockDataProcessor;

#[async_trait]
impl Handler<Quotes> for StockDataProcessor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Quotes) {
        // Process a Quotes message here
        // convert to [f64]
        let prices: Vec<f64>;
        if msg.quotes.is_empty() {
            prices = vec![0.0_f64];
        } else {
            prices = msg.quotes.iter().map(|q| q.close).collect();
        }
        let ts = TimeStamp(DataStamp::parse_stream(&msg.symbol, msg.from, prices).await);
        Broker::from_registry().await.unwrap().publish(ts).unwrap();
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
