fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let b = [60, 70, 80, 90, 100];

    for element in b {
        println!("the value is: {}", element);
    }

    println!("{:?}", b);

    for number in (1..=10).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!");
}
