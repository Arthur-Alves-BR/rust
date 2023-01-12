fn main() {
    let text = "abc";
    let reverse = reverse_str(text);
    println!("{}", reverse);
}

fn reverse_str(input: &str) -> String {
    let reverse = String::from(input);
    reverse.chars().rev().collect()
}