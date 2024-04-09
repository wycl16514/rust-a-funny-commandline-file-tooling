Rust is aimming for the replacement of c++, the later is widly used for low-level and infrustructre building for software system, and its not friendly for user facing application
like desktop GUI or mobile app. Therefore the ecological niche for Rust is the same as c++, and rust is even better than c++ at this regard, in this section we will see how to use
rust to make a funny commandline file tooling.

Let's create a new folder and init a new project with following command:
```r
cargo new filetooling
```

we will make a vim like application, it can show content for given file and replace given text in the file by using certain command like vim, therefore we need to hightlight and 
searching given text in the file, and we need some crats to help us, in the cargo.toml, add the following dependencies:
```r
[dependencies]
text-colorizer="1"
regex="1"
```
text-colorizer helps us for text highlighting and regex helps searching text by using regular expression. Run "cargo run" and make sure we download those dependencies.Our app will take
four arguments from the commandline, one for regular expression for searching text, one for regular expression for text we want to replaced, one for the path of input file and one for
the path of output file, let's create a structure for these four arguments, in the main.rs, we add the following:
```r
#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}
```
We have seen before the "#[derive(Debug)]" is a directive that can instruct Rust compiler to generate some code that can let us print out the Argument structue by using the symbol 
"{:?}", Let's see how we can print some colorful text onto the console, we add a function to print usage text with green color:
```r
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - replace all occurrences of one string with the oter",
        "filetooling".green()
    );

    eprintln!(
        "{}",
        "Usage: filetooling <target> <replacement> <INPUT> <OUTPUT>".red()
    );
}

fn main() {
    print_usage()
}

```
the .green() will turn the given string with color of green, and .red() will turn the string with color of red when they are showing on the console, remember we need to bring the 
text_colorizer crat into our project by using "use text_colorizer::*;", running the above code we will get the following result:

<img width="680" alt="截屏2024-04-08 22 56 27" src="https://github.com/wycl16514/rust-a-funny-commandline-file-tooling/assets/7506958/76838b32-8ab3-4355-aeb5-535c4697ef49">

you can see the string "filetooling" is green on the console and string "Usage..." is red on the console. Now we can collect arguments from console and assembly them into the structure of Argument:
```r
fn parse_args() -> Arguments {
    //the first argument is name of the app
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
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

fn main() {
    let args = parse_args();
    /*
    attribute #[derive(Debug)] enable us to print Argument using {:?}
    */
    println!("{:?}", args);
}

```
Let's run the programm using command like "cargo run hello world file1.txt file2.txt", then the output will like following:
![截屏2024-04-09 14 54 21](https://github.com/wycl16514/rust-a-funny-commandline-file-tooling/assets/7506958/73a2f814-409d-4035-904c-7bcf751499a5)

Let's see how to read and write file by using Rust:
```r
fn main() {
    let args = parse_args();
    /*
    attribute #[derive(Debug)] enable us to print Argument using {:?}
    */
    println!("{:?}", args);

    /*
    Read and Write file, fs::read_to_string will get file content into a string buffer
    */
    let data = match fs::read_to_string(&args.filename) {
        Ok(content) => {
            println!("file content is: {}", content);
            //return the content to data
            content
        }
        Err(e) => {
            //open file error
            eprintln!(
                "{} failed to read from file: '{}': {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &data) {
        Ok(_) => {
            println!("{}", "file written ok".green());
        }
        Err(e) => {
            eprintln!(
                "{} failed to write file '{}': {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };
}

```
Running the code above will give result like following:

![截屏2024-04-09 15 06 52](https://github.com/wycl16514/rust-a-funny-commandline-file-tooling/assets/7506958/13a6cf81-7c19-4606-9c56-433ca2beb49b)

and we can see there is a new file named file2.txt created in the same diretory. Now let's do the find and replace like vim, we use regular expression to set the string we want to find and replace 
the occurrences of the string with another one, we need to use the regex crat this time:
```r
use regex::Regex;
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    /*
    compile a regular expression by using the target string, if it fail, it will
    return Err(regex::Error) because of the ? operator
     */
    let regex = Regex::new(target)?;
    /*
    replace_all return a new string with the given target string replaced by the
    replacement string
    */
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();
    /*
    attribute #[derive(Debug)] enable us to print Argument using {:?}
    */
    println!("{:?}", args);

    /*
    Read and Write file, fs::read_to_string will get file content into a string buffer
    */
    let data = match fs::read_to_string(&args.filename) {
        Ok(content) => {
            println!("file content is: {}", content);
            //return the content to data
            content
        }
        Err(e) => {
            //open file error
            eprintln!(
                "{} failed to read from file: '{}': {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {
            println!("{}", "file written ok".green());
        }
        Err(e) => {
            eprintln!(
                "{} failed to write file '{}': {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };
}

```
running the above code by using "cargo run orange apple file1.txt file2.txt", then you will see the content in file2.txt is the same as file1.txt except the word orange change to apple
