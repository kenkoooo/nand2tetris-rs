pub mod formatter;
pub mod model;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use tools;

    #[test]
    fn assemble_test() {
        let source_code = tools::read_file("tests/06/add/Add.asm").unwrap();
        let commands = parser::parse(&source_code).unwrap();
        let binary = formatter::format_to_binary(&commands).unwrap();

        let output = tools::read_file("tests/06/add/Add.hack")
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(output, binary);
    }

}
