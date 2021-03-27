pub mod calc;
pub mod actors;
pub mod utility;
pub mod server;
pub mod datastamp;

use chrono::{DateTime, Utc};
use std::{time::Duration, usize};
use async_std::{fs::File, io::BufWriter, prelude::*, task};
use async_std::stream;
use xactor::*;
use actors::{DataWriterCsv, DataWriterStdout, QuoteRequest, QuoteRouter, StockDataProcessor, DataStoreBuffer};
use server::run_server;

// move visibility to front of crate
use datastamp::DataStamp;
use datastamp::DATA_HEADER;

const BUFFER_SIZE: usize = 5_000;
const INTERVAL_SEC: usize = 30;

pub fn run_program(symbols: Vec<String>, from: String, pool_num: String, file_name: Option<String>, server: bool) -> Result<()> {

    // setup parameters for actors
    let from_date: DateTime<Utc> = utility::date_parse(&from).unwrap();
    let pool_size:usize = pool_num.parse::<usize>().unwrap_or(5);

    task::block_on(
        async {
            let _router = Supervisor::start(move || QuoteRouter::new(pool_size)).await.unwrap();
            let _processor = StockDataProcessor.start().await.unwrap();
            let _screen_writer = DataWriterStdout.start().await.unwrap();

            let (_data_store, _server) =
                match server {
                    true => {
                        let dsb = DataStoreBuffer::new(BUFFER_SIZE).start().await.unwrap();
                        let dsbs = dsb.clone();
                        let s = task::spawn( run_server(dsbs) );
                        (Some(dsb), Some(s))
                    },
                    false => (None, None)
                };

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
        
            let mut interval = stream::interval(Duration::from_secs(INTERVAL_SEC as u64));
        
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
