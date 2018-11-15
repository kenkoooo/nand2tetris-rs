pub mod model;
pub mod parser;
pub mod translator;

#[cfg(test)]
mod tests {
    use super::*;
    use assembler;
    use tools;

    #[test]
    fn simple_add_test() {
        let lines = tools::read_file("tests/07/StackArithmetic/SimpleAdd/SimpleAdd.vm").unwrap();
        let lines = lines
            .trim()
            .split("\n")
            .map(|line| parser::parse_one_line(line).unwrap())
            .collect();
        let translator = translator::Translator::new("SimpleAdd", 256, 300, 400, 3000, 3010);
        let assembly = translator.translate(&lines).unwrap();
        let commands = assembly
            .iter()
            .map(|line| assembler::parser::parse(line).unwrap())
            .collect::<Vec<_>>();
        let commands = assembler::optimizer::optimize(&commands)
            .iter()
            .map(|cmd| cmd.clone().unwrap())
            .collect::<Vec<_>>();
        let mut runner = assembler::runner::Runner::new();
        runner.run(&commands);
        assert_eq!(runner.memory[0], 257);
        assert_eq!(runner.memory[256], 15);
    }

    #[test]
    fn stack_test() {
        let lines = tools::read_file("tests/07/StackArithmetic/StackTest/StackTest.vm").unwrap();
        let lines = lines
            .trim()
            .split("\n")
            .map(|line| parser::parse_one_line(line).unwrap())
            .collect();
        let translator = translator::Translator::new("StackTest", 256, 300, 400, 3000, 3010);
        let assembly = translator.translate(&lines).unwrap();
        let commands = assembly
            .iter()
            .map(|line| assembler::parser::parse(line).unwrap())
            .collect::<Vec<_>>();
        let commands = assembler::optimizer::optimize(&commands)
            .iter()
            .map(|cmd| cmd.clone().unwrap())
            .collect::<Vec<_>>();
        let mut runner = assembler::runner::Runner::new();
        runner.run(&commands);
        assert_eq!(runner.memory[0], 266);
        assert_eq!(runner.memory[256], -1);
        assert_eq!(runner.memory[257], 0);
        assert_eq!(runner.memory[258], 0);
        assert_eq!(runner.memory[259], 0);
        assert_eq!(runner.memory[260], -1);
        assert_eq!(runner.memory[261], 0);
        assert_eq!(runner.memory[262], -1);
        assert_eq!(runner.memory[263], 0);
        assert_eq!(runner.memory[264], 0);
        assert_eq!(runner.memory[265], -91);
    }

    #[test]
    fn basic_test() {
        let lines = tools::read_file("tests/07/MemoryAccess/BasicTest/BasicTest.vm").unwrap();
        let lines = lines
            .trim()
            .split("\n")
            .map(|line| parser::parse_one_line(line).expect(line))
            .collect();

        let translator = translator::Translator::new("BasicTest", 256, 300, 400, 3000, 3010);
        let assembly = translator.translate(&lines).unwrap();
        let commands = assembly
            .iter()
            .map(|line| assembler::parser::parse(line).unwrap())
            .collect::<Vec<_>>();
        let commands = assembler::optimizer::optimize(&commands)
            .iter()
            .map(|cmd| cmd.clone().unwrap())
            .collect::<Vec<_>>();
        let mut runner = assembler::runner::Runner::new();
        runner.run(&commands);
    }

}
