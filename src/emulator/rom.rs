use computer::memory::MemoryTrait;

pub struct ROM32K<T> {
    memory: T,
}

impl<T: MemoryTrait> ROM32K<T> {
    pub fn new(prog: &Vec<[bool; 16]>) -> Self {
        let mut memory = T::new();
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
