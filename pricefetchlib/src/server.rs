use xactor::Addr;
use crate::DataStoreBuffer;
use crate::actors::Getn;

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
    app.at("/tail/:num").get(get_tail);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

// TODO: need to output JSON instead

async fn get_tail(req: tide::Request<State>) -> tide::Result<String> {
    let n: usize = req.param("num")?.parse().unwrap_or(0);
    let mut res = String::from("period start,symbol,price,change %,min,max,30d avg\n");
    let mut data = req.state().actor.call(Getn(n)).await.unwrap();
    if let Some(sym) = req.url().query() {
        let symbol: Vec<&str> = sym.split("=").collect();
        data = data.into_iter().filter(|l| l.contains(symbol[1])).collect();
    }
    res += &data.join("\n");
    Ok(res)
}

