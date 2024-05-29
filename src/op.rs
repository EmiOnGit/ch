pub fn skip_if_neq_nn(x: u16, nn: u16) -> u16 {
    assert!(x < 16);
    assert!(nn < 256);
    0x4000 + (x << 8) + nn
}
pub fn skip_if_eq_nn(x: u16, nn: u16) -> u16 {
    assert!(x < 16);
    assert!(nn < 256);
    0x3000 + (x << 8) + nn
}
pub fn add_xy(x: u16, y: u16) -> u16 {
    assert!(x < 16);
    assert!(y < 16);
    0x8004 + (x << 8) + (y << 4)
}
pub fn add_nn(x: u16, nn: u16) -> u16 {
    assert!(x < 16);
    assert!(nn < 256);
    0x7000 + (x << 8) + nn
}
pub fn set_nn(x: u16, nn: u16) -> u16 {
    assert!(x < 16);
    assert!(nn < 256);
    0x6000 + (x << 8) + nn
}
pub fn draw(x: u16, y: u16) -> u16 {
    assert!(x < 16);
    assert!(y < 16);
    0xd005 + (x << 8) + (y << 4)
}
pub fn sub_xy(x: u16, y: u16) -> u16 {
    assert!(x < 16);
    assert!(y < 16);
    0x8005 + (x << 8) + (y << 4)
}
