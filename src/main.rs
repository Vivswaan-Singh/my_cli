use std::env;
use std::process;
use my_cli::Input;

fn main() {
    let args:Vec<String>=env::args().collect();
    let input=Input::new(&args).unwrap_or_else(|err |{
        eprintln!("Problem parsing arguments: {:?}", err);
        process::exit(1);
    });

    if let Err(e)= my_cli::run(input){
        eprintln!("Error in run function: {:?}", e);
        process::exit(1);
    }
}



