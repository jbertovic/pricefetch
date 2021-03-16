pub mod calc;
pub mod actors;
pub mod utility;

use chrono::{DateTime, Utc};
use std::time::Duration;
use async_std::{prelude::*, task};
use async_std::stream;
use xactor::*;
use actors::{QuoteRequest, QuoteRouter, StockDataProcessor};


pub fn run_program(symbols: Vec<String>, from: String) -> Result<()> {

    let from_date: DateTime<Utc> = utility::date_parse(&from).unwrap();

    println!("period start,symbol,price,change %,min,max,30d avg");

    task::block_on(
        async {
            let _router = Supervisor::start(|| QuoteRouter::new(5)).await.unwrap();
            let _processor = StockDataProcessor.start().await.unwrap();
        
            let mut symbroker: Addr<Broker<QuoteRequest>> = Broker::from_registry().await.unwrap();
        
            let mut interval = stream::interval(Duration::from_secs(10));
        
            while let Some(_) = interval.next().await {
                let to_date: DateTime<Utc> = Utc::now();
                for sym in &symbols {
                    let msg = QuoteRequest {
                        symbol: sym.clone(),
                        start: from_date,
                        end: to_date,
                    };
                    symbroker.publish(msg).unwrap();
                }
            };
        }
    );

    Ok(())
}
