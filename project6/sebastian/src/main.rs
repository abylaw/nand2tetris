extern crate hack_assembler;

use std::env;
use std::process;
use hack_assembler::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Compiling assembly file {} into binary file {}", config.infile, config.outfile);
    
    if let Err(e) = hack_assembler::assemble(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}