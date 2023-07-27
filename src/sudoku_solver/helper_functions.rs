
pub(crate) fn block_to_board(b: usize) -> (usize, usize){
    let r_conv = b / 3;
    let c_conv = b - r_conv*3;
    let row = r_conv*3;
    let col = c_conv*3;
    return (row, col);
}

#[allow(dead_code)]
pub(crate) fn board_to_block(r: usize, c: usize) -> usize {
    let r_conv = r / 3;
    let c_conv = c / 3;
    return 3*r_conv + c_conv;
}
pub(crate) fn encode_to_decimal(v: i16) -> i32{
    let mut check: i32 = 9;
    loop {
        if check <= 0 {
            break;
        }
        if v == 1<<check-1{
            return check;
        }
        check -= 1;
    }
    return check;
}
