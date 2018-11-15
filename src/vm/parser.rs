use vm::model::{Segment, VMCommand};

pub fn parse_one_line(line: &str) -> Result<VMCommand, ()> {
    let line = line.split("//").next().unwrap().trim();
    if line.len() == 0 {
        return Ok(VMCommand::Comment);
    }

    let split_line: Vec<&str> = line.trim().split(" ").collect();
    assert!(split_line.len() > 0);
    match split_line[0] {
        "push" | "pop" => {
            if split_line.len() < 3 {
                return Err(());
            }

            let index = split_line[2].parse::<usize>();
            let segment = match split_line[1] {
                "local" => Ok(Segment::Local),
                "constant" => Ok(Segment::Constant),
                "static" => Ok(Segment::Static),
                _ => Err(()),
            };

            match (split_line[0], segment, index) {
                ("pop", Ok(segment), Ok(index)) => Ok(VMCommand::Pop(segment, index)),
                ("push", Ok(segment), Ok(index)) => Ok(VMCommand::Push(segment, index)),
                _ => Err(()),
            }
        }
        "eq" => Ok(VMCommand::EQ),
        "gt" => Ok(VMCommand::GT),
        "lt" => Ok(VMCommand::LT),
        "sub" => Ok(VMCommand::Sub),
        "add" => Ok(VMCommand::Add),
        "and" => Ok(VMCommand::And),
        "or" => Ok(VMCommand::Or),
        "neg" => Ok(VMCommand::Neg),
        "not" => Ok(VMCommand::Not),
        _ => unimplemented!("{}", line),
    }
}

pub fn parse(x: &Vec<&str>, file_label: &str) -> Result<Vec<VMCommand>, String> {
    x.iter()
        .map(|line| parse_one_line(line).map_err(|_| format!("Parse Error: {}", line)))
        .collect()
}
