use assembler::model::{Command, Jump, Operator, Place};

fn parse_assign(line: &str) -> Result<Command, ()> {
    let line: Vec<&str> = line.split('=').collect();
    if line.is_empty() || line.len() > 2 {
        Err(())
    } else if line[1].contains("+")
        || line[1].contains("-")
        || line[1].contains("&")
        || line[1].contains("|")
    {
        let mut prefix = line[1]
            .split(|c| c == '+' || c == '-' || c == '&' || c == '|')
            .collect::<Vec<_>>();
        if prefix.len() != 2 {
            Err(())
        } else {
            if prefix[0] == "" {
                prefix[0] = "0";
            }
            let dest = Place::parse(line[0]);
            let left = Place::parse(prefix[0]);
            let right = Place::parse(prefix[1]);
            let operator = if line[1].contains("+") {
                Operator::Plus
            } else if line[1].contains("-") {
                Operator::Minus
            } else if line[1].contains("&") {
                Operator::And
            } else {
                Operator::Or
            };
            match (dest, left, right) {
                (Ok(dest), Ok(left), Ok(right)) => Ok(Command::Operation {
                    dest: dest,
                    left: left,
                    right: right,
                    operator: operator,
                }),
                _ => Err(()),
            }
        }
    } else {
        let dest = Place::parse(line[0]);
        let src = Place::parse(line[1]);
        match (dest, src) {
            (Ok(dest), Ok(src)) => Ok(Command::Assign {
                dest: dest,
                src: src,
            }),
            _ => Err(()),
        }
    }
}

fn parse_internal(line: &str) -> Result<Command, ()> {
    let line = line.trim();
    if line.is_empty() {
        Ok(Command::Comment)
    } else if &line[..1] == "@" {
        match &line[1..].parse::<i16>() {
            Ok(a) => Ok(Command::Address(*a)),
            Err(_) => Ok(Command::AddressSymbol(line[1..].to_owned())),
        }
    } else if line.contains("=") {
        parse_assign(line)
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
    } else if &line[..1] == "(" {
        let label = &line[1..(line.len() - 1)];
        Ok(Command::Label(label.to_owned()))
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
