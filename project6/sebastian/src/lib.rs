use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

mod parser;
mod code;

pub struct Config {
    pub infile: String,
    pub outfile: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Wrong number of arguments: expected 2")
        }
        let infile = args[1].clone();
        let outfile = args[2].clone();
        Ok(Config {
            infile: infile,
            outfile: outfile,
        })
    }
}

fn symbol_table() -> HashMap<String, u16> {
    let mut table = HashMap::new();
    for n in 0..16 {
        let r = format!("R{}", n);
        table.insert(r, n);
    }

    table.insert(String::from("SP"), 0);
    table.insert(String::from("LCL"), 1);
    table.insert(String::from("ARG"), 2);
    table.insert(String::from("THIS"), 3);
    table.insert(String::from("THAT"), 4);
    table.insert(String::from("SCREEN"), 16384);
    table.insert(String::from("KBD"), 24576);
    table
}

pub fn assemble(config: Config) -> Result<(), Box<Error>> {
    let mut asm = File::open(config.infile)?;
    let mut contents = String::new();
    asm.read_to_string(&mut contents)?;
    //println!("Contents of infile:\n{}", contents);
    let mut hack = File::create(config.outfile)?;
    
    //first pass
    let mut symbols = symbol_table();
    let mut current_instruction: u16 = 0;
    for line in contents.lines() {
        let stripped = parser::strip_comments(line);
        if stripped == "" { continue }
        if parser::command_type(&stripped) == "L_COMMAND" {
            let symbol = parser::symbol(stripped)?;
            symbols.insert(String::from(symbol), current_instruction);
        } else {
            current_instruction += 1;
        }
    }
    
    //second pass
    let mut next_available_ram: u16 = 16;
    for line in contents.lines() {
        let half_parsed = parser::strip_comments(line);
        if half_parsed == "" { continue }
        let command = parser::command_type(&half_parsed);
        let mut word: u16 = 0b0;
        let symbol;
        let comp;
        let dest;
        let jump;
        match command {
            "A_COMMAND" => {
                symbol = parser::symbol(&half_parsed)?;
                let symbol_num: Result<u16, _> = symbol.parse::<u16>();
                if let Ok(symbol_num) = symbol_num {
                    word += symbol_num;
                } else {
                    let key = String::from(symbol);
                    match symbols.entry(key) {
                        Entry::Occupied(entry) => {
                            let address = entry.get();
                            word += *address;
                        }
                        
                        Entry::Vacant(entry) => {
                            let address = entry.insert(next_available_ram);
                            word += *address;
                            next_available_ram += 1;
                        }
                    }
                }
            }
            
            "L_COMMAND" => {
                continue
            }
            
            "C_COMMAND" => {
                comp = parser::comp(&half_parsed)?;
                dest = parser::dest(&half_parsed)?;
                jump = parser::jump(&half_parsed)?;
                
                let comp_code = code::comp(comp)? as u16;
                let dest_code = code::dest(dest)? as u16;
                let jump_code = code::jump(jump)? as u16;
                
                word += 0b1110_0000_0000_0000;
                word += comp_code << 6;
                word += dest_code << 3;
                word += jump_code;
            }
            _ => { }
        }
        let word_str = format!("{:016b}\n", word);
        hack.write_all(word_str.into_bytes().as_slice())?;
    }
    
    println!("Done!");
    Ok(())
}