fn main() {
    let words = ["ball", "foot", "airline", "house"];
    let mut ordered_words = ["", "", "", ""];
    
    for (i, w) in words.iter().rev().enumerate() {
        ordered_words[i] = w;
    }

    println!("Before ordenation: ");
    print_array_values(&words);

    println!("\nAfter ordenation: ");
    print_array_values(&ordered_words);
}

fn print_array_values(arr: &[&str]) {
    for value in arr {
        println!("{value}");
    }
}