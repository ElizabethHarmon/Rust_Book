use std::io;

fn main() {
    println!("Do you want to want to...");
    println!("1) Convert Celsius to Farenheit, or 2) Convert Farenheit to Celsius?");
    println!("Please input 1 or 2: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u8 = input.trim().parse().expect("Please type a number!");

    if input == 1 {
        let choice = "Celsius";
        let x = read_input(choice);
        let output = c_to_f(x);
        println!("{}C = {}F", x, output);
    } else if input == 2 {
        let choice = "Farenheit";
        let x = read_input(choice);
        let output = f_to_c(x);
        println!("{}F = {}C", x, output);
    } else {
        println!("Please enter 1 or 2");
    }
}

fn read_input(choice :&str) -> f64 {
    
    
    loop {
        println!("Please input a {} value: ", choice);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f64 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => continue,
        };

        break input;
    }
}

fn c_to_f(number: f64) -> f64 {
        number * 9.0 / 5.0 + 32.0
    }

fn f_to_c(number: f64) -> f64 {
        (number - 32.0) * 5.0 / 9.0
}
