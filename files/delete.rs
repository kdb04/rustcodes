use std::fs;
fn main(){
    fs::remove_file("delete.txt").expect("Could not remove file");
    println!("File removed succesfully");
}
