// use std::io;
// mod control;
// mod sh;
mod datastructures;

// fn scanner() -> u32 {
//     let mut input_text = String::new();
//     io::stdin()
//         .read_line(&mut input_text)
//         .expect("failed to read from stdin");
//
//     let trimmed = input_text.trim();
//    return match trimmed.parse::<u32>() {
//         Ok(i) => i,
//         Err(..) => 9,
//
//     };
// }

fn main() {
    // println!("-----------------------------------------\n");
    // println!("+              menu                      \n");
    // println!("-----------------------------------------\n");
    // println!(" 1. control statement");
    // println!(" 2. static memory and dynamic memory");
    // let  number = scanner();
    // let mut given =0;
    // if number == 1 {
    //     given = scanner();
    //     if given == 1 {
    //         control::if_statement()
    //     } else if given ==2 {
    //         control::loop_statement();
    //     }else if given == 3 {
    //         control::match_statement();
    //     }
    // }else if number == 2 {
    //     sh::stack_heap();
    // }

    //datastructures::option();
    //datastructures::arrays();
    //datastructures::vectors();
    //datastructures::slices();
    //datastructures::strings();
    datastructures::tuples();
}