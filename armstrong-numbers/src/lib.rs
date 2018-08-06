pub fn is_armstrong_number(num: u32) -> bool {
    let mut res = 0;
    let str = num.to_string();
    let len = str.len();
    for c in str.chars() {
        res = res + c.to_digit(10).unwrap().pow(len as u32)
    }
    res == num
}
