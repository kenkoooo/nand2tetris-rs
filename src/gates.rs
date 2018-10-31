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

pub fn or16way(input: [bool; 16]) -> bool {
    let a = or8way([
        input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7],
    ]);
    let b = or8way([
        input[8], input[9], input[10], input[11], input[12], input[13], input[14], input[15],
    ]);
    or(a, b)
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

pub fn mux8way16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    e: [bool; 16],
    f: [bool; 16],
    g: [bool; 16],
    h: [bool; 16],
    sel: [bool; 3],
) -> [bool; 16] {
    let x = mux4way16(a, b, c, d, [sel[0], sel[1]]);
    let y = mux4way16(e, f, g, h, [sel[0], sel[1]]);
    mux16(x, y, sel[2])
}

pub fn dmux4way(i: bool, sel: [bool; 2]) -> (bool, bool, bool, bool) {
    let a = and(i, and(not(sel[0]), not(sel[1])));
    let b = and(i, and(sel[0], not(sel[1])));
    let c = and(i, and(not(sel[0]), sel[1]));
    let d = and(i, and(sel[0], sel[1]));
    (a, b, c, d)
}

pub fn dmux8way(i: bool, sel: [bool; 3]) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    let (a, b, c, d) = dmux4way(i, [sel[0], sel[1]]);
    let (e, f, g, h) = dmux4way(i, [sel[0], sel[1]]);
    let a = and(a, not(sel[2]));
    let b = and(b, not(sel[2]));
    let c = and(c, not(sel[2]));
    let d = and(d, not(sel[2]));
    let e = and(e, sel[2]);
    let f = and(f, sel[2]);
    let g = and(g, sel[2]);
    let h = and(h, sel[2]);
    (a, b, c, d, e, f, g, h)
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

    #[test]
    fn mux8way16_test() {
        let cases = [
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b000,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b001,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b010,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b011,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b100,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b101,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b110,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b111,
                0b0000000000000000,
            ),
            (
                0b0001001000110100,
                0b0010001101000101,
                0b0011010001010110,
                0b0100010101100111,
                0b0101011001111000,
                0b0110011110001001,
                0b0111100010011010,
                0b1000100110101011,
                0b000,
                0b0001001000110100,
            ),
            (
                0b0001001000110100,
                0b0010001101000101,
                0b0011010001010110,
                0b0100010101100111,
                0b0101011001111000,
                0b0110011110001001,
                0b0111100010011010,
                0b1000100110101011,
                0b001,
                0b0010001101000101,
            ),
            (
                0b0001001000110100,
                0b0010001101000101,
                0b0011010001010110,
                0b0100010101100111,
                0b0101011001111000,
                0b0110011110001001,
                0b0111100010011010,
                0b1000100110101011,
                0b010,
                0b0011010001010110,
            ),
            (
                0b0001001000110100,
                0b0010001101000101,
                0b0011010001010110,
                0b0100010101100111,
                0b0101011001111000,
                0b0110011110001001,
                0b0111100010011010,
                0b1000100110101011,
                0b011,
                0b0100010101100111,
            ),
            (
                0b0001001000110100,
                0b0010001101000101,
                0b0011010001010110,
                0b0100010101100111,
                0b0101011001111000,
                0b0110011110001001,
                0b0111100010011010,
                0b1000100110101011,
                0b100,
                0b0101011001111000,
            ),
            (
                0b0001001000110100,
                0b0010001101000101,
                0b0011010001010110,
                0b0100010101100111,
                0b0101011001111000,
                0b0110011110001001,
                0b0111100010011010,
                0b1000100110101011,
                0b101,
                0b0110011110001001,
            ),
            (
                0b0001001000110100,
                0b0010001101000101,
                0b0011010001010110,
                0b0100010101100111,
                0b0101011001111000,
                0b0110011110001001,
                0b0111100010011010,
                0b1000100110101011,
                0b110,
                0b0111100010011010,
            ),
            (
                0b0001001000110100,
                0b0010001101000101,
                0b0011010001010110,
                0b0100010101100111,
                0b0101011001111000,
                0b0110011110001001,
                0b0111100010011010,
                0b1000100110101011,
                0b111,
                0b1000100110101011,
            ),
        ];
        for &(a, b, c, d, e, f, g, h, sel, out) in cases.iter() {
            let a = convert16(a);
            let b = convert16(b);
            let c = convert16(c);
            let d = convert16(d);
            let e = convert16(e);
            let f = convert16(f);
            let g = convert16(g);
            let h = convert16(h);
            let sel = [(sel & 1) != 0, (sel & 2) != 0, (sel & 4) != 0];
            let out = convert16(out);
            assert_eq!(mux8way16(a, b, c, d, e, f, g, h, sel), out);
        }
    }

    #[test]
    fn dmux4way_test() {
        let cases = [
            (0, 0b00, 0, 0, 0, 0),
            (0, 0b01, 0, 0, 0, 0),
            (0, 0b10, 0, 0, 0, 0),
            (0, 0b11, 0, 0, 0, 0),
            (1, 0b00, 1, 0, 0, 0),
            (1, 0b01, 0, 1, 0, 0),
            (1, 0b10, 0, 0, 1, 0),
            (1, 0b11, 0, 0, 0, 1),
        ];
        for &(i, sel, a, b, c, d) in cases.iter() {
            let i = i == 1;
            let a = a == 1;
            let b = b == 1;
            let c = c == 1;
            let d = d == 1;
            let sel = [(sel & 1) != 0, (sel & 2) != 0];
            assert_eq!(dmux4way(i, sel), (a, b, c, d));
        }
    }

    #[test]
    fn dmux8way_test() {
        let cases = [
            (0, 0b000, 0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0b001, 0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0b010, 0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0b011, 0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0b100, 0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0b101, 0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0b110, 0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0b111, 0, 0, 0, 0, 0, 0, 0, 0),
            (1, 0b000, 1, 0, 0, 0, 0, 0, 0, 0),
            (1, 0b001, 0, 1, 0, 0, 0, 0, 0, 0),
            (1, 0b010, 0, 0, 1, 0, 0, 0, 0, 0),
            (1, 0b011, 0, 0, 0, 1, 0, 0, 0, 0),
            (1, 0b100, 0, 0, 0, 0, 1, 0, 0, 0),
            (1, 0b101, 0, 0, 0, 0, 0, 1, 0, 0),
            (1, 0b110, 0, 0, 0, 0, 0, 0, 1, 0),
            (1, 0b111, 0, 0, 0, 0, 0, 0, 0, 1),
        ];
        for &(i, sel, a, b, c, d, e, f, g, h) in cases.iter() {
            let i = i == 1;
            let sel = [(sel & 1) != 0, (sel & 2) != 0, (sel & 4) != 0];
            let a = a == 1;
            let b = b == 1;
            let c = c == 1;
            let d = d == 1;
            let e = e == 1;
            let f = f == 1;
            let g = g == 1;
            let h = h == 1;
            assert_eq!(dmux8way(i, sel), (a, b, c, d, e, f, g, h));
        }
    }
}
