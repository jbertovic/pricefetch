use std::collections::VecDeque;

use async_trait::async_trait;
use xactor::*;
use super::{Output, Getn};

/// DataStore
/// Start - subscribed to <Output>
/// <Output>
/// - split off symbol
/// - store record by symbol
/// <Getn>
/// - return last n records
/// 

#[derive(Default)]
pub struct DataStoreBuffer{
    store: VecDeque<String>,
    n: usize,
}

impl DataStoreBuffer {
    pub fn new(n: usize) -> DataStoreBuffer {
        DataStoreBuffer {
            store: VecDeque::with_capacity(n),
            n,
        }
    }
}

#[async_trait]
impl Handler<Output> for DataStoreBuffer {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Output) {
        // check size of store: if >= n then delete on from the front
        if self.store.len() >= self.n {
            self.store.pop_back();
        }
        // append new output
        self.store.push_front(msg.0);
    }
}

#[async_trait]
impl Handler<Getn> for DataStoreBuffer {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Getn) -> Vec<String> {
        self.store.iter().take(msg.0).map(|o| o.to_owned()).collect()
    }
}

#[async_trait]
impl Actor for DataStoreBuffer {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<Output>().await?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[async_std::test]
    async fn check_datastorebuffer_overcapacity() {
        let dsb = DataStoreBuffer::new(5).start().await.unwrap();
        let test: Vec<&str> = vec!["1", "2", "3", "4", "5", "6"];
        for s in test { 
            dsb.call(Output(String::from_str(s).unwrap())).await.unwrap();
        }

        // check proper order - latest data first and that it rolls
        assert_eq!(dsb.call(Getn(1)).await.unwrap(), vec!(String::from("6")));
        assert_eq!(dsb.call(Getn(3)).await.unwrap(), 
            vec!(String::from("6"),
                String::from("5"),
                String::from("4"),
        ));

        // check asking for more but only gives the latest
        assert_eq!(dsb.call(Getn(6)).await.unwrap(), 
            vec!(String::from("6"),
                String::from("5"),
                String::from("4"),
                String::from("3"),
                String::from("2"),
        ));

    }

}