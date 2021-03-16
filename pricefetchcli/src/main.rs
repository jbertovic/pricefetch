#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let (from, symbols) = cli_args();
    
    pricefetchlib::run_program(symbols, from).unwrap();

}

fn cli_args() -> (String, Vec<String>) {
    let yaml = load_yaml!("app.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let symbols = matches
        .values_of("symbols")
        .unwrap()
        .map(String::from)
        .collect();
    let from = matches.value_of("from").unwrap().to_owned();
    (from, symbols)
}


