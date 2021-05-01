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
pub fn arrays(){
    //one dimensional array
    let mut x:[i32;3] =[1,2,3];

    println!("{:?}",x);
    x[2] = 45;
    //multi dimensional array
    let  y: [[f64; 3]; 2] = [
        [2.0,3.0,34.0],
        [3.0,0.0,0.0]
    ];

    println!("multi dimensional array");
    for i in 0..y.len(){
        for j in 0..y[i].len(){
            print!("{} \t",y[i][j]);
        }
        println!();
    }

}
