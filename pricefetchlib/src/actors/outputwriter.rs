use async_std::{fs::File, io::{BufWriter, prelude::WriteExt}};
use async_trait::async_trait;
use xactor::*;
use super::Output;

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
    pub writer: BufWriter<File>,
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
        self.writer.write(b"period start,symbol,price,change %,min,max,30d avg\n").await?;
        Ok(())
    }
}

#[async_trait]
impl Handler<Output> for DataWriterCsv {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Output) {
        self.writer.write(format!("{}\n", msg.0).as_bytes()).await.unwrap();
        self.writer.flush().await.unwrap();
    }
}