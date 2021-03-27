#[macro_use]
extern crate clap;
use anyhow::Result;
use clap::App;

fn main() -> Result<()> {
    let (from, symbols, pool_num, file_name, server) = cli_args();

    pricefetchlib::run_program(symbols, from, pool_num, file_name, server)?;

    Ok(())
}

fn cli_args() -> (String, Vec<String>, String, Option<String>, bool) {
    let yaml = load_yaml!("app.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let symbols = matches
        .values_of("symbols")
        .unwrap()
        .map(String::from)
        .collect();
    let from = matches.value_of("from").unwrap().to_owned();
    let pool_num = matches.value_of("pool").unwrap().to_owned();
    let file_name = match matches.value_of("csv") {
        Some(name) => Some(name.to_owned()),
        None => None,
    };
    let server = matches.is_present("server");
    (from, symbols, pool_num, file_name, server)
}
