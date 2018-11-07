pub mod formatter;
pub mod model;
pub mod optimizer;
pub mod parser;

use tools;

pub fn assemble(filename: &str) -> Result<Vec<String>, String> {
    tools::read_file(filename)
        .map_err(|_| format!("IO Error: {}", filename))
        .and_then(|content| {
            content
                .trim()
                .split('\n')
                .map(|line| parser::parse(line))
                .collect::<Result<Vec<_>, _>>()
        })
        .and_then(|commands| {
            optimizer::optimize(&commands)
                .iter()
                .cloned()
                .map(|result| result.and_then(|cmd| formatter::format_to_binary(&cmd)))
                .collect::<Result<Vec<_>, _>>()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assemble_test1() {
        let binary = assemble("tests/06/add/Add.asm").unwrap();
        let output = tools::read_file("tests/06/add/Add.hack")
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(output, binary);
    }

    #[test]
    fn assemble_test2() {
        let binary = assemble("tests/06/max/MaxL.asm").unwrap();
        let output = tools::read_file("tests/06/max/MaxL.hack")
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(output, binary);
    }

    #[test]
    fn assemble_test_max() {
        let binary = assemble("tests/06/max/Max.asm").unwrap();
        let output = tools::read_file("tests/06/max/Max.hack")
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(output, binary);
    }

    #[test]
    fn assemble_test_pongl() {
        let binary = assemble("tests/06/pong/PongL.asm").unwrap();
        let output = tools::read_file("tests/06/pong/PongL.hack")
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(output, binary);
    }
}
