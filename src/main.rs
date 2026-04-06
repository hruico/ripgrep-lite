use std::env;
use std::fs;
use std::process;
use std::error::Error;
use myCLI::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    //unwrap_or_else: for the same functionality below

    let config: Config<'_>;
    match Config::build(&args) {
        Ok(val) => config = val,
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);

            // process exit 1 signifies that the code ended with some errors and as name suggests, it stops the process immediately
        }
    }

    println!("Searching for {}", config.query);
    println!("In File: {}", config.file_path);

    if let Err(e) = run(config){
        println!("Application Error: {e}");
        process::exit(2);
    };
}

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl Config<'_> {
    fn build<'a>(args: &'a Vec<String>) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let query = args.get(1).unwrap();
        let file_path = args.get(2).unwrap();

        return Ok(Config { query, file_path });
    }
}

// dyn means dynamic. Box<dyn Error> where Error is a trait gives us the flexibility to manage the errors type. 
fn run<'a>(config: Config<'a>) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    println!("With text: \n {}", contents);
    Ok(())
}
