pub fn find() -> Option<u32> {
    // a = mn b = m*m - n*n / 2   c= m*m + n*n / 2
    for i in 1..100 {
        let m = i;
        for j in i..100 {
            let n = j;
            let a = m * n;
            let b = (n*n - m*m) / 2;
            let c = (m*m + n*n) / 2;
            if a + b + c == 1000 {
                return Some(a * b * c);
            } 
        }
    }
    None
}
