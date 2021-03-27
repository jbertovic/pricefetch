//use tide::Body;
use tide::Body;
use xactor::Addr;
use crate::DataStoreBuffer;
use crate::actors::Getn;
use crate::DATA_HEADER;

//use std::sync::Arc;

#[derive(Clone)]
struct State {
    actor: Addr<DataStoreBuffer>,
}

impl State {
    fn new(actor: Addr<DataStoreBuffer>) -> Self {
        Self {
            actor,
        }
    }
}

pub async fn run_server(ds_actor: Addr<DataStoreBuffer>) -> tide::Result<()> {

    let mut app = tide::with_state(State::new(ds_actor));

    app.at("/").get(|_| async { Ok("*Test Server for LiveProject*") });
    app.at("/text/:num").get(get_tail);
    app.at("/tail/:num").get(get_tail_json);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn get_tail(req: tide::Request<State>) -> tide::Result {
    let n: usize = req.param("num")?.parse().unwrap_or(0);
    let mut res = String::from(format!("{}\n",DATA_HEADER));
    let mut data = req.state().actor.call(Getn(n)).await.unwrap();
    if let Some(sym) = req.url().query() {
        let symbol: Vec<&str> = sym.split("=").collect();
        if symbol.len() > 1 {
            data = data.into_iter().filter(|ds| ds.get_symbol().contains(symbol[1])).collect();
        }
    }
    res += &data.into_iter().fold(String::new(), |acc, ds| acc+format!("{}\n",ds).as_str());
    Ok(res.into())
}

async fn get_tail_json(req: tide::Request<State>) -> tide::Result {
    let n: usize = req.param("num")?.parse().unwrap_or(0);
    let mut data = req.state().actor.call(Getn(n)).await.unwrap();
    if let Some(sym) = req.url().query() {
        let symbol: Vec<&str> = sym.split("=").collect();
        if symbol.len() > 1 {
            data = data.into_iter().filter(|ds| ds.get_symbol().contains(symbol[1])).collect();
        }
    }
    let body = Body::from_json(&data)?;
    Ok(body.into())
}

