use std::fs;
use std::path::Path;
use std::error::Error;
use colored::*;
pub mod cmds;

#[derive(Debug)]
pub struct Input{
    pub query: String,
    pub cmnd: Vec<String>
}

impl Input{
    pub fn new(args: &Vec<String>)->Result<Input,&str>{
        if args.len()<2{
            return Err("NOT ENOUGH ARGUMENTS!!!");
        }
        let query=args[1].clone(); 
        let mut cmnd:Vec<String>=vec![];
        if args.len()>2 {
            cmnd=args[2..].to_vec(); 
        }
        Ok(Input{query,cmnd}) 
    }
}

pub fn run(input: Input)->Result<(),Box<dyn Error>>{
    match &input.query[..] {
        "echo" => {
            if let Err(ref e) = cmds::echo::show(&input.cmnd){
		        return Err(e.to_string().into()); 
	        } 
        },
        "ls" => {
            if let Err(ref e) = cmds::ls::see_folder(Path::new(".")) {
                return Err(e.to_string().into()); 
	        }       
        },
        "grep" => {
            if input.cmnd.is_empty(){
                return Err("Not pattern to be searched given".into());
            }
            if input.cmnd.len()<2{
                return Err("Not file(s) to be searched given".into());
            }
            let word=&input.cmnd[0];
            for i in 1..input.cmnd.len(){
                let content=fs::read_to_string(&input.cmnd[i])?;
                let ans = cmds::grep::search(word,&content);
                let mut cnt=1;
                println!("Result for query {} Document {}",word.red(),&input.cmnd[i].yellow());
                for line in ans{
                    println!("Line {cnt}. {:?}",line);
                    cnt+=1;
                }
                println!("");
            }
        },
        "cat" => {
            if input.cmnd.is_empty(){
                return Err("No filename(s) given".into()); 
            }
            for i in input.cmnd{
                println!("Document {} :",i.purple());
                let content=fs::read_to_string(i.clone())?;
                if let Err(ref e) = cmds::cat::show_file(&content){
		            return Err(e.to_string().into());
	            }
                println!("");
            }
        },
        "find" => {
            let pattern=&input.cmnd[0];
            if let Err(ref e) = cmds::find::find_out(Path::new("."), &pattern) {
		        return Err(e.to_string().into()); 
	        } 
        },
        _ => {
            eprintln!("Command named {:?} does not exist. Try again.",input.query);
            return Err("No such command exists".into()); 
            
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_echo(){
        let v=vec![String::from("1")];
        if let Err(ref e) = cmds::echo::show(&v){
		        println!("{}", e);
	        }
    }
}