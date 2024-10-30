use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Read String From {}", config.file_path);
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have access to read file");
    println!("{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("No enough arguments");
        };
        Ok(Self { query: args[1].clone(), file_path: args[2].clone() })
    }
}

