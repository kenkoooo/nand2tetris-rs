use elements::circuits::RAM64;
use elements::gates;

fn load_address3(address: [bool; 3]) -> [bool; 8] {
    use self::gates::{and, not};
    [
        and(not(address[0]), and(not(address[1]), not(address[2]))),
        and(address[0], and(not(address[1]), not(address[2]))),
        and(not(address[0]), and(address[1], not(address[2]))),
        and(address[0], and(address[1], not(address[2]))),
        and(not(address[0]), and(not(address[1]), address[2])),
        and(address[0], and(not(address[1]), address[2])),
        and(not(address[0]), and(address[1], address[2])),
        and(address[0], and(address[1], address[2])),
    ]
}

fn and16way(input: [bool; 16], sel: bool) -> [bool; 16] {
    use self::gates::and;
    [
        and(input[0], sel),
        and(input[1], sel),
        and(input[2], sel),
        and(input[3], sel),
        and(input[4], sel),
        and(input[5], sel),
        and(input[6], sel),
        and(input[7], sel),
        and(input[8], sel),
        and(input[9], sel),
        and(input[10], sel),
        and(input[11], sel),
        and(input[12], sel),
        and(input[13], sel),
        and(input[14], sel),
        and(input[15], sel),
    ]
}

struct RAM512 {
    ram64s: [RAM64; 8],
}
impl RAM512 {
    fn new() -> Self {
        RAM512 {
            ram64s: [
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
            ],
        }
    }
    fn set(&mut self, input: [bool; 16], addr: [bool; 9], load: bool) {
        use self::gates::and;
        let addr3 = load_address3([addr[0], addr[1], addr[2]]);
        let addr6 = [addr[3], addr[4], addr[5], addr[6], addr[7], addr[8]];
        self.ram64s[0].set(input, addr6, and(addr3[0], load));
        self.ram64s[1].set(input, addr6, and(addr3[1], load));
        self.ram64s[2].set(input, addr6, and(addr3[2], load));
        self.ram64s[3].set(input, addr6, and(addr3[3], load));
        self.ram64s[4].set(input, addr6, and(addr3[4], load));
        self.ram64s[5].set(input, addr6, and(addr3[5], load));
        self.ram64s[6].set(input, addr6, and(addr3[6], load));
        self.ram64s[7].set(input, addr6, and(addr3[7], load));
    }
    fn get(&mut self, addr: [bool; 9]) -> [bool; 16] {
        use self::gates::or16;
        let addr3 = load_address3([addr[0], addr[1], addr[2]]);
        let addr6 = [addr[3], addr[4], addr[5], addr[6], addr[7], addr[8]];
        let a = or16(
            and16way(self.ram64s[0].get(addr6), addr3[0]),
            and16way(self.ram64s[1].get(addr6), addr3[1]),
        );
        let a = or16(a, and16way(self.ram64s[2].get(addr6), addr3[2]));
        let a = or16(a, and16way(self.ram64s[3].get(addr6), addr3[3]));
        let a = or16(a, and16way(self.ram64s[4].get(addr6), addr3[4]));
        let a = or16(a, and16way(self.ram64s[5].get(addr6), addr3[5]));
        let a = or16(a, and16way(self.ram64s[6].get(addr6), addr3[6]));
        or16(a, and16way(self.ram64s[7].get(addr6), addr3[7]))
    }
}

struct RAM4K {
    ram512s: [RAM512; 8],
}
impl RAM4K {
    fn new() -> Self {
        RAM4K {
            ram512s: [
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
            ],
        }
    }

    fn set(&mut self, input: [bool; 16], addr: [bool; 12], load: bool) {
        use self::gates::and;
        let addr3 = load_address3([addr[0], addr[1], addr[2]]);
        let addr9 = [
            addr[3], addr[4], addr[5], addr[6], addr[7], addr[8], addr[9], addr[10], addr[11],
        ];
        self.ram512s[0].set(input, addr9, and(addr3[0], load));
        self.ram512s[1].set(input, addr9, and(addr3[1], load));
        self.ram512s[2].set(input, addr9, and(addr3[2], load));
        self.ram512s[3].set(input, addr9, and(addr3[3], load));
        self.ram512s[4].set(input, addr9, and(addr3[4], load));
        self.ram512s[5].set(input, addr9, and(addr3[5], load));
        self.ram512s[6].set(input, addr9, and(addr3[6], load));
        self.ram512s[7].set(input, addr9, and(addr3[7], load));
    }
    fn get(&mut self, addr: [bool; 12]) -> [bool; 16] {
        use self::gates::or16;
        let addr3 = load_address3([addr[0], addr[1], addr[2]]);
        let addr9 = [
            addr[3], addr[4], addr[5], addr[6], addr[7], addr[8], addr[9], addr[10], addr[11],
        ];
        let a = or16(
            and16way(self.ram512s[0].get(addr9), addr3[0]),
            and16way(self.ram512s[1].get(addr9), addr3[1]),
        );
        let a = or16(a, and16way(self.ram512s[2].get(addr9), addr3[2]));
        let a = or16(a, and16way(self.ram512s[3].get(addr9), addr3[3]));
        let a = or16(a, and16way(self.ram512s[4].get(addr9), addr3[4]));
        let a = or16(a, and16way(self.ram512s[5].get(addr9), addr3[5]));
        let a = or16(a, and16way(self.ram512s[6].get(addr9), addr3[6]));
        or16(a, and16way(self.ram512s[7].get(addr9), addr3[7]))
    }
}

struct RAM32K {
    ram4ks: [RAM4K; 8],
}
impl RAM32K {
    fn new() -> Self {
        RAM32K {
            ram4ks: [
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
            ],
        }
    }
    fn set(&mut self, input: [bool; 16], addr: [bool; 15], load: bool) {
        use self::gates::and;
        let addr3 = load_address3([addr[0], addr[1], addr[2]]);
        let addr12 = [
            addr[3], addr[4], addr[5], addr[6], addr[7], addr[8], addr[9], addr[10], addr[11],
            addr[12], addr[13], addr[14],
        ];
        self.ram4ks[0].set(input, addr12, and(addr3[0], load));
        self.ram4ks[1].set(input, addr12, and(addr3[1], load));
        self.ram4ks[2].set(input, addr12, and(addr3[2], load));
        self.ram4ks[3].set(input, addr12, and(addr3[3], load));
        self.ram4ks[4].set(input, addr12, and(addr3[4], load));
        self.ram4ks[5].set(input, addr12, and(addr3[5], load));
        self.ram4ks[6].set(input, addr12, and(addr3[6], load));
        self.ram4ks[7].set(input, addr12, and(addr3[7], load));
    }
    fn get(&mut self, addr: [bool; 15]) -> [bool; 16] {
        use self::gates::or16;
        let addr3 = load_address3([addr[0], addr[1], addr[2]]);
        let addr12 = [
            addr[3], addr[4], addr[5], addr[6], addr[7], addr[8], addr[9], addr[10], addr[11],
            addr[12], addr[13], addr[14],
        ];
        let a = or16(
            and16way(self.ram4ks[0].get(addr12), addr3[0]),
            and16way(self.ram4ks[1].get(addr12), addr3[1]),
        );
        let a = or16(a, and16way(self.ram4ks[2].get(addr12), addr3[2]));
        let a = or16(a, and16way(self.ram4ks[3].get(addr12), addr3[3]));
        let a = or16(a, and16way(self.ram4ks[4].get(addr12), addr3[4]));
        let a = or16(a, and16way(self.ram4ks[5].get(addr12), addr3[5]));
        let a = or16(a, and16way(self.ram4ks[6].get(addr12), addr3[6]));
        or16(a, and16way(self.ram4ks[7].get(addr12), addr3[7]))
    }
}

pub struct Memory {
    ram: RAM32K,
}

impl Memory {
    pub fn new() -> Self {
        Memory { ram: RAM32K::new() }
    }

    pub fn access(&mut self, input: [bool; 16], load: bool, address: [bool; 15]) -> [bool; 16] {
        let out = self.ram.get(address);
        self.ram.set(input, address, load);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use tools;

    fn bit_16_to_15(x: [bool; 16]) -> [bool; 15] {
        let mut res = [false; 15];
        for i in 0..15 {
            res[i] = x[i];
        }
        res
    }

    #[test]
    fn memory_test() {
        let child = thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let t = tools::read_test_data("tests/05/Memory.cmp").unwrap();
                let mut iter = t[1..].iter().map(|t| {
                    let input = tools::convert16(t[1].parse::<i16>().unwrap());
                    let load = t[2] == "1";
                    let address = tools::convert16_str(&("0".to_owned() + &t[3]));
                    let out = tools::convert16(t[4].parse::<i16>().unwrap());

                    (input, load, bit_16_to_15(address), out)
                });
                let mut memory = Memory::new();

                while let Some((input, load, address, t_out)) = iter.next() {
                    let out = memory.access(input, load, address);
                    assert_eq!(out, t_out);
                }
            })
            .unwrap();
        child.join().unwrap();
    }
}
