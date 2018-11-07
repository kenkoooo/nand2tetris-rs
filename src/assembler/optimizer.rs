use assembler::model::Command;
use std::collections::BTreeMap;

pub fn optimize(commands: &Vec<Command>) -> Vec<Result<Command, String>> {
    let (map, _) = commands
        .iter()
        .filter(|command| match command {
            Command::Comment => false,
            _ => true,
        })
        .fold(
            (BTreeMap::new(), 0),
            |(mut map, mut count): (BTreeMap<String, i16>, i16), command| {
                match command {
                    Command::Label(label) => {
                        map.insert(label.clone(), count);
                    }
                    _ => {
                        count += 1;
                    }
                }
                (map, count)
            },
        );
    let mut variables = BTreeMap::new();
    commands
        .iter()
        .filter(|command| match command {
            Command::Comment | Command::Label(_) => false,
            _ => true,
        })
        .map(|command| match command {
            Command::AddressSymbol(symbol) => match map.get(symbol) {
                Some(&address) => Ok(Command::Address(address)),
                None => Err(()),
            }
            .or_else(|_| {
                match symbol.as_str() {
                    "SP" => Ok(0),
                    "LCL" => Ok(1),
                    "ARG" => Ok(2),
                    "THIS" => Ok(3),
                    "THAT" => Ok(4),
                    _ => Err(()),
                }
                .map(|address| Command::Address(address))
            })
            .or_else(|_| {
                if &symbol[..1] == "R" {
                    match symbol[1..].parse::<i16>() {
                        Ok(address) => Ok(Command::Address(address)),
                        Err(_) => Err(format!("Symbol Error: {}", symbol)),
                    }
                } else {
                    Err(format!("Symbol Error: {}", symbol))
                }
            })
            .or_else(|_| {
                let variable_num = variables.len() as i16;
                let address = *variables.entry(symbol).or_insert(variable_num + 16);
                Ok(Command::Address(address))
            }),
            _ => Ok(command.clone()),
        })
        .collect()
}
