fn main() {
    let words = ["ball", "foot", "airline", "house"];
    let mut ordered_words = words.clone();

    loop {
        let mut finished = true;
        for index in 0..ordered_words.len()-1 {
            let actual_index_char_value = get_string_char_by_index(ordered_words[index], 0) as u32;
            let next_index_char_value = get_string_char_by_index(ordered_words[index+1], 0) as u32;
            if next_index_char_value < actual_index_char_value {
                let aux_for_swap = ordered_words[index];
                ordered_words[index] = ordered_words[index+1];
                ordered_words[index+1] = aux_for_swap;
                finished = false;
            }
        }
        if finished {break;}
    }

    println!("Before ordenation: ");
    print_array_values(&words);

    println!("\nAfter ordenation: ");
    print_array_values(&ordered_words);
}

fn get_string_char_by_index(string: &str, index: usize) -> char {
    string.chars().nth(index).unwrap()
}

fn print_array_values(arr: &[&str]) {
    for value in arr {
        println!("{value}");
    }
}