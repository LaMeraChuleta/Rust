use structopt::StructOpt;
use std::env;

#[derive(StructOpt)]
struct Cli {

    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::from_args();
    let file = std::fs::File::open(&args.path)?;  
    let mut buf_reader = std::io::BufReader::new(file);


    for line in buf_reader {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}
