// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("This is another function...")
// }

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x)
// }

fn main() {
    another_function(5, 6);

    let z = seven();
    println!("The value of z is: {}", z);

    let a = plus_one(10);
    println!("The number is {}", a);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// function with return
fn seven() -> i32 {
    7
}

// function with argument and return
fn plus_one(a: i32) -> i32 {
    a + 1
}
