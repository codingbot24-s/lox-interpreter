use std::env::args;
use std::fs;
// TODO:
pub fn run_file (path:&str) {
    let bytes = fs::read(path)
        .expect("error reading the file");
}

pub fn run_prompt () {
    println!("running prompt")
}

pub fn main() {
    let args:Vec<String> = args().collect();
    if args.len() > 1 {
        println!("Usage: jlox [script]");
    }
    else if args.len() == 1 {
        run_file("");        
    }
    else {
        run_prompt();
    }

}

