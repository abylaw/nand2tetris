pub fn dest(mnemonic: &str) -> Result<u8, &'static str> {
    let dest = match mnemonic {
        "null"  => { 0b000 }
        "M"     => { 0b001 }
        "D"     => { 0b010 }
        "MD"    => { 0b011 }
        "A"     => { 0b100 }
        "AM"    => { 0b101 }
        "AD"    => { 0b110 }
        "AMD"   => { 0b111 }
        _       => { 0b1000 } //should never happen
    };
    if dest > 0b111 {return Err("unable to code bad dest mnemonic")}
    Ok(dest)
}

pub fn comp(mnemonic: &str) -> Result<u8, &'static str> {
    let mut comp = if mnemonic.contains("M") {
        0b0100_0000
    } else { 0 };
    let parsed = mnemonic.replace("M", "A");
    let parsed_mnemonic = parsed.as_str();
    comp += match parsed_mnemonic {
        "0"     => { 0b101010 }
        "1"     => { 0b111111 }
        "-1"    => { 0b111010 }
        "D"     => { 0b001100 }
        "A"     => { 0b110000 }
        "!D"    => { 0b001101 }
        "!A"    => { 0b110001 }
        "-D"    => { 0b001111 }
        "-A"    => { 0b110011 }
        "D+1"   => { 0b011111 }
        "A+1"   => { 0b110111 }
        "D-1"   => { 0b001110 }
        "A-1"   => { 0b110010 }
        "D+A"   => { 0b000010 }
        "D-A"   => { 0b010011 }
        "A-D"   => { 0b000111 }
        "D&A"   => { 0b000000 }
        "D|A"   => { 0b010101 }
        _       => { 0b1000_0000 }
    };
    if comp > 0b0111_1111 {return Err("unable to code bad comp mnemonic")}
    Ok(comp)
}

pub fn jump(mnemonic: &str) -> Result<u8, &'static str> {
    let jump = match mnemonic {
        "null" => { 0b000 }
        "JGT" => { 0b001 }
        "JEQ" => { 0b010 }
        "JGE" => { 0b011 }
        "JLT" => { 0b100 }
        "JNE" => { 0b101 }
        "JLE" => { 0b110 }
        "JMP" => { 0b111 }
        _     => { 0b1000 } //should never happen
    };
    if jump > 0b111 {return Err("unable to code bad jump mnemonic")}
    Ok(jump)
}