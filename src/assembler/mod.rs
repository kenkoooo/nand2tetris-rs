pub mod formatter;
pub mod model;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use tools;

    #[test]
    fn assemble_test() {
        let binary = tools::read_file("tests/06/add/Add.asm")
            .unwrap()
            .trim()
            .split('\n')
            .map(|line| parser::parse(line))
            .filter(|result| result != &Ok(model::Command::Comment))
            .map(|result| result.and_then(|cmd| formatter::format_to_binary(&cmd)))
            .collect::<Result<Vec<_>, ()>>()
            .unwrap();

        let output = tools::read_file("tests/06/add/Add.hack")
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(output, binary);
    }

}
