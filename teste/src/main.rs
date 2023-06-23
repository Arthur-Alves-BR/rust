use std::collections::HashMap;

fn main() {
    println!("{}", roman_to_int("III".to_string()));
    println!("{}", roman_to_int("LVIII".to_string()));
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}

pub fn roman_to_int(s: String) -> i32 {
    let symbol_value_map = HashMap::from([
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000),
    ]);             
    let mut sum = 0;
    let values: Vec<i32> = s.chars().map(|symbol| *symbol_value_map.get(&symbol.to_string().as_str()).unwrap()).collect();
    for (index, &current_value) in values.iter().enumerate() {
        let next_value = *values.iter().nth(index+1).unwrap_or(&0);
        sum += if next_value > current_value { -current_value } else { current_value };
    }
    sum
}