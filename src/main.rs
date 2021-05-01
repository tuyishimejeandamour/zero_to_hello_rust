use std::io;
mod control;
mod sh;

fn scanner() -> u32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
   return match trimmed.parse::<u32>() {
        Ok(i) => i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

fn main() {
    println!("-----------------------------------------\n");
    println!("+              menu                      \n");
    println!("-----------------------------------------\n");
    println!(" 1. control statement");
    println!(" 2. static memory and dynamic memory");
    let mut number = scanner();
    if number == 1 {
        if given == 1 {
            control::if_statement()
        } else if given ==2 {
            control::loop_statement();
        }else if given == 3 {
            control::switch_statement();
        }
    }else if number == 2 {
        sh::stack_heap();
    }

}