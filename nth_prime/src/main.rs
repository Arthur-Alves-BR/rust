fn main() {
    println!("{}", nth(0));
    println!("{}", nth(1));
    println!("{}", nth(2));
    println!("{}", nth(3));
    println!("{}", nth(4));
    println!("{}", nth(5));
    println!("{}", nth(10_000));
}

pub fn nth(n: u32) -> u32 {
    let mut num = 3;
    let mut prime_index = 1;
    if n == 0 {
        return 2
    }
    loop {
        if prime_index == n {
            break;
        }
   
        num += 2;
        let mut is_prime = true;
   
        for i in 2..num {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            prime_index += 1;
        }
    }
    num
}
