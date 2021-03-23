pub mod calc;
pub mod actors;
pub mod utility;

use chrono::{DateTime, Utc};
use std::time::Duration;
use async_std::{fs::File, io::BufWriter, prelude::*, task};
use async_std::stream;
use xactor::*;
use actors::{DataWriterCsv, DataWriterStdout, QuoteRequest, QuoteRouter, StockDataProcessor};


pub fn run_program(symbols: Vec<String>, from: String, pool_num: String, file_name: Option<String>) -> Result<()> {

    // setup parameters for actors
    let from_date: DateTime<Utc> = utility::date_parse(&from).unwrap();
    let pool_size:usize = pool_num.parse::<usize>().unwrap_or(5);

    // see if csv file option is selected and get filename
    // create File, Writer and Buffer to handle output to csv
    // quite if there are any errors


    // pass csv option along to setup to actor system somehow?

    task::block_on(
        async {
            let _router = Supervisor::start(move || QuoteRouter::new(pool_size)).await.unwrap();
            let _processor = StockDataProcessor.start().await.unwrap();
            let _screen_writer = DataWriterStdout.start().await.unwrap();

            let _file_writer = 
                match file_name {
                    Some(name) => {
                        let file = File::create(name).await.unwrap();
                        let writer = BufWriter::new(file);
                        Some(DataWriterCsv{writer}.start().await.unwrap())
                    },
                    None => None,
                };
        
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
