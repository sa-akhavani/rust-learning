fn main() {
    let mut s = String::from("Borrow me pls");
    append_to_string(&mut s);
    println!("{s}");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

    let s = String::from("Firstword might be long");
    let word = first_word_idx(&s);
    println!("String Length is: {word}");
    println!("{}", &s[..word]);

    let sliced_word = first_word(&s);
    println!("{}", sliced_word);
}

fn first_word_idx(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn append_to_string(s: &mut String) {
    s.push_str("!");
}
