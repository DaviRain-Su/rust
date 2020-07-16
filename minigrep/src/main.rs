extern crate minigrep;
use std::process;
use std::env;


use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    //let query = &args[1];
    //let filename = &args[2];

    //let config = prase_config(&args);
    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("problem parsing arguments : {}", err);
        process::exit(1);
    });

    println!("--------------------------");
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("--------------------------");

    //run(config);
    if let Err(e) = minigrep::run(config) {
        println!("Application error : {}", e);
        process::exit(1);
    }
    //println!("{:?}", args);

//    let mut f = File::open(config.filename).expect("file not found");

//    let mut contents = String::new();

//    f.read_to_string(&mut contents)
//        .expect("something went wrong reading the file");

//    println!("With text : \n{}", contents);
    
}

//fn run(config : Config){
//    let mut f = File::open(config.filename).expect("file not found");

//    let mut contents = Stirng::new();
//    f.read_to_string(&mut contents).expect("something went reading the file");

//    println!("With text: \n{}", contents);
//}


/*
fn run (config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text : \n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}
*/

//fn prase_config(args: &[String]) -> (&str, &str) {
//    let query = &args[1];
//    let filename = &args[2];

//    (query, filename)
//}

//fn prase_config(args: &[String]) -> Config {
//    let query = args[1].clone();
//    let filename = args[2].clone();

//    Config{ query, filename }
//}

/*
impl Config {
    fn new(args: &[String]) -> Config{
        if args.len () < 3 {
            panic!("not enought arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config{ query, filename } 
    }
}
*/


/*
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str > {
        if args.len () < 3 {
            return Err("not enought arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{ query, filename }) 
    }
}
*/
