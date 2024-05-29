use op::{add_nn, add_xy, draw, set_nn, skip_if_eq_nn, skip_if_neq_nn};

use crate::op::sub_xy;

mod op;

const CLEARSCREEN: u16 = 0x00e0;
const HEADER_LEN: usize = 2;
const LETTERS: [[u8; 5]; 10] = [H, I, V, I, K, I, H1, H2, H3, H4];
const H: [u8; 5] = [
    0b1001_0000,
    0b1001_0000,
    0b1111_0000,
    0b1001_0000,
    0b1001_0000,
];
const I: [u8; 5] = [
    0b0010_0000,
    0b0010_0000,
    0b0010_0000,
    0b0010_0000,
    0b0010_0000,
];
const V: [u8; 5] = [
    0b1000_1000,
    0b1000_1000,
    0b0101_0000,
    0b0111_0000,
    0b0010_0000,
];
const K: [u8; 5] = [
    0b1001_0000,
    0b1010_0000,
    0b1100_0000,
    0b1010_0000,
    0b1011_0000,
];
const H1: [u8; 5] = [
    0b0000_1100,
    0b0001_0010,
    0b0010_0001,
    0b0010_0000,
    0b0001_0000,
];
const H2: [u8; 5] = [
    0b0011_0000,
    0b0100_1000,
    0b1000_0100,
    0b0000_0100,
    0b0000_1000,
];
const H3: [u8; 5] = [
    0b0000_1000,
    0b0000_0100,
    0b0000_0010,
    0b0000_0001,
    0b0000_0000,
];
const H4: [u8; 5] = [
    0b0001_0000,
    0b0010_0000,
    0b0100_0000,
    0b1000_0000,
    0b0000_0000,
];
const HEADER: [u16; 2] = [
    CLEARSCREEN,
    // jump to first instr after letters
    0x1200 + HEADER_LEN as u16 * 2 + LETTERS.len() as u16 * 6,
];

const fn letter_addr(i: usize) -> u16 {
    0x200 + HEADER_LEN as u16 * 2 + i as u16 * 6
}
const fn body_addr() -> u16 {
    0x0200 + HEADER_LEN as u16 * 2 + LETTERS.len() as u16 * 6
}
fn main() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER);
    for letter in LETTERS.into_iter() {
        let v: Vec<u16> = letter
            .chunks(2)
            .map(|b| {
                if b.len() == 2 {
                    (b[0] as u16) << 8 | b[1] as u16
                } else {
                    (b[0] as u16) << 8
                }
            })
            .collect();
        buf.extend_from_slice(&v);
    }
    buf.extend_from_slice(&[
        ////
        //// 0 loop var
        //// 1 sprite offset
        //// 2 const x sub for heart
        //// 3 const sprite letter offset
        //// 4 sprite x pos
        //// 9 sprite y pos
        ////
        set_nn(1, 4),
        set_nn(2, 16),
        set_nn(3, 6),
        set_nn(9, 7),
        set_nn(5, 2),
        set_nn(4, 4),
        // set r[i] = letter[0]
        0xa000 + letter_addr(0),
        //--- loop1 ---
        // set r[8] = delay timer
        0xf807,
        skip_if_eq_nn(8, 00),
        0x1000 + body_addr() + 14,
        // draw at r[4], r[9] the sprite at i
        draw(4, 9),
        // double distance for hi_viky
        skip_if_neq_nn(0, 1),
        // set r[4] = r[4] + r[1]
        add_xy(4, 1),
        // increase distance for heart
        skip_if_neq_nn(0, 5),
        // set r[1] = r[1] + nn
        add_nn(1, 4),
        skip_if_neq_nn(0, 5),
        sub_xy(9, 5),
        skip_if_neq_nn(0, 7),
        add_nn(9, 5),
        skip_if_neq_nn(0, 7),
        sub_xy(4, 2),
        add_xy(4, 1),
        // set r[i] = r[i] + r[3]
        0xf31e,
        // set r[0] = r[0] + 1
        add_nn(0, 1),
        // set delay to r[5]
        0xf215,
        // skip if r[0] == 10
        skip_if_eq_nn(0, 10),
        // jump to --- loop1 ---
        0x1000 + body_addr() + 14,
        // --- loop1 end ---

        // wait for key
        0xf00a,
    ]);
    let bytes: Vec<u8> = buf
        .into_iter()
        .map(|buf| buf.to_be_bytes())
        .flatten()
        .collect();
    std::fs::write("hello.ch", &bytes).unwrap();
}
