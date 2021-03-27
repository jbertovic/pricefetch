use async_std::{fs::File, io::{BufWriter, prelude::WriteExt}};
use async_trait::async_trait;
use xactor::*;
use crate::DATA_HEADER;
use super::TimeStamp;

/// DataWriterStdout
/// Start - subscribed to <Output>
/// <TimeStamp>
/// - write to stdout
/// 
/// DataWriterCSV
/// Start - subscribed to <Output>
/// <TimeStamp>
/// - write to csv file
/// 

pub struct DataWriterStdout;

#[async_trait]
impl Actor for DataWriterStdout {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<TimeStamp>().await?;
        println!("{}", DATA_HEADER);
        Ok(())
    }
}

#[async_trait]
impl Handler<TimeStamp> for DataWriterStdout {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: TimeStamp) {
        println!("{}", msg.0);
    }
}

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
        ctx.subscribe::<TimeStamp>().await?;
        self.writer.write(format!("{}\n", DATA_HEADER).as_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl Handler<TimeStamp> for DataWriterCsv {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: TimeStamp) {
        self.writer.write(format!("{}\n", msg.0).as_bytes()).await.unwrap();
        self.writer.flush().await.unwrap();
    }
}