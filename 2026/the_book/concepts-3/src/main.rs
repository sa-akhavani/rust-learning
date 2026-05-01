fn main() {
    const CURRENT_YEAR: u32 = 2026;
    println!("Year is {}", CURRENT_YEAR);
    let x: i32 = -10_000;
    println!("X is {}", x);
    let x: &str = "Sage Zard";
    println!("X is {}", x);
    let res = data_types("Gang");
    println!("{}", res);
    loops();
}

fn data_types(passed: &str) -> u32 {
    // // Scaler Data Types
    // Integer
    // Floating Point Numbers
    // Boolean
    // Character

    // // Compound Data Types
    // Tuples
    let tup: (&str, i32) = ("Behx", 12);
    let (channel, number) = tup;
    println!("{channel} {number}");
    println!("{}", tup.0);

    // Arrays (fixed length)
    let error_codes = [200, 404, 500];
    println!("{}", passed);
    error_codes[1] // Return value of the function. Last line.
}

fn loops() {
    let mut counter: u32 = 0;
    let final_result: u32 = loop {
        if counter == 10 {
            break counter;
        }
        counter += 1
    };
    println!("{}", final_result);

    while counter != 15 {
        println!("{}", counter);
        counter += 1;
    }

    let arr = [100, 200, 300, 400];

    for element in arr.iter() {
        println!("{}", element);
    }

    for number in 5..10 {
        println!("{}", number);
    }
}
