use assembler::model::{Command, Jump, Operator, Place};

fn parse_internal(line: &str) -> Result<Command, ()> {
    let line = line.trim();
    if line.is_empty() {
        Ok(Command::Comment)
    } else if &line[..1] == "@" {
        match &line[1..].parse::<i16>() {
            Ok(a) => Ok(Command::Address(*a)),
            Err(_) => Ok(Command::AddressSymbol(line[1..].to_owned())),
        }
    } else if &line[1..2] == "=" && line.len() == 3 {
        Place::parse(&line[0..1])
            .and_then(|dest| Place::parse(&line[2..3]).map(|src| Command::Assign { dest, src }))
    } else if &line[1..2] == "=" && line.len() == 5 {
        let dest = Place::parse(&line[0..1]);
        let left = Place::parse(&line[2..3]);
        let operator = Operator::parse(&line[3..4]);
        let right = Place::parse(&line[4..5]);

        match (dest, left, operator, right) {
            (Ok(dest), Ok(left), Ok(operator), Ok(right)) => Ok(Command::Operation {
                dest,
                left,
                operator,
                right,
            }),
            _ => Err(()),
        }
    } else if line.contains(";") {
        let cmds = line.split(";").collect::<Vec<&str>>();
        if cmds.len() != 2 {
            Err(())
        } else {
            let comp = Place::parse(cmds[0]);
            let jump = Jump::parse(cmds[1]);
            match (comp, jump) {
                (Ok(comp), Ok(jump)) => Ok(Command::Jump {
                    dest: comp,
                    jump: jump,
                }),
                _ => Err(()),
            }
        }
    } else {
        Err(())
    }
}

pub fn parse(line: &str) -> Result<Command, String> {
    match line.trim().split("//").next() {
        Some(line) => parse_internal(line).map_err(|_| format!("Parse error: {}", line)),
        _ => unreachable!(),
    }
}
