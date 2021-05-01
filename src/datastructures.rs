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

pub fn strings(){
    // this form of string is not mutable and there is no much you can do with it
    //you can not access the characters in it by applying indexing to a[0]
    //you can not change it
    let a:&'static str = "hello here we go";
    println!("{}",a);

    // inorder to have many operation on string you have to use head allocated

    let  mut a = String::new();

    //let take example like you want to  put all character fom a-z in one string;

    let mut x = 'a' as u8;
    while x <= 'z' as u8{
        a.push(x as char);
        x += 1;
    }

    println!("{:?}",a);

    //concatenation

    // in order to  concatenate the strings you can use + symbol
    // one must be &str in order to concatenate

    //to change string to &str  you use & to
    let n  = &a;
    //you can use to_string() also to convert str to string;
    let mut s = "hello there".to_string();
    s = a+ &s;
    println!("{}",s);

}
fn sum_divide(a:i32,b:i32) ->(i32,i64){
    (a+b, (a / b) as i64)
}
pub fn tuples(){
    // tuple data structure is used when you want  output two  different datatype element
    // act as array but can hold two different datatype

    let (a ,b) = (12,34);
    //accessing the element when there is destruction
    println!("{}",a);

    //now let's check if the  tuple is in one variable name

    let sp = sum_divide(8,3);
    println!("{0}+{1} = {2} and {0}+{1} = {3}",8,3,sp.0,sp.1)
}