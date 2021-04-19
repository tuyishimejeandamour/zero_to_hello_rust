#![allow(dead_code)]

pub fn if_statement() {
    let x = 2;
    if x == 2
    {
        println!("number is 2")
    } else { println!("number is not 2") }

    println!("if returning");
    let day = if x == 2 { "sunny" } else { "good" };
    println!("printed {}", day);
}

pub fn loop_statement() {
    let mut x = 2;
    while x < 1000 {
        print!("{} x 2 =", { x });
        x *= 2;
        println!("{}", x);
    }
    println!("implementation using loop");

    loop {
        print!("{} x 2 =", { x });
        x *= 2;
        println!("{}", x);
        if x > 1000 { break; }
    }
    println!("using for loop");

    for x in 1..11 {
        println!("{}", x);
    }
    for (pos, y) in (30..41).enumerate() {
        println!("{}:{}", pos, y);
    }
}

pub fn match_statement() {
    let country_code = 77;

    let country = match country_code {
        77 => "rwanda",
        1...88 => "country in state",
        _ => "invalid"
    };
    println!("count is :{}", country)
}