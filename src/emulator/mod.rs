use computer::memory::Memory;

pub struct ROM32K {
    memory: Memory,
}

impl ROM32K {
    pub fn new(prog: &Vec<[bool; 16]>) -> Self {
        let mut memory = Memory::new();
        for (a, &data) in prog.iter().enumerate() {
            let mut address = [false; 15];
            for i in 0..15 {
                address[i] = (1 << i) & a != 0;
            }
            memory.access(data, true, address);
        }
        ROM32K { memory: memory }
    }

    pub fn access(&mut self, address: [bool; 15]) -> [bool; 16] {
        self.memory.access([false; 16], false, address)
    }
}

pub fn load(content: &str) -> Vec<[bool; 16]> {
    let mut result = vec![];
    for line in content.trim().split('\n') {
        assert_eq!(line.len(), 16, "{}", line);
        let mut input = [false; 16];
        for (i, c) in line.chars().rev().enumerate() {
            input[i] = c == '1';
            assert!(c == '1' || c == '0');
        }
        result.push(input);
    }
    result
}
