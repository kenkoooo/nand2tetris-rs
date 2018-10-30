pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn and(a: bool, b: bool) -> bool {
    let c = nand(a, b);
    not(c)
}

pub fn or(a: bool, b: bool) -> bool {
    let a = not(a);
    let b = not(b);
    nand(a, b)
}

pub fn xor(a: bool, b: bool) -> bool {
    or(and(not(a), b), and(a, not(b)))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    let a = and(a, not(sel));
    let b = and(b, sel);
    or(a, b)
}

pub fn dmux(input: bool, sel: bool) -> (bool, bool) {
    let a = and(input, not(sel));
    let b = and(input, sel);
    (a, b)
}

pub fn not16(input: [bool; 16]) -> [bool; 16] {
    let mut result = [false; 16];
    result[0] = not(input[0]);
    result[1] = not(input[1]);
    result[2] = not(input[2]);
    result[3] = not(input[3]);
    result[4] = not(input[4]);
    result[5] = not(input[5]);
    result[6] = not(input[6]);
    result[7] = not(input[7]);
    result[8] = not(input[8]);
    result[9] = not(input[9]);
    result[10] = not(input[10]);
    result[11] = not(input[11]);
    result[12] = not(input[12]);
    result[13] = not(input[13]);
    result[14] = not(input[14]);
    result[15] = not(input[15]);
    result
}

pub fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut result = [false; 16];
    result[0] = and(a[0], b[0]);
    result[1] = and(a[1], b[1]);
    result[2] = and(a[2], b[2]);
    result[3] = and(a[3], b[3]);
    result[4] = and(a[4], b[4]);
    result[5] = and(a[5], b[5]);
    result[6] = and(a[6], b[6]);
    result[7] = and(a[7], b[7]);
    result[8] = and(a[8], b[8]);
    result[9] = and(a[9], b[9]);
    result[10] = and(a[10], b[10]);
    result[11] = and(a[11], b[11]);
    result[12] = and(a[12], b[12]);
    result[13] = and(a[13], b[13]);
    result[14] = and(a[14], b[14]);
    result[15] = and(a[15], b[15]);
    result
}

pub fn or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut result = [false; 16];
    result[0] = or(a[0], b[0]);
    result[1] = or(a[1], b[1]);
    result[2] = or(a[2], b[2]);
    result[3] = or(a[3], b[3]);
    result[4] = or(a[4], b[4]);
    result[5] = or(a[5], b[5]);
    result[6] = or(a[6], b[6]);
    result[7] = or(a[7], b[7]);
    result[8] = or(a[8], b[8]);
    result[9] = or(a[9], b[9]);
    result[10] = or(a[10], b[10]);
    result[11] = or(a[11], b[11]);
    result[12] = or(a[12], b[12]);
    result[13] = or(a[13], b[13]);
    result[14] = or(a[14], b[14]);
    result[15] = or(a[15], b[15]);
    result
}

pub fn mux16(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    let mut result = [false; 16];
    result[0] = mux(a[0], b[0], sel);
    result[1] = mux(a[1], b[1], sel);
    result[2] = mux(a[2], b[2], sel);
    result[3] = mux(a[3], b[3], sel);
    result[4] = mux(a[4], b[4], sel);
    result[5] = mux(a[5], b[5], sel);
    result[6] = mux(a[6], b[6], sel);
    result[7] = mux(a[7], b[7], sel);
    result[8] = mux(a[8], b[8], sel);
    result[9] = mux(a[9], b[9], sel);
    result[10] = mux(a[10], b[10], sel);
    result[11] = mux(a[11], b[11], sel);
    result[12] = mux(a[12], b[12], sel);
    result[13] = mux(a[13], b[13], sel);
    result[14] = mux(a[14], b[14], sel);
    result[15] = mux(a[15], b[15], sel);
    result
}

pub fn or8way(input: [bool; 8]) -> bool {
    let a1 = or(input[0], input[1]);
    let a2 = or(a1, input[2]);
    let a3 = or(a2, input[3]);
    let a4 = or(a3, input[4]);
    let a5 = or(a4, input[5]);
    let a6 = or(a5, input[6]);
    or(a6, input[7])
}

pub fn mux4way16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    sel: [bool; 2],
) -> [bool; 16] {
    let x = mux16(a, b, sel[0]);
    let y = mux16(c, d, sel[0]);
    mux16(x, y, sel[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert16(x: u16) -> [bool; 16] {
        let mut result = [false; 16];
        for i in 0..16 {
            result[i] = (x & (1 << i)) != 0;
        }
        result
    }

    fn convert8(x: u8) -> [bool; 8] {
        let mut result = [false; 8];
        for i in 0..8 {
            result[i] = (x & (1 << i)) != 0;
        }
        result
    }

    #[test]
    fn not_test() {
        let t = [(true, false), (false, true)];
        for &(input, out) in t.iter() {
            assert_eq!(not(input), out);
        }
    }

    #[test]
    fn and_test() {
        let cases = [
            (true, false, false),
            (false, false, false),
            (false, true, false),
            (true, true, true),
        ];
        for &(a, b, out) in cases.iter() {
            assert_eq!(and(a, b), out);
        }
    }

    #[test]
    fn or_test() {
        let cases = [
            (true, true, true),
            (true, false, true),
            (false, true, true),
            (false, false, false),
        ];
        for &(a, b, out) in cases.iter() {
            assert_eq!(or(a, b), out);
        }
    }

    #[test]
    fn xor_test() {
        let cases = [
            (true, true, false),
            (true, false, true),
            (false, true, true),
            (false, false, false),
        ];
        for &(a, b, out) in cases.iter() {
            assert_eq!(xor(a, b), out);
        }
    }

    #[test]
    fn mux_test() {
        let ab = [(true, true), (false, true), (true, false), (false, false)];
        let sel = [true, false];
        for &(a, b) in ab.iter() {
            for &sel in sel.iter() {
                let out = if sel { b } else { a };
                assert_eq!(mux(a, b, sel), out);
            }
        }
    }

    #[test]
    fn dmux_test() {
        let inputs = [(true, true), (false, true), (true, false), (false, false)];
        for &(input, sel) in inputs.iter() {
            if sel {
                assert_eq!(dmux(input, sel), (false, input));
            } else {
                assert_eq!(dmux(input, sel), (input, false));
            }
        }
    }

    #[test]
    fn not16_test() {
        let cases = [
            (0b0000000000000000, 0b1111111111111111),
            (0b1111111111111111, 0b0000000000000000),
            (0b1010101010101010, 0b0101010101010101),
            (0b0011110011000011, 0b1100001100111100),
            (0b0001001000110100, 0b1110110111001011),
        ];
        for &(input, output) in cases.iter() {
            let input = convert16(input);
            let output = convert16(output);
            assert_eq!(not16(input), output);
        }
    }

    #[test]
    fn and16_test() {
        let cases = [
            (0b0000000000000000, 0b0000000000000000, 0b0000000000000000),
            (0b0000000000000000, 0b1111111111111111, 0b0000000000000000),
            (0b1111111111111111, 0b1111111111111111, 0b1111111111111111),
            (0b1010101010101010, 0b0101010101010101, 0b0000000000000000),
            (0b0011110011000011, 0b0000111111110000, 0b0000110011000000),
            (0b0001001000110100, 0b1001100001110110, 0b0001000000110100),
        ];
        for &(a, b, out) in cases.iter() {
            let a = convert16(a);
            let b = convert16(b);
            let out = convert16(out);
            assert_eq!(and16(a, b), out);
        }
    }

    #[test]
    fn or16_test() {
        let cases = [
            (0b0000000000000000, 0b0000000000000000, 0b0000000000000000),
            (0b0000000000000000, 0b1111111111111111, 0b1111111111111111),
            (0b1111111111111111, 0b1111111111111111, 0b1111111111111111),
            (0b1010101010101010, 0b0101010101010101, 0b1111111111111111),
            (0b0011110011000011, 0b0000111111110000, 0b0011111111110011),
            (0b0001001000110100, 0b1001100001110110, 0b1001101001110110),
        ];
        for &(a, b, out) in cases.iter() {
            let a = convert16(a);
            let b = convert16(b);
            let out = convert16(out);
            assert_eq!(or16(a, b), out);
        }
    }

    #[test]
    fn mux16_test() {
        let cases = [
            (
                0b0000000000000000,
                0b0000000000000000,
                0,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                1,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0001001000110100,
                0,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0001001000110100,
                1,
                0b0001001000110100,
            ),
            (
                0b1001100001110110,
                0b0000000000000000,
                0,
                0b1001100001110110,
            ),
            (
                0b1001100001110110,
                0b0000000000000000,
                1,
                0b0000000000000000,
            ),
            (
                0b1010101010101010,
                0b0101010101010101,
                0,
                0b1010101010101010,
            ),
            (
                0b1010101010101010,
                0b0101010101010101,
                1,
                0b0101010101010101,
            ),
        ];
        for &(a, b, sel, out) in cases.iter() {
            let a = convert16(a);
            let b = convert16(b);
            let sel = sel == 1;
            let out = convert16(out);
            assert_eq!(mux16(a, b, sel), out);
        }
    }

    #[test]
    fn or8way_test() {
        let cases = [
            (0b00000000, 0),
            (0b11111111, 1),
            (0b00010000, 1),
            (0b00000001, 1),
            (0b00100110, 1),
        ];

        for &(input, out) in cases.iter() {
            let input = convert8(input);
            let out = out == 1;
            assert_eq!(or8way(input), out);
        }
    }

    #[test]
    fn mux4way16_test() {
        let cases = [
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b00,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b01,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b10,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b11,
                0b0000000000000000,
            ),
            (
                0b0001001000110100,
                0b1001100001110110,
                0b1010101010101010,
                0b0101010101010101,
                0b00,
                0b0001001000110100,
            ),
            (
                0b0001001000110100,
                0b1001100001110110,
                0b1010101010101010,
                0b0101010101010101,
                0b01,
                0b1001100001110110,
            ),
            (
                0b0001001000110100,
                0b1001100001110110,
                0b1010101010101010,
                0b0101010101010101,
                0b10,
                0b1010101010101010,
            ),
            (
                0b0001001000110100,
                0b1001100001110110,
                0b1010101010101010,
                0b0101010101010101,
                0b11,
                0b0101010101010101,
            ),
        ];

        for &(a, b, c, d, sel, out) in cases.iter() {
            let a = convert16(a);
            let b = convert16(b);
            let c = convert16(c);
            let d = convert16(d);
            let sel = [(sel & 1) == 1, (sel & 2) != 0];
            let out = convert16(out);
            assert_eq!(mux4way16(a, b, c, d, sel), out);
        }
    }
}
