use async_trait::async_trait;
use xactor::*;
use super::{Quotes, Output};
use chrono::{DateTime, Utc};
use crate::calc::*;

/// StockDataProcessor
/// Start - subscribed to <Quotes>
/// <Quotes>
/// - calculate stats
/// - publish<Output>

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
