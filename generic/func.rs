use std::fmt::Display;
fn main(){
    genfunc(10 as u8);
    genfunc(7 as u8);
}

fn genfunc<T:Display>(t:T){
    println!("Inside Generic Function");
    println!("{}", t);
}
