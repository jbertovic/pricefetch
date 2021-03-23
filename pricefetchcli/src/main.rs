#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let (from, symbols, pool_num, file_name) = cli_args();
    
    pricefetchlib::run_program(symbols, from, pool_num, file_name).unwrap();

}

fn cli_args() -> (String, Vec<String>, String, Option<String>) {
    let yaml = load_yaml!("app.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let symbols = matches
        .values_of("symbols")
        .unwrap()
        .map(String::from)
        .collect();
    let from = matches.value_of("from").unwrap().to_owned();
    let pool_num = matches.value_of("pool").unwrap().to_owned();
    let file_name = 
        match matches.value_of("csv") {
            Some(name) => Some(name.to_owned()),
            None => None,
        };
    (from, symbols, pool_num, file_name)
}


