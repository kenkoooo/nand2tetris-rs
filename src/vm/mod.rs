pub fn compile(x: &Vec<&str>) -> Result<Vec<String>, String> {
    let mut output = vec![];
    let mut stack_pointer = 256;

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
                            output.push(format!("@{}", stack_pointer));
                            output.push("M=D".to_owned());
                            stack_pointer += 1;
                        }
                        Err(_) => return Err(format!("Parse Error: {}", line)),
                    }
                } else {
                    unimplemented!();
                }
            }
            "add" => {
                output.push(format!("@{}", stack_pointer - 1));
                output.push("D=M".to_owned());
                output.push(format!("@{}", stack_pointer - 2));
                output.push("D=D+M".to_owned());
                output.push("M=D".to_owned());
                stack_pointer -= 1;
            }
            "pop" => unimplemented!(),
            _ => unimplemented!("{}", line),
        }
    }
    Ok(output)
}
