fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    let mut v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    take_elemet_ownership(v1[1]);
    take_vec_ownership(&mut v1);
    println!("{:?}", v1);
}

fn take_elemet_ownership(element: i32) {
    println!("does not take : {}", element);
}

fn take_vec_ownership(v: &mut Vec<i32>) {
    v[0] = 22;
    println!("{:?}", v);
}
