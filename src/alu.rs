use gates;

fn half_adder(a: bool, b: bool) -> (bool, bool) {
    let carry = gates::and(a, b);
    let sum = gates::xor(a, b);
    (carry, sum)
}

fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (carry1, sum) = half_adder(a, b);
    let (carry2, sum) = half_adder(sum, c);
    let carry = gates::or(carry2, carry1);
    (carry, sum)
}

fn add16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut result = [false; 16];
    let (carry, sum) = half_adder(a[0], b[0]);
    result[0] = sum;
    let (carry, sum) = full_adder(a[1], b[1], carry);
    result[1] = sum;
    let (carry, sum) = full_adder(a[2], b[2], carry);
    result[2] = sum;
    let (carry, sum) = full_adder(a[3], b[3], carry);
    result[3] = sum;
    let (carry, sum) = full_adder(a[4], b[4], carry);
    result[4] = sum;
    let (carry, sum) = full_adder(a[5], b[5], carry);
    result[5] = sum;
    let (carry, sum) = full_adder(a[6], b[6], carry);
    result[6] = sum;
    let (carry, sum) = full_adder(a[7], b[7], carry);
    result[7] = sum;
    let (carry, sum) = full_adder(a[8], b[8], carry);
    result[8] = sum;
    let (carry, sum) = full_adder(a[9], b[9], carry);
    result[9] = sum;
    let (carry, sum) = full_adder(a[10], b[10], carry);
    result[10] = sum;
    let (carry, sum) = full_adder(a[11], b[11], carry);
    result[11] = sum;
    let (carry, sum) = full_adder(a[12], b[12], carry);
    result[12] = sum;
    let (carry, sum) = full_adder(a[13], b[13], carry);
    result[13] = sum;
    let (carry, sum) = full_adder(a[14], b[14], carry);
    result[14] = sum;
    let (_, sum) = full_adder(a[15], b[15], carry);
    result[15] = sum;
    result
}

fn inc16(i: [bool; 16]) -> [bool; 16] {
    let one = [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    add16(i, one)
}

fn if_zero(x: [bool; 16], zx: bool) -> [bool; 16] {
    let zx = gates::not(zx);
    gates::and16(
        x,
        [
            zx, zx, zx, zx, zx, zx, zx, zx, zx, zx, zx, zx, zx, zx, zx, zx,
        ],
    )
}

fn if_not(x: [bool; 16], nx: bool) -> [bool; 16] {
    let not_x = gates::and16(
        gates::not16(x),
        [
            nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx,
        ],
    );

    let nx = gates::not(nx);
    let x = gates::and16(
        x,
        [
            nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx, nx,
        ],
    );
    gates::or16(x, not_x)
}

pub fn alu(
    x: [bool; 16],
    y: [bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> ([bool; 16], bool, bool) {
    let x = if_zero(x, zx);
    let x = if_not(x, nx);
    let y = if_zero(y, zy);
    let y = if_not(y, ny);

    let add = add16(x, y);
    let and = gates::and16(x, y);

    let add = gates::and16(add, [f, f, f, f, f, f, f, f, f, f, f, f, f, f, f, f]);
    let nf = gates::not(f);
    let and = gates::and16(
        and,
        [
            nf, nf, nf, nf, nf, nf, nf, nf, nf, nf, nf, nf, nf, nf, nf, nf,
        ],
    );

    let out = gates::or16(add, and);

    let out = if_not(out, no);
    let zr = gates::not(gates::or16way(out));
    (out, zr, out[15])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert16(x: u16) -> [bool; 16] {
        let mut result = [false; 16];
        for i in 0..16 {
            result[i] = (1 << i) & x != 0;
        }
        result
    }

    #[test]
    fn alu_test() {
        let cases = [
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                0,
                1,
                0,
                1,
                0,
                0b0000000000000000,
                1,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                1,
                1,
                1,
                1,
                0b0000000000000001,
                0,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                1,
                0,
                1,
                0,
                0b1111111111111111,
                0,
                1,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                1,
                1,
                0,
                0,
                0b0000000000000000,
                1,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                0,
                0,
                0,
                0b1111111111111111,
                0,
                1,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                1,
                1,
                0,
                1,
                0b1111111111111111,
                0,
                1,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                0,
                0,
                1,
                0b0000000000000000,
                1,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                1,
                1,
                1,
                1,
                0b0000000000000000,
                1,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                0,
                1,
                1,
                0b0000000000000001,
                0,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                1,
                1,
                1,
                1,
                1,
                0b0000000000000001,
                0,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                1,
                1,
                1,
                0b0000000000000000,
                1,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                1,
                1,
                1,
                0,
                0b1111111111111111,
                0,
                1,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                0,
                1,
                0,
                0b1111111111111110,
                0,
                1,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                0,
                0,
                1,
                0,
                0b1111111111111111,
                0,
                1,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                1,
                0,
                0,
                1,
                1,
                0b0000000000000001,
                0,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                0,
                1,
                1,
                1,
                0b1111111111111111,
                0,
                1,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                0,
                0,
                0,
                0,
                0b0000000000000000,
                1,
                0,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                1,
                0,
                1,
                0,
                1,
                0b1111111111111111,
                0,
                1,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                1,
                0,
                1,
                0,
                1,
                0,
                0b0000000000000000,
                1,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                1,
                1,
                1,
                1,
                1,
                1,
                0b0000000000000001,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                1,
                1,
                1,
                0,
                1,
                0,
                0b1111111111111111,
                0,
                1,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                0,
                1,
                1,
                0,
                0,
                0b0000000000010001,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                1,
                1,
                0,
                0,
                0,
                0,
                0b0000000000000011,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                0,
                1,
                1,
                0,
                1,
                0b1111111111101110,
                0,
                1,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                1,
                1,
                0,
                0,
                0,
                1,
                0b1111111111111100,
                0,
                1,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                0,
                1,
                1,
                1,
                1,
                0b1111111111101111,
                0,
                1,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                1,
                1,
                0,
                0,
                1,
                1,
                0b1111111111111101,
                0,
                1,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                1,
                1,
                1,
                1,
                1,
                0b0000000000010010,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                1,
                1,
                0,
                1,
                1,
                1,
                0b0000000000000100,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                0,
                1,
                1,
                1,
                0,
                0b0000000000010000,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                1,
                1,
                0,
                0,
                1,
                0,
                0b0000000000000010,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                0,
                0,
                0,
                1,
                0,
                0b0000000000010100,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                1,
                0,
                0,
                1,
                1,
                0b0000000000001110,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                0,
                0,
                1,
                1,
                1,
                0b1111111111110010,
                0,
                1,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                0,
                0,
                0,
                0,
                0,
                0b0000000000000001,
                0,
                0,
            ),
            (
                0b0000000000010001,
                0b0000000000000011,
                0,
                1,
                0,
                1,
                0,
                1,
                0b0000000000010011,
                0,
                0,
            ),
        ];

        for &(x, y, zx, nx, zy, ny, f, no, out, zr, ng) in cases.iter() {
            let x = convert16(x);
            let y = convert16(y);
            let zx = zx == 1;
            let nx = nx == 1;
            let zy = zy == 1;
            let ny = ny == 1;
            let f = f == 1;
            let no = no == 1;
            let out = convert16(out);
            let zr = zr == 1;
            let ng = ng == 1;
            assert_eq!(
                alu(x, y, zx, nx, zy, ny, f, no),
                (out, zr, ng),
                "{:?}",
                (x, y, zx, nx, zy, ny, f, no)
            );
        }
    }
}
