#![allow(dead_code)]


pub fn option(){
    let x= 1.0;
    let y =0.0;

    let result:Option<f64> = if y != 0.0 { Some(x/y) }else { None };
    println!("{:?}",result);
    //using match statement to get the actual result

    match result{
        Some(x) => println!("{}",x),
        None=>print!("can not divide {} by {}",x,y)
    }
    // also you can use let of while to check for the condition

    if let Some(x) = result {println!(" {}",x)}
}