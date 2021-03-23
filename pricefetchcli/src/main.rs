#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let (from, symbols, pool_num) = cli_args();
    
    pricefetchlib::run_program(symbols, from, pool_num).unwrap();

}

fn cli_args() -> (String, Vec<String>, String) {
    let yaml = load_yaml!("app.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let symbols = matches
        .values_of("symbols")
        .unwrap()
        .map(String::from)
        .collect();
    let from = matches.value_of("from").unwrap().to_owned();
    let pool_num = matches.value_of("pool").unwrap().to_owned();
    (from, symbols, pool_num)
}


