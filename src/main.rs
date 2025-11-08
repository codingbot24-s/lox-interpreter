use std::env::args;

// TODO:
pub fn run_file () {
    println!("running run file")
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
        run_file();        
    }
    else {
        run_prompt();
    }

}

