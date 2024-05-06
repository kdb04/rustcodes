fn main(){
    match is_even(5){
        Some((data))=>{
            if data == true{
                println!("Even");
            }
        },
        None =>{
            println!("Odd");
        }
    }
}

fn is_even(no:i32)->Option<bool>{
    if no%2==0{
        Some(true)
    }
    else{
        None
    }
}
