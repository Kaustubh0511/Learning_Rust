use rand::Rng;
use std::io;
macro_rules! line {
    () => {
        println!("--------------------------------------------------");
    };
}

pub fn functions() {
    let mut a = String::new();

    println!("Enter a number : ");
    io::stdin().read_line(&mut a).expect("Nothing Found");

    let b: i32 = a.trim().parse().expect("Nothing in string!!");

    let mut cnt: i32 = 0;
    loop {
        cnt += 1;
        let c = rand::thread_rng().gen_range(20..=30);
        if cnt <= 5 {
            if b == c {
                println!("Yeaaaa you guess it!!!!");
                break;
            } else {
                println!("Opps, try again!!!!");
            }
            println!("{}", c);
        } else {
            break;
        }
    }
    line!();
    println!("{}", a);
    println!("{}", b);
    line!();
}
