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

you can see the string "filetooling" is green on the console and string "Usage..." is red on the console
