fn main(){
    let result = is_even(7).unwrap();
    println!("{}", result);
}

fn is_even(no:i32)->Result<bool,String>{
    if no%2==0{
         return Ok(true);
    }
    else{
        return Err("Not even".to_string());
    }
}
