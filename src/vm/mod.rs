pub fn compile(x: &Vec<&str>) -> Result<Vec<String>, String> {
    let mut output = vec![];
    output.push("@256".to_owned());
    output.push("D=A".to_owned());
    output.push("@SP".to_owned());
    output.push("M=D".to_owned());

    for &line in x.iter() {
        let line = line.split("//").next().unwrap().trim();
        if line.len() == 0 {
            continue;
        }
        let spilit_line: Vec<&str> = line.trim().split(" ").collect();
        assert!(spilit_line.len() > 0);
        match spilit_line[0] {
            "push" => {
                if spilit_line.len() >= 3 && spilit_line[1] == "constant" {
                    match spilit_line[2].parse::<i16>() {
                        Ok(n) => {
                            output.push(format!("@{}", n));
                            output.push("D=A".to_owned());
                            output.push("@SP".to_owned());
                            output.push("A=M".to_owned());
                            output.push("M=D".to_owned());
                            output.push("A=A+1".to_owned());
                            output.push("D=A".to_owned());
                            output.push("@SP".to_owned());
                            output.push("M=D".to_owned());
                        }
                        Err(_) => return Err(format!("Parse Error: {}", line)),
                    }
                } else {
                    unimplemented!();
                }
            }
            "add" => {
                output.push("@SP".to_owned());
                output.push("A=M".to_owned());
                output.push("A=A-1".to_owned());
                output.push("D=M".to_owned());
                output.push("A=A-1".to_owned());
                output.push("D=D+M".to_owned());
                output.push("M=D".to_owned());
                output.push("D=A+1".to_owned());
                output.push("@SP".to_owned());
                output.push("M=D".to_owned());
            }
            "pop" => unimplemented!(),
            _ => unimplemented!("{}", line),
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tools;

    #[test]
    fn simple_add_test() {
        let lines = tools::read_file("tests/07/StackArithmetic/SimpleAdd/SimpleAdd.vm").unwrap();
        let lines = lines.trim().split("\n").collect();
        let assembly = compile(&lines).unwrap();
        let lines = tools::read_file("tests/07/StackArithmetic/SimpleAdd/SimpleAdd.asm").unwrap();
        let lines = lines
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(assembly, lines);
    }
}
