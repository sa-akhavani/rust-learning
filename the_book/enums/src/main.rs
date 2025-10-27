enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let some_number = Some(34);
    let some_char = Some('A');
    let some_string = Some(String::from("KEK"));

    let absent_number: Option<u32> = None;

    println!(
        "{:?}, {:?}, {:?}, {:?}",
        some_number, some_char, some_string, absent_number
    );

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // throws error!
    // let absent_behx: i8;
    // let sm = x + absent_behx;

    let x = Some(54);
    println!("{:?}", plus_one(x));
    let y = None;
    println!("{:?}", plus_one(y));

    let dice_roll = 9;
    let my_string: &str = match dice_roll {
        1 => "sadkek",
        6 => "kek!",
        _ => "hmm",
    };
    println!("{}", my_string);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(val) => Some(val + 1),
        None => {
            println!("need a number to be able to do this!");
            None
        }
    }
}
