#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    let coin = Coin::Quarter(State::Connecticut);
    let value = value_in_cents(coin);
    println!("The value of this coin is {} cents", value);

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);

    let some_u8_value = Some(4);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // prefer x.map(|i| i + 1)
    }
}
