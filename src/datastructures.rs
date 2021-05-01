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
pub fn vectors(){
    let mut x= Vec::new();
    x.push(7);
    x.push(0);
    x.push(5);

    println!("{:?}",x);

    // note the type of number put in index must be of the memory of machine you are UseSingleDictionary
    //for example usize is the appropriate type you must use

    //accessing the element of the vector  using loop_statement

    for i in &x{println!("{}",i)}

    //you can access also the element of the vector by using  index

    //deleting the element of the vector

    // as indicated by the editor this pop function return option so we have to apply match to get the actual item value

    let removed = x.pop();

    match removed{
        Some(x)=> println!("{}",x),
        None => print!("the vector is input")
    }

    //what will happen if we want to access the index which is not in the vector
    // and use get method provided by vector  datastructures
    let index:usize = 9;

    match x.get(index){
        Some(r)=>println!("value at x[{}] = {}",index,r),
        None =>println!("the index doesn't exist")
    }


}
//if you want to apply the slices data structure
//use the slice when you want to pass array as parameter to fn and also apply mut to chang e actual value
fn apply_slice(slices: &[i32]){

    println!("slice one = {:?}",slices);


}
fn apply_slice_mut(slices2: &mut [i32]){
    println!("slice two = {:?}",slices2);
    slices2[1] = 34;
    println!("after changing the valuable at index 1 in mutable slice a={:?}",slices2);
}
pub fn slices(){
    let mut a =[1,2,3,4,5,6];
    // number which goes into parameter in the indexes
    //check that the next parameter we have passed  a which means that we have passed all the element of the arrau
    apply_slice(&a[1..4],);
    apply_slice_mut(&mut a);
}