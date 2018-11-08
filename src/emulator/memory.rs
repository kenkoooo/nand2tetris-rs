use computer::memory::MemoryTrait;

pub struct EmulatedMemory {
    mem: [[bool; 16]; 32768],
}

impl MemoryTrait for EmulatedMemory {
    fn new() -> Self {
        EmulatedMemory {
            mem: [[false; 16]; 32768],
        }
    }
    fn access(&mut self, input: [bool; 16], load: bool, address: [bool; 15]) -> [bool; 16] {
        let mut a = 0;
        for i in 0..15 {
            if address[i] {
                a += 1 << i;
            }
        }
        let out = self.mem[a];
        if load {
            self.mem[a] = input;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tools;

    fn bit_16_to_15(x: [bool; 16]) -> [bool; 15] {
        let mut res = [false; 15];
        for i in 0..15 {
            res[i] = x[i];
        }
        res
    }

    #[test]
    fn emulated_memory_test() {
        let t = tools::read_test_data("tests/05/Memory.cmp").unwrap();
        let mut iter = t[1..].iter().map(|t| {
            let input = tools::convert16(t[1].parse::<i16>().unwrap());
            let load = t[2] == "1";
            let address = tools::convert16_str(&("0".to_owned() + &t[3]));
            let out = tools::convert16(t[4].parse::<i16>().unwrap());

            (input, load, bit_16_to_15(address), out)
        });
        let mut memory = EmulatedMemory::new();

        while let Some((input, load, address, t_out)) = iter.next() {
            let out = memory.access(input, load, address);
            assert_eq!(out, t_out);
        }
    }
}
