use std::fs::File;
fn main(){
    let f = File::open("main.jpg").expect("File not able to open");
    println!("End of main");
}
