use std::env;
use std::process;
use Cli::{Config,run,search_case_insensitive,search};


fn main()
{ 
    let args:Vec<String>=env::args().collect();
     let config =Config::parse_config(&args).unwrap_or_else(|_err|{
     eprintln!("Error occured :{}",err);
   process::exit(1);
} );

  if let Err(_e)=run(config){ 
    eprintln!("Error occurred :{}",_e);
    process::exit(1);
  }
    
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn case_sensitive() {
 let query = "duct";
 let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
 assert_eq!(
 vec!["safe, fast, productive."],
 search(query, contents)
 );
 }
 #[test]
 fn case_insensitive() {
 let query = "rUsT";
 let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
 assert_eq!(
 vec!["Rust:", "Trust me."],
 search_case_insensitive(query, contents)
 );
 }
}