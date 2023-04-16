fn main() {
    println!("{:?}", factors(93_819_012_551))
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut factors: Vec<u64> = Vec::new();
    for i in 2..=n {
        while num % i == 0 {
            num /= i;
            factors.push(i);
        }
        if num == 1 {break}
    }
    factors
}