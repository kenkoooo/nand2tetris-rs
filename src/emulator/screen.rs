use computer::memory::MemoryTrait;
use tools;

const HIGHT: usize = 256;
const WIDTH: usize = 512;

pub struct Screen {
    pub memory: [[bool; WIDTH]; HIGHT],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            memory: [[false; WIDTH]; HIGHT],
        }
    }

    pub fn update<T: MemoryTrait>(&mut self, memory: &mut T) -> Vec<(usize, usize)> {
        let mut updated_pixels = vec![];
        for r in 0..HIGHT {
            for c in 0..(WIDTH / 16) {
                let address = (16384 + r * 32 + c) as u16;
                let screen_address = tools::convert_address(address);
                let screen_word = memory.access([false; 16], false, screen_address);
                for i in 0..16 {
                    if self.memory[r][c * 16 + i] != screen_word[i] {
                        self.memory[r][c * 16 + i] = screen_word[i];
                        updated_pixels.push((r, c * 16 + i));
                    }
                }
            }
        }
        updated_pixels
    }
}
