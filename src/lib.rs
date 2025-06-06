use std::fs;
use std::path::Path;
use std::error::Error;
use colored::*;

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

pub fn show(ips: &Vec<String>) -> Result<(), Box<dyn Error>> {
	for i in ips{
        print!("{} ",i.red().on_white());
    }
    println!("");
	Ok(())
}

pub fn see_folder(dir: &Path) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				print!("{} ", file_name.truecolor(40, 200, 200));
		}
	}
	Ok(())
}

pub fn find_out(dir: &Path, pattern: &str) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
                if file_name.contains(pattern) {
                    print!("{} ",file_name.truecolor(40, 200, 200));
                }
                let way=entry.path();
                if way.is_dir() && way!=dir {
                    if let Err(ref e) = find_out(&way,pattern) {
		            return Err(e.to_string().into()); 
	                }
                }
		}
	}
	Ok(())
}

pub fn search<'a>(query: &'a str, file: &'a str)->Vec<&'a str>{
    let mut ans=Vec::new();
    for line in file.lines(){
        if line.contains(query){
            ans.push(line.trim());
        }
    }
    ans
}

fn show_file(content: &String)->Result<(),Box<dyn Error>>{
    println!("{}",content);
    println!("");
    Ok(())
}

pub fn run(input: Input)->Result<(),Box<dyn Error>>{
    match &input.query[..] {
        "echo" => {
            if let Err(ref e) = show(&input.cmnd){
		        return Err(e.to_string().into()); 
	        } 
        },
        "ls" => {
            if let Err(ref e) = see_folder(Path::new(".")) {
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
                let ans = search(word,&content);
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
                if let Err(ref e) = show_file(&content){
		            return Err(e.to_string().into());
	            }
                println!("");
            }
        },
        "find" => {
            let pattern=&input.cmnd[0];
            if let Err(ref e) = find_out(Path::new("."), &pattern) {
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
        if let Err(ref e) = show(&v){
		        println!("{}", e);
	        }
    }
}