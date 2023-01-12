fn main() {
    let number = 153;
    println!("{}", is_armstrong_number(number));
}

pub fn is_armstrong_number(num: u32) -> bool {
    let string = num.to_string();
    
    let mut sum = 0;
    for value in  string.chars() {
        sum += value.to_digit(10).expect("Invalid number").pow(string.len() as u32);
    }

    if sum == num {
        return true
    }
    false
}