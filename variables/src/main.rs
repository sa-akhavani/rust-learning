const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("X in the inner loop is {}", x);
    }
    println!("X is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    let tup: (i32, f64, u8) = (-300, 2.0, 3);
    let (_x, y, _z) = tup;
    println!("{y}, {}", tup.0);

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _my_arr: [i32; 5] = [1, 2, 3, 4, 5];
    let my_arr_2 = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{}", my_arr_2[4]);

    println!("{}", my_function(11));

    let condition = true;
    let z = if condition { 1 } else { 2 };
    println!("{z}");

    let mut counter: i32 = 0;
    let result = loop {
        println!("Looping!");
        if counter == 10 {
            break 1;
        }
        counter += 1;
    };
    println!("result is {}", result);

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count: {count}");

    // for
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for element in arr {
        println!("{element}");
    }

    for i in (1..=4).rev() {
        println!("{i}");
    }
    println!("liftof!");
}

fn my_function(x: i32) -> bool {
    if x > 10 {
        return true;
    }
    false
}
