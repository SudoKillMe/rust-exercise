fn main () {
    // let p: u64 = 4_294_967_299;

    // let g: u64 = 8;

    // let private_key: u64 = 4_294_967_296;

    let g: u64 = 8;
    let private_key: u64 = 7;
    let p: u64 = 11;

    // let res = modular_pow(g, private_key, p);
    let n = 100;
    let res = prime(n);
    println!("{:?}", res);
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

fn prime (n: u64) -> Vec<u64> {
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