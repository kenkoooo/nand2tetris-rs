pub fn compile(x: &Vec<&str>, file_label: &str) -> Result<Vec<String>, String> {
    let mut output = vec![];
    output.add("@256");
    output.add("D=A");
    output.add("@SP");
    output.add("M=D");

    for &line in x.iter() {
        let line = line.split("//").next().unwrap().trim();
        if line.len() == 0 {
            continue;
        }
        let split_line: Vec<&str> = line.trim().split(" ").collect();
        assert!(split_line.len() > 0);
        match split_line[0] {
            "push" => {
                if split_line.len() >= 3 && split_line[1] == "constant" {
                    match split_line[2].parse::<i16>() {
                        Ok(n) => {
                            output.add(&format!("@{}", n));
                            output.add("D=A");
                            output.add("@SP");
                            output.add("A=M");
                            output.add("M=D");
                            output.add("A=A+1");
                            output.add("D=A");
                            output.add("@SP");
                            output.add("M=D");
                        }
                        Err(_) => return Err(format!("Parse Error: {}", line)),
                    }
                } else {
                    unimplemented!();
                }
            }
            "eq" | "gt" | "lt" => {
                output.get_2_numbers_from_stack();
                match split_line[0] {
                    "eq" => {
                        let equal_label = format!("{}{}", "equal_label", output.len());
                        let finish_label = format!("{}{}", "finish_label", output.len());

                        // if D!=M, jump to equal_label and D will be 0
                        output.add("D=M-D");
                        output.add(&format!("@{}", equal_label));
                        output.add("D;JNE"); // if D!=0, jump to equal_label

                        // if D==M, D will be -1 and jump to finish_label
                        output.add("D=-1");
                        output.add(&format!("@{}", finish_label));
                        output.add("0;JMP"); // jump to finish_label

                        output.add(&format!("({})", equal_label));
                        output.add("D=0");

                        output.add(&format!("({})", finish_label));
                    }
                    "lt" => {
                        // true if D > M
                        let larger_label = format!("{}{}", "larger_label", output.len());
                        let finish_label = format!("{}{}", "finish_label", output.len());

                        // if , jump to larger_label and D will be 0
                        output.add("D=D-M");
                        output.add(&format!("@{}", larger_label));
                        output.add("D;JGT"); // if D>0, jump to larger_label

                        // if D<=M, D will be 0 and jump to finish_label
                        output.add("D=0");
                        output.add(&format!("@{}", finish_label));
                        output.add("0;JMP"); // jump to finish_label

                        output.add(&format!("({})", larger_label));
                        output.add("D=-1");

                        output.add(&format!("({})", finish_label));
                    }
                    "gt" => {
                        // true if M > D
                        let smaller_label = format!("{}{}", "smaller_label", output.len());
                        let finish_label = format!("{}{}", "finish_label", output.len());

                        // if , jump to smaller_label and D will be 0
                        output.add("D=M-D");
                        output.add(&format!("@{}", smaller_label));
                        output.add("D;JGT"); // if D>0, jump to smaller_label

                        // if D>=M, D will be 0 and jump to finish_label
                        output.add("D=0");
                        output.add(&format!("@{}", finish_label));
                        output.add("0;JMP"); // jump to finish_label

                        output.add(&format!("({})", smaller_label));
                        output.add("D=-1");

                        output.add(&format!("({})", finish_label));
                    }
                    _ => unreachable!(),
                }
                output.remove_2_and_push_data();
            }
            "add" | "sub" | "and" | "or" => {
                output.get_2_numbers_from_stack();
                match split_line[0] {
                    "add" => output.add("D=D+M"),
                    "sub" => output.add("D=M-D"),
                    "and" => output.add("D=M&D"),
                    "or" => output.add("D=M|D"),
                    _ => unreachable!(),
                }
                output.remove_2_and_push_data();
            }
            "neg" | "not" => {
                output.add("@SP");
                output.add("A=M");
                output.add("A=A-1");
                match split_line[0] {
                    "neg" => output.add("M=-M"),
                    "not" => output.add("M=!M"),
                    _ => unreachable!(),
                }
                output.add("D=A+1");
                output.add("@SP");
                output.add("M=D");
            }
            "pop" => unimplemented!(),
            _ => unimplemented!("{}", line),
        }
    }
    Ok(output)
}

trait PushStringRef {
    fn add(&mut self, s: &str);
    fn get_2_numbers_from_stack(&mut self) {
        self.add("@SP");
        self.add("A=M");
        self.add("A=A-1");
        self.add("D=M");
        self.add("A=A-1");
    }

    fn remove_2_and_push_data(&mut self) {
        self.add("@SP");
        self.add("A=M");
        self.add("A=A-1");
        self.add("A=A-1");
        self.add("M=D");
        self.add("D=A+1");
        self.add("@SP");
        self.add("M=D");
    }
}

impl PushStringRef for Vec<String> {
    fn add(&mut self, s: &str) {
        self.push(s.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tools;

    #[test]
    fn simple_add_test() {
        let lines = tools::read_file("tests/07/StackArithmetic/SimpleAdd/SimpleAdd.vm").unwrap();
        let lines = lines.trim().split("\n").collect();
        let assembly = compile(&lines, "SimpleAdd").unwrap();
        let lines = tools::read_file("tests/07/StackArithmetic/SimpleAdd/SimpleAdd.asm").unwrap();
        let lines = lines
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(assembly, lines);
    }

    #[test]
    fn stack_test() {
        let lines = tools::read_file("tests/07/StackArithmetic/StackTest/StackTest.vm").unwrap();
        let lines = lines.trim().split("\n").collect();
        let assembly = compile(&lines, "StackTest").unwrap();
        let lines = tools::read_file("tests/07/StackArithmetic/StackTest/StackTest.asm").unwrap();
        let lines = lines
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(assembly, lines);
    }
}
