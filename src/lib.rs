use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config{
        let query:String = args[1].clone();
        let file_path:String = args[2].clone();
        Config { query, file_path }
    }
}

pub fn run<'a>(args: &[String]) -> Result<(), Box<dyn Error>>{

    if args.len() < 3 {
        let num_args = args.len() - 1;
        let err_msg = format!("Expected at least 2 arguments, got {num_args}.");
        return Err(err_msg.into());
    }

    let mut configs: Vec<Config> = vec![];

    for (index, __arg) in args.iter().enumerate() {
        if index < 2 {continue;}
        let temp_args: Vec<String> = vec![String::from("placeholder").clone(), args[1].clone(), args[index].clone()];
        let config = Config::new(&temp_args);
        configs.push(config);
    }

    for config in configs {

        let contents:String = fs::read_to_string(&config.file_path)?;

        println!("In the file {}:", config.file_path);
        
        for line in contents.lines(){
            if line.contains(&config.query){
            println!("{line}");
            }
        }

        if args.len() > 3{
            println!(); // Print an empty line if between each file
        }
    }

    Ok(())
}
