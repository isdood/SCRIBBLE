use colored::*;
use std::env;
use std::path::PathBuf;
use std::process;

mod compiler;

use compiler::safety::SafetyLevel;
use compiler::parser::SparkParser;
use compiler::codegen::SparkCodeGen;

fn print_usage() {
    println!("Usage: forge compile <file> [--test]");
    println!("Options:");
    println!("  --test    Run in test mode");
}

fn compile(file: PathBuf, test: bool) -> Result<(), String> {
    println!("{} Compiling {}...", "✨".purple(), file.display());
    
    let mut parser = SparkParser::new();
    parser.parse_file(&file)?;
    
    if test {
        println!("{} Running in test mode...", "✨".purple());
    }
    
    let codegen = SparkCodeGen::new(SafetyLevel::default());
    codegen.generate()?;
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let file = PathBuf::from(&args[2]);
    let test = args.iter().any(|arg| arg == "--test");

    match command.as_str() {
        "compile" => {
            if let Err(e) = compile(file, test) {
                eprintln!("{} Error: {}", "✨".purple(), e);
                process::exit(1);
            }
        },
        _ => {
            eprintln!("{} Unknown forge command: {}", "✨".purple(), command);
            print_usage();
            process::exit(1);
        }
    }
}
