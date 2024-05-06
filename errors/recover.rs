use std::fs::File;
fn main(){
    let f = File::open("f1.jpg");  //Doesn't exist
    match f{
        Ok(f)=>{
            println!("File: {:?}", f);
        },
        Err(e)=>{
            println!("File not found: {:?}", e);
        }
    }
}
