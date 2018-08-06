
pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        return None;
    }
    let mut primes = Vec::new();
    let mut i = 1;
    loop {
        if is_prime_middle(i) {
            primes.push(i);
            let len: u32 = primes.len() as u32;
            if len == n {
                return Some(i);
            }
        }
        i = i + 1;
    }
}

// 最低效的判断素数的方式
fn is_prime_low (number: u32) -> bool {
    if number == 1 {
        return false;
    }
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}


fn is_prime_middle (number: u32) -> bool {
    if number == 1 {
        return false;
    }
    let inter = (number as f32).sqrt();
    let mut i = 2;
    while i <= inter as u32 {
        if number % i == 0 {
            return false
        }
        if i == 2 {
            i = i + 1;
        } else {
            i = i + 2;
        }
    }
    true
}
