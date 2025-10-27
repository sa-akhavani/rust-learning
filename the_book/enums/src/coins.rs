#[derive(Debug)]
enum UsState {
    Alabama,
    Massachusetts,
    California,
    Maine,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let my_penny: Coin = Coin::Penny;
    println!("{}", value_in_cents(my_penny));
    let my_mass_quarter = Coin::Quarter(UsState::Massachusetts);
    println!("{}", value_in_cents(my_mass_quarter));
}

fn value_in_cents(coin: Coin) -> u16 {
    match coin {
        Coin::Penny => return 1,
        Coin::Nickel => return 5,
        Coin::Dime => return 10,
        Coin::Quarter(state) => {
            println!("THIS IS A {:?} Quarter!", state);
            25
        }
    }
}
