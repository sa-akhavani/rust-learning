#[derive(Debug)]
enum UsState {
    Alabama,
    Massachusetts,
    California,
    Maine,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Massachusetts => year >= 1800,
            UsState::California => year >= 1825,
            UsState::Maine => year >= 2010,
        }
    }
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
    println!("{}", value_in_cents(&my_penny));
    let my_mass_quarter = Coin::Quarter(UsState::Massachusetts);
    println!("{}", value_in_cents(&my_mass_quarter));
    // if let Coin::Quarter(state) = my_mass_quarter {
    //     println!("This coin is a Quarter!");
    //     println!("it's {} enough", state.existed_in(1000));
    // }
    println!("{:?}", describe_state_quarter(&my_mass_quarter));

    let my_maine_quarter = Coin::Quarter(UsState::Maine);
    println!("{:?}", describe_state_quarter(&my_maine_quarter));
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    /*
    match coin {
        Coin::Quarter(state) => {
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old!"))
            } else {
                Some(format!("{state:?} is pretty young!"))
            }
        }
        _ => None,
    }
    */
    /*
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old!"))
        } else {
            Some(format!("{state:?} is pretty young!"))
        }
    } else {
        None
    }
    */
    /*
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old!"))
    } else {
        Some(format!("{state:?} is pretty young!"))
    }
    */
    let Coin::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old!"))
    } else {
        Some(format!("{state:?} is pretty young!"))
    }
}

fn value_in_cents(coin: &Coin) -> u16 {
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
