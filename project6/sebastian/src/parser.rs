pub fn command_type(line: &str) -> &str {
    if line.starts_with("@") {
        return "A_COMMAND"
    } else if line.starts_with("(") {
        return "L_COMMAND"
    } else {
        return "C_COMMAND"
    }
}

pub fn strip_comments(line: &str) -> &str {
    if line.trim().starts_with("//") {return ""}
    let stripped;
    let comment = line.find("//");
    stripped = match comment {
        Some(i) => { &line[0..i] }
        
        None => { &line }
    };
    stripped.trim()
}

pub fn symbol(line: &str) -> Result<&str, &'static str> {
    let command = command_type(line);
    match command {
        "A_COMMAND" => {
            return Ok(&line[1..line.len()])
        }
        "L_COMMAND" => {
            return Ok(&line[1..(line.len() - 1)])
        }
        _ => {
            return Err("unable to parse symbol from C_COMMAND")
        }
    }
}

pub fn dest(line: &str) -> Result<&str, &'static str> {
    if command_type(line) != "C_COMMAND" {
        return Err("unable to parse dest from non-C_COMMAND")
    }
    let eqsign = line.find("=");
    let dest;
    match eqsign {
        Some(i) => {
            dest = &line[0..i];
        }
        None => {
            dest = "null";
        }
    }
    Ok(dest)
}

pub fn comp(line: &str) -> Result<&str, &'static str> {
    if command_type(line) != "C_COMMAND" {
        return Err("unable to parse comp from non-C_COMMAND")
    }
    let eqsign = line.find("=");
    let semicolon = line.find(";");
    let len = line.len();
    let comp;
    match eqsign {
        Some(i) => {
            match semicolon {
                Some(j) => {
                    comp = &line[(i+1)..j];
                }
                None => {
                    comp = &line[(i+1)..len];
                }
            }
        }
        None => {
            match semicolon {
                Some(j) => {
                    comp = &line[0..j];
                }
                None => {
                    return Err("unable to parse comp from bad C_COMMAND")
                }
            }
        }
    }
    Ok(comp)
}

pub fn jump(line: &str) -> Result<&str, &'static str> {
    if command_type(line) != "C_COMMAND" {
        return Err("unable to parse jump from non-C_COMMAND")
    }
    let semicolon = line.find(";");
    let len = line.len();
    let jump;
    match semicolon {
        Some(i) => {
            jump = &line[(i+1)..len];
        }
        None => {
            jump = "null";
        }
    }
    Ok(jump)
}