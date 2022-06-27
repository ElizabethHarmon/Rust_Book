fn main() {
    // const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "     ";
    let spaces = spaces.len();

    println!("The number of spaces is {}", spaces);

    // floating point
    let a = 2.0;
    let b: f32 = 3.0;

    // calculations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let division = 56.7 / 32.2;
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false;

    // char
    let c = 'z';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("Third value of tup is: {}", tup.2);

    // array
    let d = [1, 2, 3, 4, 5];

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]; // etc

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let first = a[0];
}
