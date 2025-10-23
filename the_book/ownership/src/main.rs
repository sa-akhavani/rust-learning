fn main() {
    // string literals
    let s0 = "Hello, World!";
    let s: &str = "Hello World";
    println!("{s0},{s}");

    // String type
    let mut s1: String = String::from("Hi World!");
    s1.push_str(" kek");
    println!("{s1}");
    let s2 = s1; // move
    // println!("{s1}"); // Doesn't work because s1 is out of scope after s2 = s1
    //
    let s3 = s2.clone(); // deep copy
    println!("{s2}, {s3}");

    let s4 = String::from("Happy");

    takes_ownership(s4);
    // s4 is no longer usable after here since it's dropped

    let i = 60;
    make_copy(i);
    // i is still usable since it was copied
    println!("after i: {i}");

    let s5 = give_ownership();
    let s6 = String::from("my_str");
    let s7 = take_and_give_back(s6);
    // s6 no longer usable
    println!("{s5}, {s7}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn give_ownership() -> String {
    let new_str = String::from("newborn string");
    new_str
}

fn take_and_give_back(source_string: String) -> String {
    source_string
}
