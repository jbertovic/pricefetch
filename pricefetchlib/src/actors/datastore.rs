use std::collections::HashMap;

use async_trait::async_trait;
use xactor::*;
use super::{Output, GetSymbol};

/// DataStore
/// Start - subscribed to <Output>
/// <Output>
/// - split off symbol
/// - store record by symbol
/// <Get>
/// - return record for symbol
/// 

#[derive(Default)]
pub struct DataStore(HashMap<String, String>);

impl DataStore {
    pub fn new() -> DataStore {
        DataStore(HashMap::new())
    }
}

#[async_trait]
impl Handler<Output> for DataStore {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Output) {
        // Split symbol from Output located at 2nd index position
        let symbol = msg.0.split(',').take(2).last().unwrap().to_string();
        self.0.insert(symbol, msg.0);
    }
}

#[async_trait]
impl Handler<GetSymbol> for DataStore {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetSymbol) -> String {
        match self.0.get(&msg.0) {
            Some(response) => response.to_string(),
            None => String::new(),
        }
    }
}

#[async_trait]
impl Actor for DataStore {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<Output>().await?;
        Ok(())
    }
}


