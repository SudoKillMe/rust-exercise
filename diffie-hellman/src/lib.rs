pub fn private_key(p: u64) -> u64 {
    let primes = primes(p);
    primes[0]
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}


fn modular_pow (base: u64, exponent: u64, modular: u64) -> u64 {
    if modular == 1 {
        return 0;
    }
    let mut result = 1;
    let mut e_prime = 0;
    while e_prime < exponent {
        result = (result * base) % modular;
        e_prime += 1;
    }
    result
}

fn primes (n: u64) -> Vec<u64> {
    let mut list: Vec<bool> = Vec::with_capacity((n as usize));
    list.push(false);
    list.push(false);
    for i in 2..(n as usize) {
        list.push(true);
    }
    for i in 2..(n as usize) {
         if list[i] {
             let mut j = i+i;
             while j < (n as usize) {
                 list[j] = false;
                 j += i;
             }
         }
    }

    let mut result: Vec<u64> = vec![]; 
    for (i, &v) in list.iter().enumerate() {
        if v {
            result.push(i as u64);
        }
    }
    result
}