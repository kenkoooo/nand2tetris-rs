pub mod translator;

#[cfg(test)]
mod tests {
    use super::*;
    use assembler;
    use tools;

    #[test]
    fn simple_add_test() {
        let lines = tools::read_file("tests/07/StackArithmetic/SimpleAdd/SimpleAdd.vm").unwrap();
        let lines = lines.trim().split("\n").collect();
        let assembly = translator::translate(&lines, "SimpleAdd").unwrap();
        let commands = assembly
            .iter()
            .map(|line| assembler::parser::parse(line).unwrap())
            .collect::<Vec<_>>();
        let commands = assembler::optimizer::optimize(&commands)
            .iter()
            .map(|cmd| cmd.clone().unwrap())
            .collect::<Vec<_>>();

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
        let assembly = translator::translate(&lines, "StackTest").unwrap();
        let lines = tools::read_file("tests/07/StackArithmetic/StackTest/StackTest.asm").unwrap();
        let lines = lines
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(assembly, lines);
    }
}
