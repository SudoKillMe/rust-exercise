pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res_vec = vec![];
    let mut fac = 2..;
    while n > 1 {
        let x = fac.next().unwrap();

        while n % x == 0 {
            n = n / x;
            res_vec.push(x);
        }
    }
    res_vec
}