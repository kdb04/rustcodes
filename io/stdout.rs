use std::io::Write;
fn main(){
    let b1 = std::io::stdout().write("A".as_bytes()).unwrap();
    let b2 = std::io::stdout().write("B".as_bytes()).unwrap();
    std::io::stdout().write(format!("Bytes written {}", (b1+b2)).as_bytes()).unwrap();
}
