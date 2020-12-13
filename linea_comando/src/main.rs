use std::env;
use std::process;

use linea_comando::Config;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problema pasando el argumento: {}", err);
        process::exit(1);
    });

    println!("Buscando -> {}", config.query);
    println!("En el Fichero -> {}", config.filename);

    if let Err(e) = linea_comando::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
