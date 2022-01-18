
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(0);
    });

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}",e);

        process::exit(0);
    }
}