use std::io;
macro_rules! line {
    () => {
        println!("------------------------------------------------");
    };
}

pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn sub(a: i32, b: i32) -> i32 {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

pub fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}

pub fn functions() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter first number : ");
    io::stdin().read_line(&mut a).expect("No input");
    println!("Enter Second number : ");
    io::stdin().read_line(&mut b).expect("No input");

    let c: i32 = a.trim().parse().expect("Nothing");
    let d: i32 = b.trim().parse().expect("Nothing");
    line!();
    let res_1 = add(c, d);
    println!("{}", res_1);
    let res_2 = sub(c, d);
    println!("{}", res_2);
    let res_3 = mul(c, d);
    println!("{}", res_3);
}
