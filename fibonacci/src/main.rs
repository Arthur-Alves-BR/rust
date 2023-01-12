use std::io;

fn main() {
    println!("Qual o tamanho desejado?");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    
    let len: usize= input_line.trim().parse().expect("Input is not an integer");
    let mut fib: Vec<i64> = vec![0; len];

    for i in 0..fib.len() {
        match i {
            0 | 1 => fib[i] = 1,
            _others => fib[i] = fib[i-1] + fib[i-2],
        };
        print!("{} ", fib[i]);
    }
    println!();
}
