use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        eprintln!(
            "{} wrong number of arguments, expected 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn print_usage() {
    eprintln!(
        "{} -replace all occurrences of one string with the other",
        "filetoling".green().bold()
    );

    eprintln!(
        "{}",
        "Usage: filetooling <target> <replacement> <INPUT> <OUTPUT>".red()
    );
}

use regex::Regex;
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);
    //Result, Ok(content), Err(err)
    let data = match fs::read_to_string(&args.filename) {
        Ok(content) => {
            println!("file content is: {}", content.green().bold());
            content
        }
        Err(err) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error".red(),
                args.filename.green(),
                err
            );
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{} failed to replace text: {:?}", "Error".red().bold(), err);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {
            println!("{}", "file written ok".green());
        }
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}' , {:?}",
                "Error".red(),
                args.output,
                e
            );
            std::process::exit(1);
        }
    };
}
