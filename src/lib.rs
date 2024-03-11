use std::fs;
use std::error::Error;
use std::env;

#[derive(PartialEq, PartialOrd)]
pub struct Config{ 
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool
}

impl Config { 
  pub   fn parse_config(args:&[String])->Result<Config,&'static str>
{ 
    if args.len() <2{
        return Err("Arguments are not enough");
    }
   
    let query =args[1].clone();
    let filename=args[2].clone();
    let case_sensitive=env::var("CASE_INSENSITIVE").is_err();
    Ok(Config{query,filename,case_sensitive})
}
}

pub fn run (args:Config)->Result<(),Box<dyn Error>>{ 
    
    let contents=fs::read_to_string(args.filename).expect("Error in reading");

    let results=if args.case_sensitive{ 
        search(&args.query,&contents)
    }
    else{ 
        search_case_insensitive(&args.query,&contents)
    };
      for line in search(&args.query,&contents){
        println!("{}",line);
      }
    println!("The contents are :{}",contents);
    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>
{ 
    let mut  results=Vec::new();
    for line in contents.lines(){ 
        if line.contains(query){ 
            println!("The word is found:{}",query);
            results.push(line);
        }

    }
    results
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>
{ 
    let query=query.to_lowercase();
    let mut results=Vec::new();
    for line in contents.lines(){ 
       if line.to_lowercase().contains(&query){ 
        results.push(line);
       }
    }
    results
}