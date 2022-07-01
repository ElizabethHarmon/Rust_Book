fn main() {
    let s1 = String::from("hello world");
    // // let (s2, len) = calculate_length(s1);
    // // println!("The length of '{}' is {}", s2, len);

    
    let word = first_word(&s1);
    println!("{}", word);
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
    
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
