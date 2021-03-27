use super::{Getn, TimeStamp};
use crate::DataStamp;
use async_trait::async_trait;
use std::collections::VecDeque;
use xactor::*;

/// DataStore
/// Start - subscribed to <Output>
/// <TimeStamp>
/// - split off symbol
/// - store record by symbol
/// <Getn>
/// - return last n records
///

#[derive(Default)]
pub struct DataStoreBuffer {
    store: VecDeque<DataStamp>,
}

impl DataStoreBuffer {
    pub fn new(n: usize) -> DataStoreBuffer {
        DataStoreBuffer {
            store: VecDeque::with_capacity(n),
        }
    }
}

#[async_trait]
impl Handler<TimeStamp> for DataStoreBuffer {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: TimeStamp) {
        // check size of store: if >= n then delete on from the front
        if self.store.len() >= self.store.capacity() {
            self.store.pop_back();
        }
        // append new output
        self.store.push_front(msg.0);
    }
}

#[async_trait]
impl Handler<Getn> for DataStoreBuffer {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Getn) -> Vec<DataStamp> {
        self.store
            .iter()
            .take(msg.0)
            .map(|o| o.to_owned())
            .collect()
    }
}

#[async_trait]
impl Actor for DataStoreBuffer {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<TimeStamp>().await?;
        Ok(())
    }
}
