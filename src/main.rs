#![allow(unused)]
use clap::Parser;
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main () -> Result<(),Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let mut count = 0;
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => {
            for line in content.lines(){
                if line.contains(&args.pattern){
                    count += 1;
                    println!("Pattern: {}\n{}: {}", args.pattern,count,line);
                };
        }
    },
        Err(error) => {return Err(error.into());}
    };
    Ok(())
}
