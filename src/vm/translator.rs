pub fn translate(x: &Vec<&str>, file_label: &str) -> Result<Vec<String>, String> {
    let mut writer = VirtualMachineWriter { output: vec![] };
    writer.add_str("@256");
    writer.add_str("D=A");
    writer.add_str("@SP");
    writer.add_str("M=D");

    for &line in x.iter() {
        let line = line.split("//").next().unwrap().trim();
        if line.len() == 0 {
            continue;
        }

        let split_line: Vec<&str> = line.trim().split(" ").collect();
        assert!(split_line.len() > 0);
        match split_line[0] {
            "push" | "pop" => {
                if split_line.len() < 3 {
                    return Err(format!("Parse error: {}", line));
                }

                let index = split_line[2].parse::<usize>();
                if index.is_err() {
                    return Err(format!("Parse error: {}", line));
                }
                let index = index.unwrap();

                match split_line[0] {
                    "push" => writer.push(split_line[1], index),
                    "pop" => writer.pop(split_line[1], index),
                    _ => unreachable!(),
                }
            }
            "eq" | "gt" | "lt" => {
                writer.get_2_numbers_from_stack();
                match split_line[0] {
                    "eq" => {
                        let equal_label = writer.generate_label("equal_label");
                        let finish_label = writer.generate_label("finish_label");

                        // if D!=M, jump to equal_label and D will be 0
                        writer.add_str("D=M-D");
                        writer.add_str(&format!("@{}", equal_label));
                        writer.add_str("D;JNE"); // if D!=0, jump to equal_label

                        // if D==M, D will be -1 and jump to finish_label
                        writer.add_str("D=-1");
                        writer.add_str(&format!("@{}", finish_label));
                        writer.add_str("0;JMP"); // jump to finish_label

                        writer.add_str(&format!("({})", equal_label));
                        writer.add_str("D=0");

                        writer.add_str(&format!("({})", finish_label));
                    }
                    "lt" => {
                        // true if D > M
                        let larger_label = writer.generate_label("larger_label");
                        let finish_label = writer.generate_label("finish_label");

                        // if , jump to larger_label and D will be 0
                        writer.add_str("D=D-M");
                        writer.add_str(&format!("@{}", larger_label));
                        writer.add_str("D;JGT"); // if D>0, jump to larger_label

                        // if D<=M, D will be 0 and jump to finish_label
                        writer.add_str("D=0");
                        writer.add_str(&format!("@{}", finish_label));
                        writer.add_str("0;JMP"); // jump to finish_label

                        writer.add_str(&format!("({})", larger_label));
                        writer.add_str("D=-1");

                        writer.add_str(&format!("({})", finish_label));
                    }
                    "gt" => {
                        // true if M > D
                        let smaller_label = writer.generate_label("smaller_label");
                        let finish_label = writer.generate_label("finish_label");

                        // if , jump to smaller_label and D will be 0
                        writer.add_str("D=M-D");
                        writer.add_str(&format!("@{}", smaller_label));
                        writer.add_str("D;JGT"); // if D>0, jump to smaller_label

                        // if D>=M, D will be 0 and jump to finish_label
                        writer.add_str("D=0");
                        writer.add_str(&format!("@{}", finish_label));
                        writer.add_str("0;JMP"); // jump to finish_label

                        writer.add_str(&format!("({})", smaller_label));
                        writer.add_str("D=-1");

                        writer.add_str(&format!("({})", finish_label));
                    }
                    _ => unreachable!(),
                }
                writer.remove_2_and_push_data();
            }
            "add" | "sub" | "and" | "or" => {
                writer.get_2_numbers_from_stack();
                match split_line[0] {
                    "add" => writer.add_str("D=D+M"),
                    "sub" => writer.add_str("D=M-D"),
                    "and" => writer.add_str("D=M&D"),
                    "or" => writer.add_str("D=M|D"),
                    _ => unreachable!(),
                }
                writer.remove_2_and_push_data();
            }
            "neg" | "not" => {
                writer.add_str("@SP");
                writer.add_str("A=M");
                writer.add_str("A=A-1");
                match split_line[0] {
                    "neg" => writer.add_str("M=-M"),
                    "not" => writer.add_str("M=!M"),
                    _ => unreachable!(),
                }
                writer.add_str("D=A+1");
                writer.add_str("@SP");
                writer.add_str("M=D");
            }
            _ => unimplemented!("{}", line),
        }
    }
    Ok(writer.output)
}

struct VirtualMachineWriter {
    output: Vec<String>,
}

impl VirtualMachineWriter {
    fn add_str(&mut self, s: &str) {
        self.output.push(s.to_owned());
    }

    fn push(&mut self, segment: &str, index: usize) {
        match segment {
            "constant" => {
                self.add_str(&format!("@{}", index));
                self.add_str("D=A");
                self.add_str("@SP");
                self.add_str("A=M");
                self.add_str("M=D");
                self.add_str("A=A+1");
                self.add_str("D=A");
                self.add_str("@SP");
                self.add_str("M=D");
            }
            _ => unimplemented!(),
        }
    }

    fn pop(&mut self, segment: &str, index: usize) {
        unimplemented!()
    }

    fn generate_label(&self, label: &str) -> String {
        format!("{}{}", label, self.output.len())
    }

    /// Get 2 numbers from stack and 1st one will be D, and 2nd will be M.
    fn get_2_numbers_from_stack(&mut self) {
        self.add_str("@SP");
        self.add_str("A=M");
        self.add_str("A=A-1");
        self.add_str("D=M");
        self.add_str("A=A-1");
    }

    fn remove_2_and_push_data(&mut self) {
        self.add_str("@SP");
        self.add_str("A=M");
        self.add_str("A=A-1");
        self.add_str("A=A-1");
        self.add_str("M=D");
        self.add_str("D=A+1");
        self.add_str("@SP");
        self.add_str("M=D");
    }
}
