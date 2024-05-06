fn main(){
    let n1 = "Hello World";
    println!("{}", n1.len());
    let n2 = &n1[1..7];
    println!("{}", n2);
}
